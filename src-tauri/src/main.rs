#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::io::Read;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use serde::Serialize;

use anyhow::anyhow;
use anyhow::Result;
use wasmer::{Instance, Module, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_universal::Universal;
use wasmer_vfs::host_fs::File;
use wasmer_wasi::{Stdout, WasiEnv, WasiFs, WasiState};

// exec expects a temp directory to be created and populated with any inputs
// that the wasm program may expect.
pub async fn exec(working_dir: impl AsRef<Path>) -> Result<String> {
    let working_dir = working_dir.as_ref().to_path_buf();
    let stdout_path = working_dir.join("__stdout");
    let wasm_bytes = include_bytes!("../../applications/hello/target/wasm32-wasi/release/hello.wasm");
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
    println!("{}", content);

    Ok(content)
}

#[derive(Serialize)]
struct ExecuteResponse {
    Stdout: String,
}

#[tauri::command]
async fn execute() -> Result<ExecuteResponse, String> {
  let working_dir = "/Users/supershabam/wasms";
  let result = exec(&working_dir).await;
  match result {
    Ok(stdout) => {
      Ok(ExecuteResponse{
        Stdout: stdout,
      })
    },
    Err(err) => {
      Err(format!("{}", err))
    }
  }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
