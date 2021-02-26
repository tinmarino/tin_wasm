/// From: Javascript: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial
/// Log with: console::log_1(&(&*format!("Now {:?}", now) as &str).into());
use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{
    console,
    WebGlProgram,
    WebGlTexture,
    WebGlRenderingContext,
    HtmlCanvasElement,
    //WebGlUniformLocation,
};
use nalgebra::{ Isometry3, Vector3 };

use crate::util::*;
use crate::util_gl::*;

use std::sync::Arc;
//use crate::camera::*;


/// From: https://github.com/rustwasm/wasm-bindgen/blob/master/examples/request-animation-frame/src/lib.rs
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}


/// Holds all Gl stuff
pub struct GlContext {
    gl: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    camera: super::camera::Camera,
    buffers: Buffers,
    texture: Rc<WebGlTexture>,
    // Contains program: WebGlProgram,
    program_info: ProgramInfo,
}

impl GlContext { pub fn new() -> Result<Self, JsValue> {
    let canvas = create_canvas("id_canvas_webgl")?;

    let gl = canvas
        .get_context("webgl")
        .expect("Getting GL context <- Canvas")
        .unwrap()
        .dyn_into::<GL>()
        .expect("Casting dynamicaly GL Context");

    // Set clear color to black, fully opaque
    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    // Clear the color buffer with specified clear color
    gl.clear(GL::COLOR_BUFFER_BIT);

    // Init program <- Vertex strings
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
    Ok(Self {
        gl: gl,
        canvas: canvas,
        camera: super::camera::Camera::new(),
        buffers: buffers,
        texture: texture,
        program_info: program_info,
    })

}}

/// From MDN (translated) see html
#[allow(dead_code)]
pub fn canvas_gl2() -> Result<(), JsValue> {
    let game = GameGl::new()
        .expect("Creating game");

    //let state = Some().take();
    super::camera::attach_handlers(&game.gl_context.canvas)
        .expect("Cannot attach input");

    game.start_loop().
        expect("Launching game loop");
    Ok(())
}

pub struct GameGl {
    gl_context: GlContext,
}

impl GameGl { pub fn new() -> Result<Self, JsValue> {
    Ok(Self {
        gl_context: GlContext::new()?,
    })
}}

impl GameGl { pub fn start_loop(self) -> Result<(), JsValue> {
    // Render loop
    // Dont ask me
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    const FPS_THROTTLE: f64 = 1000.0 / 60.0; // milliseconds / frames
    let mut previous: f64 = js_sys::Date::now();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        //console::log_1(&"In loop".into());
        request_animation_frame(f.borrow().as_ref().unwrap());

        // Get time (miliseconds)
        let now = js_sys::Date::now();

        // Clause: must work or sleeep ?
        // The current rotation angle 1 rad/sec
        if now < previous + FPS_THROTTLE {
           return ();
        }

        // Update time
        let delta_time = now - previous;
        previous = now;

        // Update game
        {
            let mut state = STATE.lock().unwrap();
            console::log_1(&(&*format!("Now {:?}", state.cube_rotation) as &str).into());
            *state = Arc::new(State {
                cube_rotation: state.cube_rotation + delta_time as f32 * 0.001,
                ..*state.clone()
            });
        }

        // Draw
        draw_scene(&self.gl_context).unwrap();
            //&self.gl, &self.program_info, &self.texture, &buffers, &state).unwrap();

    }) as Box<dyn FnMut()>));

    console::log_1(&"Requesting animation frame".into());
    request_animation_frame(g.borrow().as_ref().unwrap());
    //let program_info = 
    Ok(())
}}

