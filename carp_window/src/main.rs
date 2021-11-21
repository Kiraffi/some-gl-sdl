#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


use std::os::raw::*;

pub type wchar_t = u16;

pub type POINTER_64_INT = usize;
pub type INT8 = c_schar;
pub type PINT8 = *mut c_schar;
pub type INT16 = c_short;
pub type PINT16 = *mut c_short;
pub type INT32 = c_int;
pub type PINT32 = *mut c_int;
pub type INT64 = i64;
pub type PINT64 = *mut i64;
pub type UINT8 = c_uchar;
pub type PUINT8 = *mut c_uchar;
pub type UINT16 = c_ushort;
pub type PUINT16 = *mut c_ushort;
pub type UINT32 = c_uint;
pub type PUINT32 = *mut c_uint;
pub type UINT64 = u64;
pub type PUINT64 = *mut u64;
pub type LONG32 = c_int;
pub type PLONG32 = *mut c_int;
pub type ULONG32 = c_uint;
pub type PULONG32 = *mut c_uint;
pub type DWORD32 = c_uint;
pub type PDWORD32 = *mut c_uint;
pub type INT_PTR = isize;
pub type PINT_PTR = *mut isize;
pub type UINT_PTR = usize;
pub type PUINT_PTR = *mut usize;
pub type LONG_PTR = isize;
pub type PLONG_PTR = *mut isize;
pub type ULONG_PTR = usize;
pub type PULONG_PTR = *mut usize;
pub type SHANDLE_PTR = isize;
pub type HANDLE_PTR = usize;

pub type ULONG = c_ulong;
pub type PULONG = *mut ULONG;
pub type USHORT = c_ushort;
pub type PUSHORT = *mut USHORT;
pub type UCHAR = c_uchar;
pub type PUCHAR = *mut UCHAR;
pub type PSZ = *mut c_char;
pub const MAX_PATH: usize = 260;
pub const FALSE: BOOL = 0;
pub const TRUE: BOOL = 1;
pub type DWORD = c_ulong;
pub type BOOL = c_int;
pub type BYTE = c_uchar;
pub type WORD = c_ushort;
pub type FLOAT = c_float;
pub type PFLOAT = *mut FLOAT;
pub type PBOOL = *mut BOOL;
pub type LPBOOL = *mut BOOL;
pub type PBYTE = *mut BYTE;
pub type LPBYTE = *mut BYTE;
pub type PINT = *mut c_int;
pub type LPINT = *mut c_int;
pub type PWORD = *mut WORD;
pub type LPWORD = *mut WORD;
pub type LPLONG = *mut c_long;
pub type PDWORD = *mut DWORD;
pub type LPDWORD = *mut DWORD;
pub type LPVOID = *mut c_void;
pub type LPCVOID = *const c_void;
pub type VOID = c_void;
pub type INT = c_int;
pub type UINT = c_uint;
pub type PUINT = *mut c_uint;
pub type WPARAM = UINT_PTR;
pub type LPARAM = LONG_PTR;
pub type LRESULT = LONG_PTR;
pub type WCHAR = wchar_t;
pub type CHAR = c_char;

pub type LPCWSTR = *const WCHAR;
pub type LPCSTR = *const CHAR;

pub type LPWSTR = *mut WCHAR;
pub type LPSTR = *mut CHAR;

pub type PVOID = *mut c_void;
pub type PVOID64 = u64; // This is a 64-bit pointer, even when in 32-bit

pub const NULL: PVOID = 0 as PVOID;
pub const NULL64: PVOID64 = 0;


pub type COLORREF = DWORD;
pub type LPCOLORREF = *mut DWORD;






pub type HANDLE = *mut c_void;
pub type CHARSTRING = *const c_char;

pub enum EMPTYTYPE {}
pub type HINSTANCE = *mut EMPTYTYPE;
pub type HWND = *mut EMPTYTYPE;
pub type HFONT = *mut EMPTYTYPE;
pub type HICON = *mut EMPTYTYPE;
pub type HMENU = *mut EMPTYTYPE;
pub type HCURSOR = *mut EMPTYTYPE;
pub type HBRUSH = *mut EMPTYTYPE;
pub type HMODULE = *mut EMPTYTYPE;

pub type HRSRC = *mut EMPTYTYPE;
pub type HBITMAP = *mut EMPTYTYPE;

pub type HDC = *mut EMPTYTYPE;
pub type HRGN = *mut EMPTYTYPE;
pub type HGLRC = *mut EMPTYTYPE;





pub type HGLOBAL = HANDLE;
pub type HLOCAL = HANDLE;





pub type FARPROC = *mut EMPTYTYPE;
pub type NEARPROC = *mut EMPTYTYPE;
pub type PROC = *mut EMPTYTYPE;


pub type ATOM = WORD;

pub const CW_USEDEFAULT: c_int = 0x80000000u32 as i32;
pub const HWND_DESKTOP: HWND = 0 as HWND;

pub type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;

pub type PROPENUMPROCEXA = Option<unsafe extern "system" fn(HWND, LPSTR, HANDLE, ULONG_PTR) -> BOOL>;
pub type PROPENUMPROCEXW = Option<unsafe extern "system" fn(HWND, LPWSTR, HANDLE, ULONG_PTR) -> BOOL>;
pub type PROPENUMPROCA = Option<unsafe extern "system" fn(HWND, LPCSTR, HANDLE) -> BOOL>;
pub type PROPENUMPROCW  = Option<unsafe extern "system" fn(HWND, LPCWSTR, HANDLE) -> BOOL>;

static mut createContextAttribsARB: Option<extern "system" fn(_: HDC, _: HGLRC, _: *const INT) -> HGLRC> = None;





#[repr(C)]

