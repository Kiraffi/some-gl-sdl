
#[cfg(windows)]
pub mod win32_win;
#[cfg(windows)]
use win32_win::*;



#[cfg(target_os = "linux")]
pub mod linux_win;
#[cfg(target_os = "linux")]
use linux_win::*;


fn main()
{
    #[cfg(windows)]
    win_main();

    #[cfg(target_os = "linux")]
    linux_main();
}