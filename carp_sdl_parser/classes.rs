extern "C"
{
	// File: include/SDL.h
	fn SDL_Init(flags: u32) -> i32;
	fn SDL_InitSubSystem(flags: u32) -> i32;
	fn SDL_QuitSubSystem(flags: u32) -> ();
	fn SDL_WasInit(flags: u32) -> u32;
	fn SDL_Quit() -> ();

	// File: include/SDL_video.h
	fn SDL_GetNumVideoDrivers() -> i32;
	fn SDL_GetVideoDriver(index: i32) -> *const c_char;
	fn SDL_VideoInit(driver_name: *const c_char) -> i32;
	fn SDL_VideoQuit() -> ();
	fn SDL_GetCurrentVideoDriver() -> *const c_char;
	fn SDL_GetNumVideoDisplays() -> i32;
	fn SDL_GetDisplayName(displayIndex: i32) -> *const c_char;
	fn SDL_GetDisplayBounds(displayIndex: i32, rect: *mut SDL_Rect) -> i32;
	fn SDL_GetDisplayUsableBounds(displayIndex: i32, rect: *mut SDL_Rect) -> i32;
	fn SDL_GetDisplayDPI(displayIndex: i32, ddpi: *mut f32, hdpi: *mut f32, vdpi: *mut f32) -> i32;
	fn SDL_GetDisplayOrientation(displayIndex: i32) -> SDL_DisplayOrientation;
	fn SDL_GetNumDisplayModes(displayIndex: i32) -> i32;
	fn SDL_GetDisplayMode(displayIndex: i32, modeIndex: i32, mode: *mut SDL_DisplayMode) -> i32;
	fn SDL_GetDesktopDisplayMode(displayIndex: i32, mode: *mut SDL_DisplayMode) -> i32;
	fn SDL_GetCurrentDisplayMode(displayIndex: i32, mode: *mut SDL_DisplayMode) -> i32;
	fn SDL_GetClosestDisplayMode(displayIndex: i32, mode: *const SDL_DisplayMode, closest: *mut SDL_DisplayMode) -> *mut SDL_DisplayMode;
	fn SDL_GetWindowDisplayIndex(window: *mut SDL_Window) -> i32;
	fn SDL_SetWindowDisplayMode(window: *mut SDL_Window, mode: *const SDL_DisplayMode) -> i32;
	fn SDL_GetWindowDisplayMode(window: *mut SDL_Window, mode: *mut SDL_DisplayMode) -> i32;
	fn SDL_GetWindowPixelFormat(window: *mut SDL_Window) -> u32;
	fn SDL_CreateWindow(title: *const c_char, x: i32, y: i32, w: i32, h: i32, flags: u32) -> *mut SDL_Window;
	fn SDL_CreateWindowFrom(data: *const c_void) -> *mut SDL_Window;
	fn SDL_GetWindowID(window: *mut SDL_Window) -> u32;
	fn SDL_GetWindowFromID(id: u32) -> *mut SDL_Window;
	fn SDL_GetWindowFlags(window: *mut SDL_Window) -> u32;
	fn SDL_SetWindowTitle(window: *mut SDL_Window, title: *const c_char) -> ();
	fn SDL_GetWindowTitle(window: *mut SDL_Window) -> *const c_char;
	fn SDL_SetWindowIcon(window: *mut SDL_Window, icon: *mut SDL_Surface) -> ();
	fn SDL_SetWindowData(window: *mut SDL_Window, name: *const c_char, userdata: *mut c_void) -> *mut ();
	fn SDL_GetWindowData(window: *mut SDL_Window, name: *const c_char) -> *mut ();
	fn SDL_SetWindowPosition(window: *mut SDL_Window, x: i32, y: i32) -> ();
	fn SDL_GetWindowPosition(window: *mut SDL_Window, x: *mut i32, y: *mut i32) -> ();
	fn SDL_SetWindowSize(window: *mut SDL_Window, w: i32, h: i32) -> ();
	fn SDL_GetWindowSize(window: *mut SDL_Window, w: *mut i32, h: *mut i32) -> ();
	fn SDL_GetWindowBordersSize(window: *mut SDL_Window, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> i32;
	fn SDL_SetWindowMinimumSize(window: *mut SDL_Window, min_w: i32, min_h: i32) -> ();
	fn SDL_GetWindowMinimumSize(window: *mut SDL_Window, w: *mut i32, h: *mut i32) -> ();
	fn SDL_SetWindowMaximumSize(window: *mut SDL_Window, max_w: i32, max_h: i32) -> ();
	fn SDL_GetWindowMaximumSize(window: *mut SDL_Window, w: *mut i32, h: *mut i32) -> ();
	fn SDL_SetWindowBordered(window: *mut SDL_Window, bordered: SDL_bool) -> ();
	fn SDL_SetWindowResizable(window: *mut SDL_Window, resizable: SDL_bool) -> ();
	fn SDL_SetWindowAlwaysOnTop(window: *mut SDL_Window, on_top: SDL_bool) -> ();
	fn SDL_ShowWindow(window: *mut SDL_Window) -> ();
	fn SDL_HideWindow(window: *mut SDL_Window) -> ();
	fn SDL_RaiseWindow(window: *mut SDL_Window) -> ();
	fn SDL_MaximizeWindow(window: *mut SDL_Window) -> ();
	fn SDL_MinimizeWindow(window: *mut SDL_Window) -> ();
	fn SDL_RestoreWindow(window: *mut SDL_Window) -> ();
	fn SDL_SetWindowFullscreen(window: *mut SDL_Window, flags: u32) -> i32;
	fn SDL_GetWindowSurface(window: *mut SDL_Window) -> *mut SDL_Surface;
	fn SDL_UpdateWindowSurface(window: *mut SDL_Window) -> i32;
	fn SDL_UpdateWindowSurfaceRects(window: *mut SDL_Window, rects: *const SDL_Rect, numrects: i32) -> i32;
	fn SDL_SetWindowGrab(window: *mut SDL_Window, grabbed: SDL_bool) -> ();
	fn SDL_SetWindowKeyboardGrab(window: *mut SDL_Window, grabbed: SDL_bool) -> ();
	fn SDL_SetWindowMouseGrab(window: *mut SDL_Window, grabbed: SDL_bool) -> ();
	fn SDL_GetWindowGrab(window: *mut SDL_Window) -> SDL_bool;
	fn SDL_GetWindowKeyboardGrab(window: *mut SDL_Window) -> SDL_bool;
	fn SDL_GetWindowMouseGrab(window: *mut SDL_Window) -> SDL_bool;
	fn SDL_GetGrabbedWindow() -> *mut SDL_Window;
	fn SDL_SetWindowBrightness(window: *mut SDL_Window, brightness: f32) -> i32;
	fn SDL_GetWindowBrightness(window: *mut SDL_Window) -> f32;
	fn SDL_SetWindowOpacity(window: *mut SDL_Window, opacity: f32) -> i32;
	fn SDL_GetWindowOpacity(window: *mut SDL_Window, out_opacity: *mut f32) -> i32;
	fn SDL_SetWindowModalFor(modal_window: *mut SDL_Window, parent_window: *mut SDL_Window) -> i32;
	fn SDL_SetWindowInputFocus(window: *mut SDL_Window) -> i32;
	fn SDL_SetWindowGammaRamp(window: *mut SDL_Window, red: *const Uint16, green: *const Uint16, blue: *const Uint16) -> i32;
	fn SDL_GetWindowGammaRamp(window: *mut SDL_Window, red: *mut Uint16, green: *mut Uint16, blue: *mut Uint16) -> i32;
	fn SDL_SetWindowHitTest(window: *mut SDL_Window, callback: SDL_HitTest, callback_data: *mut c_void) -> i32;
	fn SDL_FlashWindow(window: *mut SDL_Window, operation: SDL_FlashOperation) -> i32;
	fn SDL_DestroyWindow(window: *mut SDL_Window) -> ();
	fn SDL_IsScreenSaverEnabled() -> SDL_bool;
	fn SDL_EnableScreenSaver() -> ();
	fn SDL_DisableScreenSaver() -> ();
	fn SDL_GL_LoadLibrary(path: *const c_char) -> i32;
	fn SDL_GL_GetProcAddress(proc: *const c_char) -> *mut ();
	fn SDL_GL_UnloadLibrary() -> ();
	fn SDL_GL_ExtensionSupported(extension: *const c_char) -> SDL_bool;
	fn SDL_GL_ResetAttributes() -> ();
	fn SDL_GL_SetAttribute(attr: SDL_GLattr, value: i32) -> i32;
	fn SDL_GL_GetAttribute(attr: SDL_GLattr, value: *mut i32) -> i32;
	fn SDL_GL_CreateContext(window: *mut SDL_Window) -> SDL_GLContext;
	fn SDL_GL_MakeCurrent(window: *mut SDL_Window, context: SDL_GLContext) -> i32;
	fn SDL_GL_GetCurrentWindow() -> *mut SDL_Window;
	fn SDL_GL_GetCurrentContext() -> SDL_GLContext;
	fn SDL_GL_GetDrawableSize(window: *mut SDL_Window, w: *mut i32, h: *mut i32) -> ();
	fn SDL_GL_SetSwapInterval(interval: i32) -> i32;
	fn SDL_GL_GetSwapInterval() -> i32;
	fn SDL_GL_SwapWindow(window: *mut SDL_Window) -> ();
	fn SDL_GL_DeleteContext(context: SDL_GLContext) -> ();

	// File: include/SDL_events.h
	fn SDL_PumpEvents() -> ();
	fn SDL_PeepEvents(events: *mut SDL_Event, numevents: i32, action: SDL_eventaction, minType: u32, maxType: u32) -> i32;
	fn SDL_HasEvent(type_name: u32) -> SDL_bool;
	fn SDL_HasEvents(minType: u32, maxType: u32) -> SDL_bool;
	fn SDL_FlushEvent(type_name: u32) -> ();
	fn SDL_FlushEvents(minType: u32, maxType: u32) -> ();
	fn SDL_PollEvent(event: *mut SDL_Event) -> i32;
	fn SDL_WaitEvent(event: *mut SDL_Event) -> i32;
	fn SDL_WaitEventTimeout(event: *mut SDL_Event, timeout: i32) -> i32;
	fn SDL_PushEvent(event: *mut SDL_Event) -> i32;
	fn SDL_SetEventFilter(filter: SDL_EventFilter, userdata: *mut c_void) -> ();
	fn SDL_GetEventFilter(filter: *mut SDL_EventFilter, userdata: *mut *mut c_void) -> SDL_bool;
	fn SDL_AddEventWatch(filter: SDL_EventFilter, userdata: *mut c_void) -> ();
	fn SDL_DelEventWatch(filter: SDL_EventFilter, userdata: *mut c_void) -> ();
	fn SDL_FilterEvents(filter: SDL_EventFilter, userdata: *mut c_void) -> ();
	fn SDL_EventState(type_name: u32, state: i32) -> u8;
	fn SDL_RegisterEvents(numevents: i32) -> u32;

	// File: include/SDL_mouse.h
	fn SDL_GetMouseFocus() -> *mut SDL_Window;
	fn SDL_GetMouseState(x: *mut i32, y: *mut i32) -> u32;
	fn SDL_GetGlobalMouseState(x: *mut i32, y: *mut i32) -> u32;
	fn SDL_GetRelativeMouseState(x: *mut i32, y: *mut i32) -> u32;
	fn SDL_WarpMouseInWindow(window: *mut SDL_Window, x: i32, y: i32) -> ();
	fn SDL_WarpMouseGlobal(x: i32, y: i32) -> i32;
	fn SDL_SetRelativeMouseMode(enabled: SDL_bool) -> i32;
	fn SDL_CaptureMouse(enabled: SDL_bool) -> i32;
	fn SDL_GetRelativeMouseMode() -> SDL_bool;
	fn SDL_CreateCursor(data: *const u8, mask: *const u8, w: i32, h: i32, hot_x: i32, hot_y: i32) -> *mut SDL_Cursor;
	fn SDL_CreateColorCursor(surface: *mut SDL_Surface, hot_x: i32, hot_y: i32) -> *mut SDL_Cursor;
	fn SDL_CreateSystemCursor(id: SDL_SystemCursor) -> *mut SDL_Cursor;
	fn SDL_SetCursor(cursor: *mut SDL_Cursor) -> ();
	fn SDL_GetCursor() -> *mut SDL_Cursor;
	fn SDL_GetDefaultCursor() -> *mut SDL_Cursor;
	fn SDL_FreeCursor(cursor: *mut SDL_Cursor) -> ();
	fn SDL_ShowCursor(toggle: i32) -> i32;

	// File: include/SDL_main.h
	fn SDL_SetMainReady() -> ();
	fn SDL_RegisterApp(name: *mut c_char, style: u32, hInst: *mut c_void) -> i32;
	fn SDL_UnregisterApp() -> ();
	fn SDL_WinRTRunApp(mainFunction: SDL_main_func, reserved: *mut c_void) -> i32;
	fn SDL_UIKitRunApp(argc: i32, argv: *mut c_char, mainFunction: SDL_main_func) -> i32;

	// File: include/SDL_keyboard.h
	fn SDL_GetKeyboardFocus() -> *mut SDL_Window;
	fn SDL_GetKeyboardState(numkeys: *mut i32) -> *const u8;
	fn SDL_GetModState() -> SDL_Keymod;
	fn SDL_SetModState(modstate: SDL_Keymod) -> ();
	fn SDL_GetKeyFromScancode(scancode: SDL_Scancode) -> SDL_Keycode;
	fn SDL_GetScancodeFromKey(key: SDL_Keycode) -> SDL_Scancode;
	fn SDL_GetScancodeName(scancode: SDL_Scancode) -> *const c_char;
	fn SDL_GetScancodeFromName(name: *const c_char) -> SDL_Scancode;
	fn SDL_GetKeyName(key: SDL_Keycode) -> *const c_char;
	fn SDL_GetKeyFromName(name: *const c_char) -> SDL_Keycode;
	fn SDL_StartTextInput() -> ();
	fn SDL_IsTextInputActive() -> SDL_bool;
	fn SDL_StopTextInput() -> ();
	fn SDL_SetTextInputRect(rect: *mut SDL_Rect) -> ();
	fn SDL_HasScreenKeyboardSupport() -> SDL_bool;
	fn SDL_IsScreenKeyboardShown(window: *mut SDL_Window) -> SDL_bool;

	// File: include/SDL_keycode.h

}