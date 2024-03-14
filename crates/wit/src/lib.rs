use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::{collections::HashMap, io::Write};
use tracing::debug;
use wit_bindgen_core::{wit_parser::Resolve, Files, WorldGenerator};
use wit_component::ComponentEncoder;

/// GuestGeneratorType is the type of the guest generator.
pub enum GuestGeneratorType {
    Rust,
    TinyGo,
}

impl GuestGeneratorType {
    /// create a new guest generator
    fn create(&self, _gen_exports: HashMap<String, String>) -> Result<Box<dyn WorldGenerator>> {
        /*let mut exports = HashMap::new();
        for (name, content) in gen_exports.iter() {
            exports.insert(
                wit_bindgen_rust::ExportKey::Name(name.to_string()),
                content.to_string(),
            );
        }*/
        match self {
            GuestGeneratorType::Rust => {
                let opts = wit_bindgen_rust::Opts {
                    // exports,
                    rustfmt: true,
                    pub_export_macro: true,
                    ..Default::default()
                };
                let builder = opts.build();
                Ok(builder)
            }
            _ => Err(anyhow!("Unsupport guest generator")),
        }
    }
}

/// generate_guest parse wit file and return world id
pub fn generate_guest(
    wit_dir: &Path,
    world: Option<String>,
    t: GuestGeneratorType,
    gen_exports: HashMap<String, String>,
) -> Result<HashMap<String, String>> {
    let mut generator = t.create(gen_exports)?;

    let mut resolve = Resolve::default();
    let pkg = resolve.push_dir(wit_dir)?.0;

    let mut output_maps = HashMap::new();
    let mut files = Files::default();
    let world = resolve.select_world(pkg, world.as_deref())?;
    generator.generate(&resolve, world, &mut files)?;
    for (name, contents) in files.iter() {
        output_maps.insert(
            name.to_string(),
            String::from_utf8_lossy(contents).to_string(),
        );
    }
    Ok(output_maps)
}

fn find_cmd(cmd: &str) -> Result<PathBuf> {
    let c = match which::which(cmd) {
        Ok(c) => c,
        Err(_) => {
            // find wasm-opt binary in current exe directroy ./wasm-opt-bin/wasm
            let exe_path = std::env::current_exe()?;
            let file = exe_path
                .parent()
                .unwrap()
                .join(format!("{}-bin/{}", cmd, cmd));

            #[cfg(target_os = "windows")]
            let file = file.with_extension("exe");

            if file.exists() {
                return Ok(file);
            }
            return Err(anyhow!("cannot find '{}' binary", cmd));
        }
    };
    Ok(c)
}

/// compile_js compile js file to wasm module
pub fn compile_js(src_path: &str, dst_path: &str, js_engine: Option<String>) -> Result<()> {
    debug!("Compile js file: {}", src_path);
    let cmd = find_cmd("wizer")?;
    let dir = std::path::Path::new(src_path).parent().unwrap();
    let js_engine_file = dir.join("js_engine.wasm");

    // copy js_engine.wasm to path dir
    let js_engine_bytes = if let Some(js_engine) = js_engine {
        std::fs::read(js_engine)?
    } else {
        include_bytes!("../engine/js-engine.wasm").to_vec()
    };
    debug!("Read js_engine_bytes: {}", js_engine_bytes.len());
    std::fs::write(&js_engine_file, js_engine_bytes)?;

    let src_content = std::fs::read(src_path)?;

    // call wizer to compile js to wasm
    // wizer js_engine.wasm -o {path}.wasm --allow-wasi --inherit-stdio=true --inherit-env=true
    let mut child = Command::new(cmd)
        .arg(&js_engine_file)
        .arg("-o")
        .arg(dst_path)
        .arg("--allow-wasi")
        .arg("--inherit-stdio=true")
        .arg("--inherit-env=true")
        .arg("--wasm-bulk-memory=true")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute wizer child process");
    let mut stdin = child.stdin.take().expect("Failed to get stdin");

    std::thread::spawn(move || {
        stdin
            .write_all(src_content.as_slice())
            .expect("Failed to write to stdin");
    });

    let output = child
        .wait_with_output()
        .expect("Failed to wait on wizer child process");
    if !output.status.success() {
        let err = String::from_utf8(output.stderr)?;
        return Err(anyhow!(err));
    }
    // print output
    debug!(
        "Wizer output: \n{}",
        std::str::from_utf8(&output.stdout).unwrap()
    );
    debug!("Wizer success, from {} to {}", src_path, dst_path);
    let _ = std::fs::remove_file(&js_engine_file);
    Ok(())
}

/// encode_component encode wasm file to component
pub fn encode_component(src: &str, dest: &str) -> Result<()> {
    let file_bytes = std::fs::read(src)?;
    let wasi_adapter = include_bytes!("../engine/wasi_snapshot_preview1.reactor.wasm");
    let component = ComponentEncoder::default()
        .module(&file_bytes)
        .expect("Pull custom sections from module")
        .validate(true)
        .adapter("wasi_snapshot_preview1", wasi_adapter)
        .expect("Add adapter to component")
        .encode()
        .expect("Encode component");
    let output = src.replace(".wasm", ".component.wasm");
    std::fs::write(&output, component)?;
    debug!("Convert component success, from {} to {}", src, dest);
    // remove *.component.wasm temp file
    if output != dest {
        std::fs::rename(&output, dest)?;
        let _ = std::fs::remove_file(output);
    }
    Ok(())
}

/// optimize wasm component
pub fn optimize(path: &str) -> Result<Option<String>> {
    let cmd = match find_cmd("wasm-opt") {
        Ok(cmd) => cmd,
        Err(_err) => {
            return Ok(None);
        }
    };
    let target = path.replace(".wasm", ".opt.wasm");
    let child = Command::new(cmd)
        .arg("-O")
        .arg("--strip-debug")
        .arg("-o")
        .arg(&target)
        .arg(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to execute wasm-opt child process");
    let output = child
        .wait_with_output()
        .expect("Failed to wait on wasm-opt child process");
    if !output.status.success() {
        let err = String::from_utf8(output.stderr)?;
        return Err(anyhow::anyhow!(err));
    }
    debug!("Wasm-opt success, from {} to {}", path, target);
    let _ = std::fs::remove_file(path);
    Ok(Some(target))
}
