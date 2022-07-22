#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use std::io::Read;
use std::path::{Path, PathBuf};

use anyhow::Result;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use wasmer_vfs::host_fs::File;
use wasmer_wasi::{WasiState};

// exec expects a temp directory to be created and populated with any inputs
// that the wasm program may expect.
pub async fn exec(wasm_path: impl AsRef<Path>, working_dir: impl AsRef<Path>) -> Result<String> {
    let working_dir = working_dir.as_ref().to_path_buf();
    let stdout_path = working_dir.join("__stdout");
    let mut f = std::fs::File::open(wasm_path)?;
    let mut wasm_bytes: Vec<u8> = Vec::new();
    f.read_to_end(&mut wasm_bytes)?;
    let store = Store::new(&Universal::new(Cranelift::default()).engine());
    let module = Module::new(&store, wasm_bytes)?;
    // TODO (2022-05-10) can't figure out how to make the vfs_memory work
    // so tying stdout to an actual file on the host, and then reading it back
    let stdout_file = std::fs::File::create(&stdout_path)?;
    let stdout = File::new(
        stdout_file,
        PathBuf::try_from("./stdout.host")?,
        true,
        true,
        true,
    );

    let mut wasi_env = WasiState::new("program_name")
        .map_dir(".", &working_dir.as_path())?
        .preopen_dir(&working_dir.as_path())?
        .stdout(Box::new(stdout))
        .finalize()?;

    let import_object = wasi_env.import_object(&module)?;
    let instance = Instance::new(&module, &import_object)?;
    let start = instance.exports.get_function("_start")?;

    start.call(&[])?;

    let content = std::fs::read_to_string(&stdout_path)?;

    Ok(content)
}

#[derive(Serialize)]
struct ExecuteResponse {
    stdout: String,
}

#[tauri::command]
async fn execute(wasm_path: &str) -> Result<ExecuteResponse, String> {
    let working_dir = "/Users/supershabam/wasms";
    let result = exec(wasm_path, &working_dir).await;
    match result {
        Ok(stdout) => Ok(ExecuteResponse { stdout: stdout }),
        Err(err) => Err(format!("{}", err)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
