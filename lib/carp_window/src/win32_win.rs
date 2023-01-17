#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

use core::ffi::*;

use window_state::*;

#[repr(C)]

pub struct CarpWindow
{
    pub window_state: WindowState,

    win32_hwnd : HWND,
    win32_dc : HDC,
    hglrc : HGLRC,
}

const class_name: *const i8 = b"my_very_own_carp_window\0x0".as_ptr() as *const i8;




impl Drop for CarpWindow
{
    fn drop(&mut self)
    {
        // destroy opengl
        unsafe
        {
            if !self.hglrc.is_null()
            {
                wglDeleteContext(self.hglrc);
                self.hglrc = 0 as HGLRC;
            }
        }

        if self.win32_hwnd.is_null()
        {
            return;
        }

        unsafe
        {
            DestroyWindow(self.win32_hwnd);
            self.win32_hwnd = 0 as HWND;

            UnregisterClassA(class_name, GetModuleHandleA(NULL as _));
        }
    }
}

impl CarpWindow
{


    pub fn enable_vsync(&mut self, enable_vsync: bool) -> Result<(), String>
    {
        let vsync_value = if enable_vsync {1} else {0};
        unsafe
        {
            if swapIntervalEXT.is_some()
            {
                if swapIntervalEXT.unwrap()(vsync_value)
                {
                    self.window_state.vsync = enable_vsync;
                    return Ok(());
                }
            }
        }
        self.window_state.vsync = true;
        return Ok(());
    }

    pub fn set_timer_resolution(&self, res: u32) -> u32
    {
        unsafe
        {
            return timeBeginPeriod(res);
        }
    }
    pub fn unset_timer_resolution(&self, res: u32) -> u32
    {
        unsafe
        {
            return timeEndPeriod(res);
        }
    }

