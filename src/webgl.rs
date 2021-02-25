// From: https://rustwasm.github.io/wasm-bindgen/examples/webgl.html
use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{
    console,
    WebGlProgram,
    WebGlTexture,
    WebGlBuffer,
    //WebGlUniformLocation,
};
use nalgebra::{ Isometry3, Perspective3, Vector3 };

use crate::util::*;
use crate::util_gl::{
    FRAGMENT_SHADER, VERTEX_SHADER,
    link_program, compile_shader, init_program,
    load_texture,
    buffer_f32_data, buffer_u16_indices,
    ProgramInfo,
};

/// From: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


pub struct Buffers {
    position: WebGlBuffer,
    normal: WebGlBuffer,
    texture_coord: WebGlBuffer,
    indice: WebGlBuffer,
}

pub struct State {
    cube_rotation: f32,
}

/// From MDN (translated) see html
#[allow(dead_code)]
pub fn canvas_gl2() -> Result<(), JsValue> {
    let canvas = create_canvas("id_canvas_webgl")?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<GL>()?;

    // Set clear color to black, fully opaque
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    // Clear the color buffer with specified clear color
    gl.clear(GL::COLOR_BUFFER_BIT);

    let program: WebGlProgram = init_program(&gl, VERTEX_SHADER, FRAGMENT_SHADER)
        .expect("Failed at init_program");

    // Set program fields
    let buffers = init_buffers(&gl, &program)
        .expect("Init buffers");
    let texture = load_texture(&gl, "blue.png")
    //let texture = load_texture(&gl, "noble_rat_460.png")
        .expect("Failed: load texture");

    // Explicity retrieve fields <- WebGl compile shaders -> int
    let program_info = ProgramInfo {
        a_vertex_position: gl.get_attrib_location(&program, "aVertexPosition"),
        a_vertex_normal: gl.get_attrib_location(&program, "aVertexNormal"),
        a_texture_coord: gl.get_attrib_location(&program, "aTextureCoord"),

        u_projection_matrix: gl.get_uniform_location(&program, "uProjectionMatrix").unwrap(),
        u_model_view_matrix: gl.get_uniform_location(&program, "uModelViewMatrix").unwrap(),
        u_normal_matrix: gl.get_uniform_location(&program, "uNormalMatrix").unwrap(),
        u_sampler: gl.get_uniform_location(&program, "uSampler").unwrap(),
        // Pass me at the end so that I can keep owning it
        program: program,
    };

    // Render loop
    // Dont ask me
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut state = State {
        cube_rotation: 0.0,
    };
    //let mut now: f32 = 0.0;
    //let mut then: f32 = 0.0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        //now *= 0.001;  // convert to seconds
        let delta_time = 0.010;
        //let delta_time = now - then;
        //then = now;

        // Draw
        draw_scene(&gl, &program_info, &texture, &buffers, &mut state, delta_time).unwrap();

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    console::log_1(&"Requesting animation frame".into());
    request_animation_frame(g.borrow().as_ref().unwrap());
    //let program_info = 
    Ok(())
}

