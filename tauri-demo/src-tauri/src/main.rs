// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::Error, path::Path};
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    dock::VmDock,
    Module, NeverType, VmBuilder,
};
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    let wasm_file =
        Path::new("D:\\project\\edge-plugin\\target\\wasm32-wasi\\release\\edge_plugin.wasm");
    let module = Module::from_file(None, wasm_file).unwrap();
    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()
        .unwrap();
    let vm = VmBuilder::new()
        .with_config(config)
        .build::<NeverType>()
        .unwrap()
        .register_module(None, module)
        .unwrap();
    let vm = VmDock::new(vm);
    let message = match vm.run_func("hello", vec![]) {
        Ok(_) => "Run init Ok".to_string(),
        Err(err) => {
            format!("error: {}", err)
        }
    };
    message
}

fn main() -> anyhow::Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
