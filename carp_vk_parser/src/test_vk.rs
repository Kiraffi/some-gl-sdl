use carp_lib_loader::*;


use super::vk_all::*;

use std::ptr::{null, null_mut};
use std::os::raw::*;


pub type PFN_vkVoidFunction = extern "system" fn() -> ();
pub const VK_NULL_HANDLE: u64 = 0u64;





extern "system"
{
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: * const c_char, ) -> PFN_vkVoidFunction;
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: * const c_char, ) -> PFN_vkVoidFunction;
}



/*

macro_rules! gl_macro_func_generator 
{
    ( $( $fn:ident ( $($arg:ident : $t:ty),* ) -> $res:ty ),* ) => 
    {
        mod __temp_funcs 
        {
            use super::*;

            $(
                pub static mut $fn: Option<extern "C" fn ($($arg: $t),*) -> $res> = None;
            )*
        }

        $(
            pub unsafe fn $fn($($arg: $t),*) -> $res 
            {
                __temp_funcs::$fn.unwrap()( $($arg),* )
            }
        )*

        pub fn load_with<F>(mut loadfn: F, instance: *const c_void ) -> bool  where F: FnMut(*const c_void, *const u8) -> PFN_vkVoidFunction
        {
            $(
                unsafe 
                {
                    println!("tyeing!");
                    let fn_name = stringify!($fn);
                    println!("{}", fn_name);
                    let proc_ptr = loadfn(instance, fn_name.as_ptr());
                    if proc_ptr.is_null()
                    {
                        println!("Load vk func {:?} failed.", fn_name);
                        return false;
                    }
                    __temp_funcs::$fn = Some(std::mem::transmute(proc_ptr));
                }
            )*
            return true;
        }
    };
}





gl_macro_func_generator!
(
    vkCreateInstance(pCreateInfo: * const VkInstanceCreateInfo, pAllocator: * const c_void, pInstance: * mut VkInstance) -> VkResult,
    vkDestroyInstance(instance: VkInstance, pAllocator: * const c_void) -> ()
);

*/
/*
fn load_funcs()
{
    //pub fn vkCreateInstance(pCreateInfo: * const VkInstanceCreateInfo, pAllocator: * const VkAllocationCallbacks, pInstance: * mut VkInstance,) -> VkResult;
    //pub fn vkDestroyInstance(instance: VkInstance, pAllocator: * const VkAllocationCallbacks,) -> ();


    pub fn vkCreateInstance(pCreateInfo: * const VkInstanceCreateInfo, pAllocator: * const c_void, pInstance: * mut VkInstance,) -> VkResult;
    pub fn vkDestroyInstance(instance: VkInstance, pAllocator: * const c_void,) -> ();
    pub fn vkGetDeviceProcAddr(device: VkDevice, pName: * const c_char, ) -> PFN_vkVoidFunction;
    pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: * const c_char, ) -> PFN_vkVoidFunction;
}
*/




#[cfg(target_os = "linux")]
const VULKAN_LIB: &str = "libvulkan.so.1";

#[cfg(windows)]
const VULKAN_LIB: &str = "vulkan-1.dll";




pub struct Vulkan 
{
    vulkan_lib: *mut c_void,
    vk_proc: *mut c_void,
}

impl Vulkan {
    pub fn new(carp_lib_loader: &mut CarpLibLoader) -> Result<Self, String> 
    {
        let vulkan_lib = carp_lib_loader.load_lib(VULKAN_LIB)?;

        let vk_proc = unsafe{ get_proc_address(vulkan_lib, b"vkGetInstanceProcAddr\0".as_ptr() as *const i8) } ;
        if vk_proc.is_null()
        {
            return Err("Failed to get instanceprocaddress".to_string());
        }
        //let _gl = load_with(&|s| vk_proc(null(), s) as PFN_vkVoidFunction);
        println!("load success?");


       // __temp_funcs::$fn = Some(std::mem::transmute(proc_ptr));

        let mut fn_inst_loader: fn(*const c_void, *const u8) -> PFN_vkVoidFunction = unsafe {  std::mem::transmute(vk_proc)  };
        println!("load success2?");
        let mut create_ptr = fn_inst_loader(null(), b"vkCreateInstance\0".as_ptr() );
        println!("load success3?");
        let mut crateInstanceFn: 
            fn(pCreateInfo: * const VkInstanceCreateInfo, pAllocator: * const c_void, pInstance: * mut VkInstance) -> VkResult =
            unsafe {  std::mem::transmute(create_ptr)  };
        
        println!("load success4?");
 
        

        //fn vkEnumerateInstanceExtensionProperties(pLayerName: * const c_char, pPropertyCount: * mut u32, pProperties: * mut VkExtensionProperties, ) -> VkResult;



        let mut ext_query_ptr = fn_inst_loader(null(), b"vkEnumerateInstanceExtensionProperties\0".as_ptr() );
        println!("load success5?");
        let mut ext_query_fn: 
            fn(pLayerName: * const c_char, pPropertyCount: * mut u32, pProperties: * mut VkExtensionProperties) -> VkResult =
            unsafe {  std::mem::transmute(ext_query_ptr)  };       
        println!("load success6?");
 


        let mut app_info = VkApplicationInfo::new();
        app_info.pApplicationName = b"Hello Triangle\0".as_ptr();
        app_info.applicationVersion = vk_make_version(0, 1, 2, 0);
        app_info.pEngineName = b"No Engine\0".as_ptr();
        app_info.engineVersion = vk_make_version(0, 1, 0, 0);
        app_info.apiVersion = vk_make_version(0, 1, 2, 0);

        let mut create_info = VkInstanceCreateInfo::new();
        create_info.pApplicationInfo = &app_info;

        let mut instance: VkInstance = 0;
        let result  = crateInstanceFn(&create_info, null(), &mut instance);




        let mut extensionCount = 0u32;
        let null_ext_mut = null_mut();
        let result = ext_query_fn(null(), &mut extensionCount, null_ext_mut);
        
        print!("result {}, ext count: {} \r\n", result as i32, extensionCount);


        let mut props: Vec<VkExtensionProperties> = Vec::new();
        props.resize(extensionCount as usize, VkExtensionProperties::new());

        let result = ext_query_fn(null(), &mut extensionCount, &mut props[0]);
        print!("result {}, ext count: {} \r\n", result as i32, extensionCount);
        
        for prop in &props
        {
            let mut s = String::new();
            for i in 0..prop.extensionName.len()
            {
                if prop.extensionName[i] == '\0' as c_uchar
                {
                    break;
                }
                s.push(prop.extensionName[i] as char);
            }
            
            
            println!("Extension: {}", &s);
        }

        Ok(
        Vulkan {
            vulkan_lib,
            vk_proc
            //GetInstanceProcAddr: func as *const c_void,
        })
    }
}