#[allow(dead_code)]
pub fn draw_scene(ctx: &GlContext) -> Result<(), JsValue> {
        //gl: &GL,
        //program_info: &ProgramInfo,
        //texture: &WebGlTexture,
        //buffers: &Buffers,
        //state: &State,
        //) -> Result<(), JsValue> {
    let gl = &ctx.gl;

    let  read_state = get_curr_state();
    //console::log_1(&(&*format!("Now {:?}", state.cube_rotation) as &str).into());

    // Clear the canvas before we start drawing on it.
    gl.clear_color(0.3, 0.3, 0.3, 1.0);  // Clear to black, fully opaque
    gl.clear_depth(1.0);                 // Clear everything
    gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

    // Enable depth shading
    gl.enable(GL::DEPTH_TEST);             // Enable depth testing to use depth_func)
    gl.depth_func(GL::LEQUAL);             // Near things obscure far things

    // Tell WebGL how to pull out the positions from the position
    // buffer into the vertexPosition attribute
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&ctx.buffers.position));
    // 3D, float, normalize, stride, offset
    gl.vertex_attrib_pointer_with_i32(
        ctx.program_info.a_vertex_position as u32,
        3, GL::FLOAT, false, 0, 0);
    // If you comment the next line, you won't see anything
    gl.enable_vertex_attrib_array(ctx.program_info.a_vertex_position as u32);

    // Tell webgl how to pull out the texture coordinates from buffer
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&ctx.buffers.texture_coord));
    gl.vertex_attrib_pointer_with_i32(
        ctx.program_info.a_texture_coord as u32,
        2, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(ctx.program_info.a_texture_coord as u32);

    // Tell WebGL how to pull out the normals from
    // the normal buffer into the vertexNormal attribute.
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&ctx.buffers.normal));
    gl.vertex_attrib_pointer_with_i32(
        ctx.program_info.a_vertex_normal as u32,
        3, GL::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(ctx.program_info.a_vertex_normal as u32);

    // Create a perspective matrix, a special matrix that is
    // used to simulate the distortion of perspective in a camera.
    // Our field of view is 45 degrees, with a width/height
    // ratio that matches the display size of the canvas
    // and we only want to see objects between 0.1 units
    // and 100 units away from the camera.
    // let perspective: Perspective3<f32> = Perspective3::new(
    //     45.0 * 3.14 / 180.0, 1.0, 0.1, 100.0);
    // let mat_projection = perspective.as_matrix().as_slice();
    let mat_projection = ctx.camera.view();
    //let mut camera_pos = [camera_pos.x, camera_pos.y, camera_pos.z];
    //gl.uniform3fv_with_f32_array(camera_pos_uni.as_ref(), &mut camera_pos);
    //gl.uniform1i(mesh_texture_uni.as_ref(), TextureUnit::Stone.texture_unit());

    // Set the drawing position to the "identity" point, which is
    // the center of the scene.
    let model = Isometry3::new(
        // Translate
        Vector3::new(-0.0, 0.0, -6.0),
        // Rotate
        Vector3::new(0.2, 0.7, 0.3).scale(read_state.cube_rotation),
    );
    let model4 = model.to_homogeneous();
    let mat_model = model4.as_slice();

    // Fill normal buffer
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
    gl.use_program(Some(&ctx.program_info.program));

    // Set the shader uniforms
    gl.uniform_matrix4fv_with_f32_array(
            Some(&ctx.program_info.u_projection_matrix),
            false,
            &mat_projection);
    gl.uniform_matrix4fv_with_f32_array(
            Some(&ctx.program_info.u_model_view_matrix),
            false,
            &mat_model);
    gl.uniform_matrix4fv_with_f32_array(
            Some(&ctx.program_info.u_normal_matrix),
            false,
            &mat_norm);

    // Tell WebGL which indices to use to index the vertices
    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&ctx.buffers.indice));

    // Tell WebGL we want to affect texture unit 0
    gl.active_texture(GL::TEXTURE0);

    // Bind the texture to texture unit 0
    gl.bind_texture(GL::TEXTURE_2D, Some(&ctx.texture));

    // DRAW !
    // Tell the shader we bound the texture to texture unit 0
    gl.uniform1i(Some(&ctx.program_info.u_sampler), 0);
    gl.draw_elements_with_i32(GL::TRIANGLES, 36, GL::UNSIGNED_SHORT, 0);

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
