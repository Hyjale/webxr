use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

fn link_program(context: &WebGl2RenderingContext,
                vert_shader: &WebGlShader,
                frag_shader: &WebGlShader,
                ) -> Result<WebGlProgram, String> {

    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Ok(program);
    }

    Err(context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")))
}

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
