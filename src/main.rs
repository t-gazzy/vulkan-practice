use ash::{
    vk::{
        self, ExtensionProperties, Result
    },
    Entry, Instance,
};
use ash_handler::ash_handler::{AshHandler, GraphisHandler};

mod glfw_handler;
mod ash_handler;

extern crate glfw;
use std::{
    borrow::Borrow,
    boxed::{self, Box},
    ffi::c_char,
};

use crate::glfw_handler::handler::{GLFWHandler, GLHandler};

fn main() {
    let mut grapics_handler = Box::new(AshHandler::new());

    let mut gl = Box::new(GLFWHandler::new(300, 300, "test"));
    gl.run();
}