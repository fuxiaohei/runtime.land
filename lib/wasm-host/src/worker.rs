use crate::hostcall::HttpHandlerPre;
use anyhow::Result;
use axum::body::Body;
use tracing::debug;
use wasmtime::{
    component::{Component, Linker},
    Engine, Store, UpdateDeadline,
};

/// Worker is used to run wasm component
#[derive(Clone)]
pub struct Worker {
    path: String,
    engine: Engine,
    instance_pre: HttpHandlerPre<crate::context::Context>,
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Worker").field("path", &self.path).finish()
    }
}

impl Worker {
    // from_binary is used to create worker from bytes
    pub async fn from_binary(bytes: &[u8], path: Option<String>) -> Result<Self> {
        let engine = crate::engine::get("default")?;
        let component = Component::from_binary(&engine, bytes)?;
        debug!("Load wasm component from binary, size:{}", bytes.len());

        // create linker
        let mut linker: Linker<crate::context::Context> = Linker::new(&engine);

        // init wasi context
        wasmtime_wasi::add_to_linker_async(&mut linker)?;

        // init http_service
        crate::hostcall::HttpService::add_to_linker(&mut linker, crate::context::Context::host_ctx)
            .expect("add http_service failed");

        // create instance-pre
        let instance_pre = linker.instantiate_pre(&component)?;
        Ok(Self {
            path: path.unwrap_or("binary".to_string()),
            engine,
            instance_pre: HttpHandlerPre::new(instance_pre)?,
        })
    }

    async fn from_aot(path: String) -> Result<Self> {
        let engine = crate::engine::get("default")?;
        let bytes = std::fs::read(&path)?;
        debug!(
            "Load wasm component from AOT file: {}, size: {}",
            path,
            bytes.len()
        );

        let component = unsafe { Component::deserialize(&engine, bytes)? };

        // create linker
        let mut linker: Linker<crate::context::Context> = Linker::new(&engine);
        // init wasi context
        wasmtime_wasi::add_to_linker_async(&mut linker)?;
        // init http_service
        crate::hostcall::HttpService::add_to_linker(&mut linker, crate::context::Context::host_ctx)
            .expect("add http_service failed");

        let instance_pre = linker.instantiate_pre(&component)?;
        Ok(Self {
            path,
            engine,
            instance_pre: HttpHandlerPre::new(instance_pre)?,
        })
    }

    pub fn compile_aot(src: &str, dst: &str) -> Result<()> {
        let engine = super::engine::get("default")?;
        let component = Component::from_file(&engine, src)?;
        let bytes = Component::serialize(&component)?;
        debug!("Write AOT from {} to {}, size: {}", src, dst, bytes.len());
        std::fs::write(dst, bytes)?;
        Ok(())
    }

    /// new a worker from path
    pub async fn new(path: &str, is_aot: bool) -> Result<Self> {
        let binary = std::fs::read(path)?;

        // compile aot wasm
        if is_aot {
            let suffix = format!(".wasm.{}.aot", crate::engine::MODULE_VERSION);
            let aot_path = path.replace(".wasm", &suffix);
            if std::path::Path::new(&aot_path).exists() {
                return Self::from_aot(aot_path).await;
            }
            let path2 = path.to_string();
            std::thread::spawn(move || {
                match Self::compile_aot(&path2, &aot_path) {
                    Ok(_) => debug!("Compile AOT success: {}", &aot_path),
                    Err(e) => debug!("Compile AOT failed: {}", e),
                };
            });
        }
        Self::from_binary(&binary, Some(path.to_string())).await
    }

