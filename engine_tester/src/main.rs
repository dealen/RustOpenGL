use std::ptr;

use gl::types::{GLfloat, GLsizei};
use kubol_core::{custom_errors, graphics::{gl_wrappers::{BufferObject, VertexAttribute, VAO}, window::Window}};
mod test;

fn main() {

    kubol_core::logger::init();
    // error_test(0);
    println!("Hello, world!");

    // show_simple_window();
    
    // show_triangle();

    indices();
}

fn show_simple_window(){
    let mut window = Window::new(800, 600, "Hello, world!");
    
    window.init_gl();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }
}

fn show_triangle() {
    let mut window = Window::new(1080, 720, "Hello, world!");
    
    window.init_gl();

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0,
        0.5, -0.8, 0.0,
        0.0, 0.5, 0.0
    ];

    let vao = VAO::new();
    vao.bind();

    let buffer_object = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    buffer_object.bind();
    buffer_object.store_f32_data(&vertices);

    let stride = 3 * std::mem::size_of::<GLfloat>();
    let position_attribute = VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, stride as GLsizei, ptr::null());

    position_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
        
    }
}

fn error_test(input: i32) -> Result<(), custom_errors::Errors> {
    if input == 0 {
        kubol_core::logger::info!("Input is zero");
        return Err(custom_errors::Errors::TestError.into());
    }
    Ok(())
}

fn indices() {
    let mut window = Window::new(1080, 720, "Hello, world!");

    let verticles: [f32; 12] = [
        0.5, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0,
        -0.5, 0.5, 0.0,
    ];

    let indices = [0, 1, 3, 1, 2, 3];
    window.init_gl();

    let vao = VAO::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&verticles);

    let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ibo.bind();

    ibo.store_i32_data(&indices);

    let stride = (3 * std::mem::size_of::<GLfloat>()) as GLsizei;
    let position_attribute = VertexAttribute::new(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
    let index_attribute = VertexAttribute::new(1, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

    position_attribute.enable();
    index_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.5, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
        window.update();
    }
}
