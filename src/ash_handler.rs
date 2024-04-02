extern crate ash;

pub mod ash_handler {

    use ash::{
        extensions,
        vk::{self, MacOSSurfaceCreateInfoMVK, StructureType, SurfaceKHR},
        Entry, Instance,
    };

    pub trait GraphisHandler: private::Sealed {
        fn new() -> Self;
        // fn drop(&mut self);
    }

    pub struct AshHandler {
        entry: Entry,
        instance: Instance,
        physical_device: vk::PhysicalDevice,
        logical_device: ash::Device,
        surface: vk::SurfaceKHR,
    }

    impl GraphisHandler for AshHandler {
        fn new() -> Self {
            let entry = match unsafe { Entry::load() } {
                Ok(entry) => entry,
                Err(e) => panic!("load failed: {}", e),
            };

            let app_info = vk::ApplicationInfo {
                api_version: vk::make_api_version(0, 1, 2, 0),
                ..Default::default()
            };
            let create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                flags: vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR,
                ..Default::default()
            };

            // as a sample to handle `Result` type
            // `unwrap()` is better
            let instance = match unsafe { entry.create_instance(&create_info, None) } {
                Ok(instance) => instance,
                Err(e) => panic!("failed to create instance. :{}", e),
            };

            println!("[App] instance created: {:p}", &instance);

            let surface = <AshHandler as private::Sealed>::make_surface(&entry, &instance);

            let physical_device = <AshHandler as private::Sealed>::make_physical_device(&instance);

            println!("[App] physical device created: {:p}", &physical_device);
            // create a logical device
            // logical device = virtual memory supplyed by the OS
            let create_info = vk::DeviceCreateInfo {
                ..Default::default()
            };
            let logical_device = match unsafe {
                Instance::create_device(&instance, physical_device, &create_info, None)
            } {
                Ok(instance) => instance,
                Err(e) => panic!("error occured: {}", e),
            };

            return AshHandler {
                entry,
                instance,
                physical_device,
                logical_device,
                surface,
            };
        }
    }

    // (crate) means access restriction.
    // in this case, permit to access from this file
    pub(crate) mod private {
        use ash::{vk, Device, Entry, Instance};

        pub trait Sealed {
            fn make_surface(entry: &Entry, instance: &Instance) -> vk::SurfaceKHR;
            fn make_physical_device(instance: &Instance) -> vk::PhysicalDevice;
            fn make_logical_device(
                instance: &Instance,
                physical_device: vk::PhysicalDevice,
            ) -> Device;
        }
    }

    impl private::Sealed for AshHandler {
        // create window surface
        fn make_surface(entry: &Entry, instance: &Instance) -> vk::SurfaceKHR {
            let surface = ash::extensions::ext::MetalSurface::new(entry, instance);
            let surface_info = vk::MetalSurfaceCreateInfoEXT {
                s_type: StructureType::METAL_SURFACE_CREATE_INFO_EXT,
                ..Default::default()
            };
            return match unsafe {
                ash::extensions::ext::MetalSurface::create_metal_surface(
                    &surface,
                    &surface_info,
                    None,
                )
            } {
                Ok(surface_khr) => surface_khr,
                Err(e) => panic!("Error occured: {}", e),
            };
        }

        // create a physical device
        // physical device = graphics card(GPU)
        // get all devices the computer has
        fn make_physical_device(instance: &Instance) -> vk::PhysicalDevice {
            let physical_devices = match unsafe { Instance::enumerate_physical_devices(&instance) }
            {
                Ok(devices) => devices,
                Err(e) => panic!("error occured in creating device: {}", e),
            };

            if physical_devices.capacity() == 1 {
                println!("devices count: {}", physical_devices.capacity());
                let queues = unsafe {
                    instance.get_physical_device_queue_family_properties(
                        *physical_devices.first().unwrap(),
                    )
                };
                println!("queues count: {}", queues.capacity());
                let q_prop = queues
                    .iter()
                    .find(|queue| queue.queue_flags == vk::QueueFlags::GRAPHICS);

                return match q_prop {
                    Some(_) => *physical_devices.first().unwrap(),
                    None => panic!("Error occurred: No matching physical device."),
                };
            } else {
                // Using variable for `for` statement make it move
                for p_device in &physical_devices {
                    let queues =
                        unsafe { instance.get_physical_device_queue_family_properties(*p_device) };
                    for q in queues {
                        let is_graphics_support = if q.queue_flags == vk::QueueFlags::GRAPHICS {
                            "OK"
                        } else {
                            "NG"
                        };
                        let is_compute_support = if q.queue_flags == vk::QueueFlags::COMPUTE {
                            "OK"
                        } else {
                            "NG"
                        };
                        let is_transfer_support = if q.queue_flags == vk::QueueFlags::TRANSFER {
                            "OK"
                        } else {
                            "NG"
                        };
                        println!("Queue Count: {}, Graphic Support: {}, Compute Support: {}, Transfer Support: {}", q.queue_count, is_graphics_support, is_compute_support, is_transfer_support);
                    }
                }

                let device = physical_devices.iter().find(|device| unsafe {
                    instance
                        .get_physical_device_queue_family_properties(**device)
                        .iter()
                        .find(|queue| queue.queue_flags == vk::QueueFlags::GRAPHICS)
                        .is_some()
                });

                return match device {
                    Some(d) => d.clone(),
                    None => panic!("Error occurred: No matching physical device."),
                };
            }
        }

        // create a logical device
        // logical device = virtual memory supplyed by the OS
        fn make_logical_device(
            instance: &Instance,
            physical_device: vk::PhysicalDevice,
        ) -> ash::Device {
            let create_info = vk::DeviceCreateInfo {
                ..Default::default()
            };
            return match unsafe {
                Instance::create_device(&instance, physical_device, &create_info, None)
            } {
                Ok(instance) => instance,
                Err(e) => panic!("error occured: {}", e),
            };
        }
    }
}