    pub fn load_fn(&self, proc: &'static str) -> *const c_void
    {

        let proc_cstr = std::ffi::CString::new(proc).unwrap();
        let proc_ptr = proc_cstr.as_ptr();

        let proc = unsafe
        {
            wglGetProcAddress(proc_ptr as _)
        };

        return proc as _;
    }


    pub fn swap_buffer(&mut self)
    {
        unsafe
        {
            SwapBuffers(self.win32_dc);
        }
    }
    pub fn set_window_title(&mut self, title: &str)
    {
        let title_cstr = std::ffi::CString::new(title).unwrap();
        unsafe
        {
            // Set window title
            SetWindowTextA(self.win32_hwnd, title_cstr.as_ptr() as _);
        }

    }

    pub fn update(&mut self)
    {
        unsafe
        {
            let mut msg: MSG = std::mem::zeroed();

            global_key_downs = std::mem::zeroed();
            global_key_half_count = std::mem::zeroed();

            while PeekMessageA(&mut msg as *mut _ as _, NULL as _, 0, 0, PM_REMOVE) != 0
                && !self.window_state.quit
            {
                let mut dispatch = false;

                match msg.message
                {
                    WM_QUIT =>
                    {
                        self.window_state.quit = true;
                        continue;
                    },
                    WM_KEYDOWN | WM_SYSKEYDOWN =>
                    {
                        let button: u32 = HIWORD(msg.lParam as u32) as u32 & 0x1ff; //0x1ff;
                        let translated = get_key(button);
                        let mut transusize = translated as usize;
                        if transusize & 0x40000000 == 0x40000000
                        {
                            transusize = transusize & 0xfff + 128;
                        }
                        if translated != MyKey::InvalidKey && transusize < 512
                        {
                            global_key_downs[transusize] = 1;
                            global_key_half_count[transusize] += 1;
                        }
                        else
                        {
                            dispatch = true;
                        }

                        if translated as i32 == VK_ESCAPE
                        {
                            quit_requested = true;
                        }
                        println!("button down: {}, b2: {}, transl: {}",  button, msg.wParam, transusize);
                    },
                    _ =>
                    {
                        dispatch = true;
                    }
                }
                if dispatch
                {
                    TranslateMessage(&mut msg);
                    DispatchMessageA(&mut msg);
                }
            }

            for i in 0..512
            {
                if global_key_half_count[i] > 0
                {
                    println!("button: {} was pressed/released: {} times", i, global_key_half_count[i]);
                }
            }

            if resized
            {
                let mut rect: RECT = std::mem::zeroed();
                if GetClientRect(self.win32_hwnd, &mut rect) != 0
                {
                    let width = rect.right - rect.left;
                    let height = rect.bottom - rect.top;

                    if self.window_state.window_width != width || self.window_state.window_height != height
                    {
                        self.window_state.resized = true;
                    }
                    self.window_state.window_width = width;
                    self.window_state.window_height = height;


                    resized = false;
                }
            }
            if quit_requested
            {
                PostMessageA(self.win32_hwnd, WM_CLOSE, 0, 0);
            }
        }
    }

    pub fn init(width: i32, height: i32, title: &str) -> Result<Self, String>
    {

        let mut carp_wind = CarpWindow {
            window_state: unsafe { std::mem::zeroed() },
            win32_hwnd : 0 as HWND, win32_dc : 0 as HDC, hglrc : 0 as HGLRC,
        };


        let window_name = match std::ffi::CString::new(title)
        {
            Ok(str) => str,
            Err(e) => return Err(e.to_string())
        };
        unsafe
        {
            let mut wndclass: WNDCLASSA = std::mem::zeroed();

            wndclass.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
            wndclass.lpfnWndProc = Some(win32_wndproc);
            wndclass.hInstance = GetModuleHandleA(NULL as _);
            wndclass.hCursor = LoadCursorA(NULL as _, IDC_ARROW);
            wndclass.hIcon = LoadIconA(NULL as _, IDI_WINLOGO);
            wndclass.lpszClassName = class_name;
            RegisterClassA(&wndclass);

            let win_style: DWORD;
            let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
            let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0 };

            win_style = WS_OVERLAPPED | WS_CAPTION  | WS_THICKFRAME | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

            carp_wind.window_state.window_width = width;
            carp_wind.window_state.window_height = height;
            rect.right = width;
            rect.bottom = height;

            AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
            //AdjustWindowRect(&rect as *const _ as _, win_style, false as _);
            let win_width = rect.right - rect.left;
            let win_height = rect.bottom - rect.top;

            carp_wind.win32_hwnd = CreateWindowExA(win_ex_style, class_name, window_name.as_ptr() as _,
                win_style, CW_USEDEFAULT, CW_USEDEFAULT, win_width, win_height,
                NULL as _, // hWndParent
                NULL as _, // hMenu
                GetModuleHandleA(NULL as _), // hInstance
                NULL as _ // lParam
            );

            assert!(carp_wind.win32_hwnd.is_null() == false);

            if carp_wind.win32_hwnd.is_null()
            {
                return Err("Window handle is null!".to_string());
            }

            ShowWindow(carp_wind.win32_hwnd, SW_SHOW);
            let dc = GetDC(carp_wind.win32_hwnd);
            assert!(dc.is_null() == false);


            carp_wind.win32_dc = dc;

            if carp_wind.win32_dc.is_null()
            {
                return Err("Window dc is null!".to_string());
            }




            // Create opengl

            let mut px_format_desired : PIXELFORMATDESCRIPTOR = std::mem::zeroed();
            px_format_desired.nSize = std::mem::size_of_val(&px_format_desired) as u16;
            px_format_desired.nVersion = 1;
            px_format_desired.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
            px_format_desired.iPixelType = PFD_TYPE_RGBA;
            px_format_desired.cColorBits = 32;
            px_format_desired.cAlphaBits = 8;
            px_format_desired.iLayerType = PFD_MAIN_PLANE;

            let suggested_pixel_format_index = ChoosePixelFormat(carp_wind.win32_dc, &px_format_desired);
            let mut suggested_px_format : PIXELFORMATDESCRIPTOR = std::mem::zeroed();
            let descriptor_success = DescribePixelFormat(carp_wind.win32_dc, suggested_pixel_format_index,
                std::mem::size_of_val(&suggested_pixel_format_index) as _, &mut suggested_px_format);
            if descriptor_success == 0 && false
            {
                return Err("Failed to describe pixel format!".to_string());
            }

            if SetPixelFormat(carp_wind.win32_dc, suggested_pixel_format_index, &suggested_px_format) == 0
            {
                return Err("Failed to set pixel format!".to_string());
            }

            let rc = wglCreateContext(carp_wind.win32_dc);
            if rc.is_null()
            {
                return Err("Failed to create opengl context.".to_string());
            }

            carp_wind.hglrc = rc;

            // Check thread?
            if wglMakeCurrent(carp_wind.win32_dc, rc) == 0
            {
                return Err("Failed to make current opengl context.".to_string());
            }
            // should check extensions support

            let proc = wglGetProcAddress(b"wglCreateContextAttribsARB\0".as_ptr() as *const i8);
            if proc.is_null()
            {
                return Err("wglCreateContextAttribsARB not found!".to_string());
            }

            createContextAttribsARB = Some(std::mem::transmute_copy(&proc));

            let proc = wglGetProcAddress(b"wglSwapIntervalEXT\0".as_ptr() as *const i8);
            if !proc.is_null()
            {
                swapIntervalEXT = Some(std::mem::transmute_copy(&proc));
            }

            let attrs = [
                WGL_CONTEXT_MAJOR_VERSION_ARB, 4,
                WGL_CONTEXT_MINOR_VERSION_ARB, 5,
                WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
                //WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
                WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
                0, 0,
            ];


            let share_context: HGLRC = 0 as HGLRC;
            let modern_rc = createContextAttribsARB.unwrap()(carp_wind.win32_dc, share_context,  attrs.as_ptr() as *const i32);
            if !modern_rc.is_null()
            {
                if wglMakeCurrent(carp_wind.win32_dc, modern_rc) != 0
                {
                    wglDeleteContext(carp_wind.hglrc);
                    carp_wind.hglrc = modern_rc;
                }
            }
        }
        carp_wind.window_state.resized = true;

        return Ok(carp_wind);

    }
}



type UINT_PTR = usize;
type LONG_PTR = isize;

type DWORD = c_ulong;
type BOOL = c_int;
type BYTE = c_uchar;
type WORD = c_ushort;

type LPVOID = *mut c_void;
type INT = c_int;
type UINT = c_uint;
type WPARAM = UINT_PTR;
type LPARAM = LONG_PTR;
type LRESULT = LONG_PTR;
type CHAR = c_char;

type LPCSTR = *const CHAR;


type PVOID = *mut c_void;

const NULL: PVOID = 0 as PVOID;



pub enum EMPTYTYPE {}
type HINSTANCE = *mut EMPTYTYPE;
type HWND = *mut EMPTYTYPE;
type HICON = *mut EMPTYTYPE;
type HMENU = *mut EMPTYTYPE;
type HCURSOR = *mut EMPTYTYPE;
type HBRUSH = *mut EMPTYTYPE;
type HMODULE = *mut EMPTYTYPE;


type HDC = *mut EMPTYTYPE;
type HGLRC = *mut EMPTYTYPE;







type PROC = *mut EMPTYTYPE;


type ATOM = WORD;


type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;

static mut createContextAttribsARB: Option<extern "system" fn(_: HDC, _: HGLRC, _: *const INT) -> HGLRC> = None;
static mut swapIntervalEXT: Option<extern "system" fn(_: i32) -> bool> = None;



#[repr(C)]

struct WNDCLASSA
{
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
}



#[repr(C)]
struct WNDCLASSEXA
{
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCSTR,
    lpszClassName: LPCSTR,
    hIconSm: HICON,
}



