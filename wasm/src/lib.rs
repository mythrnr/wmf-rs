#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};

use wasm_bindgen::prelude::*;

/// Converts WMF binary data to an SVG string.
///
/// # Arguments
///
/// - `buf` - Byte array of a WMF file
///
/// # Returns
///
/// - SVG string (UTF-8)
/// - On failure, returns a JsValue containing error details
///
/// # Example
///
/// ```js
/// import { convertWmf2Svg } from "wmf-wasm";
///
/// // svg is a string containing SVG data
/// const svg = convertWmf2Svg(wmfBytes);
/// ```
#[wasm_bindgen(js_name = convertWmf2Svg)]
pub fn convert_wmf_to_svg(buf: &[u8]) -> Result<String, JsValue> {
    // When the `console_error_panic_hook` feature is enabled, we can call
    // the `set_panic_hook` function at least once during
    // initialization, and then we will get better error messages if
    // our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    set_log_level("info");

    let player = wmf_core::converter::SVGPlayer::new();
    let converter = wmf_core::converter::WMFConverter::new(buf, player);
    let output =
        converter.run().map_err(|err| JsValue::from(err.to_string()))?;

    Ok(String::from_utf8_lossy(&output).to_string())
}

/// Sets the log level (only when the `tracing` feature is enabled).
///
/// # Arguments
///
/// - `level` - e.g. "info", "debug", etc.
///
/// # Example
///
/// ```js
/// import { setLogLevel } from "wmf-wasm";
/// setLogLevel("debug");
/// // Now debug logs will be shown in the browser console (if tracing feature is enabled)
/// ```
#[cfg(feature = "tracing")]
#[wasm_bindgen(js_name = setLogLevel)]
pub fn set_log_level(level: &str) {
    static INITIALIZED: core::sync::atomic::AtomicBool =
        core::sync::atomic::AtomicBool::new(false);

    if !INITIALIZED.load(core::sync::atomic::Ordering::Acquire) {
        INITIALIZED.store(true, core::sync::atomic::Ordering::Release);

        tracing_wasm::set_as_global_default_with_config(
            tracing_wasm::WASMLayerConfigBuilder::new()
                .set_max_level(level.parse().expect("should be parsed"))
                .build(),
        );
    }
}

/// Sets the log level (no-op if `tracing` feature is disabled).
#[cfg(not(feature = "tracing"))]
#[wasm_bindgen(js_name = setLogLevel)]
pub fn set_log_level(_: &str) {
    // NOOP
}
