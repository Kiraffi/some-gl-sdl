extern crate sdl2;
//extern crate vulkano;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


use ash::{util::read_spv, vk::{self, DeviceQueueCreateInfo}};

fn main() 
{
	match vulkan_init()
	{
		Ok(_) => { println!("Run success!"); }
		Err(_r) => { println!("Error:"); }
	}
}


struct VulkanQueueIndices
{
	graphics_queue: u32,
	transfer_queue: u32,
	compute_queue: u32,
	surface_queue: u32,
}

impl VulkanQueueIndices
{
	fn is_valid(&self) -> bool
	{
		return self.graphics_queue != !0u32 && 
			self.compute_queue != !0u32 &&
			self.transfer_queue != !0u32 &&
			self.surface_queue != !0u32;
	}
}

struct VulkanQueues
{
	graphics_queue: vk::Queue,
	#[allow(dead_code)]
	compute_queue: vk::Queue,
	#[allow(dead_code)]
	transfer_queue: vk::Queue,
	surface_queue: vk::Queue
}

struct VulkanObject
{
	pub _entry: ash::Entry,
	pub instance: ash::Instance,
	pub surface: vk::SurfaceKHR,
	pub surface_fn: ash::extensions::khr::Surface,
 
	#[allow(dead_code)]
	pub physical_device: vk::PhysicalDevice,
	pub logical_device: ash::Device,
	#[allow(dead_code)]
	pub device_queue_indices: VulkanQueueIndices,
	pub device_queues: VulkanQueues,
 
	pub swapchain: vk::SwapchainKHR,
	pub swapchain_loader: ash::extensions::khr::Swapchain,
 
	pub swapchain_imageviews: Vec<vk::ImageView>,
 
	pub renderpass: vk::RenderPass,
	pub framebuffers: Vec<vk::Framebuffer>,
	pub pipeline: Pipeline,

	pub pools: Pools,
	pub commandbuffers: Vec<vk::CommandBuffer>,
	
}


fn find_swapchain(surface_queue: u32, instance : &ash::Instance, physical_device: &vk::PhysicalDevice, logical_device: &ash::Device, 
	surface: &vk::SurfaceKHR, surface_fn: &ash::extensions::khr::Surface) 
	-> Result<(vk::SwapchainKHR, ash::extensions::khr::Swapchain, vk::Extent2D), &'static str>
{
	let surface_capabilities = unsafe 
	{
		match surface_fn.get_physical_device_surface_capabilities(*physical_device, *surface)
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to query surface capabilities!")
		}
	};
	
	let _surface_present_modes = unsafe 
	{
		match surface_fn.get_physical_device_surface_present_modes(*physical_device, *surface)
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to query surface present modes!")
		}
	};
	let surface_formats = unsafe
	{ 
		match surface_fn.get_physical_device_surface_formats(*physical_device, *surface)
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to query surface formats!")
		}
	};
	//dbg!(&surface_capabilities);
	//dbg!(&surface_present_modes);
	//dbg!(&surface_formats);


	let queuefamilies = [surface_queue];
    let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(*surface)
        .min_image_count(
            3.max(surface_capabilities.min_image_count)
                .min(surface_capabilities.max_image_count),
        )
        .image_format(surface_formats.first().unwrap().format)
        .image_color_space(surface_formats.first().unwrap().color_space)
        .image_extent(surface_capabilities.current_extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .queue_family_indices(&queuefamilies)
        .pre_transform(surface_capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(vk::PresentModeKHR::FIFO);
    let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &logical_device);
    let swapchain = unsafe 
	{ 
		match swapchain_loader.create_swapchain(&swapchain_create_info, None)
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to create swapchain!")
		}
	 };
	
	Ok((swapchain, swapchain_loader, surface_capabilities.current_extent))
}