#[allow(dead_code)]
pub fn draw_scene(
    gl: &GL,
    program_info: &ProgramInfo,
    texture: &WebGlTexture,
    buffers: &Buffers,
    state: &mut State,
    delta_time : f32,
) -> Result<(), JsValue> {
    // Hi
    //web_sys::console::log_1(&"Drawing".into());

    // Clear
    gl.clear_color(0.3, 0.3, 0.3, 1.0);  // Clear to black, fully opaque
    gl.clear_depth(1.0);                 // Clear everything
    gl.enable(GL::DEPTH_TEST);           // Enable depth testing
    gl.depth_func(GL::LEQUAL);            // Near things obscure far things
    // Clear the canvas before we start drawing on it.
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    // Tell WebGL how to pull out the positions from the position
    // buffer into the vertexPosition attribute
    {
        let num = 3;  // 3d for position
        let typ = GL::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffers.position));
        gl.vertex_attrib_pointer_with_i32(
                program_info.a_vertex_position as u32,
                num,
                typ,
                normalize,
                stride,
                offset);
        gl.enable_vertex_attrib_array(program_info.a_vertex_position as u32);
    }

    // Tell webgl how to pull out the texture coordinates from buffer
    {
        let num = 2; // every coordinate composed of 2 values
        let typ = GL::FLOAT; // the data in the buffer is 32 bit float
        let normalize = false; // don't normalize
        let stride = 0; // how many bytes to get from one set to the next
        let offset = 0; // how many bytes inside the buffer to start from
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffers.texture_coord));
        gl.vertex_attrib_pointer_with_i32(
            program_info.a_texture_coord as u32,
            num, typ, normalize, stride, offset);
        gl.enable_vertex_attrib_array(program_info.a_texture_coord as u32);
    }

    // Tell WebGL how to pull out the normals from
    // the normal buffer into the vertexNormal attribute.
    {
        let num = 3;
        let typ = GL::FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffers.normal));
        gl.vertex_attrib_pointer_with_i32(
                program_info.a_vertex_normal as u32,
                num, typ, normalize, stride, offset);
        gl.enable_vertex_attrib_array(program_info.a_vertex_normal as u32);
    }

    // Create a perspective matrix, a special matrix that is
    // used to simulate the distortion of perspective in a camera.
    // Our field of view is 45 degrees, with a width/height
    // ratio that matches the display size of the canvas
    // and we only want to see objects between 0.1 units
    // and 100 units away from the camera.
    let fov = 45.0 * 3.14 / 180.0;   // in radians
    let aspect = 1.0; // TODO gl.canvas.clientWidth / gl.canvas.clientHeight;
    let near = 0.1;
    let far = 100.0;
    let perspective: Perspective3<f32> = Perspective3::new(fov, aspect, near, far);
    let mat_projection = perspective.as_matrix().as_slice();
    //let msg: &str = &*format!("{:?}", mat_projection);
    //console::log_1(&msg.into());

    // Update
    // The current rotation angle
    state.cube_rotation += delta_time;

    //// OK now
    //let mat_model = [
    //    1.0, 0.0,  0.0, 0.0,
    //    0.0, 1.0,  0.0, 0.0,
    //    0.0, 0.0,  1.0, 0.0,
    //    0.0, 0.0, -6.0, 1.0,];

    //let mat4 = Identity3.new();

    // Set the drawing position to the "identity" point, which is
    // the center of the scene.
    let model = Isometry3::new(
        // Translate
        Vector3::new(-0.0, 0.0, -6.0),
        // Rotate
        Vector3::new(0.2, 0.7, 0.3).scale(state.cube_rotation),
    );
    let model4 = model.to_homogeneous();
    let mat_model = model4.as_slice();

    //let msg: &str = &*format!("{:?}", iso);
    //console::log_2(&"Model:".into(), &msg.into());

    //let msg: &str = &*format!("{:?}", state.cube_rotation);
    //console::log_2(&"Rotation:".into(), &msg.into());
    //let msg: &str = &*format!("{:?}", delta_time);
    //console::log_2(&"delta:".into(), &msg.into());


    // TODO
    let mut norm = model.clone();
    norm.inverse_mut();
    let mut norm4 = norm.to_homogeneous();
    norm4.transpose_mut();
    //let mat_norm = norm4.to_homogeneous();
    let mat_norm = norm4.as_slice();
    //let msg: &str = &*format!("Norm: {:?}", mat_norm);
    //console::log_2(&"Norm:".into(), &msg.into());
    //const normalMatrix = mat4.create();
    //mat4.invert(normalMatrix, modelViewMatrix);
    //mat4.transpose(normalMatrix, normalMatrix);

    //// Tell WebGL to use our program when drawing
    gl.use_program(Some(&program_info.program));

    // Set the shader uniforms
    // TODO need math
    gl.uniform_matrix4fv_with_f32_array(
            Some(&program_info.u_projection_matrix),
            false,
            &mat_projection);
    gl.uniform_matrix4fv_with_f32_array(
            Some(&program_info.u_model_view_matrix),
            false,
            &mat_model);
    gl.uniform_matrix4fv_with_f32_array(
            Some(&program_info.u_normal_matrix),
            false,
            &mat_norm);

    // Tell WebGL which indices to use to index the vertices
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffers.indice));

    // Tell WebGL we want to affect texture unit 0
    gl.active_texture(GL::TEXTURE0);

    // Bind the texture to texture unit 0
    gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

    // DRAW !
    // Tell the shader we bound the texture to texture unit 0
    gl.uniform1i(Some(&program_info.u_sampler), 0);
    {
        let vertex_count = 36;
        let typ = GL::UNSIGNED_SHORT;
        let offset = 0;
        gl.draw_elements_with_i32(GL::TRIANGLES, vertex_count, typ, offset);
    }

    Ok(())
}

#[allow(dead_code)]
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
pub fn canvas_gl1() -> Result<(), JsValue> {
    let canvas = create_canvas("id_canvas_webgl")?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<GL>()?;

    let vert_shader = compile_shader(
        &gl,
        GL::VERTEX_SHADER,
        VERTEX_SHADER
    )?;
    let frag_shader = compile_shader(
        &gl,
        GL::FRAGMENT_SHADER,
        FRAGMENT_SHADER
    )?;
    let program = link_program(&gl, &vert_shader, &frag_shader)?;
    gl.use_program(Some(&program));

    let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &vert_array,
            GL::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(0);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(GL::COLOR_BUFFER_BIT);

    gl.draw_arrays(
        GL::TRIANGLES,
        0,
        (vertices.len() / 3) as i32,
    );
    Ok(())
}