    /// handle_request is used to handle http request
    pub async fn handle_request(
        &self,
        req: crate::hostcall::Request,
        context: crate::context::Context,
    ) -> Result<(crate::hostcall::Response, Body)> {
        // create store
        let mut store = Store::new(&self.engine, context);
        store.set_epoch_deadline(1);
        store.epoch_deadline_callback(move |store| {
            debug!(
                "epoch_deadline_callback, cost:{:.2?}",
                store.data().elapsed()
            );
            Ok(UpdateDeadline::Yield(1))
        });
        store.limiter(|ctx| &mut ctx.limiter);

        // get exports and call handle_request
        let exports = self.instance_pre.instantiate_async(&mut store).await?;
        let resp = exports
            .land_http_incoming()
            .call_handle_request(&mut store, &req)
            .await?;
        let body_handle = resp.body.unwrap();
        let body = store.data_mut().take_body(body_handle).unwrap();
        debug!("response is ready, body:{}", body_handle);
        // check async task is pending
        let is_pending = exports
            .land_asyncio_context()
            .call_is_pending(&mut store)
            .await?;
        debug!("async task is pending: {}", is_pending);
        if is_pending {
            // let req_id = store.data().req_id();
            // let span = debug_span!("[ASYNC]", req_id);
            tokio::task::spawn(async move {
                let now = tokio::time::Instant::now();
                loop {
                    let res = match exports.land_asyncio_context().call_select(&mut store).await {
                        Ok(res) => res,
                        Err(e) => {
                            debug!("async task pending failed, error: {}", e);
                            return;
                        }
                    };
                    debug!("async task is done, res: {:?}", res);
                    if !res {
                        break;
                    }
                }
                debug!("async tasks all done, cost:{:.2?}", now.elapsed());
                // println!("async task is done, cost:{:.2?}", now.elapsed());
            });
        }
        Ok((resp, body))
    }
}

#[cfg(test)]
mod worker_test {
    use crate::{hostcall::Request, Context, Worker};

    #[tokio::test]
    async fn run_hello_wasm() {
        let test_wasm_file = "../../target/wasm32-wasi/release/hello_wasm.wasm";
        land_wasm_gen::componentize_wasm(&test_wasm_file).expect("componentize wasm failed");

        let worker = Worker::new(test_wasm_file, false)
            .await
            .expect("load worker failed");
        let request = Request {
            method: "GET".to_string(),
            uri: "/".to_string(),
            headers: vec![("X-Request-Id".to_string(), "123456".to_string())],
            body: None,
        };
        let context = Context::default();
        let resp = worker
            .handle_request(request, context)
            .await
            .expect("handle request failed");
        assert_eq!(resp.0.status, 200);
        for (h, v) in resp.0.headers {
            if h == "X-Request-Method" {
                assert_eq!(v, "GET");
            }
        }
        let body_handle = resp.1;
        let body = axum::body::to_bytes(body_handle, 9999).await.unwrap();
        assert_eq!(body, b"Hello Runtime.land!!".to_vec());
    }

    #[tokio::test]
    async fn run_wait_until() {
        let test_wasm_file = "../../target/wasm32-wasi/release/wait_until.wasm";
        land_wasm_gen::componentize_wasm(&test_wasm_file).expect("componentize wasm failed");

        let worker = Worker::new(test_wasm_file, false)
            .await
            .expect("load worker failed");
        let request = Request {
            method: "GET".to_string(),
            uri: "/".to_string(),
            headers: vec![("X-Request-Id".to_string(), "123456".to_string())],
            body: None,
        };
        let context = Context::default();
        let resp = worker
            .handle_request(request, context)
            .await
            .expect("handle request failed");
        assert_eq!(resp.0.status, 200);
        for (h, v) in resp.0.headers {
            if h == "X-Request-Method" {
                assert_eq!(v, "GET");
            }
        }
        let body_handle = resp.1;
        let body = axum::body::to_bytes(body_handle, 9999).await.unwrap();
        assert_eq!(body, b"Hello Runtime.land!!".to_vec());

        // wait until
        tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
    }

    #[tokio::test]
    async fn run_js_hello() {
        let test_js = "../../examples/js-hello/src/index.js";
        let test_wasm_path = "../../target/wasm32-wasi/release/js_hello.wasm";

        let test_wasm_dir = std::path::Path::new(test_wasm_path).parent().unwrap();
        std::fs::create_dir_all(test_wasm_dir).expect("create dir failed");
        land_wasm_gen::componentize_js(test_js, test_wasm_path, None)
            .expect("componentize js failed");

        let worker = Worker::new(test_wasm_path, false)
            .await
            .expect("load worker failed");
        let request = Request {
            method: "GET".to_string(),
            uri: "/".to_string(),
            headers: vec![("X-Request-Id".to_string(), "123456".to_string())],
            body: None,
        };
        let context = Context::default();
        let resp = worker
            .handle_request(request, context)
            .await
            .expect("handle request failed");
        assert_eq!(resp.0.status, 200);
        for (h, v) in resp.0.headers {
            if h == "X-Request-Method" {
                assert_eq!(v, "GET");
            }
        }
        let body_handle = resp.1;
        let body = axum::body::to_bytes(body_handle, 9999).await.unwrap();
        assert_eq!(body, b"Hello, Runtime.land JS SDK".to_vec());
    }
}