// Notice we try to find queue index that has all features, which isn't exactly best.
fn find_queues(instance : &ash::Instance, physical_device: &vk::PhysicalDevice, surface: &vk::SurfaceKHR,
	surface_fn: &ash::extensions::khr::Surface) -> VulkanQueueIndices
{
	let queuefamilyproperties =
        unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
	let mut graphics_queue = !0u32;
	let mut compute_queue = !0u32;
	let mut transfer_queue = !0u32;
	let mut surface_queue = !0u32;

	let mut max_simi = 0u32;
	for (index, qfam) in queuefamilyproperties.iter().enumerate() 
	{
		if qfam.queue_count > 0
		{
			let ind: u32 = index as u32;
			let is_surface = unsafe 
			{ 
				match surface_fn.get_physical_device_surface_support(*physical_device, ind, *surface)
				{
					Ok(v) => v,
					Err(_) => false,
				}
			};
			


			if qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS |	vk::QueueFlags::TRANSFER | vk::QueueFlags::COMPUTE)
			{
				graphics_queue = ind;
				compute_queue = ind;
				transfer_queue = ind;
				max_simi = 4;
				if is_surface
				{
					surface_queue = ind;
					break;
				}
			}
			else if qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE) && max_simi < 3
			{
				graphics_queue = ind;
				compute_queue = ind;
				if is_surface
				{
					surface_queue = ind;
					max_simi = 3;
				}
				else 
				{
					max_simi = 2;					
				}
			}
			if transfer_queue == !0u32 && qfam.queue_flags.contains(vk::QueueFlags::TRANSFER)
			{
				transfer_queue = ind;
			}
			if graphics_queue == !0u32 && qfam.queue_flags.contains(vk::QueueFlags::GRAPHICS)
			{
				graphics_queue = ind;
			}
			if compute_queue == !0u32 && qfam.queue_flags.contains(vk::QueueFlags::COMPUTE)
			{
				compute_queue = ind;
			}
			if surface_queue == !0u32 && is_surface
			{
				surface_queue = ind;
			}
		}
	}

	return VulkanQueueIndices{ graphics_queue, compute_queue, transfer_queue, surface_queue };
}

impl VulkanObject
{
	pub fn new(mut extension_strs: Vec<&'static str>, window_handle: &dyn raw_window_handle::HasRawWindowHandle)
		-> Result<Self, &'static str>
	{
		let entry: ash::Entry = unsafe 
		{ 
			match ash::Entry::new() { Ok(v) => v, Err(_) => return Err("Failed to open vulkan loader!") }
		};

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

		extension_strs.push(
			match ash::extensions::ext::DebugUtils::name().to_str()
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to push vulkan debug util extensions to array!")
			}
		);
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
		
		let instance = unsafe 
		{ 
			match entry.create_instance(&instance_desc, None) 
			{
				Ok(v) => v, Err(_) => return Err("Failed to create vulkan instance!")
			}
		};
	
		/* 
		let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);

		let utils_messenger = unsafe 
		{ 
			debug_utils.create_debug_utils_messenger(&debugcreateinfo, None)? 
		};
	*/
	
		// Create a surface from winit window.
		let surface = unsafe 
		{ 
			match ash_window::create_surface(&entry, &instance, window_handle, None)
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to create surface!")
			}
		};
		let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
		println!("surface: {:?}", surface);

