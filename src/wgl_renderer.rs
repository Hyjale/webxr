use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Failed to create shader"))?;

    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        return Ok(shader);
    }

    Err(context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")))
}

fn link_program(
    context: &WebGl2RenderingContext,
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

    let vert_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("triangle.vert"),
    )?;

    let frag_shader = compile_shader(
        &gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("triangle.frag"),
    )?;

    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    Ok(gl)
}
