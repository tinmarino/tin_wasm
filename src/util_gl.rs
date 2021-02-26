use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::WebAssembly;
//use nalgebra::{Isometry3, Perspective3, Point3, Vector3, Matrix4};

use web_sys::{
    console,
    HtmlImageElement,
    WebGlProgram,
    WebGlShader,
    WebGlTexture,
    WebGlBuffer,
    WebGlUniformLocation,
};
use web_sys::WebGlRenderingContext as GL;


/// Just a single container to keep our arrays
pub struct Buffers {
    pub position: WebGlBuffer,
    pub normal: WebGlBuffer,
    pub texture_coord: WebGlBuffer,
    pub indice: WebGlBuffer,
}


pub struct ProgramInfo {
    // I own the stuff and ciao bambino
    pub program: WebGlProgram,
    pub a_vertex_position: i32,
    pub a_vertex_normal: i32,
    pub a_texture_coord: i32,

    pub u_projection_matrix: WebGlUniformLocation,
    pub u_model_view_matrix: WebGlUniformLocation,
    pub u_normal_matrix: WebGlUniformLocation,
    pub u_sampler: WebGlUniformLocation,
}

/// Matrix 3D (9 comp) -> 4D (16 comp)
/// From: webgl water tut: src/app/store/camera.rs
/// Cannot really fail
//#[allow(dead_code)]
//pub fn mat_to_array(mat: &Matrix4<f32>) -> [f32; 16] {
//    let mut mat4 = [0.; 16];
//    mat4.copy_from_slice(mat4.as_slice());
//    mat4
//}


/// Bufferise
/// size: A GLint specifying the number of components per vertex attribute. Must be 1, 2, 3, or 4.
pub fn buffer_f32_data(gl: &GL, program: &WebGlProgram, data: &[f32], attribute_name: &str, size: usize) -> Result<WebGlBuffer, JsValue> {
    // Get attribute (alias GlSl variable) handle
    let attrib: u32 = gl.get_attrib_location(&program, attribute_name) as u32;

    // Leak
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    // Get location is 4 bytes
    let data_location = data.as_ptr() as u32 / 4;

    let data_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(data_location, data_location + data.len() as u32);

    let buffer = gl.create_buffer().unwrap();

    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &data_array, GL::STATIC_DRAW);
    gl.vertex_attrib_pointer_with_i32(attrib, size as i32, GL::FLOAT, false, 0 as i32, 0 as i32);
    Ok(buffer)
}

pub fn buffer_u16_indices(gl: &GL, indices: &[u16]) -> Result<WebGlBuffer, JsValue> {
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .unwrap()
        .buffer();

    let indices_location = indices.as_ptr() as u32 / 2;
    let indices_array = js_sys::Uint16Array::new(&memory_buffer)
        .subarray(indices_location, indices_location + indices.len() as u32);

    let index_buffer = gl.create_buffer().unwrap();
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
    gl.buffer_data_with_array_buffer_view(
        GL::ELEMENT_ARRAY_BUFFER,
        &indices_array,
        GL::STATIC_DRAW,
    );
    Ok(index_buffer)
}


/// Shaders string -> program
pub fn init_program(
    gl: &GL,
    s_vert_shader: &str,
    s_frag_shader: &str,
) -> Result<WebGlProgram, String> {
    let vert = compile_shader(gl, GL::VERTEX_SHADER, s_vert_shader)
        .expect("Error at compile vertex");
    let frag = compile_shader(gl, GL::FRAGMENT_SHADER, s_frag_shader)
        .expect("Error at compile fragment");

    let program = link_program(gl, &vert, &frag)
        .expect("Could not link program");

    Ok(program)
}