		let phys_devs = unsafe 
		{ 
			match instance.enumerate_physical_devices()
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to enumerate devices!")
			}
		};
		let (physical_device, physical_device_properties, device_queue_indices) = 
		{
			let mut chosen = None;
			for p in phys_devs 
			{
				let queue = find_queues(&instance, &p, &surface, &surface_fn);
				let properties = unsafe { instance.get_physical_device_properties(p) };
				if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU && queue.is_valid()
				{
					chosen = Some((p, properties, queue));
					break;
				}
				else if properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU && queue.is_valid()
				{
					chosen = Some((p, properties, queue));
				}
			}
			chosen.unwrap()
		};

		if physical_device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
		{
			println!("Discrete gpu chosen!");
		}
		else if physical_device_properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU
		{
			println!("Integrated gpu chosen!");
		}

		let mut queue_indices: Vec<u32> = Vec::new();
		queue_indices.push(device_queue_indices.graphics_queue);
		if device_queue_indices.graphics_queue != device_queue_indices.compute_queue
		{
			queue_indices.push(device_queue_indices.compute_queue);
		}
		if device_queue_indices.transfer_queue != device_queue_indices.graphics_queue &&
			device_queue_indices.transfer_queue != device_queue_indices.compute_queue
		{
			queue_indices.push(device_queue_indices.transfer_queue);
		}

		if device_queue_indices.surface_queue != device_queue_indices.graphics_queue &&
			device_queue_indices.surface_queue != device_queue_indices.compute_queue &&
			device_queue_indices.surface_queue != device_queue_indices.transfer_queue
		{
			queue_indices.push(device_queue_indices.surface_queue);
		}


		let mut queue_infos: Vec<DeviceQueueCreateInfo> = Vec::new();
		let priorities = [1.0f32];

		for indice in queue_indices
		{
			queue_infos.push(	
				vk::DeviceQueueCreateInfo::builder()
				.queue_family_index(indice)
				.queue_priorities(&priorities)
				.build());
		}

		dbg!(&queue_infos);


	


		let device_extension_name_pointers: Vec<*const i8> =
	        vec![ash::extensions::khr::Swapchain::name().as_ptr()];

		let device_create_info = vk::DeviceCreateInfo::builder()
			.queue_create_infos(&queue_infos)
			.enabled_extension_names(&device_extension_name_pointers)
			.enabled_layer_names(&layer_name_pointers);

		let logical_device = unsafe 
		{ 
			match instance.create_device(physical_device, &device_create_info, None)
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to create logical device!")
			} 
		};
		
		let graphics_queue = unsafe { logical_device.get_device_queue(device_queue_indices.graphics_queue, 0) };
		let compute_queue = unsafe { logical_device.get_device_queue(device_queue_indices.compute_queue, 0) };
		let transfer_queue = unsafe { logical_device.get_device_queue(device_queue_indices.transfer_queue, 0) };
		let surface_queue = unsafe { logical_device.get_device_queue(device_queue_indices.surface_queue, 0) };
	

		let device_queues = VulkanQueues{ graphics_queue, compute_queue, transfer_queue, surface_queue };


		let (swapchain, swapchain_loader, extent) = find_swapchain(
			device_queue_indices.surface_queue, &instance, &physical_device, &logical_device, &surface, &surface_fn)?;


		let swapchain_images = unsafe 
		{ 
			match swapchain_loader.get_swapchain_images(swapchain)
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to get swapchain images!")
			} 
		};


		let mut swapchain_imageviews = Vec::with_capacity(swapchain_images.len());
		for image in &swapchain_images {
			let subresource_range = vk::ImageSubresourceRange::builder()
				.aspect_mask(vk::ImageAspectFlags::COLOR)
				.base_mip_level(0)
				.level_count(1)
				.base_array_layer(0)
				.layer_count(1);
			let imageview_create_info = vk::ImageViewCreateInfo::builder()
				.image(*image)
				.view_type(vk::ImageViewType::TYPE_2D)
				.format(vk::Format::B8G8R8A8_UNORM)
				.subresource_range(*subresource_range);
			let imageview = unsafe 
			{ 
				match logical_device.create_image_view(&imageview_create_info, None)
				{
					Ok(v) => v,
					Err(_) => return Err("Failed to create image view for swapchain!")
				}
			};
			swapchain_imageviews.push(imageview);
		}

		let renderpass = init_renderpass(&logical_device, &physical_device, &surface, &surface_fn)?;

		let framebuffers = 
			create_framebuffers(&logical_device, renderpass, &swapchain_imageviews, extent)?;


		let pipeline = match Pipeline::init(&logical_device, &renderpass, extent)
		{
			Ok(v) => v,
			Err(_) => return Err("failed to create pipeline")
		};





		let pools = match Pools::init(&logical_device, &device_queue_indices)
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to create pools")
		};

        let commandbuffers = match create_commandbuffers(&logical_device, &pools, framebuffers.len())
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to create command buffers")
		};

		match fill_commandbuffers(&commandbuffers, &logical_device, &renderpass, &framebuffers, &pipeline, extent )
		{
			Ok(_) => (),
			Err(_) => return Err("Failed to fill command buffers")
		};









		Ok( Self{_entry: entry, instance, surface, surface_fn,
			physical_device, logical_device, device_queue_indices, device_queues,
			swapchain, swapchain_loader, swapchain_imageviews, renderpass, framebuffers, pipeline, pools, commandbuffers } )
	}


}

impl Drop for VulkanObject
{
	fn drop(&mut self)
	{
		unsafe 
		{
			self.pools.cleanup(&self.logical_device);
			self.pipeline.cleanup(&self.logical_device);
			for imageview in &self.swapchain_imageviews 
			{
				self.logical_device.destroy_image_view(*imageview, None);
			}
			for fb in &self.framebuffers
			{
				self.logical_device.destroy_framebuffer(*fb, None);
			}
			self.swapchain_imageviews.clear();

			self.logical_device.destroy_render_pass(self.renderpass, None);

			self.swapchain_loader.destroy_swapchain(self.swapchain, None);
			self.logical_device.destroy_device(None);
			//debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
			self.surface_fn.destroy_surface(self.surface, None);
			self.instance.destroy_instance(None); 
		};
	
	}
}





