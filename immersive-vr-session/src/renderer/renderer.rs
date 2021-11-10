use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

pub fn create_webgl_context(xr_mode: bool) -> Result<WebGl2RenderingContext, JsValue> {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    let gl: WebGl2RenderingContext = if xr_mode {
        let gl_attribs = HashMap::new()
            .insert(String::from("xrCompatible"), true);
        let js_gl_attribs = JsValue::from_serde(&gl_attribs).unwrap();

        canvas
            .get_context_with_context_options("webgl2", &js_gl_attribs)?
            .unwrap()
            .dyn_into()?
    } else {
        canvas.get_context("webgl2")?.unwrap().dyn_into()?
    };

    Ok(gl)
}
