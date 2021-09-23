use std::os::raw::*;

pub type HANDLE = *mut c_void;
pub type CHARSTRING = *const c_char;
pub type BOOL = c_int;

#[cfg(windows)]
extern "system" 
{
    pub fn GetProcAddress(lib_module_handle: HANDLE, proc_name: CHARSTRING) -> HANDLE;
    pub fn LoadLibraryA(file_name: CHARSTRING) -> HANDLE;
    pub fn FreeLibrary(lib_module_handle: HANDLE) -> BOOL;
}

#[cfg(target_os = "linux")]
extern "system" 
{
    pub fn dlsym(lib_module_handle: HANDLE, proc_name: CHARSTRING) -> HANDLE;
    pub fn dlopen(file_name: CHARSTRING, flags: i32) -> HANDLE;
    pub fn dlclose(lib_module_handle: HANDLE) -> BOOL;
}
#[cfg(target_os = "linux")]
pub fn GetProcAddress(lib_module_handle: HANDLE, proc_name: CHARSTRING) -> HANDLE { dlsym(lib_module_handle, proc_name ) }
#[cfg(target_os = "linux")]
pub fn LoadLibraryA(file_name: CHARSTRING) -> HANDLE { dlopen(file_name, RTLD_LAZY ) }
#[cfg(target_os = "linux")]
 pub fn FreeLibrary(lib_module_handle: HANDLE) -> BOOL { dlclose(lib_module_handle) }



pub struct CarpLibLoader
{
    loaded_libs: Vec<(HANDLE, &'static str)>,
}

impl CarpLibLoader
{
    pub fn new() -> Self
    {
        return Self{ loaded_libs: Vec::new() };
    }
    pub fn load_lib(&mut self, lib_name: &'static str) -> Result<HANDLE, String>
    {
        let module = match std::ffi::CString::new(lib_name) 
        {
            Ok(lib_name) => unsafe 
            {
                LoadLibraryA(lib_name.as_ptr())
            },
            Err(_) => std::ptr::null_mut(),
        };

        if module.is_null()
        {
            return Err(format!("Failed to load library: {}", lib_name));
        }

        self.loaded_libs.push((module, lib_name));
        return Ok(module)
    }   
}


impl Drop for CarpLibLoader
{
	fn drop(&mut self)
	{
		unsafe
		{
			for lib in &self.loaded_libs
            {
                #[cfg(windows)]
                FreeLibrary(lib.0);
            }
            self.loaded_libs.clear();
		}
	}
}