struct Pipeline 
{
    pipeline: vk::Pipeline,
    layout: vk::PipelineLayout,
}

impl Pipeline 
{
    fn cleanup(&self, logical_device: &ash::Device) 
	{
        unsafe 
		{
            logical_device.destroy_pipeline(self.pipeline, None);
            logical_device.destroy_pipeline_layout(self.layout, None);
        }
    }

    fn init(logical_device: &ash::Device,renderpass: &vk::RenderPass, extent: vk::Extent2D ) -> Result<Pipeline, vk::Result>
	{
		let mut vertex_spv_file = std::io::Cursor::new(&include_bytes!("triangle_vert.spv")[..]);
        let mut frag_spv_file = std::io::Cursor::new(&include_bytes!("triangle_frag.spv")[..]);

        let vertex_code =
            read_spv(&mut vertex_spv_file).expect("Failed to read vertex shader spv file");
		let frag_code =
            read_spv(&mut frag_spv_file).expect("Failed to read vertex shader spv file");

        let vertexshader_createinfo = vk::ShaderModuleCreateInfo::builder().code(&vertex_code);
        let vertexshader_module =
            unsafe { logical_device.create_shader_module(&vertexshader_createinfo, None)? };
        let fragmentshader_createinfo = vk::ShaderModuleCreateInfo::builder().code(&frag_code);
        let fragmentshader_module =
            unsafe { logical_device.create_shader_module(&fragmentshader_createinfo, None)? };
        let mainfunctionname = std::ffi::CString::new("main").unwrap();
        let vertexshader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vertexshader_module)
            .name(&mainfunctionname);
        let fragmentshader_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(fragmentshader_module)
            .name(&mainfunctionname);
        let shader_stages = vec![vertexshader_stage.build(), fragmentshader_stage.build()];
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();
        let input_assembly_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
		// fix these to be optional?
		let viewports = [vk::Viewport 
		{
            x: 0.,
            y: 0.,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.,
            max_depth: 1.,
        }];
        let scissors = [vk::Rect2D
		{
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: extent,
        }];

        let viewport_info = vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterizer_info = vk::PipelineRasterizationStateCreateInfo::builder()
            .line_width(1.0)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .cull_mode(vk::CullModeFlags::NONE)
            .polygon_mode(vk::PolygonMode::FILL);
        let multisampler_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);
        let colourblend_attachments = [vk::PipelineColorBlendAttachmentState::builder()
            .blend_enable(true)
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .alpha_blend_op(vk::BlendOp::ADD)
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )
            .build()];
        let colourblend_info =
            vk::PipelineColorBlendStateCreateInfo::builder().attachments(&colourblend_attachments);
        let pipelinelayout_info = vk::PipelineLayoutCreateInfo::builder();
        let pipelinelayout =
            unsafe { logical_device.create_pipeline_layout(&pipelinelayout_info, None) }?;
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stages)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly_info)
            .viewport_state(&viewport_info)
            .rasterization_state(&rasterizer_info)
            .multisample_state(&multisampler_info)
            .color_blend_state(&colourblend_info)
            .layout(pipelinelayout)
            .render_pass(*renderpass)
            .subpass(0);
        let graphicspipeline = unsafe {
            logical_device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[pipeline_info.build()],
                    None,
                )
                .expect("A problem with the pipeline creation")
        }[0];
        unsafe {
            logical_device.destroy_shader_module(fragmentshader_module, None);
            logical_device.destroy_shader_module(vertexshader_module, None);
        }
        Ok(Pipeline {
            pipeline: graphicspipeline,
            layout: pipelinelayout,
        })
    }
}
















