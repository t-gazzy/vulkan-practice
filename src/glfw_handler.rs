extern crate glfw;

pub mod handler {

    use std::borrow::Borrow;

    use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};

    pub trait GLHandler {
        fn new(width: u32, height: u32, title: &str) -> Self;
        fn run(&mut self);
        fn clear(&mut self);
    }

    pub struct GLFWHandler {
        glfw_: Glfw,
        window_: PWindow,
        events_: GlfwReceiver<(f64, WindowEvent)>,
    }

    impl GLHandler for GLFWHandler {
        fn new(width: u32, height: u32, title: &str) -> Self {
            use glfw::fail_on_errors;
            let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

            let (mut window, events) = glfw
                .create_window(width, height, title, glfw::WindowMode::Windowed)
                .expect("Failed to create window");

            window.make_current();
            window.set_key_polling(true);
            
            GLFWHandler {
                glfw_: glfw,
                window_: window,
                events_: events
            }
        }

        fn run(&mut self) {
            let ref mut window = self.window_;
            let ref mut glfw = self.glfw_;
            let ref mut events = self.events_;

            while !window.should_close() {
                window.swap_buffers();

                glfw.poll_events();
                for (_, event) in glfw::flush_messages(&events) {
                    match event {
                        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                            window.set_should_close(true)
                        }
                        _ => {}
                    }
                }
            }
        }

        fn clear(&mut self) {
            
        }
    }
}
