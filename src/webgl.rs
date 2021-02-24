// From: https://rustwasm.github.io/wasm-bindgen/examples/webgl.html
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{
    WebGlProgram,
    WebGlTexture,
};

use crate::util::*;
use crate::util_gl::{
    FRAGMENT_SHADER, VERTEX_SHADER,
    link_program, compile_shader, init_program,
    load_texture,
    buffer_f32_data, buffer_u16_indices,
    ProgramInfo,
};


/// Copied from MDN, see html
#[allow(dead_code)]
pub fn canvas_gl2() -> Result<(), JsValue> {


    let canvas = create_canvas("id_canvas_webgl")?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<GL>()?;

    // Set clear color to black, fully opaque
    gl.clear_color(0.0, 0.0, 0.5, 1.0);
    // Clear the color buffer with specified clear color
    gl.clear(GL::COLOR_BUFFER_BIT);

    let program: WebGlProgram = init_program(&gl, VERTEX_SHADER, FRAGMENT_SHADER)
        .expect("Failed at init_program");

    // Set program fields
    init_buffers(&gl, &program);
    let texture = load_texture(&gl, "noble_rat_460.png")
        .expect("Failed: load texture");

    // Explicity retrieve fields <- WebGl compile shaders -> int
    let program_info = ProgramInfo {
        aVertexPosition: gl.get_attrib_location(&program, "aVertexPosition"),
        aVertexNormal: gl.get_attrib_location(&program, "aVertexNormal"),
        aTextureCoord: gl.get_attrib_location(&program, "aVertexCoord"),

        uProjectionMatrix: gl.get_attrib_location(&program, "uProjectionMatrix"),
        uModelViewMatrix: gl.get_attrib_location(&program, "uModelViewMatrix"),
        uNormalMatrix: gl.get_attrib_location(&program, "uNormalMatrix"),
        uSampler: gl.get_attrib_location(&program, "uSampler"),
        program: program,
    };

    // Render loop
    let mut now: f32 = 0.0;
    let mut then: f32 = 0.0;
    // The current rotation angle
    let mut cube_rotation: f32 = 0.0;
    let cb = Closure::wrap(Box::new(move || {
        web_sys::console::log_1(&"raf called".into());
        now *= 0.001;  // convert to seconds
        let delta_time = now - then;
        then = now;

        // Draw
        draw_scene(&gl, &program_info, &texture, delta_time);

        // Update
        cube_rotation += delta_time;

        //window().request_animation_frame(cb.as_ref().unchecked_ref());
    }) as Box<FnMut()>);

    window().request_animation_frame(cb.as_ref().unchecked_ref());
    //let program_info = 
    Ok(())
}