fn create_framebuffers(logical_device: &ash::Device, renderpass: vk::RenderPass, 
	imageviews: &Vec<vk::ImageView>, extent: vk::Extent2D ) -> Result<Vec<vk::Framebuffer>, &'static str> 
{
	let mut framebuffers: Vec<vk::Framebuffer> = Vec::new();
	for imageview in imageviews {
		let iview = [*imageview];
		let framebuffer_info = vk::FramebufferCreateInfo::builder()
			.render_pass(renderpass)
			.attachments(&iview)
			.width(extent.width)
			.height(extent.height)
			.layers(1);
		let fb = unsafe 
		{ 
			match logical_device.create_framebuffer(&framebuffer_info, None)
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to create framebuffer for swapchain image!")
			}
		};
		framebuffers.push(fb);
	}
	Ok(framebuffers)
}


fn init_renderpass(logical_device: &ash::Device, physical_device: &vk::PhysicalDevice,
	surface: &vk::SurfaceKHR, surface_fn: &ash::extensions::khr::Surface) 
		-> Result<vk::RenderPass, &'static str> 
{

	let surface_formats = unsafe 
	{
		match surface_fn.get_physical_device_surface_formats(*physical_device, *surface)
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to query surface formats!")
		}
	};



    let attachments = [vk::AttachmentDescription::builder()
        .format(surface_formats
                .first()
                .unwrap()
                .format,
        )
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .samples(vk::SampleCountFlags::TYPE_1)
        .build()];
    let color_attachment_references = [vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    }];
    let subpasses = [vk::SubpassDescription::builder()
        .color_attachments(&color_attachment_references)
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .build()];
    let subpass_dependencies = [vk::SubpassDependency::builder()
        .src_subpass(vk::SUBPASS_EXTERNAL)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_subpass(0)
        .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(
            vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        )
        .build()];
    let renderpass_info = vk::RenderPassCreateInfo::builder()
        .attachments(&attachments)
        .subpasses(&subpasses)
        .dependencies(&subpass_dependencies);
    let renderpass = unsafe 
	{ 
		match logical_device.create_render_pass(&renderpass_info, None)
		{
			Ok(v) => v,
			Err(_) => return Err("Failed to create render pass!")
		}
	};
    Ok(renderpass)
}


struct Pools {
    commandpool_graphics: vk::CommandPool,
    commandpool_transfer: vk::CommandPool,
}

impl Pools 
{
    fn init(logical_device: &ash::Device, device_queue_indices: &VulkanQueueIndices) -> Result<Pools, vk::Result>
	{
        let graphics_commandpool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(device_queue_indices.graphics_queue)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let commandpool_graphics =
            unsafe { logical_device.create_command_pool(&graphics_commandpool_info, None) }?;
        let transfer_commandpool_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(device_queue_indices.transfer_queue)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let commandpool_transfer =
            unsafe { logical_device.create_command_pool(&transfer_commandpool_info, None) }?;

        Ok(Pools { commandpool_graphics, commandpool_transfer })
    }
    fn cleanup(&self, logical_device: &ash::Device) 
	{
        unsafe 
		{
            logical_device.destroy_command_pool(self.commandpool_graphics, None);
            logical_device.destroy_command_pool(self.commandpool_transfer, None);
        }
    }
}

fn create_commandbuffers(logical_device: &ash::Device, pools: &Pools, amount: usize ) ->
	 Result<Vec<vk::CommandBuffer>, vk::Result>
{
    let commandbuf_allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(pools.commandpool_graphics)
        .command_buffer_count(amount as u32);
    unsafe { logical_device.allocate_command_buffers(&commandbuf_allocate_info) }
}

fn fill_commandbuffers(commandbuffers: &[vk::CommandBuffer], logical_device: &ash::Device,
    renderpass: &vk::RenderPass, framebuffers: &Vec<vk::Framebuffer>, pipeline: &Pipeline, extent: vk::Extent2D) 
		-> Result<(), vk::Result> 
{
    for (i, &commandbuffer) in commandbuffers.iter().enumerate() 
	{
        let commandbuffer_begininfo = vk::CommandBufferBeginInfo::builder();
        unsafe { logical_device.begin_command_buffer(commandbuffer, &commandbuffer_begininfo)?; }
        let clearvalues = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.08, 1.0],
            },
        }];
        let renderpass_begininfo = vk::RenderPassBeginInfo::builder()
            .render_pass(*renderpass)
            .framebuffer(framebuffers[i])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: extent,
            }).clear_values(&clearvalues);
        unsafe 
		{
            logical_device.cmd_begin_render_pass(
                commandbuffer,
                &renderpass_begininfo,
                vk::SubpassContents::INLINE,
            );
            logical_device.cmd_bind_pipeline(
                commandbuffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline.pipeline,
            );
            logical_device.cmd_draw(commandbuffer, 3, 1, 0, 0);
            logical_device.cmd_end_render_pass(commandbuffer);
            logical_device.end_command_buffer(commandbuffer)?;
        }
    }
    Ok(())
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


