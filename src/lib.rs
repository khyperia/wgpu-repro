use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log::init().unwrap();
    console_error_panic_hook::set_once();

    let instance = wgpu::Instance::new(Default::default());
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let _ = adapter.get_info();
    let _ = adapter.get_info();
    log::info!("done");

    Ok(())
}
