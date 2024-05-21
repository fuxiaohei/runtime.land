use std::collections::HashMap;

use crate::hostcall::HttpContext;
use axum::body::Body;
use bytesize::ByteSize;
use tracing::debug;
use wasmtime::{component::ResourceTable, ResourceLimiter};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

#[derive(Default)]
pub struct Limiter {
    /// Total memory allocated so far.
    pub memory_allocated: usize,
}

impl ResourceLimiter for Limiter {
    fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        _maximum: Option<usize>,
    ) -> anyhow::Result<bool> {
        // Track the diff in memory allocated over time. As each instance will start with 0 and
        // gradually resize, this will track the total allocations throughout the lifetime of the
        // instance.
        self.memory_allocated += desired - current;
        debug!("Memory: {}", ByteSize(self.memory_allocated as u64),);
        Ok(true)
    }

    fn table_growing(
        &mut self,
        _current: u32,
        _desired: u32,
        _maximum: Option<u32>,
    ) -> anyhow::Result<bool> {
        Ok(true)
    }
}

pub struct Context {
    wasi_ctx: WasiCtx,
    table: ResourceTable,
    http_ctx: HttpContext,
    pub limiter: Limiter,
}

impl WasiView for Context {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi_ctx
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Context {
    pub fn new(envs: Option<HashMap<String, String>>) -> Self {
        let table = ResourceTable::new();
        let mut wasi_ctx_builder = WasiCtxBuilder::new();
        wasi_ctx_builder.inherit_stderr().inherit_stdout();
        if let Some(envs) = envs {
            for (k, v) in envs {
                wasi_ctx_builder.env(k, v);
            }
        }
        Context {
            wasi_ctx: wasi_ctx_builder.build(),
            http_ctx: HttpContext::new(),
            limiter: Limiter::default(),
            table,
        }
    }
    /// get http_ctx
    pub fn http_ctx(&mut self) -> &mut HttpContext {
        &mut self.http_ctx
    }
    /// take body
    pub fn take_body(&mut self, handle: u32) -> Option<Body> {
        self.http_ctx.take_body(handle)
    }
    /// set body
    pub fn set_body(&mut self, handle: u32, body: Body) -> u32 {
        self.http_ctx.set_body(handle, body)
    }
    /// elapsed returns the duration since the request started
    pub fn elapsed(&self) -> tokio::time::Duration {
        self.http_ctx.elapsed()
    }
}
