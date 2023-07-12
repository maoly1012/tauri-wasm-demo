use std::path::Path;

use wasmedge_sdk::{Module, config::{ConfigBuilder, CommonConfigOptions, HostRegistrationConfigOptions}, VmBuilder, NeverType, dock::VmDock};

fn main() {
    let wasm_file = Path::new("..\\wasm32-wasi\\debug\\plugin.wasm");
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
    println!("message:{}",message);
}
