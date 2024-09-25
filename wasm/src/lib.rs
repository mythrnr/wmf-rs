use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = convertWmf2Svg)]
pub fn convert_wmf_to_svg(buf: &[u8]) -> Result<String, JsValue> {
    let mut output: Vec<u8> = vec![];
    let out = std::io::BufWriter::new(&mut output);
    let player = wmf_core::converter::SVGPlayer::new(out);
    let converter = wmf_core::converter::WMFConverter::new(buf, player);

    if let Err(err) = converter.run() {
        return Err(err.to_string().into());
    }

    Ok(String::from_utf8_lossy(&output).to_string())
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
