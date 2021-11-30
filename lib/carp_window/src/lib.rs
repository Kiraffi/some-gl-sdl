
#[cfg(windows)]
pub mod win32_win;
#[cfg(windows)]
pub use win32_win::*;

#[cfg(target_os = "linux")]
pub mod linux_win;
#[cfg(target_os = "linux")]
pub use linux_win::*;
