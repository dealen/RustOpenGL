use glfw::{Action, Context, GlfwReceiver, Key, PWindow, WindowEvent};

pub struct Window {
    pub glfw: glfw::Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {

    pub fn new (width: u32, height: u32, title: &str) -> Window {

        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window,
            events 
        }
    }

    pub fn init_gl(&mut self) {
        self.window.make_current();
        gl::load_with(|symbol| self.window.get_proc_address(symbol) as *const _);
    }

    pub fn should_close(&mut self) -> bool {
        self.window.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window.swap_buffers();
    }

    pub fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    unsafe {
                        gl::Viewport(0, 0, width, height);
                    }
                }
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}