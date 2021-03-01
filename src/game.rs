/// From: Javascript: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial
///
/// web_sys::console::log_1(&(&*format!("Now {:?}", now) as &str).into());
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
    MouseEvent,
    WheelEvent,
    KeyboardEvent,
    //WebGlUniformLocation,
};

use super::util::*;
use super::camera::Camera;


const FPS_THROTTLE: f64 = 1000.0 / 60.0; // milliseconds / frames


/// Launch game
pub fn start() -> Result<(), JsValue> {
    // Create
    let game: Rc<GameGl> = Rc::new(GameGl::new()
        .expect("Creating game"));

    // Bind
    let gl_context = game.store.borrow_mut();
    let canvas = &gl_context.canvas;
    attach_handlers(canvas, Rc::clone(&game))
        .expect("Cannot attach input handlers");

    // Loop forever
    start_loop(Rc::clone(&game))
        .expect("Error in game loop");

    Ok(())
}

/// Holds all Gl stuff
pub struct GlContext {
    pub gl: WebGlRenderingContext,
    pub canvas: HtmlCanvasElement,
    pub camera: Camera,
    pub buffers: Buffers,
    pub texture: Rc<WebGlTexture>,
    // Contains program: WebGlProgram,
    pub program_info: ProgramInfo,
}


impl GlContext { pub fn new() -> Result<Self, JsValue> {
    let canvas = create_canvas("id_canvas_webgl")
        .expect("Could not create canvas");

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

    // Load textures
    let texture = load_texture(&gl, "/res/blue.png")
        .expect("Failed: load texture");

    // Explicity retrieve program fields <- WebGl compile shaders -> int
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

    // Return self
    Ok(Self {
        gl: gl,
        canvas: canvas,
        camera: super::camera::Camera::new(),
        buffers: buffers,
        texture: texture,
        program_info: program_info,
    })
}}

pub struct GameGl {
    pub store: Rc<RefCell<GlContext>>,
}

impl GameGl { pub fn new() -> Result<Self, JsValue> {
    Ok(Self {
        store: Rc::new(RefCell::new(GlContext::new()?)),
    })
}}

pub fn input(_key: i32, _x: f32, _y:f32){
    //console::log_1(&(&*format!("Calledback {:?}, {:?}, {:?}", key, x, y) as &str).into());
}


// the size for values of type `[(&str, dyn FnMut(MouseEvent))]` cannot be known at compilation time: doesn't have a size known at compile-time
// explicit lifetime required in the type of `state`: lifetime `'static` required: static to state
//pub fn attach_handlers(canvas: &HtmlCanvasElement, state: &'static mut State) -> Result<(), JsValue> {
pub fn attach_handlers(canvas: &HtmlCanvasElement, game: Rc<GameGl>) -> Result<(), JsValue> {
    //let toto: &mut f32 = &mut state.cube_rotation;

    { //let game = Rc::clone(&game);
    add_handler("mousedown", canvas, move |event: MouseEvent| {
        input(1, event.client_x() as f32, event.client_y() as f32);
    }).expect("Adding mousedown");
    }

    add_handler("mouseup", canvas, move |event: MouseEvent| {
        input(2, event.client_x() as f32, event.client_y() as f32);
    }).expect("Adding mouseup");

    { //let game = Rc::clone(&game);
    add_handler("wheel", canvas, move |event: WheelEvent| {
        event.prevent_default();
        let zoom_amount: f32 = event.delta_y() as f32 / 50.;
        input(3, zoom_amount, 0.);
    }).expect("Adding wheel");
    }

    { let game = Rc::clone(&game);
    add_handler("keydown", canvas, move |event: KeyboardEvent| {
        let key = event.key_code() as u32;
        input(4, key as f32, 0.);
        let mut gl_context = game.store.borrow_mut();
        gl_context.camera.keys_down.insert(key);
    }).expect("Adding keydown");
    }

    { let game = Rc::clone(&game);
    add_handler("keyup", canvas, move |event: KeyboardEvent| {
        let key = event.key_code() as u32;
        input(5, key as f32, 0.);
        let mut gl_context = game.store.borrow_mut();
        gl_context.camera.keys_down.remove(&key);
    }).expect("Adding keydown");
    }

    Ok(())
}


pub fn start_loop(game: Rc<GameGl>) -> Result<(), JsValue> {
    // Render loop
    // Dont ask me
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut previous: f64 = js_sys::Date::now();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        request_animation_frame(f.borrow().as_ref().unwrap());

        // Get time (miliseconds)
        let now = js_sys::Date::now();

        // Clause: must work or sleep ?
        // The current rotation angle 1 rad/sec
        if now < previous + FPS_THROTTLE {
           return ();
        }

        // Update time
        //let delta_time = now - previous;
        previous = now;

        // Update camera
        {
            let mut gl_context = game.store.borrow_mut();
            gl_context.camera.update();
        }

        // Draw
        {
            let gl_context = game.store.borrow_mut();
            draw_scene(&gl_context).unwrap();
        }

    }) as Box<dyn FnMut()>));

    console::log_1(&"Requesting animation frame".into());
    request_animation_frame(g.borrow().as_ref().unwrap());
    //let program_info = 
    Ok(())
}

#[allow(dead_code)]
pub fn draw_scene(ctx: &GlContext) -> Result<(), JsValue> {
    // Borrow reference
    let gl = &ctx.gl;

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

    // Get projection matrix (currently constant)
    let mat_projection = ctx.camera.project();
    
    // Get ModelView = eye
    let model = ctx.camera.view();
    let homo = model.to_homogeneous();
    let mat_model = homo.as_slice();

    // Fill normal buffer
    let mut norm = model.clone();
    norm.inverse_mut();
    let mut norm4 = norm.to_homogeneous();
    norm4.transpose_mut();
    let mat_norm = norm4.as_slice();

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