fn vulkan_init() -> Result<(), &'static str>
{

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Window", 800, 600)
        .vulkan()
        .build()
        .unwrap();


	let externsion_strs = match window.vulkan_instance_extensions()
	{
		Ok(v) => v,
		Err(_) => return Err("Failed to get required extensions from SDL") 
	};

	let vulkan_obj = VulkanObject::new(externsion_strs, &window)?;



	let mut image_available: Vec<vk::Semaphore> = Vec::new();
    let mut rendering_finished: Vec<vk::Semaphore> = Vec::new();
    let mut may_begin_drawing: Vec<vk::Fence> = Vec::new();
    let amount_of_images: u32 = vulkan_obj.swapchain_imageviews.len() as u32;
    let mut current_image: usize = 0;


	let semaphoreinfo = vk::SemaphoreCreateInfo::builder();
	let fenceinfo = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);
	for _ in 0..amount_of_images {
		let semaphore_available = unsafe 
		{ 
			match vulkan_obj.logical_device.create_semaphore(&semaphoreinfo, None) 
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to create available semaphore")
			}
		};
		let semaphore_finished = unsafe
		{ 
			match vulkan_obj.logical_device.create_semaphore(&semaphoreinfo, None) 
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to create finished semaphore")
			}
		};
		image_available.push(semaphore_available);
		rendering_finished.push(semaphore_finished);
		let fence = unsafe
		{ 
			match vulkan_obj.logical_device.create_fence(&fenceinfo, None)
			{
				Ok(v) => v,
				Err(_) => return Err("Failed to create fence")
			} 
		};
		may_begin_drawing.push(fence);
	}


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
 		let (image_index, _) = unsafe 
		{
			vulkan_obj.swapchain_loader.acquire_next_image(
					vulkan_obj.swapchain,
					std::u64::MAX,
					image_available[current_image],
					vk::Fence::null(),
				)
				.expect("image acquisition trouble")
		};
		unsafe 
		{
			vulkan_obj.logical_device.wait_for_fences(
					&[may_begin_drawing[current_image]],
					true,
					std::u64::MAX,
				).expect("fence-waiting");
			
				vulkan_obj.logical_device.reset_fences(&[may_begin_drawing[current_image]]
				).expect("resetting fences");
		}
		let semaphores_available =
			[image_available[current_image]];
		let waiting_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
		let semaphores_finished =
			[rendering_finished[current_image]];
		let commandbuffers = [vulkan_obj.commandbuffers[image_index as usize]];
		let submit_info = [vk::SubmitInfo::builder()
			.wait_semaphores(&semaphores_available)
			.wait_dst_stage_mask(&waiting_stages)
			.command_buffers(&commandbuffers)
			.signal_semaphores(&semaphores_finished)
			.build()];
		unsafe 
		{
			vulkan_obj.logical_device.queue_submit(
					vulkan_obj.device_queues.graphics_queue,
					&submit_info,
					may_begin_drawing[current_image],
				).expect("queue submission");
		};
		let swapchains = [vulkan_obj.swapchain];
		let indices = [image_index];
		let present_info = vk::PresentInfoKHR::builder()
			.wait_semaphores(&semaphores_finished)
			.swapchains(&swapchains)
			.image_indices(&indices);
		unsafe 
		{
			vulkan_obj.swapchain_loader
				.queue_present(vulkan_obj.device_queues.surface_queue, &present_info)
				.expect("queue presentation");
		};
		current_image = (current_image + 1) % amount_of_images as usize;

		::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 200));
	}


	unsafe 
	{
		vulkan_obj.logical_device.device_wait_idle().expect("something wrong while waiting");
		for fence in &may_begin_drawing 
		{
			vulkan_obj.logical_device.destroy_fence(*fence, None);
		}
		for semaphore in &image_available 
		{
			vulkan_obj.logical_device.destroy_semaphore(*semaphore, None);
		}
		for semaphore in &rendering_finished 
		{
			vulkan_obj.logical_device.destroy_semaphore(*semaphore, None);
		}
	}
	Ok(())
}

