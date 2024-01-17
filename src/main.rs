use ash::{
    vk::{
        self, DebugUtilsMessageSeverityFlagsEXT, ExtensionProperties,
        PFN_vkEnumerateInstanceExtensionProperties, PhysicalDeviceProperties, StructureType,
    },
    Entry, Instance,
};

mod glfw_handler;

extern crate glfw;
use glfw::{Action, Context, Key};
use std::{
    borrow::Borrow,
    boxed::{self, Box},
    ffi::c_char,
};

use crate::glfw_handler::handler::{GLFWHandler, GLHandler};

fn main() {
    let entry = Entry::linked();

    let app_info = vk::ApplicationInfo {
        api_version: vk::make_api_version(0, 1, 3, 0),
        ..Default::default()
    };
    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };

    validate_layer(&entry);

    // as a sample to handle `Result` type
    // `unwrap()` is better
    let instance = match unsafe { entry.create_instance(&create_info, None) } {
        Ok(instance) => instance,
        Err(e) => panic!("failed to create instance. :{}", e),
    };

    println!("Hello, world!: {}", app_info.application_version);

    let mut gl = Box::new(GLFWHandler::new(300, 300, "test"));
    gl.run();
}

// vulkan supply the function to inform of misuse
fn validate_layer(entry: &Entry) -> Vec<ExtensionProperties> {
    let layer_name = std::ffi::CString::new("layer").unwrap();
    let option = Some(layer_name.as_c_str());
    return Entry::enumerate_instance_extension_properties(entry, option).unwrap();
}

// choose physical devices(e.g. GPU) from list
fn create_physical_device(instance: &Instance) -> Option<vk::PhysicalDevice> {
    let mut device: vk::PhysicalDevice;
    let devices = match unsafe { Instance::enumerate_physical_devices(instance) } {
        Ok(devices) => devices,
        Err(e) => panic!("error occured in creating device: {}", e),
    };

    for d in devices {
        if is_device_suitable(instance, &d) {
            return Some(d);
        }
    }
    return None;
}

// check whether the GPU is suitable or not
fn is_device_suitable(instance: &Instance, device: &vk::PhysicalDevice) -> bool {
    // get GPU basic instance
    let property = unsafe { Instance::get_physical_device_properties(instance, *device) };
    // get GPU extented function
    let feat = unsafe { Instance::get_physical_device_features(instance, *device) };
    return property.device_type == vk::PhysicalDeviceType::DISCRETE_GPU && feat.geometry_shader != 0;
}

// create logical device
fn create_logical_device(instance: &Instance, device: &vk::PhysicalDevice) {
    let create_info = vk::DeviceCreateInfo {
        ..Default::default()
    };
    let result = match unsafe { Instance::create_device(instance, *device, &create_info, None) } {
        Ok(Instance) => instance,
        Err(e) => panic!("error occured: {}", e)
    };
}