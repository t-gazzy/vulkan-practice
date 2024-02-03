extern crate ash;

pub mod ash_handler {
    use ash::{extensions, vk, Entry, Instance};

    pub trait GraphisHandler {
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
            let entry = Entry::linked();
            let app_info = vk::ApplicationInfo {
                api_version: vk::make_api_version(0, 1, 3, 0),
                ..Default::default()
            };
            let create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                ..Default::default()
            };

            // as a sample to handle `Result` type
            // `unwrap()` is better
            let instance = match unsafe { entry.create_instance(&create_info, None) } {
                Ok(instance) => instance,
                Err(e) => panic!("failed to create instance. :{}", e),
            };

            // create a window surface
            let surface = ash::extensions::mvk::MacOSSurface::new(&entry, &instance);
            let surface_info = vk::MacOSSurfaceCreateInfoMVK {
                ..Default::default()
            };
            let surface = match unsafe {
                ash::extensions::mvk::MacOSSurface::create_mac_os_surface(
                    &surface,
                    &surface_info,
                    None,
                )
            } {
                Ok(surface_khr) => surface_khr,
                Err(e) => panic!("Error occured: {}", e),
            };

            // create a physical device
            // physical device = graphics card(GPU)
            // get all devices the computer has
            let physical_devices = match unsafe { Instance::enumerate_physical_devices(&instance) } {
                Ok(devices) => devices,
                Err(e) => panic!("error occured in creating device: {}", e),
            };
            let physical_device = physical_devices.first().unwrap();

            // create a logical device
            // logical device = virtual memory supplyed by the OS
            let create_info = vk::DeviceCreateInfo {
                ..Default::default()
            };
            let logical_device = match unsafe { Instance::create_device(&instance, *physical_device, &create_info, None) } {
                Ok(instance) => instance,
                Err(e) => panic!("error occured: {}", e)
            };

            return AshHandler { entry, instance, physical_device: *physical_device, logical_device, surface };
        }
    }
}
