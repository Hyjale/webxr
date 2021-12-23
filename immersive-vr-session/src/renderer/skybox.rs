use std::f32::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Skybox {
    depthFunc: u32,
    depthMask: bool,
    uniformName: String
}

impl Skybox {
    pub fn new() -> Self {
        Skybox {
            depthFunc: WebGl2RenderingContext::LEQUAL,
            depthMask: false,
            uniformName: String::from("diffuse")
        }
    }

    fn get_vertex_source() -> &'static str {
        "
        uniform int EYE_INDEX;
        uniform vec4 texCoordScaleOffset[2];
        attribute vec3 POSITION;
        attribute vec2 TEXCOORD_0;
        varying vec2 vTexCoord;

        vec4 vertex_main(mat4 proj, mat4 view, mat4 model) {
            vec4 scaleOffset = texCoordScaleOffset[EYE_INDEX];
            vTexCoord = (TEXCOORD_0 * scaleOffset.xy) + scaleOffset.zw;
            view[3].xyz = vec3(0.0, 0.0, 0.0);
            vec4 out_vec = proj * view * model * vec4(POSITION, 1.0);

            return out_vec.xyww;
        }"
    }

    fn get_fragment_source() -> &'static str {
        "
        uniform sampler2D diffuse;
        varying vec2 vTexCoord;

        vec4 fragment_main() {
            return texture2D(diffuse, vTexCoord);
        }"
    }

    fn create_buffers(&self, gl: &WebGl2RenderingContext) {
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let lat_segments = 40;
        let lon_segments = 40;

        for i in 0..lat_segments {
            let theta = f32::from(i) * PI / f32::from(lat_segments);
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            let idx_offset_a = i * (lon_segments + 1);
            let idx_offset_b = (i+1) * (lon_segments + 1);

            for j in 0..lon_segments {
                let phi = f32::from(j) * 2.0 * PI / f32::from(lon_segments);
                let x = phi.sin() * sin_theta;
                let y = cos_theta;
                let z = -phi.cos() * sin_theta;
                let u = f32::from(j / lon_segments);
                let v = f32::from(i / lat_segments);

                vertices = vec![x, y, z, u, v];

                if i < lat_segments && j < lon_segments {
                    let idx_a = idx_offset_a + j;
                    let idx_b = idx_offset_b + j;
                    indices = vec![idx_a, idx_b, idx_a+1, idx_b, idx_b+1, idx_a+1];
                }
            }
        }

        let vertex_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
        unsafe {
            let src = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &src,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let index_buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&index_buffer));
        unsafe {
            let src = js_sys::Uint16Array::view(&indices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &src,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    fn create_texture(&self, gl: &WebGl2RenderingContext) -> HtmlImageElement {
        let url = "textures/milky-way-4k.png";
        let img = HtmlImageElement::new().unwrap();
        img.set_src(&url);

        img
    }

    fn create_program(&self, gl: &WebGl2RenderingContext) {
        let program = gl.create_program().unwrap();

        let vert_shader = gl.create_shader(WebGl2RenderingContext::VERTEX_SHADER).unwrap();
        gl.attach_shader(&program, &vert_shader);
        gl.shader_source(&vert_shader, Skybox::get_vertex_source());
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(WebGl2RenderingContext::FRAGMENT_SHADER).unwrap();
        gl.attach_shader(&program, &frag_shader);
        gl.shader_source(&frag_shader, Skybox::get_fragment_source());
        gl.compile_shader(&frag_shader);

        gl.bind_attrib_location(&program, 1, "POSITION");
        gl.bind_attrib_location(&program, 4, "TEXCOORD_0");

        gl.link_program(&program);
    }

    // TODO
    fn use_program(&self, gl: &WebGl2RenderingContext) {

    }
}