#[link(name = "user32")]
#[link(name = "gdi32")]
#[link(name = "winmm")]
extern "system"
{
    fn MapVirtualKeyA(uCode: UINT, uMapType: UINT) -> UINT;


    fn timeBeginPeriod(uPeriod: UINT) -> u32;
    fn timeEndPeriod(uPeriod: UINT) -> u32;

    fn CreateWindowExA(dwExStyle: DWORD, lpClassName: LPCSTR, lpWindowName: LPCSTR, dwStyle: DWORD,
        x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hWndParent: HWND, hMenu: HMENU, hInstance: HINSTANCE, lpParam: LPVOID) -> HWND;

    /*
    fn FreeLibrary(hLibModule: HMODULE) -> BOOL;
    */
    fn GetModuleHandleA(lpModuleName: LPCSTR) -> HMODULE;

    /*
    fn GetModuleHandleExA(dwFlags: DWORD, lpModuleName: LPCSTR, phModule: *mut HMODULE) -> BOOL;
    fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    fn LoadLibraryExA(lpLibFileName: LPCSTR, hFile: HANDLE, dwFlags: DWORD) -> HMODULE;
    */
    /*
    fn LoadResource(hModule: HMODULE, hResInfo: HRSRC) -> HGLOBAL;
    fn LoadStringA(hInstance: HINSTANCE, uID: UINT, lpBuffer: LPSTR, cchBufferMax: c_int) -> c_int;
    fn LockResource(hResData: HGLOBAL) -> LPVOID;
    fn SizeofResource(hModule: HMODULE, hResInfo: HRSRC) -> DWORD;
    */


    fn PostQuitMessage(nExitCode: c_int);
    fn DefWindowProcA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;

    fn LoadCursorA(hInstance: HINSTANCE, lpCursorName: LPCSTR) -> HCURSOR;
    fn LoadIconA(hInstance: HINSTANCE, lpIconName: LPCSTR) -> HICON;

    /*
    fn GetDoubleClickTime() -> UINT;
    fn SetDoubleClickTime(uInterval: UINT) -> BOOL;
    */
    fn RegisterClassA(lpWndClass: *const WNDCLASSA) -> ATOM;
    fn UnregisterClassA(lpClassName: LPCSTR,hInstance: HINSTANCE) -> BOOL;




    fn SetWindowTextA(hWnd: HWND, lpString: LPCSTR) -> BOOL;
    /*
    fn SetWindowTextW(hWnd: HWND, lpString: LPCWSTR) -> BOOL;
    fn GetWindowTextA(hWnd: HWND, lpString: LPSTR, nMaxCount: c_int) -> c_int;
    */
    // fn GetWindowTextLengthA(hWnd: HWND) -> c_int;

    /*
    //fn GetWindowTextLengthW(hWnd: HWND) -> c_int;


    */
    fn GetClientRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    fn GetWindowRect(hWnd: HWND, lpRect: LPRECT) -> BOOL;
    fn AdjustWindowRectEx(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD) -> BOOL;
    fn AdjustWindowRect(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL) -> BOOL;

    /*
    fn AdjustWindowRectExForDpi(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL, dwExStyle: DWORD, dpi: UINT) -> BOOL;
    */







    fn DestroyWindow(hWnd: HWND) -> BOOL;
    fn ShowWindow(hWnd: HWND, nCmdShow: c_int) -> BOOL;

    //fn WindowFromDC(hDC: HDC) -> HWND;
    fn GetDC(hWnd: HWND) -> HDC;
    //fn GetDCEx(hWnd: HWND, hrgnClip: HRGN, flags: DWORD) -> HDC;

    //fn GetMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT) -> BOOL;
    fn TranslateMessage(lpmsg: *mut MSG) -> BOOL;
    fn DispatchMessageA(lpmsg: *mut MSG) -> LRESULT;
    //fn SetMessageQueue(cMessagesMax: c_int) -> BOOL;
    fn PeekMessageA(lpMsg: LPMSG, hWnd: HWND, wMsgFilterMin: UINT, wMsgFilterMax: UINT, wRemoveMsg: UINT) -> BOOL;


    fn PostMessageA(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;

    fn ChoosePixelFormat(hdc: HDC, ppfd: *const PIXELFORMATDESCRIPTOR) -> c_int;

    fn DescribePixelFormat(hdc: HDC, iPixelFormat: c_int, nBytes: UINT, ppfd: LPPIXELFORMATDESCRIPTOR) -> c_int;

    fn SetPixelFormat(hdc: HDC, iPixelFormat: c_int, ppfd: *const PIXELFORMATDESCRIPTOR) -> BOOL;

    fn wglCreateContext(hdc: HDC) -> HGLRC;

    fn wglDeleteContext(hglrc: HGLRC) -> BOOL;
    //fn wglGetCurrentContext() -> HGLRC;
    //fn wglGetCurrentDC() -> HDC;
    fn wglGetProcAddress(lpszProc: LPCSTR) -> PROC;
    fn wglMakeCurrent(hdc: HDC, hglrc: HGLRC) -> BOOL;

    fn SwapBuffers(hdc: HDC) -> BOOL;
}


const IDC_ARROW: LPCSTR = 32512 as LPCSTR;
const IDI_WINLOGO: LPCSTR = 32517 as LPCSTR;


type LONG = c_long;

#[repr(C)]
struct POINT
{
    x: LONG,
    y: LONG,
}

#[repr(C)]

struct RECT
{
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

type LPRECT = *mut RECT;

#[repr(C)]
struct MSG
{
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}

type LPMSG = *mut MSG;

const CW_USEDEFAULT: c_int = 0x80000000u32 as i32;

/*
const WM_NULL: UINT = 0x0000;
const WM_CREATE: UINT = 0x0001;
const WM_DESTROY: UINT = 0x0002;


const WM_ACTIVATE: UINT = 0x0006;

const WA_INACTIVE: WORD = 0;
const WA_ACTIVE: WORD = 1;
const WA_CLICKACTIVE: WORD = 2;
const WM_SETFOCUS: UINT = 0x0007;
const WM_KILLFOCUS: UINT = 0x0008;
const WM_ENABLE: UINT = 0x000A;
const WM_SETREDRAW: UINT = 0x000B;
const WM_SETTEXT: UINT = 0x000C;
const WM_GETTEXT: UINT = 0x000D;
const WM_GETTEXTLENGTH: UINT = 0x000E;
const WM_PAINT: UINT = 0x000F;
*/
const WM_CLOSE: UINT = 0x0010;
const WM_SIZE: UINT = 0x0005;
const WM_MOVE: UINT = 0x0003;
/*
const WM_QUERYENDSESSION: UINT = 0x0011;
const WM_QUERYOPEN: UINT = 0x0013;
const WM_ENDSESSION: UINT = 0x0016;
*/
const WM_QUIT: UINT = 0x0012;
//const WM_WINDOWPOSCHANGED : UINT = 0x0047;
const WM_WINDOWPOSCHANGING : UINT = 0x0046;


const WM_WINDOWPOSCHANGED : UINT = 0x0046;

/*
const WM_ERASEBKGND: UINT = 0x0014;
const WM_SYSCOLORCHANGE: UINT = 0x0015;
const WM_SHOWWINDOW: UINT = 0x0018;
const WM_WININICHANGE: UINT = 0x001A;
const WM_SETTINGCHANGE: UINT = WM_WININICHANGE;
const WM_DEVMODECHANGE: UINT = 0x001B;
const WM_ACTIVATEAPP: UINT = 0x001C;
const WM_FONTCHANGE: UINT = 0x001D;
const WM_TIMECHANGE: UINT = 0x001E;
const WM_CANCELMODE: UINT = 0x001F;
const WM_SETCURSOR: UINT = 0x0020;
const WM_MOUSEACTIVATE: UINT = 0x0021;
const WM_CHILDACTIVATE: UINT = 0x0022;
const WM_QUEUESYNC: UINT = 0x0023;
const WM_GETMINMAXINFO: UINT = 0x0024;
*/












const WS_OVERLAPPED: DWORD = 0x00000000;
const WS_CAPTION: DWORD = 0x00C00000;
const WS_SYSMENU: DWORD = 0x00080000;
const WS_THICKFRAME: DWORD = 0x00040000;
const WS_MINIMIZEBOX: DWORD = 0x00020000;
const WS_MAXIMIZEBOX: DWORD = 0x00010000;
const WS_EX_WINDOWEDGE: DWORD = 0x00000100;
const WS_EX_APPWINDOW: DWORD = 0x00040000;

/*
const WS_VSCROLL: DWORD = 0x00200000;
const WS_HSCROLL: DWORD = 0x00100000;
const WS_TABSTOP: DWORD = 0x00010000;

const WS_POPUP: DWORD = 0x80000000;
const WS_CHILD: DWORD = 0x40000000;
const WS_MINIMIZE: DWORD = 0x20000000;
const WS_VISIBLE: DWORD = 0x10000000;
const WS_DISABLED: DWORD = 0x08000000;
const WS_CLIPSIBLINGS: DWORD = 0x04000000;
const WS_CLIPCHILDREN: DWORD = 0x02000000;
const WS_MAXIMIZE: DWORD = 0x01000000;
const WS_BORDER: DWORD = 0x00800000;
const WS_DLGFRAME: DWORD = 0x00400000;
const WS_GROUP: DWORD = 0x00020000;
const WS_TILED: DWORD = WS_OVERLAPPED;
const WS_ICONIC: DWORD = WS_MINIMIZE;
const WS_SIZEBOX: DWORD = WS_THICKFRAME;
const WS_TILEDWINDOW: DWORD = WS_OVERLAPPEDWINDOW;
const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME
                | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
const WS_POPUPWINDOW: DWORD = WS_POPUP | WS_BORDER | WS_SYSMENU;
const WS_CHILDWINDOW: DWORD = WS_CHILD;
const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;
const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;
const WS_EX_TOPMOST: DWORD = 0x00000008;
const WS_EX_ACCEPTFILES: DWORD = 0x00000010;
const WS_EX_TRANSPARENT: DWORD = 0x00000020;
const WS_EX_MDICHILD: DWORD = 0x00000040;
const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
const WS_EX_CONTEXTHELP: DWORD = 0x00000400;
const WS_EX_RIGHT: DWORD = 0x00001000;
const WS_EX_LEFT: DWORD = 0x00000000;
const WS_EX_RTLREADING: DWORD = 0x00002000;
const WS_EX_LTRREADING: DWORD = 0x00000000;
const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;
const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;
const WS_EX_CONTROLPARENT: DWORD = 0x00010000;
const WS_EX_STATICEDGE: DWORD = 0x00020000;
const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
const WS_EX_PALETTEWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
const WS_EX_LAYERED: DWORD = 0x00080000;
const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;
const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;
const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
const WS_EX_COMPOSITED: DWORD = 0x02000000;
const WS_EX_NOACTIVATE: DWORD = 0x08000000;
*/
const CS_VREDRAW: UINT = 0x0001;
const CS_HREDRAW: UINT = 0x0002;
//const CS_DBLCLKS: UINT = 0x0008;
const CS_OWNDC: UINT = 0x0020;

/*
const CS_CLASSDC: UINT = 0x0040;
const CS_PARENTDC: UINT = 0x0080;
const CS_NOCLOSE: UINT = 0x0200;
const CS_SAVEBITS: UINT = 0x0800;
const CS_BYTEALIGNCLIENT: UINT = 0x1000;
const CS_BYTEALIGNWINDOW: UINT = 0x2000;
const CS_GLOBALCLASS: UINT = 0x4000;
const CS_IME: UINT = 0x00010000;
const CS_DROPSHADOW: UINT = 0x00020000;
*/
























const SW_SHOW: c_int = 5;

const VK_LBUTTON: c_int = 0x01;
const VK_RBUTTON: c_int = 0x02;
const VK_CANCEL: c_int = 0x03;
const VK_MBUTTON: c_int = 0x04;
const VK_XBUTTON1: c_int = 0x05;
const VK_XBUTTON2: c_int = 0x06;
const VK_BACK: c_int = 0x08;
const VK_TAB: c_int = 0x09;
const VK_CLEAR: c_int = 0x0C;
const VK_RETURN: c_int = 0x0D;
const VK_SHIFT: c_int = 0x10;
const VK_CONTROL: c_int = 0x11;
const VK_MENU: c_int = 0x12;
const VK_PAUSE: c_int = 0x13;
const VK_CAPITAL: c_int = 0x14;
const VK_KANA: c_int = 0x15;
const VK_HANGEUL: c_int = 0x15;
const VK_HANGUL: c_int = 0x15;
const VK_JUNJA: c_int = 0x17;
const VK_FINAL: c_int = 0x18;
const VK_HANJA: c_int = 0x19;
const VK_KANJI: c_int = 0x19;
const VK_ESCAPE: c_int = 0x1B;
const VK_CONVERT: c_int = 0x1C;
const VK_NONCONVERT: c_int = 0x1D;
const VK_ACCEPT: c_int = 0x1E;
const VK_MODECHANGE: c_int = 0x1F;
const VK_SPACE: c_int = 0x20;
const VK_PRIOR: c_int = 0x21;
const VK_NEXT: c_int = 0x22;
const VK_END: c_int = 0x23;
const VK_HOME: c_int = 0x24;
const VK_LEFT: c_int = 0x25;
const VK_UP: c_int = 0x26;
const VK_RIGHT: c_int = 0x27;
const VK_DOWN: c_int = 0x28;
const VK_SELECT: c_int = 0x29;
const VK_PRINT: c_int = 0x2A;
const VK_EXECUTE: c_int = 0x2B;
const VK_SNAPSHOT: c_int = 0x2C;
const VK_INSERT: c_int = 0x2D;
const VK_DELETE: c_int = 0x2E;
const VK_HELP: c_int = 0x2F;
const VK_LWIN: c_int = 0x5B;
const VK_RWIN: c_int = 0x5C;
const VK_APPS: c_int = 0x5D;
const VK_SLEEP: c_int = 0x5F;
const VK_NUMPAD0: c_int = 0x60;
const VK_NUMPAD1: c_int = 0x61;
const VK_NUMPAD2: c_int = 0x62;
const VK_NUMPAD3: c_int = 0x63;
const VK_NUMPAD4: c_int = 0x64;
const VK_NUMPAD5: c_int = 0x65;
const VK_NUMPAD6: c_int = 0x66;
const VK_NUMPAD7: c_int = 0x67;
const VK_NUMPAD8: c_int = 0x68;
const VK_NUMPAD9: c_int = 0x69;
const VK_MULTIPLY: c_int = 0x6A;
const VK_ADD: c_int = 0x6B;
const VK_SEPARATOR: c_int = 0x6C;
const VK_SUBTRACT: c_int = 0x6D;
const VK_DECIMAL: c_int = 0x6E;
const VK_DIVIDE: c_int = 0x6F;
const VK_F1: c_int = 0x70;
const VK_F2: c_int = 0x71;
const VK_F3: c_int = 0x72;
const VK_F4: c_int = 0x73;
const VK_F5: c_int = 0x74;
const VK_F6: c_int = 0x75;
const VK_F7: c_int = 0x76;
const VK_F8: c_int = 0x77;
const VK_F9: c_int = 0x78;
const VK_F10: c_int = 0x79;
const VK_F11: c_int = 0x7A;
const VK_F12: c_int = 0x7B;
const VK_F13: c_int = 0x7C;
const VK_F14: c_int = 0x7D;
const VK_F15: c_int = 0x7E;
const VK_F16: c_int = 0x7F;
const VK_F17: c_int = 0x80;
const VK_F18: c_int = 0x81;
const VK_F19: c_int = 0x82;
const VK_F20: c_int = 0x83;
const VK_F21: c_int = 0x84;
const VK_F22: c_int = 0x85;
const VK_F23: c_int = 0x86;
const VK_F24: c_int = 0x87;
const VK_NAVIGATION_VIEW: c_int = 0x88;
const VK_NAVIGATION_MENU: c_int = 0x89;
const VK_NAVIGATION_UP: c_int = 0x8A;
const VK_NAVIGATION_DOWN: c_int = 0x8B;
const VK_NAVIGATION_LEFT: c_int = 0x8C;
const VK_NAVIGATION_RIGHT: c_int = 0x8D;
const VK_NAVIGATION_ACCEPT: c_int = 0x8E;
const VK_NAVIGATION_CANCEL: c_int = 0x8F;
const VK_NUMLOCK: c_int = 0x90;
const VK_SCROLL: c_int = 0x91;
const VK_OEM_NEC_EQUAL: c_int = 0x92;
const VK_OEM_FJ_JISHO: c_int = 0x92;
const VK_OEM_FJ_MASSHOU: c_int = 0x93;
const VK_OEM_FJ_TOUROKU: c_int = 0x94;
const VK_OEM_FJ_LOYA: c_int = 0x95;
const VK_OEM_FJ_ROYA: c_int = 0x96;
const VK_LSHIFT: c_int = 0xA0;
const VK_RSHIFT: c_int = 0xA1;
const VK_LCONTROL: c_int = 0xA2;
const VK_RCONTROL: c_int = 0xA3;
const VK_LMENU: c_int = 0xA4;
const VK_RMENU: c_int = 0xA5;
const VK_BROWSER_BACK: c_int = 0xA6;
const VK_BROWSER_FORWARD: c_int = 0xA7;
const VK_BROWSER_REFRESH: c_int = 0xA8;
const VK_BROWSER_STOP: c_int = 0xA9;
const VK_BROWSER_SEARCH: c_int = 0xAA;
const VK_BROWSER_FAVORITES: c_int = 0xAB;
const VK_BROWSER_HOME: c_int = 0xAC;
const VK_VOLUME_MUTE: c_int = 0xAD;
const VK_VOLUME_DOWN: c_int = 0xAE;
const VK_VOLUME_UP: c_int = 0xAF;
const VK_MEDIA_NEXT_TRACK: c_int = 0xB0;
const VK_MEDIA_PREV_TRACK: c_int = 0xB1;
const VK_MEDIA_STOP: c_int = 0xB2;
const VK_MEDIA_PLAY_PAUSE: c_int = 0xB3;
const VK_LAUNCH_MAIL: c_int = 0xB4;
const VK_LAUNCH_MEDIA_SELECT: c_int = 0xB5;
const VK_LAUNCH_APP1: c_int = 0xB6;
const VK_LAUNCH_APP2: c_int = 0xB7;
const VK_OEM_1: c_int = 0xBA;
const VK_OEM_PLUS: c_int = 0xBB;
const VK_OEM_COMMA: c_int = 0xBC;
const VK_OEM_MINUS: c_int = 0xBD;
const VK_OEM_PERIOD: c_int = 0xBE;
const VK_OEM_2: c_int = 0xBF;
const VK_OEM_3: c_int = 0xC0;
const VK_GAMEPAD_A: c_int = 0xC3;
const VK_GAMEPAD_B: c_int = 0xC4;
const VK_GAMEPAD_X: c_int = 0xC5;
const VK_GAMEPAD_Y: c_int = 0xC6;
const VK_GAMEPAD_RIGHT_SHOULDER: c_int = 0xC7;
const VK_GAMEPAD_LEFT_SHOULDER: c_int = 0xC8;
const VK_GAMEPAD_LEFT_TRIGGER: c_int = 0xC9;
const VK_GAMEPAD_RIGHT_TRIGGER: c_int = 0xCA;
const VK_GAMEPAD_DPAD_UP: c_int = 0xCB;
const VK_GAMEPAD_DPAD_DOWN: c_int = 0xCC;
const VK_GAMEPAD_DPAD_LEFT: c_int = 0xCD;
const VK_GAMEPAD_DPAD_RIGHT: c_int = 0xCE;
const VK_GAMEPAD_MENU: c_int = 0xCF;
const VK_GAMEPAD_VIEW: c_int = 0xD0;
const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: c_int = 0xD1;
const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: c_int = 0xD2;
const VK_GAMEPAD_LEFT_THUMBSTICK_UP: c_int = 0xD3;
const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: c_int = 0xD4;
const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: c_int = 0xD5;
const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: c_int = 0xD6;
const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: c_int = 0xD7;
const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: c_int = 0xD8;
const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: c_int = 0xD9;
const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: c_int = 0xDA;
const VK_OEM_4: c_int = 0xDB;
const VK_OEM_5: c_int = 0xDC;
const VK_OEM_6: c_int = 0xDD;
const VK_OEM_7: c_int = 0xDE;
const VK_OEM_8: c_int = 0xDF;
const VK_OEM_AX: c_int = 0xE1;
const VK_OEM_102: c_int = 0xE2;
const VK_ICO_HELP: c_int = 0xE3;
const VK_ICO_00: c_int = 0xE4;
const VK_PROCESSKEY: c_int = 0xE5;
const VK_ICO_CLEAR: c_int = 0xE6;
const VK_PACKET: c_int = 0xE7;
const VK_OEM_RESET: c_int = 0xE9;
const VK_OEM_JUMP: c_int = 0xEA;
const VK_OEM_PA1: c_int = 0xEB;
const VK_OEM_PA2: c_int = 0xEC;
const VK_OEM_PA3: c_int = 0xED;
const VK_OEM_WSCTRL: c_int = 0xEE;
const VK_OEM_CUSEL: c_int = 0xEF;
const VK_OEM_ATTN: c_int = 0xF0;
const VK_OEM_FINISH: c_int = 0xF1;
const VK_OEM_COPY: c_int = 0xF2;
const VK_OEM_AUTO: c_int = 0xF3;
const VK_OEM_ENLW: c_int = 0xF4;
const VK_OEM_BACKTAB: c_int = 0xF5;
const VK_ATTN: c_int = 0xF6;
const VK_CRSEL: c_int = 0xF7;
const VK_EXSEL: c_int = 0xF8;
const VK_EREOF: c_int = 0xF9;
const VK_PLAY: c_int = 0xFA;
const VK_ZOOM: c_int = 0xFB;
const VK_NONAME: c_int = 0xFC;
const VK_PA1: c_int = 0xFD;
const VK_OEM_CLEAR: c_int = 0xFE;
const WH_MIN: c_int = -1;
const WH_MSGFILTER: c_int = -1;
const WH_JOURNALRECORD: c_int = 0;
const WH_JOURNALPLAYBACK: c_int = 1;
const WH_KEYBOARD: c_int = 2;
const WH_GETMESSAGE: c_int = 3;
const WH_CALLWNDPROC: c_int = 4;
const WH_CBT: c_int = 5;
const WH_SYSMSGFILTER: c_int = 6;
const WH_MOUSE: c_int = 7;
const WH_HARDWARE: c_int = 8;
const WH_DEBUG: c_int = 9;
const WH_SHELL: c_int = 10;
const WH_FOREGROUNDIDLE: c_int = 11;
const WH_CALLWNDPROCRET: c_int = 12;
const WH_KEYBOARD_LL: c_int = 13;
const WH_MOUSE_LL: c_int = 14;
const WH_MAX: c_int = 14;
const WH_MINHOOK: c_int = WH_MIN;
const WH_MAXHOOK: c_int = WH_MAX;
const HC_ACTION: c_int = 0;
const HC_GETNEXT: c_int = 1;
const HC_SKIP: c_int = 2;
const HC_NOREMOVE: c_int = 3;
const HC_NOREM: c_int = HC_NOREMOVE;
const HC_SYSMODALON: c_int = 4;
const HC_SYSMODALOFF: c_int = 5;
const HCBT_MOVESIZE: c_int = 0;
const HCBT_MINMAX: c_int = 1;
const HCBT_QS: c_int = 2;
const HCBT_CREATEWND: c_int = 3;
const HCBT_DESTROYWND: c_int = 4;
const HCBT_ACTIVATE: c_int = 5;
const HCBT_CLICKSKIPPED: c_int = 6;
const HCBT_KEYSKIPPED: c_int = 7;
const HCBT_SYSCOMMAND: c_int = 8;
const HCBT_SETFOCUS: c_int = 9;









const WM_MOUSEFIRST: UINT = 0x0200;
const WM_MOUSEMOVE: UINT = 0x0200;
const WM_LBUTTONDOWN: UINT = 0x0201;
const WM_LBUTTONUP: UINT = 0x0202;
const WM_LBUTTONDBLCLK: UINT = 0x0203;
const WM_RBUTTONDOWN: UINT = 0x0204;
const WM_RBUTTONUP: UINT = 0x0205;
const WM_RBUTTONDBLCLK: UINT = 0x0206;
const WM_MBUTTONDOWN: UINT = 0x0207;
const WM_MBUTTONUP: UINT = 0x0208;
const WM_MBUTTONDBLCLK: UINT = 0x0209;
const WM_MOUSEWHEEL: UINT = 0x020A;
const WM_XBUTTONDOWN: UINT = 0x020B;
const WM_XBUTTONUP: UINT = 0x020C;
const WM_XBUTTONDBLCLK: UINT = 0x020D;
const WM_MOUSEHWHEEL: UINT = 0x020E;
const WM_MOUSELAST: UINT = 0x020E;


const WM_NCMOUSEMOVE: UINT = 0x00A0;
const WM_NCLBUTTONDOWN: UINT = 0x00A1;
const WM_NCLBUTTONUP: UINT = 0x00A2;
const WM_NCLBUTTONDBLCLK: UINT = 0x00A3;
const WM_NCRBUTTONDOWN: UINT = 0x00A4;
const WM_NCRBUTTONUP: UINT = 0x00A5;
const WM_NCRBUTTONDBLCLK: UINT = 0x00A6;
const WM_NCMBUTTONDOWN: UINT = 0x00A7;
const WM_NCMBUTTONUP: UINT = 0x00A8;
const WM_NCMBUTTONDBLCLK: UINT = 0x00A9;





const SPI_SETSNAPTODEFBUTTON: UINT = 0x0060;
const WM_DWMNCRENDERINGCHANGED: UINT = 0x031F;
const WM_PAINT: UINT = 0x000F;
const WM_NCMOUSELEAVE: UINT = 0x02A2;

//const WM_INPUT: UINT = 0x00FF;
//const WM_KEYFIRST: UINT = 0x0100;
const WM_KEYDOWN: UINT = 0x0100;
//const WM_KEYUP: UINT = 0x0101;
const WM_SYSKEYDOWN: UINT = 0x0104;
//const WM_SYSKEYUP: UINT = 0x0105;


const WM_TIMER: UINT = 0x0113;

static mut quit_requested : bool = false;
static mut resized: bool = false;

fn LOWORD(l: DWORD) -> WORD
{
    return (l & 0xffff) as WORD;
}
fn HIWORD(l: DWORD) -> WORD
{
    return ((l >> 16) & 0xffff) as WORD;
}

//const WM_CLOSE: UINT = 0x0010;
const SWP_FRAMECHANGED: UINT = 0x0020;
const WM_NCHITTEST: UINT = 0x0084;

const WM_ENTERSIZEMOVE: UINT = 0x0231;
const WM_EXITSIZEMOVE: UINT = 0x0232;

static mut global_key_downs: [u8; 512] = [0; 512];
static mut global_key_half_count: [u8; 512] = [0; 512];

unsafe extern "system" fn win32_wndproc(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT
{

    match uMsg
    {
        WM_CLOSE =>
        {
            quit_requested = true;
            PostQuitMessage(0);

        },

        /*
        WM_SIZE =>
        {
            let width = LOWORD(lParam as u32);
            let height = HIWORD(lParam as u32);

            println!("WM SIZE HERE: {}:{}, lparam: {}, wparam: {}", width, height, lParam, wParam);
        },
        WM_MOVE =>
        {
            let width = LOWORD(lParam as u32);
            let height = HIWORD(lParam as u32);

            println!("WM MOVE HERE: {}:{}", width, height);
        },
        WM_WINDOWPOSCHANGED =>
        {
            let width = LOWORD(lParam as u32);
            let height = HIWORD(lParam as u32);

            println!("WM pos changed: {}:{}", width, height);
        }
        */
        WM_KEYDOWN | WM_SYSKEYDOWN =>
        {
            let button: u32 = HIWORD(lParam as u32) as u32 & 0x1ff; //0x1ff;
            let b2 = wParam as u32;

            let translated = get_key(button);
            let mut transusize = translated as usize;
            if transusize & 0x40000000 == 0x40000000
            {
                transusize = transusize & 0xfff + 128;
            }
            if translated != MyKey::InvalidKey && transusize < 512
            {
                global_key_downs[transusize] = 1;
                global_key_half_count[transusize] += 1;
            }


            if b2 == VK_ESCAPE as _
            {
                quit_requested = true;
            }
            println!("button down: {}, modifier1: {}, modifier2: {}, b2: {}, transl: {}",  button, (wParam & 0x40000000) != 0,
                (lParam & 0x40000000) != 0, wParam, transusize);
        },

        WM_ENTERSIZEMOVE =>
        {
            /*
            let width = LOWORD(lParam as u32);
            let height = HIWORD(lParam as u32);

            println!("WM pos sizemove start: {}:{}, lparam: {}, wparam: {}", width, height, lParam, wParam);
            */
        }

        WM_EXITSIZEMOVE =>
        {
            /*
            let width = LOWORD(lParam as u32);
            let height = HIWORD(lParam as u32);

            println!("WM pos sizemove end: {}:{}, lparam: {}, wparam: {}", width, height, lParam, wParam);
            */

            resized = true;
        },

        _ =>
        {
            if !(
                (uMsg == WM_SIZE || uMsg == WM_MOVE || uMsg == WM_WINDOWPOSCHANGED) ||
                (uMsg >= WM_MOUSEFIRST && uMsg <= WM_MOUSELAST)
                || (uMsg == SWP_FRAMECHANGED || uMsg == WM_NCHITTEST || uMsg == WM_NCMOUSELEAVE)
                || ( uMsg >= WM_NCMOUSEMOVE && uMsg <= WM_NCMBUTTONDBLCLK)
            )
            {
                //println!("window message: {}, lparam: {}, wparam: {}", uMsg, wParam, lParam);
            }
        }
    }

    DefWindowProcA(hWnd, uMsg, wParam, lParam)
}



#[repr(C)]

struct PIXELFORMATDESCRIPTOR
{
    nSize: WORD,
    nVersion: WORD,
    dwFlags: DWORD,
    iPixelType: BYTE,
    cColorBits: BYTE,
    cRedBits: BYTE,
    cRedShift: BYTE,
    cGreenBits: BYTE,
    cGreenShift: BYTE,
    cBlueBits: BYTE,
    cBlueShift: BYTE,
    cAlphaBits: BYTE,
    cAlphaShift: BYTE,
    cAccumBits: BYTE,
    cAccumRedBits: BYTE,
    cAccumGreenBits: BYTE,
    cAccumBlueBits: BYTE,
    cAccumAlphaBits: BYTE,
    cDepthBits: BYTE,
    cStencilBits: BYTE,
    cAuxBuffers: BYTE,
    iLayerType: BYTE,
    bReserved: BYTE,
    dwLayerMask: DWORD,
    dwVisibleMask: DWORD,
    dwDamageMask: DWORD,
}
type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;





/* pixel types */
const PFD_TYPE_RGBA: BYTE =        0;
//const PFD_TYPE_COLORINDEX: BYTE =  1;

/* layer types */
const PFD_MAIN_PLANE: BYTE =     0;
//const PFD_OVERLAY_PLANE: BYTE =  1;
//const PFD_UNDERLAY_PLANE: BYTE = -1i8 as BYTE;

/* PIXELFORMATDESCRIPTOR flags */
const PFD_DOUBLEBUFFER : DWORD =            0x00000001;
const PFD_DRAW_TO_WINDOW : DWORD =          0x00000004;
const PFD_SUPPORT_OPENGL : DWORD =          0x00000020;

/*
const PFD_STEREO : DWORD =                  0x00000002;
const PFD_DRAW_TO_BITMAP : DWORD =          0x00000008;
const PFD_SUPPORT_GDI : DWORD =             0x00000010;
const PFD_GENERIC_FORMAT : DWORD =          0x00000040;
const PFD_NEED_PALETTE : DWORD =            0x00000080;
const PFD_NEED_SYSTEM_PALETTE : DWORD =     0x00000100;
const PFD_SWAP_EXCHANGE : DWORD =           0x00000200;
const PFD_SWAP_COPY : DWORD =               0x00000400;
const PFD_SWAP_LAYER_BUFFERS : DWORD =      0x00000800;
const PFD_GENERIC_ACCELERATED : DWORD =     0x00001000;
const PFD_SUPPORT_DIRECTDRAW : DWORD =      0x00002000;
const PFD_DIRECT3D_ACCELERATED : DWORD =    0x00004000;
const PFD_SUPPORT_COMPOSITION : DWORD =     0x00008000;


/* PIXELFORMATDESCRIPTOR flags for use in ChoosePixelFormat only */
const PFD_DEPTH_DONTCARE: DWORD =          0x20000000;
const PFD_DOUBLEBUFFER_DONTCARE: DWORD =   0x40000000;
const PFD_STEREO_DONTCARE: DWORD =         0x80000000;
*/

//const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
//const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
//const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;



//const PM_NOREMOVE: UINT = 0x0000;
const PM_REMOVE: UINT = 0x0001;
//const PM_NOYIELD: UINT = 0x0002;

const MAPVK_VK_TO_VSC: UINT = 0;
const MAPVK_VSC_TO_VK: UINT = 1;
const MAPVK_VK_TO_CHAR: UINT = 2;
const MAPVK_VSC_TO_VK_EX: UINT = 3;
const MAPVK_VK_TO_VSC_EX: UINT = 4;



fn get_key(key: u32) -> MyKey
{
    let key_code = unsafe { MapVirtualKeyA(key, MAPVK_VSC_TO_VK) as i32 };
    println!("key: {}, mapped: {}", key, key_code);
    let result = match key_code
    {
        VK_LEFT => MyKey::Left,
        VK_UP => MyKey::Up,
        VK_RIGHT => MyKey::Right,
        VK_DOWN => MyKey::Down,
        VK_ESCAPE => MyKey::Escape,
        VK_F1 => MyKey::F1,
        VK_F2 => MyKey::F2,
        VK_F3 => MyKey::F3,
        VK_F4 => MyKey::F4,
        VK_F5 => MyKey::F5,
        VK_F6 => MyKey::F6,
        VK_F7 => MyKey::F7,
        VK_F8 => MyKey::F8,
        VK_F9 => MyKey::F9,
        VK_F10 => MyKey::F10,
        VK_F11 => MyKey::F11,
        VK_F12 => MyKey::F12,
        _=>
        {
            if key >= 32 && key <= 128
            {
                unsafe{ std::mem::transmute_copy(&key) }
            }
            else
            {
                MyKey::InvalidKey
            }
        }
    };

    return result;
}
