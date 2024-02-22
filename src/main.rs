use ash_handler::ash_handler::{AshHandler, GraphisHandler};

mod ash_handler;
mod glfw_handler;

extern crate glfw;
use std::boxed::Box;

use crate::glfw_handler::handler::{GLFWHandler, GLHandler};

fn main() {
    let mut grapics_handler = Box::new(AshHandler::new());

    let mut gl = Box::new(GLFWHandler::new(300, 300, "test"));
    gl.run();
}