pub struct WNDCLASSA {
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

pub struct WNDCLASSW {
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
}



#[repr(C)]
pub struct WNDCLASSEXA {
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
pub type PWNDCLASSEXA = *mut WNDCLASSEXA;
pub type NPWNDCLASSEXA = *mut WNDCLASSEXA;
pub type LPWNDCLASSEXA = *mut WNDCLASSEXA;

#[repr(C)]
pub struct WNDCLASSEXW {
    cbSize: UINT,
    style: UINT,
    lpfnWndProc: WNDPROC,
    cbClsExtra: c_int,
    cbWndExtra: c_int,
    hInstance: HINSTANCE,
    hIcon: HICON,
    hCursor: HCURSOR,
    hbrBackground: HBRUSH,
    lpszMenuName: LPCWSTR,
    lpszClassName: LPCWSTR,
    hIconSm: HICON,
}
pub type PWNDCLASSEXW = *mut WNDCLASSEXW;
pub type NPWNDCLASSEXW = *mut WNDCLASSEXW;
pub type LPWNDCLASSEXW = *mut WNDCLASSEXW;

pub type PWNDCLASSA = *mut WNDCLASSA;
pub type NPWNDCLASSA = *mut WNDCLASSA;
pub type LPWNDCLASSA = *mut WNDCLASSA;

pub type PWNDCLASSW = *mut WNDCLASSW;
pub type NPWNDCLASSW = *mut WNDCLASSW;
pub type LPWNDCLASSW = *mut WNDCLASSW;








#[link(name = "user32")]
#[link(name = "gdi32")]
#[link(name = "opengl32")]
extern "system" {
    pub fn CreateWindowExA(
        dwExStyle: DWORD,
        lpClassName: LPCSTR,
        lpWindowName: LPCSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;















/*
    pub fn DisableThreadLibraryCalls(
        hLibModule: HMODULE,
    ) -> BOOL;
    pub fn FindResourceExW(
        hModule: HMODULE,
        lpName: LPCWSTR,
        lpType: LPCWSTR,
        wLanguage: WORD,
    ) -> HRSRC;
    pub fn FindStringOrdinal(
        dwFindStringOrdinalFlags: DWORD,
        lpStringSource: LPCWSTR,
        cchSource: c_int,
        lpStringValue: LPCWSTR,
        cchValue: c_int,
        bIgnoreCase: BOOL,
    ) -> c_int;
   
    pub fn FreeLibraryAndExitThread(
        hLibModule: HMODULE,
        dwExitCode: DWORD,
    );
    pub fn FreeResource(
        hResData: HGLOBAL,
    ) -> BOOL;
    pub fn GetModuleFileNameA(
        hModule: HMODULE,
        lpFilename: LPSTR,
        nSize: DWORD,
    ) -> DWORD;
    pub fn GetModuleFileNameW(
        hModule: HMODULE,
        lpFilename: LPWSTR,
        nSize: DWORD,
    ) -> DWORD;
    */
    pub fn FreeLibrary(
        hLibModule: HMODULE,
    ) -> BOOL;
    pub fn GetModuleHandleA(
        lpModuleName: LPCSTR,
    ) -> HMODULE;
    pub fn GetModuleHandleW(
        lpModuleName: LPCWSTR,
    ) -> HMODULE;
    pub fn GetModuleHandleExA(
        dwFlags: DWORD,
        lpModuleName: LPCSTR,
        phModule: *mut HMODULE,
    ) -> BOOL;
    pub fn GetModuleHandleExW(
        dwFlags: DWORD,
        lpModuleName: LPCWSTR,
        phModule: *mut HMODULE,
    ) -> BOOL;
    pub fn GetProcAddress(
        hModule: HMODULE,
        lpProcName: LPCSTR,
    ) -> FARPROC;
    pub fn LoadLibraryExA(
        lpLibFileName: LPCSTR,
        hFile: HANDLE,
        dwFlags: DWORD,
    ) -> HMODULE;
    pub fn LoadLibraryExW(
        lpLibFileName: LPCWSTR,
        hFile: HANDLE,
        dwFlags: DWORD,
    ) -> HMODULE;

    /*
    pub fn LoadResource(
        hModule: HMODULE,
        hResInfo: HRSRC,
    ) -> HGLOBAL;
    pub fn LoadStringA(
        hInstance: HINSTANCE,
        uID: UINT,
        lpBuffer: LPSTR,
        cchBufferMax: c_int,
    ) -> c_int;
    pub fn LoadStringW(
        hInstance: HINSTANCE,
        uID: UINT,
        lpBuffer: LPWSTR,
        cchBufferMax: c_int,
    ) -> c_int;
    pub fn LockResource(
        hResData: HGLOBAL,
    ) -> LPVOID;
    pub fn SizeofResource(
        hModule: HMODULE,
        hResInfo: HRSRC,
    ) -> DWORD;
*/











    pub fn PostQuitMessage(
        nExitCode: c_int,
    );
    pub fn DefWindowProcA(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn DefWindowProcW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
/*


    pub fn AttachThreadInput(
        idAttach: DWORD,
        idAttachTo: DWORD,
        fAttach: BOOL,
    ) -> BOOL;
    pub fn ReplyMessage(
        lResult: LRESULT,
    ) -> BOOL;
    pub fn WaitMessage() -> BOOL;
    pub fn WaitForInputIdle(
        hProcess: HANDLE,
        dwMilliseconds: DWORD,
    ) -> DWORD;

    pub fn CallWindowProcA(
        lpPrevWndFunc: WNDPROC,
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn CallWindowProcW(
        lpPrevWndFunc: WNDPROC,
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> LRESULT;
    pub fn InSendMessage() -> BOOL;
    pub fn InSendMessageEx(
        lpReserved: LPVOID,
    ) -> DWORD;


*/

























    pub fn LoadBitmapA(
        hInstance: HINSTANCE,
        lpBitmapName: LPCSTR,
    ) -> HBITMAP;
    pub fn LoadBitmapW(
        hInstance: HINSTANCE,
        lpBitmapName: LPCWSTR,
    ) -> HBITMAP;
    pub fn LoadCursorA(
        hInstance: HINSTANCE,
        lpCursorName: LPCSTR,
    ) -> HCURSOR;
    pub fn LoadCursorW(
        hInstance: HINSTANCE,
        lpCursorName: LPCWSTR,
    ) -> HCURSOR;
    pub fn LoadCursorFromFileA(
        lpFileName: LPCSTR,
    ) -> HCURSOR;
    pub fn LoadCursorFromFileW(
        lpFileName: LPCWSTR,
    ) -> HCURSOR;
    pub fn CreateCursor(
        hInst: HINSTANCE,
        xHotSpot: c_int,
        yHotSpot: c_int,
        nWidth: c_int,
        nHeight: c_int,
        pvAndPlane: *const VOID,
        pvXORPlane: *const VOID,
    ) -> HCURSOR;
    pub fn DestroyCursor(
        hCursor: HCURSOR,
    ) -> BOOL;













    pub fn LoadIconA(
        hInstance: HINSTANCE,
        lpIconName: LPCSTR,
    ) -> HICON;
    pub fn LoadIconW(
        hInstance: HINSTANCE,
        lpIconName: LPCWSTR,
    ) -> HICON;











    pub fn GetDoubleClickTime() -> UINT;
    pub fn SetDoubleClickTime(
        uInterval: UINT,
    ) -> BOOL;
    pub fn RegisterClassA(
        lpWndClass: *const WNDCLASSA,
    ) -> ATOM;
    pub fn RegisterClassW(
        lpWndClass: *const WNDCLASSW,
    ) -> ATOM;
    pub fn UnregisterClassA(
        lpClassName: LPCSTR,
        hInstance: HINSTANCE,
    ) -> BOOL;
    pub fn UnregisterClassW(
        lpClassName: LPCWSTR,
        hInstance: HINSTANCE,
    ) -> BOOL;
    pub fn GetClassInfoA(
        hInstance: HINSTANCE,
        lpClassName: LPCSTR,
        lpWndClass: LPWNDCLASSA,
    ) -> BOOL;
    pub fn GetClassInfoW(
        hInstance: HINSTANCE,
        lpClassName: LPCWSTR,
        lpWndClass: LPWNDCLASSW,
    ) -> BOOL;
    pub fn RegisterClassExA(
        lpWndClass: *const WNDCLASSEXA,
    ) -> ATOM;
    pub fn RegisterClassExW(
        lpWndClass: *const WNDCLASSEXW,
    ) -> ATOM;
    pub fn GetClassInfoExA(
        hinst: HINSTANCE,
        lpszClass: LPCSTR,
        lpwcx: LPWNDCLASSEXA,
    ) -> BOOL;
    pub fn GetClassInfoExW(
        hinst: HINSTANCE,
        lpszClass: LPCWSTR,
        lpwcx: LPWNDCLASSEXW,
    ) -> BOOL;
































/*

    pub fn SetPropA(
        hWnd: HWND,
        lpString: LPCSTR,
        hData: HANDLE,
    ) -> BOOL;
    pub fn SetPropW(
        hWnd: HWND,
        lpString: LPCWSTR,
        hData: HANDLE,
    ) -> BOOL;
    pub fn GetPropA(
        hwnd: HWND,
        lpString: LPCSTR,
    ) -> HANDLE;
    pub fn GetPropW(
        hwnd: HWND,
        lpString: LPCWSTR,
    ) -> HANDLE;
    pub fn RemovePropA(
        hWnd: HWND,
        lpStr: LPCSTR,
    ) -> HANDLE;
    pub fn RemovePropW(
        hWnd: HWND,
        lpStr: LPCWSTR,
    ) -> HANDLE;
    pub fn EnumPropsExA(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCA,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumPropsExW(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCW,
        lParam: LPARAM,
    ) -> c_int;
    pub fn EnumPropsA(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCA,
    ) -> c_int;
    pub fn EnumPropsW(
        hWnd: HWND,
        lpEnumFunc: PROPENUMPROCW,
    ) -> c_int;
    */
    pub fn SetWindowTextA(
        hWnd: HWND,
        lpString: LPCSTR,
    ) -> BOOL;
    pub fn SetWindowTextW(
        hWnd: HWND,
        lpString: LPCWSTR,
    ) -> BOOL;
    pub fn GetWindowTextA(
        hWnd: HWND,
        lpString: LPSTR,
        nMaxCount: c_int,
    ) -> c_int;
    pub fn GetWindowTextW(
        hWnd: HWND,
        lpString: LPWSTR,
        nMaxCount: c_int,
    ) -> c_int;
    pub fn GetWindowTextLengthA(
        hWnd: HWND,
    ) -> c_int;
    pub fn GetWindowTextLengthW(
        hWnd: HWND,
    ) -> c_int;
    pub fn GetClientRect(
        hWnd: HWND,
        lpRect: LPRECT,
    ) -> BOOL;
    pub fn GetWindowRect(
        hWnd: HWND,
        lpRect: LPRECT,
    ) -> BOOL;
    pub fn AdjustWindowRect(
        lpRect: LPRECT,
        dwStyle: DWORD,
        bMenu: BOOL,
    ) -> BOOL;
    pub fn AdjustWindowRectEx(
        lpRect: LPRECT,
        dwStyle: DWORD,
        bMenu: BOOL,
        dwExStyle: DWORD,
    ) -> BOOL;
    pub fn AdjustWindowRectExForDpi(
        lpRect: LPRECT,
        dwStyle: DWORD,
        bMenu: BOOL,
        dwExStyle: DWORD,
        dpi: UINT,
    ) -> BOOL;
























    pub fn IsWindow(
        hWnd: HWND,
    ) -> BOOL;
    pub fn IsMenu(
        hMenu: HMENU,
    ) -> BOOL;
    pub fn IsChild(
        hWndParent: HWND,
        hWnd: HWND,
    ) -> BOOL;
    pub fn DestroyWindow(
        hWnd: HWND,
    ) -> BOOL;
    pub fn ShowWindow(
        hWnd: HWND,
        nCmdShow: c_int,
    ) -> BOOL;
    pub fn AnimateWindow(
        hWnd: HWND,
        dwTime: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;









    pub fn WindowFromDC(
        hDC: HDC,
    ) -> HWND;
    pub fn GetDC(
        hWnd: HWND,
    ) -> HDC;
    pub fn GetDCEx(
        hWnd: HWND,
        hrgnClip: HRGN,
        flags: DWORD,
    ) -> HDC;



























    pub fn GetMessageA(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;
    pub fn GetMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
    ) -> BOOL;
    pub fn TranslateMessage(
        lpmsg: *const MSG,
    ) -> BOOL;
    pub fn DispatchMessageA(
        lpmsg: *const MSG,
    ) -> LRESULT;
    pub fn DispatchMessageW(
        lpmsg: *const MSG,
    ) -> LRESULT;
    pub fn SetMessageQueue(
        cMessagesMax: c_int,
    ) -> BOOL;
    pub fn PeekMessageA(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn PeekMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;











    pub fn PostMessageA(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn PostMessageW(
        hWnd: HWND,
        Msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn PostThreadMessageA(
        idThread: DWORD,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;
    pub fn PostThreadMessageW(
        idThread: DWORD,
        msg: UINT,
        wParam: WPARAM,
        lParam: LPARAM,
    ) -> BOOL;



    pub fn ChoosePixelFormat(
        hdc: HDC,
        ppfd: *const PIXELFORMATDESCRIPTOR,
    ) -> c_int;

    pub fn DescribePixelFormat(
        hdc: HDC,
        iPixelFormat: c_int,
        nBytes: UINT,
        ppfd: LPPIXELFORMATDESCRIPTOR,
    ) -> c_int;

    pub fn SetPixelFormat(
        hdc: HDC,
        iPixelFormat: c_int,
        ppfd: *const PIXELFORMATDESCRIPTOR,
    ) -> BOOL;

    pub fn wglCreateContext(
        hdc: HDC,
    ) -> HGLRC;

    pub fn wglDeleteContext(
        hglrc: HGLRC,
    ) -> BOOL;
    pub fn wglGetCurrentContext() -> HGLRC;
    pub fn wglGetCurrentDC() -> HDC;
    pub fn wglGetProcAddress(
        lpszProc: LPCSTR,
    ) -> PROC;
    pub fn wglMakeCurrent(
        hdc: HDC,
        hglrc: HGLRC,
    ) -> BOOL;

    pub fn SwapBuffers(
        hdc: HDC,
    ) -> BOOL;

}



//pub const CS_VREDRAW: UINT = 0x0001;
//pub const CS_HREDRAW: UINT = 0x0002;
//pub const CS_DBLCLKS: UINT = 0x0008;
//pub const CS_OWNDC: UINT = 0x0020;
pub const IDC_ARROW: LPCWSTR = 32512 as LPCWSTR;
pub const IDI_WINLOGO: LPCWSTR = 32517 as LPCWSTR;




pub type LONG = c_long;

#[repr(C)]
pub struct POINT 
{
    x: LONG,
    y: LONG,
}

#[repr(C)]

pub struct RECT 
{
    left: LONG,
    top: LONG,
    right: LONG,
    bottom: LONG,
}

pub type LPRECT = *mut RECT;

#[repr(C)]
pub struct MSG 
{
    hwnd: HWND,
    message: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
    time: DWORD,
    pt: POINT,
}
pub type PMSG = *mut MSG;
pub type NPMSG = *mut MSG;
pub type LPMSG = *mut MSG;
//POINTSTOPOINT
//POINTTOPOINTS
//MAKEWPARAM
//MAKELPARAM
//MAKELRESULT
pub const GWL_WNDPROC: c_int = -4;
pub const GWL_HINSTANCE: c_int = -6;
pub const GWL_HWNDPARENT: c_int = -8;
pub const GWL_STYLE: c_int = -16;
pub const GWL_EXSTYLE: c_int = -20;
pub const GWL_USERDATA: c_int = -21;
pub const GWL_ID: c_int = -12;
pub const GWLP_WNDPROC: c_int = -4;
pub const GWLP_HINSTANCE: c_int = -6;
pub const GWLP_HWNDPARENT: c_int = -8;
pub const GWLP_USERDATA: c_int = -21;
pub const GWLP_ID: c_int = -12;
pub const GCL_MENUNAME: c_int = -8;
pub const GCL_HBRBACKGROUND: c_int = -10;
pub const GCL_HCURSOR: c_int = -12;
pub const GCL_HICON: c_int = -14;
pub const GCL_HMODULE: c_int = -16;
pub const GCL_CBWNDEXTRA: c_int = -18;
pub const GCL_CBCLSEXTRA: c_int = -20;
pub const GCL_WNDPROC: c_int = -24;
pub const GCL_STYLE: c_int = -26;
pub const GCW_ATOM: c_int = -32;
pub const GCL_HICONSM: c_int = -34;
pub const GCLP_MENUNAME: c_int = -8;
pub const GCLP_HBRBACKGROUND: c_int = -10;
pub const GCLP_HCURSOR: c_int = -12;
pub const GCLP_HICON: c_int = -14;
pub const GCLP_HMODULE: c_int = -16;
pub const GCLP_WNDPROC: c_int = -24;
pub const GCLP_HICONSM: c_int = -34;
pub const WM_NULL: UINT = 0x0000;
pub const WM_CREATE: UINT = 0x0001;
pub const WM_DESTROY: UINT = 0x0002;
pub const WM_MOVE: UINT = 0x0003;
pub const WM_SIZE: UINT = 0x0005;
pub const WM_ACTIVATE: UINT = 0x0006;
pub const WA_INACTIVE: WORD = 0;
pub const WA_ACTIVE: WORD = 1;
pub const WA_CLICKACTIVE: WORD = 2;
pub const WM_SETFOCUS: UINT = 0x0007;
pub const WM_KILLFOCUS: UINT = 0x0008;
pub const WM_ENABLE: UINT = 0x000A;
pub const WM_SETREDRAW: UINT = 0x000B;
pub const WM_SETTEXT: UINT = 0x000C;
pub const WM_GETTEXT: UINT = 0x000D;
pub const WM_GETTEXTLENGTH: UINT = 0x000E;
pub const WM_PAINT: UINT = 0x000F;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_QUERYENDSESSION: UINT = 0x0011;
pub const WM_QUERYOPEN: UINT = 0x0013;
pub const WM_ENDSESSION: UINT = 0x0016;
pub const WM_QUIT: UINT = 0x0012;
pub const WM_ERASEBKGND: UINT = 0x0014;
pub const WM_SYSCOLORCHANGE: UINT = 0x0015;
pub const WM_SHOWWINDOW: UINT = 0x0018;
pub const WM_WININICHANGE: UINT = 0x001A;
pub const WM_SETTINGCHANGE: UINT = WM_WININICHANGE;
pub const WM_DEVMODECHANGE: UINT = 0x001B;
pub const WM_ACTIVATEAPP: UINT = 0x001C;
pub const WM_FONTCHANGE: UINT = 0x001D;
pub const WM_TIMECHANGE: UINT = 0x001E;
pub const WM_CANCELMODE: UINT = 0x001F;
pub const WM_SETCURSOR: UINT = 0x0020;
pub const WM_MOUSEACTIVATE: UINT = 0x0021;
pub const WM_CHILDACTIVATE: UINT = 0x0022;
pub const WM_QUEUESYNC: UINT = 0x0023;
pub const WM_GETMINMAXINFO: UINT = 0x0024;













pub const WS_OVERLAPPED: DWORD = 0x00000000;
pub const WS_POPUP: DWORD = 0x80000000;
pub const WS_CHILD: DWORD = 0x40000000;
pub const WS_MINIMIZE: DWORD = 0x20000000;
pub const WS_VISIBLE: DWORD = 0x10000000;
pub const WS_DISABLED: DWORD = 0x08000000;
pub const WS_CLIPSIBLINGS: DWORD = 0x04000000;
pub const WS_CLIPCHILDREN: DWORD = 0x02000000;
pub const WS_MAXIMIZE: DWORD = 0x01000000;
pub const WS_CAPTION: DWORD = 0x00C00000;
pub const WS_BORDER: DWORD = 0x00800000;
pub const WS_DLGFRAME: DWORD = 0x00400000;
pub const WS_VSCROLL: DWORD = 0x00200000;
pub const WS_HSCROLL: DWORD = 0x00100000;
pub const WS_SYSMENU: DWORD = 0x00080000;
pub const WS_THICKFRAME: DWORD = 0x00040000;
pub const WS_GROUP: DWORD = 0x00020000;
pub const WS_TABSTOP: DWORD = 0x00010000;
pub const WS_MINIMIZEBOX: DWORD = 0x00020000;
pub const WS_MAXIMIZEBOX: DWORD = 0x00010000;
pub const WS_TILED: DWORD = WS_OVERLAPPED;
pub const WS_ICONIC: DWORD = WS_MINIMIZE;
pub const WS_SIZEBOX: DWORD = WS_THICKFRAME;
pub const WS_TILEDWINDOW: DWORD = WS_OVERLAPPEDWINDOW;
pub const WS_OVERLAPPEDWINDOW: DWORD = WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME
    | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_POPUPWINDOW: DWORD = WS_POPUP | WS_BORDER | WS_SYSMENU;
pub const WS_CHILDWINDOW: DWORD = WS_CHILD;
pub const WS_EX_DLGMODALFRAME: DWORD = 0x00000001;
pub const WS_EX_NOPARENTNOTIFY: DWORD = 0x00000004;
pub const WS_EX_TOPMOST: DWORD = 0x00000008;
pub const WS_EX_ACCEPTFILES: DWORD = 0x00000010;
pub const WS_EX_TRANSPARENT: DWORD = 0x00000020;
pub const WS_EX_MDICHILD: DWORD = 0x00000040;
pub const WS_EX_TOOLWINDOW: DWORD = 0x00000080;
pub const WS_EX_WINDOWEDGE: DWORD = 0x00000100;
pub const WS_EX_CLIENTEDGE: DWORD = 0x00000200;
pub const WS_EX_CONTEXTHELP: DWORD = 0x00000400;
pub const WS_EX_RIGHT: DWORD = 0x00001000;
pub const WS_EX_LEFT: DWORD = 0x00000000;
pub const WS_EX_RTLREADING: DWORD = 0x00002000;
pub const WS_EX_LTRREADING: DWORD = 0x00000000;
pub const WS_EX_LEFTSCROLLBAR: DWORD = 0x00004000;
pub const WS_EX_RIGHTSCROLLBAR: DWORD = 0x00000000;
pub const WS_EX_CONTROLPARENT: DWORD = 0x00010000;
pub const WS_EX_STATICEDGE: DWORD = 0x00020000;
pub const WS_EX_APPWINDOW: DWORD = 0x00040000;
pub const WS_EX_OVERLAPPEDWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_CLIENTEDGE;
pub const WS_EX_PALETTEWINDOW: DWORD = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;
pub const WS_EX_LAYERED: DWORD = 0x00080000;
pub const WS_EX_NOINHERITLAYOUT: DWORD = 0x00100000;
pub const WS_EX_NOREDIRECTIONBITMAP: DWORD = 0x00200000;
pub const WS_EX_LAYOUTRTL: DWORD = 0x00400000;
pub const WS_EX_COMPOSITED: DWORD = 0x02000000;
pub const WS_EX_NOACTIVATE: DWORD = 0x08000000;
pub const CS_VREDRAW: UINT = 0x0001;
pub const CS_HREDRAW: UINT = 0x0002;
pub const CS_DBLCLKS: UINT = 0x0008;
pub const CS_OWNDC: UINT = 0x0020;
pub const CS_CLASSDC: UINT = 0x0040;
pub const CS_PARENTDC: UINT = 0x0080;
pub const CS_NOCLOSE: UINT = 0x0200;
pub const CS_SAVEBITS: UINT = 0x0800;
pub const CS_BYTEALIGNCLIENT: UINT = 0x1000;
pub const CS_BYTEALIGNWINDOW: UINT = 0x2000;
pub const CS_GLOBALCLASS: UINT = 0x4000;
pub const CS_IME: UINT = 0x00010000;
pub const CS_DROPSHADOW: UINT = 0x00020000;
pub const PRF_CHECKVISIBLE: UINT = 0x00000001;
pub const PRF_NONCLIENT: UINT = 0x00000002;
pub const PRF_CLIENT: UINT = 0x00000004;
pub const PRF_ERASEBKGND: UINT = 0x00000008;
pub const PRF_CHILDREN: UINT = 0x00000010;
pub const PRF_OWNED: UINT = 0x00000020;
pub const BDR_RAISEDOUTER: UINT = 0x0001;
pub const BDR_SUNKENOUTER: UINT = 0x0002;
pub const BDR_RAISEDINNER: UINT = 0x0004;
pub const BDR_SUNKENINNER: UINT = 0x0008;
pub const BDR_OUTER: UINT = BDR_RAISEDOUTER | BDR_SUNKENOUTER;
pub const BDR_INNER: UINT = BDR_RAISEDINNER | BDR_SUNKENINNER;
pub const BDR_RAISED: UINT = BDR_RAISEDOUTER | BDR_RAISEDINNER;
pub const BDR_SUNKEN: UINT = BDR_SUNKENOUTER | BDR_SUNKENINNER;
pub const EDGE_RAISED: UINT = BDR_RAISEDOUTER | BDR_RAISEDINNER;
pub const EDGE_SUNKEN: UINT = BDR_SUNKENOUTER | BDR_SUNKENINNER;
pub const EDGE_ETCHED: UINT = BDR_SUNKENOUTER | BDR_RAISEDINNER;
pub const EDGE_BUMP: UINT = BDR_RAISEDOUTER | BDR_SUNKENINNER;
pub const BF_LEFT: UINT = 0x0001;
pub const BF_TOP: UINT = 0x0002;
pub const BF_RIGHT: UINT = 0x0004;
pub const BF_BOTTOM: UINT = 0x0008;
pub const BF_TOPLEFT: UINT = BF_TOP | BF_LEFT;
pub const BF_TOPRIGHT: UINT = BF_TOP | BF_RIGHT;
pub const BF_BOTTOMLEFT: UINT = BF_BOTTOM | BF_LEFT;
pub const BF_BOTTOMRIGHT: UINT = BF_BOTTOM | BF_RIGHT;
pub const BF_RECT: UINT = BF_LEFT | BF_TOP | BF_RIGHT | BF_BOTTOM;
pub const BF_DIAGONAL: UINT = 0x0010;
pub const BF_DIAGONAL_ENDTOPRIGHT: UINT = BF_DIAGONAL | BF_TOP | BF_RIGHT;
pub const BF_DIAGONAL_ENDTOPLEFT: UINT = BF_DIAGONAL | BF_TOP | BF_LEFT;
pub const BF_DIAGONAL_ENDBOTTOMLEFT: UINT = BF_DIAGONAL | BF_BOTTOM | BF_LEFT;
pub const BF_DIAGONAL_ENDBOTTOMRIGHT: UINT = BF_DIAGONAL | BF_BOTTOM | BF_RIGHT;
pub const BF_MIDDLE: UINT = 0x0800;
pub const BF_SOFT: UINT = 0x1000;
pub const BF_ADJUST: UINT = 0x2000;
pub const BF_FLAT: UINT = 0x4000;
pub const BF_MONO: UINT = 0x8000;



























pub const SETWALLPAPER_DEFAULT: LPWSTR = -1isize as LPWSTR;
pub const SB_HORZ: UINT = 0;
pub const SB_VERT: UINT = 1;
pub const SB_CTL: UINT = 2;
pub const SB_BOTH: UINT = 3;
pub const SB_LINEUP: LPARAM = 0;
pub const SB_LINELEFT: LPARAM = 0;
pub const SB_LINEDOWN: LPARAM = 1;
pub const SB_LINERIGHT: LPARAM = 1;
pub const SB_PAGEUP: LPARAM = 2;
pub const SB_PAGELEFT: LPARAM = 2;
pub const SB_PAGEDOWN: LPARAM = 3;
pub const SB_PAGERIGHT: LPARAM = 3;
pub const SB_THUMBPOSITION: LPARAM = 4;
pub const SB_THUMBTRACK: LPARAM = 5;
pub const SB_TOP: LPARAM = 6;
pub const SB_LEFT: LPARAM = 6;
pub const SB_BOTTOM: LPARAM = 7;
pub const SB_RIGHT: LPARAM = 7;
pub const SB_ENDSCROLL: LPARAM = 8;
pub const SW_HIDE: c_int = 0;
pub const SW_SHOWNORMAL: c_int = 1;
pub const SW_NORMAL: c_int = 1;
pub const SW_SHOWMINIMIZED: c_int = 2;
pub const SW_SHOWMAXIMIZED: c_int = 3;
pub const SW_MAXIMIZE: c_int = 3;
pub const SW_SHOWNOACTIVATE: c_int = 4;
pub const SW_SHOW: c_int = 5;
pub const SW_MINIMIZE: c_int = 6;
pub const SW_SHOWMINNOACTIVE: c_int = 7;
pub const SW_SHOWNA: c_int = 8;
pub const SW_RESTORE: c_int = 9;
pub const SW_SHOWDEFAULT: c_int = 10;
pub const SW_FORCEMINIMIZE: c_int = 11;
pub const SW_MAX: c_int = 11;
pub const HIDE_WINDOW: c_int = 0;
pub const SHOW_OPENWINDOW: c_int = 1;
pub const SHOW_ICONWINDOW: c_int = 2;
pub const SHOW_FULLSCREEN: c_int = 3;
pub const SHOW_OPENNOACTIVATE: c_int = 4;
pub const SW_PARENTCLOSING: LPARAM = 1;
pub const SW_OTHERZOOM: LPARAM = 2;
pub const SW_PARENTOPENING: LPARAM = 3;
pub const SW_OTHERUNZOOM: LPARAM = 4;
pub const AW_HOR_POSITIVE: DWORD = 0x00000001;
pub const AW_HOR_NEGATIVE: DWORD = 0x00000002;
pub const AW_VER_POSITIVE: DWORD = 0x00000004;
pub const AW_VER_NEGATIVE: DWORD = 0x00000008;
pub const AW_CENTER: DWORD = 0x00000010;
pub const AW_HIDE: DWORD = 0x00010000;
pub const AW_ACTIVATE: DWORD = 0x00020000;
pub const AW_SLIDE: DWORD = 0x00040000;
pub const AW_BLEND: DWORD = 0x00080000;
pub const KF_EXTENDED: WORD = 0x0100;
pub const KF_DLGMODE: WORD = 0x0800;
pub const KF_MENUMODE: WORD = 0x1000;
pub const KF_ALTDOWN: WORD = 0x2000;
pub const KF_REPEAT: WORD = 0x4000;
pub const KF_UP: WORD = 0x8000;
pub const VK_LBUTTON: c_int = 0x01;
pub const VK_RBUTTON: c_int = 0x02;
pub const VK_CANCEL: c_int = 0x03;
pub const VK_MBUTTON: c_int = 0x04;
pub const VK_XBUTTON1: c_int = 0x05;
pub const VK_XBUTTON2: c_int = 0x06;
pub const VK_BACK: c_int = 0x08;
pub const VK_TAB: c_int = 0x09;
pub const VK_CLEAR: c_int = 0x0C;
pub const VK_RETURN: c_int = 0x0D;
pub const VK_SHIFT: c_int = 0x10;
pub const VK_CONTROL: c_int = 0x11;
pub const VK_MENU: c_int = 0x12;
pub const VK_PAUSE: c_int = 0x13;
pub const VK_CAPITAL: c_int = 0x14;
pub const VK_KANA: c_int = 0x15;
pub const VK_HANGEUL: c_int = 0x15;
pub const VK_HANGUL: c_int = 0x15;
pub const VK_JUNJA: c_int = 0x17;
pub const VK_FINAL: c_int = 0x18;
pub const VK_HANJA: c_int = 0x19;
pub const VK_KANJI: c_int = 0x19;
pub const VK_ESCAPE: c_int = 0x1B;
pub const VK_CONVERT: c_int = 0x1C;
pub const VK_NONCONVERT: c_int = 0x1D;
pub const VK_ACCEPT: c_int = 0x1E;
pub const VK_MODECHANGE: c_int = 0x1F;
pub const VK_SPACE: c_int = 0x20;
pub const VK_PRIOR: c_int = 0x21;
pub const VK_NEXT: c_int = 0x22;
pub const VK_END: c_int = 0x23;
pub const VK_HOME: c_int = 0x24;
pub const VK_LEFT: c_int = 0x25;
pub const VK_UP: c_int = 0x26;
pub const VK_RIGHT: c_int = 0x27;
pub const VK_DOWN: c_int = 0x28;
pub const VK_SELECT: c_int = 0x29;
pub const VK_PRINT: c_int = 0x2A;
pub const VK_EXECUTE: c_int = 0x2B;
pub const VK_SNAPSHOT: c_int = 0x2C;
pub const VK_INSERT: c_int = 0x2D;
pub const VK_DELETE: c_int = 0x2E;
pub const VK_HELP: c_int = 0x2F;
pub const VK_LWIN: c_int = 0x5B;
pub const VK_RWIN: c_int = 0x5C;
pub const VK_APPS: c_int = 0x5D;
pub const VK_SLEEP: c_int = 0x5F;
pub const VK_NUMPAD0: c_int = 0x60;
pub const VK_NUMPAD1: c_int = 0x61;
pub const VK_NUMPAD2: c_int = 0x62;
pub const VK_NUMPAD3: c_int = 0x63;
pub const VK_NUMPAD4: c_int = 0x64;
pub const VK_NUMPAD5: c_int = 0x65;
pub const VK_NUMPAD6: c_int = 0x66;
pub const VK_NUMPAD7: c_int = 0x67;
pub const VK_NUMPAD8: c_int = 0x68;
pub const VK_NUMPAD9: c_int = 0x69;
pub const VK_MULTIPLY: c_int = 0x6A;
pub const VK_ADD: c_int = 0x6B;
pub const VK_SEPARATOR: c_int = 0x6C;
pub const VK_SUBTRACT: c_int = 0x6D;
pub const VK_DECIMAL: c_int = 0x6E;
pub const VK_DIVIDE: c_int = 0x6F;
pub const VK_F1: c_int = 0x70;
pub const VK_F2: c_int = 0x71;
pub const VK_F3: c_int = 0x72;
pub const VK_F4: c_int = 0x73;
pub const VK_F5: c_int = 0x74;
pub const VK_F6: c_int = 0x75;
pub const VK_F7: c_int = 0x76;
pub const VK_F8: c_int = 0x77;
pub const VK_F9: c_int = 0x78;
pub const VK_F10: c_int = 0x79;
pub const VK_F11: c_int = 0x7A;
pub const VK_F12: c_int = 0x7B;
pub const VK_F13: c_int = 0x7C;
pub const VK_F14: c_int = 0x7D;
pub const VK_F15: c_int = 0x7E;
pub const VK_F16: c_int = 0x7F;
pub const VK_F17: c_int = 0x80;
pub const VK_F18: c_int = 0x81;
pub const VK_F19: c_int = 0x82;
pub const VK_F20: c_int = 0x83;
pub const VK_F21: c_int = 0x84;
pub const VK_F22: c_int = 0x85;
pub const VK_F23: c_int = 0x86;
pub const VK_F24: c_int = 0x87;
pub const VK_NAVIGATION_VIEW: c_int = 0x88;
pub const VK_NAVIGATION_MENU: c_int = 0x89;
pub const VK_NAVIGATION_UP: c_int = 0x8A;
pub const VK_NAVIGATION_DOWN: c_int = 0x8B;
pub const VK_NAVIGATION_LEFT: c_int = 0x8C;
pub const VK_NAVIGATION_RIGHT: c_int = 0x8D;
pub const VK_NAVIGATION_ACCEPT: c_int = 0x8E;
pub const VK_NAVIGATION_CANCEL: c_int = 0x8F;
pub const VK_NUMLOCK: c_int = 0x90;
pub const VK_SCROLL: c_int = 0x91;
pub const VK_OEM_NEC_EQUAL: c_int = 0x92;
pub const VK_OEM_FJ_JISHO: c_int = 0x92;
pub const VK_OEM_FJ_MASSHOU: c_int = 0x93;
pub const VK_OEM_FJ_TOUROKU: c_int = 0x94;
pub const VK_OEM_FJ_LOYA: c_int = 0x95;
pub const VK_OEM_FJ_ROYA: c_int = 0x96;
pub const VK_LSHIFT: c_int = 0xA0;
pub const VK_RSHIFT: c_int = 0xA1;
pub const VK_LCONTROL: c_int = 0xA2;
pub const VK_RCONTROL: c_int = 0xA3;
pub const VK_LMENU: c_int = 0xA4;
pub const VK_RMENU: c_int = 0xA5;
pub const VK_BROWSER_BACK: c_int = 0xA6;
pub const VK_BROWSER_FORWARD: c_int = 0xA7;
pub const VK_BROWSER_REFRESH: c_int = 0xA8;
pub const VK_BROWSER_STOP: c_int = 0xA9;
pub const VK_BROWSER_SEARCH: c_int = 0xAA;
pub const VK_BROWSER_FAVORITES: c_int = 0xAB;
pub const VK_BROWSER_HOME: c_int = 0xAC;
pub const VK_VOLUME_MUTE: c_int = 0xAD;
pub const VK_VOLUME_DOWN: c_int = 0xAE;
pub const VK_VOLUME_UP: c_int = 0xAF;
pub const VK_MEDIA_NEXT_TRACK: c_int = 0xB0;
pub const VK_MEDIA_PREV_TRACK: c_int = 0xB1;
pub const VK_MEDIA_STOP: c_int = 0xB2;
pub const VK_MEDIA_PLAY_PAUSE: c_int = 0xB3;
pub const VK_LAUNCH_MAIL: c_int = 0xB4;
pub const VK_LAUNCH_MEDIA_SELECT: c_int = 0xB5;
pub const VK_LAUNCH_APP1: c_int = 0xB6;
pub const VK_LAUNCH_APP2: c_int = 0xB7;
pub const VK_OEM_1: c_int = 0xBA;
pub const VK_OEM_PLUS: c_int = 0xBB;
pub const VK_OEM_COMMA: c_int = 0xBC;
pub const VK_OEM_MINUS: c_int = 0xBD;
pub const VK_OEM_PERIOD: c_int = 0xBE;
pub const VK_OEM_2: c_int = 0xBF;
pub const VK_OEM_3: c_int = 0xC0;
pub const VK_GAMEPAD_A: c_int = 0xC3;
pub const VK_GAMEPAD_B: c_int = 0xC4;
pub const VK_GAMEPAD_X: c_int = 0xC5;
pub const VK_GAMEPAD_Y: c_int = 0xC6;
pub const VK_GAMEPAD_RIGHT_SHOULDER: c_int = 0xC7;
pub const VK_GAMEPAD_LEFT_SHOULDER: c_int = 0xC8;
pub const VK_GAMEPAD_LEFT_TRIGGER: c_int = 0xC9;
pub const VK_GAMEPAD_RIGHT_TRIGGER: c_int = 0xCA;
pub const VK_GAMEPAD_DPAD_UP: c_int = 0xCB;
pub const VK_GAMEPAD_DPAD_DOWN: c_int = 0xCC;
pub const VK_GAMEPAD_DPAD_LEFT: c_int = 0xCD;
pub const VK_GAMEPAD_DPAD_RIGHT: c_int = 0xCE;
pub const VK_GAMEPAD_MENU: c_int = 0xCF;
pub const VK_GAMEPAD_VIEW: c_int = 0xD0;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: c_int = 0xD1;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: c_int = 0xD2;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: c_int = 0xD3;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: c_int = 0xD4;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: c_int = 0xD5;
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: c_int = 0xD6;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: c_int = 0xD7;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: c_int = 0xD8;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: c_int = 0xD9;
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: c_int = 0xDA;
pub const VK_OEM_4: c_int = 0xDB;
pub const VK_OEM_5: c_int = 0xDC;
pub const VK_OEM_6: c_int = 0xDD;
pub const VK_OEM_7: c_int = 0xDE;
pub const VK_OEM_8: c_int = 0xDF;
pub const VK_OEM_AX: c_int = 0xE1;
pub const VK_OEM_102: c_int = 0xE2;
pub const VK_ICO_HELP: c_int = 0xE3;
pub const VK_ICO_00: c_int = 0xE4;
pub const VK_PROCESSKEY: c_int = 0xE5;
pub const VK_ICO_CLEAR: c_int = 0xE6;
pub const VK_PACKET: c_int = 0xE7;
pub const VK_OEM_RESET: c_int = 0xE9;
pub const VK_OEM_JUMP: c_int = 0xEA;
pub const VK_OEM_PA1: c_int = 0xEB;
pub const VK_OEM_PA2: c_int = 0xEC;
pub const VK_OEM_PA3: c_int = 0xED;
pub const VK_OEM_WSCTRL: c_int = 0xEE;
pub const VK_OEM_CUSEL: c_int = 0xEF;
pub const VK_OEM_ATTN: c_int = 0xF0;
pub const VK_OEM_FINISH: c_int = 0xF1;
pub const VK_OEM_COPY: c_int = 0xF2;
pub const VK_OEM_AUTO: c_int = 0xF3;
pub const VK_OEM_ENLW: c_int = 0xF4;
pub const VK_OEM_BACKTAB: c_int = 0xF5;
pub const VK_ATTN: c_int = 0xF6;
pub const VK_CRSEL: c_int = 0xF7;
pub const VK_EXSEL: c_int = 0xF8;
pub const VK_EREOF: c_int = 0xF9;
pub const VK_PLAY: c_int = 0xFA;
pub const VK_ZOOM: c_int = 0xFB;
pub const VK_NONAME: c_int = 0xFC;
pub const VK_PA1: c_int = 0xFD;
pub const VK_OEM_CLEAR: c_int = 0xFE;
pub const WH_MIN: c_int = -1;
pub const WH_MSGFILTER: c_int = -1;
pub const WH_JOURNALRECORD: c_int = 0;
pub const WH_JOURNALPLAYBACK: c_int = 1;
pub const WH_KEYBOARD: c_int = 2;
pub const WH_GETMESSAGE: c_int = 3;
pub const WH_CALLWNDPROC: c_int = 4;
pub const WH_CBT: c_int = 5;
pub const WH_SYSMSGFILTER: c_int = 6;
pub const WH_MOUSE: c_int = 7;
pub const WH_HARDWARE: c_int = 8;
pub const WH_DEBUG: c_int = 9;
pub const WH_SHELL: c_int = 10;
pub const WH_FOREGROUNDIDLE: c_int = 11;
pub const WH_CALLWNDPROCRET: c_int = 12;
pub const WH_KEYBOARD_LL: c_int = 13;
pub const WH_MOUSE_LL: c_int = 14;
pub const WH_MAX: c_int = 14;
pub const WH_MINHOOK: c_int = WH_MIN;
pub const WH_MAXHOOK: c_int = WH_MAX;
pub const HC_ACTION: c_int = 0;
pub const HC_GETNEXT: c_int = 1;
pub const HC_SKIP: c_int = 2;
pub const HC_NOREMOVE: c_int = 3;
pub const HC_NOREM: c_int = HC_NOREMOVE;
pub const HC_SYSMODALON: c_int = 4;
pub const HC_SYSMODALOFF: c_int = 5;
pub const HCBT_MOVESIZE: c_int = 0;
pub const HCBT_MINMAX: c_int = 1;
pub const HCBT_QS: c_int = 2;
pub const HCBT_CREATEWND: c_int = 3;
pub const HCBT_DESTROYWND: c_int = 4;
pub const HCBT_ACTIVATE: c_int = 5;
pub const HCBT_CLICKSKIPPED: c_int = 6;
pub const HCBT_KEYSKIPPED: c_int = 7;
pub const HCBT_SYSCOMMAND: c_int = 8;
pub const HCBT_SETFOCUS: c_int = 9;



pub const WM_INPUT: UINT = 0x00FF;
pub const WM_KEYFIRST: UINT = 0x0100;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;


static mut quit_requested : bool = false;
#[inline]
pub fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xffff) as WORD
}
//pub const WM_CLOSE: UINT = 0x0010;

unsafe extern "system" fn win32_wndproc(hWnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT 
{
    // FIXME: refresh rendering during resize with a WM_TIMER event
    {
        match uMsg 
        {
            WM_CLOSE => 
            {
                quit_requested = true;
                PostQuitMessage(0);
                
            },


            WM_KEYDOWN | WM_SYSKEYDOWN => 
            { 
                let button: u32 = HIWORD(lParam as u32) as u32 & 0x1ff;
                if button == 1u32
                {
                    quit_requested = true;
                }
                println!("button down: {}, modifier: {}",  button, (lParam & 0x40000000) != 0);



            },

            /*
            WM_SYSCOMMAND => {
                match wParam & 0xFFF0 {
                    SC_SCREENSAVE | SC_MONITORPOWER => {
                        if _sapp.desc.fullscreen {
                            // disable screen saver and blanking in fullscreen mode
                            return 0;
                        }
                    }
                    SC_KEYMENU => {
                        // user trying to access menu via ALT
                        return 0;
                    }
                    _ => {}
                }
            }
            WM_ERASEBKGND => {
                return 1;
            }
            WM_SIZE => {
                if _sapp.cursor_grabbed {
                    update_clip_rect(hWnd);
                }

                let iconified = wParam == SIZE_MINIMIZED;
                if iconified != _sapp_win32_iconified {
                    _sapp_win32_iconified = iconified;
                    if iconified {
                        _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_ICONIFIED);
                    } else {
                        _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_RESTORED);
                    }
                }
            }
            WM_SETCURSOR => {
                if _sapp.desc.user_cursor {
                    if LOWORD(lParam as _) == HTCLIENT as _ {
                        SetCursor(_sapp_cursor);
                        _sapp_win32_app_event(sapp_event_type_SAPP_EVENTTYPE_UPDATE_CURSOR);
                        return 1;
                    }
                }
            }
            WM_LBUTTONDOWN => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN,
                sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
            ),
            WM_RBUTTONDOWN => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN,
                sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT,
            ),
            WM_MBUTTONDOWN => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_DOWN,
                sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE,
            ),
            WM_LBUTTONUP => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP,
                sapp_mousebutton_SAPP_MOUSEBUTTON_LEFT,
            ),
            WM_RBUTTONUP => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP,
                sapp_mousebutton_SAPP_MOUSEBUTTON_RIGHT,
            ),
            WM_MBUTTONUP => _sapp_win32_mouse_event(
                sapp_event_type_SAPP_EVENTTYPE_MOUSE_UP,
                sapp_mousebutton_SAPP_MOUSEBUTTON_MIDDLE,
            ),

            WM_MOUSEMOVE => {
                _sapp.mouse_x = GET_X_LPARAM(lParam) as f32 * _sapp_win32_mouse_scale;
                _sapp.mouse_y = GET_Y_LPARAM(lParam) as f32 * _sapp_win32_mouse_scale;
                if !_sapp.win32_mouse_tracked {
                    _sapp.win32_mouse_tracked = true;

                    let mut tme: TRACKMOUSEEVENT = std::mem::zeroed();

                    tme.cbSize = std::mem::size_of_val(&tme) as _;
                    tme.dwFlags = TME_LEAVE;
                    tme.hwndTrack = _sapp_win32_hwnd;
                    TrackMouseEvent(&mut tme as *mut _);
                    _sapp_win32_mouse_event(
                        sapp_event_type_SAPP_EVENTTYPE_MOUSE_ENTER,
                        sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                    );
                }

                _sapp_win32_mouse_event(
                    sapp_event_type_SAPP_EVENTTYPE_MOUSE_MOVE,
                    sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                );
            }

            WM_MOVE if _sapp.cursor_grabbed => {
                update_clip_rect(hWnd);
            }

            WM_INPUT => {
                let mut data: RAWINPUT = std::mem::zeroed();
                let mut size = std::mem::size_of::<RAWINPUT>();
                let get_succeed = GetRawInputData(
                    lParam as _,
                    RID_INPUT,
                    &mut data as *mut _ as _,
                    &mut size as *mut _ as _,
                    std::mem::size_of::<RAWINPUTHEADER>() as _,
                );
                if get_succeed as i32 == -1 {
                    panic!("failed to retrieve raw input data");
                }

                if data.data.mouse().usFlags & MOUSE_MOVE_ABSOLUTE == 1 {
                    unimplemented!("Got MOUSE_MOVE_ABSOLUTE on WM_INPUT, related issue: https://github.com/not-fl3/miniquad/issues/165");
                }

                _sapp_init_event(sapp_event_type_SAPP_EVENTTYPE_RAW_DEVICE);
                _sapp.event.mouse_dx = data.data.mouse().lLastX as f32 * _sapp_win32_mouse_scale;
                _sapp.event.mouse_dy = data.data.mouse().lLastY as f32 * _sapp_win32_mouse_scale;
                _sapp_call_event(&_sapp.event);

                update_clip_rect(hWnd);
            }

            WM_MOUSELEAVE => {
                _sapp.win32_mouse_tracked = false;
                _sapp_win32_mouse_event(
                    sapp_event_type_SAPP_EVENTTYPE_MOUSE_LEAVE,
                    sapp_mousebutton_SAPP_MOUSEBUTTON_INVALID,
                );
            }
            WM_MOUSEWHEEL => {
                _sapp_win32_scroll_event(0.0, (HIWORD(wParam as _) as i16) as f32);
            }

            WM_MOUSEHWHEEL => {
                _sapp_win32_scroll_event((HIWORD(wParam as _) as i16) as f32, 0.0);
            }
            WM_CHAR => _sapp_win32_char_event(wParam as _, !!(lParam & 0x40000000) != 0),
            WM_KEYDOWN | WM_SYSKEYDOWN => _sapp_win32_key_event(
                sapp_event_type_SAPP_EVENTTYPE_KEY_DOWN,
                HIWORD(lParam as _) as u32 & 0x1FF,
                !!(lParam & 0x40000000) != 0,
            ),
            WM_KEYUP | WM_SYSKEYUP => _sapp_win32_key_event(
                sapp_event_type_SAPP_EVENTTYPE_KEY_UP,
                HIWORD(lParam as _) as u32 & 0x1FF,
                false,
            ),
            */
            _ => {}
        }
    }

    DefWindowProcW(hWnd, uMsg, wParam, lParam)
}

struct Win32Window
{
    win32_hwnd : HWND,
    win32_dc : HDC,
    hglrc : HGLRC,
    class_name: Vec<u16>,
    window_name: Vec<u16>,

    width: i32,
    height: i32,

    ogl_extended: i32,

}

static mut win32_window : Win32Window = Win32Window {
    win32_hwnd : 0 as HWND,
    win32_dc : 0 as HDC,
    hglrc : 0 as HGLRC,
    class_name : Vec::new(),
    window_name : Vec::new(),

    width: 640,
    height: 480,

    ogl_extended: 0,
};

#[repr(C)]

pub struct PIXELFORMATDESCRIPTOR {
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
pub type PPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;
pub type LPPIXELFORMATDESCRIPTOR = *mut PIXELFORMATDESCRIPTOR;


unsafe fn create_window() -> bool
{
    let mut wndclassw: WNDCLASSW = std::mem::zeroed();

    wndclassw.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wndclassw.lpfnWndProc = Some(win32_wndproc);
    wndclassw.hInstance = GetModuleHandleW(NULL as _);
    wndclassw.hCursor = LoadCursorW(NULL as _, IDC_ARROW);
    wndclassw.hIcon = LoadIconW(NULL as _, IDI_WINLOGO);
    wndclassw.lpszClassName = win32_window.class_name.as_ptr() as _;
    RegisterClassW(&wndclassw);

    
    let win_style: DWORD;
    let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    /* fullscreen
        win_style = WS_POPUP | WS_SYSMENU | WS_VISIBLE;
        rect.right = GetSystemMetrics(SM_CXSCREEN);
        rect.bottom = GetSystemMetrics(SM_CYSCREEN);

    */
/*
    win_style = WS_CLIPSIBLINGS
        | WS_CLIPCHILDREN
        | WS_CAPTION
        //| WS_SYSMENU
        | WS_MINIMIZEBOX
        | WS_MAXIMIZEBOX
        | WS_SIZEBOX;
*/
//win_style = WS_OVERLAPPEDWINDOW;

    win_style = 
        WS_OVERLAPPED | WS_CAPTION  
        | WS_THICKFRAME 
        | WS_SYSMENU
        | WS_MINIMIZEBOX | WS_MAXIMIZEBOX
        ;

    rect.right = win32_window.width;
    rect.bottom = win32_window.height;

    AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
    let win_width = rect.right - rect.left;
    let win_height = rect.bottom - rect.top;
    
    //_sapp_win32_in_create_window = true;


    win32_window.win32_hwnd = CreateWindowExW(
        win_ex_style,                // dwExStyle
        win32_window.class_name.as_ptr(),         // lpClassName
        win32_window.window_name.as_ptr(),        // lpWindowName
        win_style,                   // dwStyle
        CW_USEDEFAULT,               // X
        CW_USEDEFAULT,               // Y
        win_width,                   // nWidth
        win_height,                  // nHeight
        NULL as _,                   // hWndParent
        NULL as _,                   // hMenu
        GetModuleHandleW(NULL as _), // hInstance
        NULL as _,                   // lParam
    );
    assert!(win32_window.win32_hwnd.is_null() == false);

    if win32_window.win32_hwnd.is_null()
    {
        return false;
    }

    ShowWindow(win32_window.win32_hwnd, SW_SHOW);
    //_sapp_win32_in_create_window = false;
    let dc = GetDC(win32_window.win32_hwnd);
    assert!(dc.is_null() == false);


    win32_window.win32_dc = dc;

    if win32_window.win32_dc.is_null()
    {
        return false;
    }



    return true;







    //update_dimensions();
    
}




/* pixel types */
pub const PFD_TYPE_RGBA: BYTE =        0;
pub const PFD_TYPE_COLORINDEX: BYTE =  1;

/* layer types */
pub const PFD_MAIN_PLANE: BYTE =     0;
pub const PFD_OVERLAY_PLANE: BYTE =  1;
pub const PFD_UNDERLAY_PLANE: BYTE = -1i8 as BYTE;

/* PIXELFORMATDESCRIPTOR flags */
pub const PFD_DOUBLEBUFFER : DWORD =            0x00000001;
pub const PFD_STEREO : DWORD =                  0x00000002;
pub const PFD_DRAW_TO_WINDOW : DWORD =          0x00000004;
pub const PFD_DRAW_TO_BITMAP : DWORD =          0x00000008;
pub const PFD_SUPPORT_GDI : DWORD =             0x00000010;
pub const PFD_SUPPORT_OPENGL : DWORD =          0x00000020;
pub const PFD_GENERIC_FORMAT : DWORD =          0x00000040;
pub const PFD_NEED_PALETTE : DWORD =            0x00000080;
pub const PFD_NEED_SYSTEM_PALETTE : DWORD =     0x00000100;
pub const PFD_SWAP_EXCHANGE : DWORD =           0x00000200;
pub const PFD_SWAP_COPY : DWORD =               0x00000400;
pub const PFD_SWAP_LAYER_BUFFERS : DWORD =      0x00000800;
pub const PFD_GENERIC_ACCELERATED : DWORD =     0x00001000;
pub const PFD_SUPPORT_DIRECTDRAW : DWORD =      0x00002000;
pub const PFD_DIRECT3D_ACCELERATED : DWORD =    0x00004000;
pub const PFD_SUPPORT_COMPOSITION : DWORD =     0x00008000;

/* PIXELFORMATDESCRIPTOR flags for use in ChoosePixelFormat only */
pub const PFD_DEPTH_DONTCARE: DWORD =          0x20000000;
pub const PFD_DOUBLEBUFFER_DONTCARE: DWORD =   0x40000000;
pub const PFD_STEREO_DONTCARE: DWORD =         0x80000000;





pub const WGL_CONTEXT_DEBUG_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_PROFILE_MASK_ARB: u32 = 0x9126;
pub const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: u32 = 0x00000001;
pub const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: u32 = 0x00000002;
pub const WGL_CONTEXT_MAJOR_VERSION_ARB: u32 = 0x2091;
pub const WGL_CONTEXT_MINOR_VERSION_ARB: u32 = 0x2092;
pub const WGL_CONTEXT_FLAGS_ARB: u32 = 0x2094;
pub const WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB: u32 = 0x00000004;

unsafe fn create_opengl() -> bool
{
    let mut px_format_desired : PIXELFORMATDESCRIPTOR = std::mem::zeroed();
    px_format_desired.nSize = std::mem::size_of_val(&px_format_desired) as u16;
    px_format_desired.nVersion = 1;
    px_format_desired.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
    px_format_desired.iPixelType = PFD_TYPE_RGBA;
    px_format_desired.cColorBits = 32;
    px_format_desired.cAlphaBits = 8;
    px_format_desired.iLayerType = PFD_MAIN_PLANE;

    let suggested_pixel_format_index = ChoosePixelFormat(win32_window.win32_dc, &px_format_desired);
    let mut suggested_px_format : PIXELFORMATDESCRIPTOR = std::mem::zeroed();
    let descriptor_success = DescribePixelFormat(win32_window.win32_dc, suggested_pixel_format_index, 
        std::mem::size_of_val(&suggested_pixel_format_index) as _, &mut suggested_px_format);
    if descriptor_success == 0 && false
    {
        println!("Failed to describe pixel format!");
        return false;
    }

    if SetPixelFormat(win32_window.win32_dc, suggested_pixel_format_index, &suggested_px_format) == 0
    {
        println!("Failed to set pixel format!");
        return false;
    }

    let rc = wglCreateContext(win32_window.win32_dc);
    if rc.is_null()
    {
        println!("Failed to create opengl context.");
        return false;
    }

    win32_window.hglrc = rc;

    // Check thread?
    if wglMakeCurrent(win32_window.win32_dc, rc) == 0
    {
        println!("Failed to make current opengl context.");
        return false;
    }

    let proc = wglGetProcAddress(b"wglCreateContextAttribsARB\0".as_ptr() as *const i8);
    if proc.is_null() 
    {
        return false;
    }
    
    createContextAttribsARB = Some(std::mem::transmute_copy(&proc));


    let attrs = [
        WGL_CONTEXT_MAJOR_VERSION_ARB, 4,
        WGL_CONTEXT_MINOR_VERSION_ARB, 5,
        WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
        //WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
        WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
        0, 0,
    ];


    let share_context: HGLRC = 0 as HGLRC;
    let modern_rc = createContextAttribsARB.unwrap()(win32_window.win32_dc, share_context,  attrs.as_ptr() as *const i32);
    if !modern_rc.is_null()
    {
        if wglMakeCurrent(win32_window.win32_dc, modern_rc) != 0
        {
            wglDeleteContext(win32_window.hglrc);
            win32_window.hglrc = modern_rc;

            win32_window.ogl_extended = 1;
        }
    }
    return true;
}

fn destroy_opengl()
{
    unsafe
    {
        if !win32_window.hglrc.is_null()
        {
            wglDeleteContext(win32_window.hglrc);
            win32_window.hglrc = 0 as HGLRC;
        }
    }
}

pub const PM_NOREMOVE: UINT = 0x0000;
pub const PM_REMOVE: UINT = 0x0001;
pub const PM_NOYIELD: UINT = 0x0002;

fn destroy_window() 
{
    unsafe 
    {
        DestroyWindow(win32_window.win32_hwnd);
        win32_window.win32_hwnd = 0 as HWND;

        UnregisterClassW(win32_window.class_name.as_mut_ptr() as _, GetModuleHandleW(NULL as _));
    }
}


pub type GLenum = c_uint;
pub type GLboolean = c_uchar;
pub type GLbitfield = c_uint;
pub type GLvoid = c_void;
pub type GLbyte = c_schar;
pub type GLshort = c_short;
pub type GLint = c_int;
pub type GLubyte = c_uchar;
pub type GLushort = c_ushort;
pub type GLuint = c_uint;
pub type GLuint64 = c_ulonglong;
pub type GLsizei = c_int;
pub type GLchar = c_char;

pub type GLsizeiptr =c_long;
pub type GLintptr = c_long;

pub type GLfloat = f32;
pub type GLclampf = f32;
pub type GLdouble = f64;
pub type GLclampd = f64;

extern "C" {
    pub fn glCheckFramebufferStatus(target: GLenum) -> GLenum;
}
extern "C" {
    pub fn glClear(mask: GLbitfield);
}
extern "C" {
    pub fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
}
extern "C" {
    pub fn glClearDepthf(d: GLfloat);
}
extern "C" {
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}
extern "C" {
    pub fn glGetString(name: GLenum) -> *const GLubyte;
}

pub const GL_DEPTH_BUFFER_BIT: u32 = 256;
pub const GL_STENCIL_BUFFER_BIT: u32 = 1024;
pub const GL_COLOR_BUFFER_BIT: u32 = 16384;

pub const GL_VENDOR: u32 = 7936;
pub const GL_RENDERER: u32 = 7937;
pub const GL_VERSION: u32 = 7938;
pub const GL_EXTENSIONS: u32 = 7939;
pub const GL_SHADING_LANGUAGE_VERSION: u32 = 35724;

unsafe fn get_gl_string(string_type: u32) -> (bool, String)
{
    let mut result: String = String::new();

    let str = glGetString(string_type);
    if str.is_null()
    {
        return (false, result);
    }
    let s8 = std::ffi::CStr::from_ptr(str as *const i8).to_bytes();
    let data_str = String::from_utf8(s8.to_vec());
    match data_str
    {
        Ok(v) => result = v,
        Err(_) => return (false, result)
    };

    return (true, result);
}


fn main() 
{
    let mut done = false;
    unsafe 
    {
        win32_window.class_name = "APP_CLASS_NAME\0".encode_utf16().collect::<Vec<u16>>();
        win32_window.window_name = "TITLE!\0".encode_utf16().collect::<Vec<u16>>();
        win32_window.width = 800;
        win32_window.height = 600;
        if !create_window()
        {
            return;
        }

        if !create_opengl()
        {
            return;
        }


        /*
        let (r1, vendor) = get_gl_string(GL_VENDOR);
        let (r2, version) = get_gl_string(GL_VERSION);
        let (r3, renderer) = get_gl_string(GL_RENDERER);
        let (r4, extensions) = get_gl_string(GL_EXTENSIONS);
        
        if !r1 || !r2 || !r3 
        {
            done = true;
        }
        else
        {
            println!("OpenGL vendor {}", vendor);
            println!("OpenGL version {}", version);
            println!("OpenGL renderer {}", renderer);
            if r4
            {
                println!("OpenGL extensions {}", extensions);
            }
            if win32_window.ogl_extended != 0
            {
                let (r5, shading_language) = get_gl_string(GL_SHADING_LANGUAGE_VERSION);
                if r5
                {
                    println!("OpenGL shading language version {}", shading_language);
                }
            }

        }
        */

    }
    println!("Hello, world!");


   
    while !(done) 
    {
        let mut msg: MSG = unsafe{ std::mem::zeroed() };
        unsafe
        {
            while PeekMessageW(&mut msg as *mut _ as _, NULL as _, 0, 0, PM_REMOVE) != 0 
            {
                if WM_QUIT == msg.message 
                {
                    done = true;
                    continue;
                } 
                else 
                {
                    TranslateMessage(&mut msg as *mut _ as _);
                    DispatchMessageW(&mut msg as *mut _ as _);
                }
            }

            if quit_requested
            {
                PostMessageW(win32_window.win32_hwnd, WM_CLOSE, 0, 0);
            }
        }
        unsafe
        {
            glViewport(0, 0, win32_window.width, win32_window.height);
            glClearColor(1.0f32, 0.0f32, 1.0f32, 0.0f32);
            glClear(GL_COLOR_BUFFER_BIT);
            SwapBuffers(win32_window.win32_dc);
        }
    }

    destroy_opengl();
    destroy_window();

}