/// Create a shader program using the WebGL APIs
pub fn compile_shader(
    gl: &GL,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {

    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

/// Link a shader program using the WebGL APIs
pub fn link_program(
    gl: &GL,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {

    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}


/// Load a new texture :)
///
/// To do so, the texture image needs to be loaded from the server first. This is done
/// asynchronously in Javascript so we can upload the image to the GPU only after the image
/// is received on the client.
///
/// One trick is to create first the texture with one single blue pixel, then add a callback to
/// load the texture when the image is loaded. See here: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Using_textures_in_WebGL
#[allow(dead_code)]
pub fn load_texture(
    gl: &GL,
    img_src: &str,
) -> Result<Rc<WebGlTexture>, JsValue> {
    let texture = gl.create_texture().expect("Cannot create gl texture");
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
    let level = 0;
    let internal_format = GL::RGBA;
    let width = 1;
    let height = 1;
    let border = 0;
    let src_format = GL::RGBA;
    let src_type = GL::UNSIGNED_BYTE;

    // Now upload single pixel.
    let pixel: [u8; 4] = [0, 0, 255, 255];
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        GL::TEXTURE_2D,
        level,
        internal_format as i32,
        width,
        height,
        border,
        src_format,
        src_type,
        Some(&pixel),
    )?;

    let img = HtmlImageElement::new().unwrap();
    img.set_cross_origin(Some(""));

    let imgrc = Rc::new(img);

    let texture = Rc::new(texture);

    {
        let img = imgrc.clone();
        let texture = texture.clone();
        let gl = Rc::new(gl.clone());
        let a = Closure::wrap(Box::new(move || {
            gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

            if let Err(e) = gl.tex_image_2d_with_u32_and_u32_and_image(
                GL::TEXTURE_2D,
                level,
                internal_format as i32,
                src_format,
                src_type,
                &img,
            ) {
                // TODO better error handling...
                console::log_2(&"Could not load Texture, see following erroa:r".into(), &e);
                return;
            }

            gl.tex_parameteri(GL::TEXTURE_2D as u32, GL::TEXTURE_WRAP_S as u32, GL::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(GL::TEXTURE_2D as u32, GL::TEXTURE_WRAP_T as u32, GL::CLAMP_TO_EDGE as i32);
            gl.tex_parameteri(GL::TEXTURE_2D as u32, GL::TEXTURE_MIN_FILTER as u32, GL::LINEAR as i32);
            // different from webgl1 where we need the pic to be power of 2
            //gl.generate_mipmap(GL::TEXTURE_2D);
        }) as Box<dyn FnMut()>);
        imgrc.set_onload(Some(a.as_ref().unchecked_ref()));

        // Normally we'd store the handle to later get dropped at an appropriate
        // time but for now we want it to be a global handler so we use the
        // forget method to drop it without invalidating the closure. Note that
        // this is leaking memory in Rust, so this should be done judiciously!
        a.forget();
    }

    imgrc.set_src(img_src);

    Ok(texture)
}

/// Create the vertices geographics array buffers
pub fn init_buffers(gl: &GL, program: &WebGlProgram) -> Result<Buffers, JsValue> {
    // Now create an array of positions for the square.
    let positions = [
        // Front face
        -1.0, -1.0,  1.0,
         1.0, -1.0,  1.0,
         1.0,  1.0,  1.0,
        -1.0,  1.0,  1.0,

        // Back face
        -1.0, -1.0, -1.0,
        -1.0,  1.0, -1.0,
         1.0,  1.0, -1.0,
         1.0, -1.0, -1.0,

        // Top face
        -1.0,  1.0, -1.0,
        -1.0,  1.0,  1.0,
         1.0,  1.0,  1.0,
         1.0,  1.0, -1.0,

        // Bottom face
        -1.0, -1.0, -1.0,
         1.0, -1.0, -1.0,
         1.0, -1.0,  1.0,
        -1.0, -1.0,  1.0,

        // Right face
         1.0, -1.0, -1.0,
         1.0,  1.0, -1.0,
         1.0,  1.0,  1.0,
         1.0, -1.0,  1.0,

        // Left face
        -1.0, -1.0, -1.0,
        -1.0, -1.0,  1.0,
        -1.0,  1.0,  1.0,
        -1.0,  1.0, -1.0,
    ];
    let buf_position = buffer_f32_data(&gl, &program, &positions, "aVertexPosition", 3)
        .expect("buf_position");

    let texture_coordinates = [
        // Front
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Back
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Top
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Bottom
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Right
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
        // Left
        0.0,  0.0,
        1.0,  0.0,
        1.0,  1.0,
        0.0,  1.0,
    ];
    let buf_texture = buffer_f32_data(&gl, &program, &texture_coordinates, "aTextureCoord", 2)
        .expect("buf_texture");

    // This array defines each face as two triangles, using the
    // indices into the vertex array to specify each triangle's
    // position.
    let indices = [
        0,  1,  2,      0,  2,  3,    // front
        4,  5,  6,      4,  6,  7,    // back
        8,  9,  10,     8,  10, 11,   // top
        12, 13, 14,     12, 14, 15,   // bottom
        16, 17, 18,     16, 18, 19,   // right
        20, 21, 22,     20, 22, 23,   // left
    ];
    let buf_indice = buffer_u16_indices(&gl, &indices)
        .expect("buf_indice");

    let vertex_normals = [
        // Front
        0.0,  0.0,  1.0,
        0.0,  0.0,  1.0,
        0.0,  0.0,  1.0,
        0.0,  0.0,  1.0,

        // Back
        0.0,  0.0, -1.0,
        0.0,  0.0, -1.0,
        0.0,  0.0, -1.0,
        0.0,  0.0, -1.0,

        // Top
        0.0,  1.0,  0.0,
        0.0,  1.0,  0.0,
        0.0,  1.0,  0.0,
        0.0,  1.0,  0.0,

        // Bottom
        0.0, -1.0,  0.0,
        0.0, -1.0,  0.0,
        0.0, -1.0,  0.0,
        0.0, -1.0,  0.0,

        // Right
        1.0,  0.0,  0.0,
        1.0,  0.0,  0.0,
        1.0,  0.0,  0.0,
        1.0,  0.0,  0.0,

        // Left
        -1.0,  0.0,  0.0,
        -1.0,  0.0,  0.0,
        -1.0,  0.0,  0.0,
        -1.0,  0.0,  0.0
    ];
    let buf_normal = buffer_f32_data(&gl, &program, &vertex_normals, "aVertexNormal", 3)
        .expect("buf_normal");
    Ok(Buffers{
        position: buf_position,
        normal: buf_normal,
        texture_coord: buf_texture,
        indice: buf_indice,
    })
}

#[allow(dead_code)]
pub const VERTEX_SHADER: &str = r#"
    attribute vec4 aVertexPosition;
    attribute vec3 aVertexNormal;
    attribute vec2 aTextureCoord;

    uniform mat4 uNormalMatrix;
    uniform mat4 uModelViewMatrix;
    uniform mat4 uProjectionMatrix;

    varying highp vec2 vTextureCoord;
    varying highp vec3 vLighting;

    void main(void) {
      gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
      vTextureCoord = aTextureCoord;

      // Apply lighting effect
      highp vec3 ambientLight = vec3(0.2, 0.2, 0.2);
      highp vec3 directionalLightColor = vec3(1, 1, 1);
      highp vec3 directionalVector = normalize(vec3(0.85, 0.8, 0.75));

      highp vec4 transformedNormal = uNormalMatrix * vec4(aVertexNormal, 1.0);

      highp float directional = max(dot(transformedNormal.xyz, directionalVector), 0.0);
      vLighting = ambientLight + (directionalLightColor * directional);
    }
"#;

#[allow(dead_code)]
pub const FRAGMENT_SHADER: &str = r#"
    varying highp vec2 vTextureCoord;
    varying highp vec3 vLighting;

    uniform sampler2D uSampler;

    void main(void) {
      highp vec4 texelColor = texture2D(uSampler, vTextureCoord);

      gl_FragColor = vec4(texelColor.rgb * vLighting, texelColor.a);
    }
"#;

#[allow(dead_code)]
pub const VERT_SHADER_01 : &str = r#"
    attribute vec4 position;
    void main() {
        gl_Position = position;
    }
"#;

#[allow(dead_code)]
pub const FRAG_SHADER_01: &str = r#"
    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;
