extern crate sdl2;
//extern crate vulkano;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use ash::vk;

fn main() 
{
	match vulkan_init()
	{
		Ok(_) => { println!("Run success!"); }
		Err(_r) => { println!("Error:"); }
	}
}

struct VulkanObject
{
	_entry: ash::Entry,
	instance: ash::Instance,
	surface: vk::SurfaceKHR,
	surface_fn: ash::extensions::khr::Surface
}

impl VulkanObject
{
	pub fn new(mut extension_strs: Vec<&'static str>, window_handle: &dyn raw_window_handle::HasRawWindowHandle)
		-> Result<Self, Box<dyn std::error::Error>>
	{
		let entry: ash::Entry = unsafe { ash::Entry::new()? };

		let mut debugcreateinfo =
		vk::DebugUtilsMessengerCreateInfoEXT::builder()
		.message_severity(
			vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
				| vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
				//| vk::DebugUtilsMessageSeverityFlagsEXT::INFO
				| vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
		)
		.message_type(
			vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
				| vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
				| vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
		)
		.pfn_user_callback(Some(vulkan_debug_utils_callback));

		extension_strs.push(ash::extensions::ext::DebugUtils::name().to_str()?);
		let instance_extensions_strings = extension_strs
			.iter()
			.map(|ext| ext.as_ptr() as *const i8)
			.collect::<Vec<_>>();
	
		
		let layer_names: Vec<std::ffi::CString> =
			vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
		let layer_name_pointers: Vec<*const i8> = layer_names
			.iter()
			.map(|layer_name| layer_name.as_ptr())
			.collect();
	
	
		let app_desc = vk::ApplicationInfo::builder()
			.api_version(vk::make_api_version(0, 1, 1, 0));
		
		let instance_desc = vk::InstanceCreateInfo::builder()
			.push_next(&mut debugcreateinfo)
			.application_info(&app_desc)
			.enabled_extension_names(&instance_extensions_strings)
			//.enabled_extension_names(&extension_name_pointers)
			.enabled_layer_names(&layer_name_pointers);
		
		let instance = unsafe { entry.create_instance(&instance_desc, None)? };
	
		/* 
		let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);

		let utils_messenger = unsafe 
		{ 
			debug_utils.create_debug_utils_messenger(&debugcreateinfo, None)? 
		};
	*/
	
		// Create a surface from winit window.
		let surface = unsafe { ash_window::create_surface(&entry, &instance, window_handle, None)? };
		let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
		println!("surface: {:?}", surface);

		Ok( Self{_entry: entry, instance, surface, surface_fn} )
	}


}

impl Drop for VulkanObject
{
	fn drop(&mut self)
	{
		unsafe 
		{ 
			//debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
			self.surface_fn.destroy_surface(self.surface, None);
			self.instance.destroy_instance(None); 
		};
	
	}
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 
{
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}


fn vulkan_init() -> Result<(), Box<dyn std::error::Error>>
{

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Window", 800, 600)
        .vulkan()
        .build()
        .unwrap();


	let mut _vulkan_obj = VulkanObject::new(window.vulkan_instance_extensions()?, &window)?;



    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop 
	{
         for event in event_pump.poll_iter() 
		 {
            match event 
			{
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => 
				{
                    break 'running
                },
                _ => {}
            }
        } 
        ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }



	Ok(())
}