#[allow(dead_code)]
pub fn draw_scene(
    gl: &GL,
    programInfo: &ProgramInfo,
    texture: &WebGlTexture,
    deltaTime : f32,
) -> Result<(), JsValue> {
    web_sys::console::log_1(&"Drawing".into());
    gl.clear_color(0.0, 0.0, 0.0, 1.0);  // Clear to black, fully opaque
    gl.clear_depth(1.0);                 // Clear everything
    gl.enable(GL::DEPTH_TEST);           // Enable depth testing
    gl.depth_func(GL::LEQUAL);            // Near things obscure far things

    // Clear the canvas before we start drawing on it.
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    // Create a perspective matrix, a special matrix that is
    // used to simulate the distortion of perspective in a camera.
    // Our field of view is 45 degrees, with a width/height
    // ratio that matches the display size of the canvas
    // and we only want to see objects between 0.1 units
    // and 100 units away from the camera.
    let fieldOfView = 45.0 * 3.14 / 180.0;   // in radians
    let aspect = 1; // TODO gl.canvas.clientWidth / gl.canvas.clientHeight;
    let zNear = 0.1;
    let zFar = 100.0;
    //let projectionMatrix = mat4.create();

    //// note: glmatrix.js always has the first argument
    //// as the destination to receive the result.
    //mat4.perspective(projectionMatrix,
    //                                  fieldOfView,
    //                                  aspect,
    //                                  zNear,
    //                                  zFar);

    //// Set the drawing position to the "identity" point, which is
    //// the center of the scene.
    //const modelViewMatrix = mat4.create();

    //// Now move the drawing position a bit to where we want to
    //// start drawing the square.
    //mat4.translate(modelViewMatrix,     // destination matrix
    //                              modelViewMatrix,     // matrix to translate
    //                              [-0.0, 0.0, -6.0]);  // amount to translate

    //// Rotate
    //mat4.rotate(modelViewMatrix,  // destination matrix
    //                        modelViewMatrix,  // matrix to rotate
    //                        cubeRotation,     // amount to rotate in radians
    //                        [0, 1, 1]);       // axis to rotate around

    //// Tell WebGL how to pull out the positions from the position
    //// buffer into the vertexPosition attribute
    //{
    //    const numComponents = 3;  // 3d for position
    //    const type = gl.FLOAT;
    //    const normalize = false;
    //    const stride = 0;
    //    const offset = 0;
    //    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
    //    gl.vertexAttribPointer(
    //            programInfo.attribLocations.vertexPosition,
    //            numComponents,
    //            type,
    //            normalize,
    //            stride,
    //            offset);
    //    gl.enableVertexAttribArray(
    //            programInfo.attribLocations.vertexPosition);
    //}

    //// tell webgl how to pull out the texture coordinates from buffer
    //{
    //    const num = 2; // every coordinate composed of 2 values
    //    const type = gl.FLOAT; // the data in the buffer is 32 bit float
    //    const normalize = false; // don't normalize
    //    const stride = 0; // how many bytes to get from one set to the next
    //    const offset = 0; // how many bytes inside the buffer to start from
    //    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.textureCoord);
    //    gl.vertexAttribPointer(programInfo.attribLocations.textureCoord, num, type, normalize, stride, offset);
    //    gl.enableVertexAttribArray(programInfo.attribLocations.textureCoord);
    //}

    //// Tell WebGL how to pull out the normals from
    //// the normal buffer into the vertexNormal attribute.
    //{
    //    const numComponents = 3;
    //    const type = gl.FLOAT;
    //    const normalize = false;
    //    const stride = 0;
    //    const offset = 0;
    //    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.normal);
    //    gl.vertexAttribPointer(
    //            programInfo.attribLocations.vertexNormal,
    //            numComponents,
    //            type,
    //            normalize,
    //            stride,
    //            offset);
    //    gl.enableVertexAttribArray(
    //            programInfo.attribLocations.vertexNormal);
    //}

    //const normalMatrix = mat4.create();
    //mat4.invert(normalMatrix, modelViewMatrix);
    //mat4.transpose(normalMatrix, normalMatrix);

    //// Tell WebGL to use our program when drawing
    //gl.useProgram(programInfo.program);

    //// Set the shader uniforms
    //gl.uniformMatrix4fv(
    //        programInfo.uniformLocations.projectionMatrix,
    //        false,
    //        projectionMatrix);
    //gl.uniformMatrix4fv(
    //        programInfo.uniformLocations.modelViewMatrix,
    //        false,
    //        modelViewMatrix);
    //gl.uniformMatrix4fv(
    //        programInfo.uniformLocations.normalMatrix,
    //        false,
    //        normalMatrix);

    //// Tell WebGL which indices to use to index the vertices
    //gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffers.indices);

    //// Tell WebGL we want to affect texture unit 0
    //gl.activeTexture(gl.TEXTURE0);

    //// Bind the texture to texture unit 0
    //gl.bindTexture(gl.TEXTURE_2D, texture);

    //// Tell the shader we bound the texture to texture unit 0
    //gl.uniform1i(programInfo.uniformLocations.uSampler, 0);
    //{
    //    const vertexCount = 36;
    //    const type = gl.UNSIGNED_SHORT;
    //    const offset = 0;
    //    gl.drawElements(gl.TRIANGLES, vertexCount, type, offset);
    //}

    //mat4.rotate(modelViewMatrix, modelViewMatrix, cubeRotation * .7, [0, 1, 0]);
    Ok(())
}

#[allow(dead_code)]
pub fn init_buffers(gl: &GL, program: &WebGlProgram) -> Result<(), JsValue> {
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
    buffer_f32_data(&gl, &program, &positions, "aVertexPosition", positions.len());

    let textureCoordinates = [
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
    buffer_f32_data(&gl, &program, &positions, "aTextureCoord", textureCoordinates.len());

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
    buffer_u16_indices(&gl, &indices);

    let vertexNormals = [
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
    buffer_f32_data(&gl, &program, &vertexNormals, "aVertexNormal", vertexNormals.len());
    Ok(())
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

