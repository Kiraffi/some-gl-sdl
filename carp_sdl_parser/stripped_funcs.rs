//File:include/SDL.h





#ifndef SDL_h_
#define SDL_h_

#include "SDL_main.h"
#include "SDL_stdinc.h"
#include "SDL_assert.h"
#include "SDL_atomic.h"
#include "SDL_audio.h"
#include "SDL_clipboard.h"
#include "SDL_cpuinfo.h"
#include "SDL_endian.h"
#include "SDL_error.h"
#include "SDL_events.h"
#include "SDL_filesystem.h"
#include "SDL_gamecontroller.h"
#include "SDL_haptic.h"
#include "SDL_hints.h"
#include "SDL_joystick.h"
#include "SDL_loadso.h"
#include "SDL_log.h"
#include "SDL_messagebox.h"
#include "SDL_metal.h"
#include "SDL_mutex.h"
#include "SDL_power.h"
#include "SDL_render.h"
#include "SDL_rwops.h"
#include "SDL_sensor.h"
#include "SDL_shape.h"
#include "SDL_system.h"
#include "SDL_thread.h"
#include "SDL_timer.h"
#include "SDL_version.h"
#include "SDL_video.h"
#include "SDL_locale.h"
#include "SDL_misc.h"

#include "begin_code.h"

#ifdef __cplusplus
extern "C" {
#endif




#define SDL_INIT_TIMER          0x00000001u
#define SDL_INIT_AUDIO          0x00000010u
#define SDL_INIT_VIDEO          0x00000020u  
#define SDL_INIT_JOYSTICK       0x00000200u  
#define SDL_INIT_HAPTIC         0x00001000u
#define SDL_INIT_GAMECONTROLLER 0x00002000u  
#define SDL_INIT_EVENTS         0x00004000u
#define SDL_INIT_SENSOR         0x00008000u
#define SDL_INIT_NOPARACHUTE    0x00100000u  
#define SDL_INIT_EVERYTHING ( \
                SDL_INIT_TIMER | SDL_INIT_AUDIO | SDL_INIT_VIDEO | SDL_INIT_EVENTS | \
                SDL_INIT_JOYSTICK | SDL_INIT_HAPTIC | SDL_INIT_GAMECONTROLLER | SDL_INIT_SENSOR \
            )



extern DECLSPEC int SDLCALL SDL_Init(Uint32 flags);


extern DECLSPEC int SDLCALL SDL_InitSubSystem(Uint32 flags);


extern DECLSPEC void SDLCALL SDL_QuitSubSystem(Uint32 flags);


extern DECLSPEC Uint32 SDLCALL SDL_WasInit(Uint32 flags);


extern DECLSPEC void SDLCALL SDL_Quit(void);


#ifdef __cplusplus
}
#endif
#include "close_code.h"

#endif 

//File:include/SDL_video.h




#ifndef SDL_video_h_
#define SDL_video_h_

#include "SDL_stdinc.h"
#include "SDL_pixels.h"
#include "SDL_rect.h"
#include "SDL_surface.h"

#include "begin_code.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef struct
{
    Uint32 format;              
    int w;                      
    int h;                      
    int refresh_rate;           
    void *driverdata;           
} SDL_DisplayMode;


typedef struct SDL_Window SDL_Window;


typedef enum
{
    SDL_WINDOW_FULLSCREEN = 0x00000001,         
    SDL_WINDOW_OPENGL = 0x00000002,             
    SDL_WINDOW_SHOWN = 0x00000004,              
    SDL_WINDOW_HIDDEN = 0x00000008,             
    SDL_WINDOW_BORDERLESS = 0x00000010,         
    SDL_WINDOW_RESIZABLE = 0x00000020,          
    SDL_WINDOW_MINIMIZED = 0x00000040,          
    SDL_WINDOW_MAXIMIZED = 0x00000080,          
    SDL_WINDOW_MOUSE_GRABBED = 0x00000100,      
    SDL_WINDOW_INPUT_FOCUS = 0x00000200,        
    SDL_WINDOW_MOUSE_FOCUS = 0x00000400,        
    SDL_WINDOW_FULLSCREEN_DESKTOP = ( SDL_WINDOW_FULLSCREEN | 0x00001000 ),
    SDL_WINDOW_FOREIGN = 0x00000800,            
    SDL_WINDOW_ALLOW_HIGHDPI = 0x00002000,      
    SDL_WINDOW_MOUSE_CAPTURE    = 0x00004000,   
    SDL_WINDOW_ALWAYS_ON_TOP    = 0x00008000,   
    SDL_WINDOW_SKIP_TASKBAR     = 0x00010000,   
    SDL_WINDOW_UTILITY          = 0x00020000,   
    SDL_WINDOW_TOOLTIP          = 0x00040000,   
    SDL_WINDOW_POPUP_MENU       = 0x00080000,   
    SDL_WINDOW_KEYBOARD_GRABBED = 0x00100000,   
    SDL_WINDOW_VULKAN           = 0x10000000,   
    SDL_WINDOW_METAL            = 0x20000000,   

    SDL_WINDOW_INPUT_GRABBED = SDL_WINDOW_MOUSE_GRABBED 
} SDL_WindowFlags;


#define SDL_WINDOWPOS_UNDEFINED_MASK    0x1FFF0000u
#define SDL_WINDOWPOS_UNDEFINED_DISPLAY(X)  (SDL_WINDOWPOS_UNDEFINED_MASK|(X))
#define SDL_WINDOWPOS_UNDEFINED         SDL_WINDOWPOS_UNDEFINED_DISPLAY(0)
#define SDL_WINDOWPOS_ISUNDEFINED(X)    \
            (((X)&0xFFFF0000) == SDL_WINDOWPOS_UNDEFINED_MASK)


#define SDL_WINDOWPOS_CENTERED_MASK    0x2FFF0000u
#define SDL_WINDOWPOS_CENTERED_DISPLAY(X)  (SDL_WINDOWPOS_CENTERED_MASK|(X))
#define SDL_WINDOWPOS_CENTERED         SDL_WINDOWPOS_CENTERED_DISPLAY(0)
#define SDL_WINDOWPOS_ISCENTERED(X)    \
            (((X)&0xFFFF0000) == SDL_WINDOWPOS_CENTERED_MASK)


typedef enum
{
    SDL_WINDOWEVENT_NONE,           
    SDL_WINDOWEVENT_SHOWN,          
    SDL_WINDOWEVENT_HIDDEN,         
    SDL_WINDOWEVENT_EXPOSED,        
    SDL_WINDOWEVENT_MOVED,          
    SDL_WINDOWEVENT_RESIZED,        
    SDL_WINDOWEVENT_SIZE_CHANGED,   
    SDL_WINDOWEVENT_MINIMIZED,      
    SDL_WINDOWEVENT_MAXIMIZED,      
    SDL_WINDOWEVENT_RESTORED,       
    SDL_WINDOWEVENT_ENTER,          
    SDL_WINDOWEVENT_LEAVE,          
    SDL_WINDOWEVENT_FOCUS_GAINED,   
    SDL_WINDOWEVENT_FOCUS_LOST,     
    SDL_WINDOWEVENT_CLOSE,          
    SDL_WINDOWEVENT_TAKE_FOCUS,     
    SDL_WINDOWEVENT_HIT_TEST        
} SDL_WindowEventID;


typedef enum
{
    SDL_DISPLAYEVENT_NONE,          
    SDL_DISPLAYEVENT_ORIENTATION,   
    SDL_DISPLAYEVENT_CONNECTED,     
    SDL_DISPLAYEVENT_DISCONNECTED   
} SDL_DisplayEventID;


typedef enum
{
    SDL_ORIENTATION_UNKNOWN,            
    SDL_ORIENTATION_LANDSCAPE,          
    SDL_ORIENTATION_LANDSCAPE_FLIPPED,  
    SDL_ORIENTATION_PORTRAIT,           
    SDL_ORIENTATION_PORTRAIT_FLIPPED    
} SDL_DisplayOrientation;


typedef enum
{
    SDL_FLASH_CANCEL,                   
    SDL_FLASH_BRIEFLY,                  
    SDL_FLASH_UNTIL_FOCUSED             
} SDL_FlashOperation;


typedef void *SDL_GLContext;


typedef enum
{
    SDL_GL_RED_SIZE,
    SDL_GL_GREEN_SIZE,
    SDL_GL_BLUE_SIZE,
    SDL_GL_ALPHA_SIZE,
    SDL_GL_BUFFER_SIZE,
    SDL_GL_DOUBLEBUFFER,
    SDL_GL_DEPTH_SIZE,
    SDL_GL_STENCIL_SIZE,
    SDL_GL_ACCUM_RED_SIZE,
    SDL_GL_ACCUM_GREEN_SIZE,
    SDL_GL_ACCUM_BLUE_SIZE,
    SDL_GL_ACCUM_ALPHA_SIZE,
    SDL_GL_STEREO,
    SDL_GL_MULTISAMPLEBUFFERS,
    SDL_GL_MULTISAMPLESAMPLES,
    SDL_GL_ACCELERATED_VISUAL,
    SDL_GL_RETAINED_BACKING,
    SDL_GL_CONTEXT_MAJOR_VERSION,
    SDL_GL_CONTEXT_MINOR_VERSION,
    SDL_GL_CONTEXT_EGL,
    SDL_GL_CONTEXT_FLAGS,
    SDL_GL_CONTEXT_PROFILE_MASK,
    SDL_GL_SHARE_WITH_CURRENT_CONTEXT,
    SDL_GL_FRAMEBUFFER_SRGB_CAPABLE,
    SDL_GL_CONTEXT_RELEASE_BEHAVIOR,
    SDL_GL_CONTEXT_RESET_NOTIFICATION,
    SDL_GL_CONTEXT_NO_ERROR
} SDL_GLattr;

typedef enum
{
    SDL_GL_CONTEXT_PROFILE_CORE           = 0x0001,
    SDL_GL_CONTEXT_PROFILE_COMPATIBILITY  = 0x0002,
    SDL_GL_CONTEXT_PROFILE_ES             = 0x0004 
} SDL_GLprofile;

typedef enum
{
    SDL_GL_CONTEXT_DEBUG_FLAG              = 0x0001,
    SDL_GL_CONTEXT_FORWARD_COMPATIBLE_FLAG = 0x0002,
    SDL_GL_CONTEXT_ROBUST_ACCESS_FLAG      = 0x0004,
    SDL_GL_CONTEXT_RESET_ISOLATION_FLAG    = 0x0008
} SDL_GLcontextFlag;

typedef enum
{
    SDL_GL_CONTEXT_RELEASE_BEHAVIOR_NONE   = 0x0000,
    SDL_GL_CONTEXT_RELEASE_BEHAVIOR_FLUSH  = 0x0001
} SDL_GLcontextReleaseFlag;

typedef enum
{
    SDL_GL_CONTEXT_RESET_NO_NOTIFICATION = 0x0000,
    SDL_GL_CONTEXT_RESET_LOSE_CONTEXT    = 0x0001
} SDL_GLContextResetNotification;




extern DECLSPEC int SDLCALL SDL_GetNumVideoDrivers(void);


extern DECLSPEC const char *SDLCALL SDL_GetVideoDriver(int index);


extern DECLSPEC int SDLCALL SDL_VideoInit(const char *driver_name);


extern DECLSPEC void SDLCALL SDL_VideoQuit(void);


extern DECLSPEC const char *SDLCALL SDL_GetCurrentVideoDriver(void);


extern DECLSPEC int SDLCALL SDL_GetNumVideoDisplays(void);


extern DECLSPEC const char * SDLCALL SDL_GetDisplayName(int displayIndex);


extern DECLSPEC int SDLCALL SDL_GetDisplayBounds(int displayIndex, SDL_Rect * rect);


extern DECLSPEC int SDLCALL SDL_GetDisplayUsableBounds(int displayIndex, SDL_Rect * rect);


extern DECLSPEC int SDLCALL SDL_GetDisplayDPI(int displayIndex, float * ddpi, float * hdpi, float * vdpi);


extern DECLSPEC SDL_DisplayOrientation SDLCALL SDL_GetDisplayOrientation(int displayIndex);


extern DECLSPEC int SDLCALL SDL_GetNumDisplayModes(int displayIndex);


extern DECLSPEC int SDLCALL SDL_GetDisplayMode(int displayIndex, int modeIndex,
                                               SDL_DisplayMode * mode);


extern DECLSPEC int SDLCALL SDL_GetDesktopDisplayMode(int displayIndex, SDL_DisplayMode * mode);


extern DECLSPEC int SDLCALL SDL_GetCurrentDisplayMode(int displayIndex, SDL_DisplayMode * mode);



extern DECLSPEC SDL_DisplayMode * SDLCALL SDL_GetClosestDisplayMode(int displayIndex, const SDL_DisplayMode * mode, SDL_DisplayMode * closest);


extern DECLSPEC int SDLCALL SDL_GetWindowDisplayIndex(SDL_Window * window);


extern DECLSPEC int SDLCALL SDL_SetWindowDisplayMode(SDL_Window * window,
                                                     const SDL_DisplayMode * mode);


extern DECLSPEC int SDLCALL SDL_GetWindowDisplayMode(SDL_Window * window,
                                                     SDL_DisplayMode * mode);


extern DECLSPEC Uint32 SDLCALL SDL_GetWindowPixelFormat(SDL_Window * window);


extern DECLSPEC SDL_Window * SDLCALL SDL_CreateWindow(const char *title,
                                                      int x, int y, int w,
                                                      int h, Uint32 flags);


extern DECLSPEC SDL_Window * SDLCALL SDL_CreateWindowFrom(const void *data);


extern DECLSPEC Uint32 SDLCALL SDL_GetWindowID(SDL_Window * window);


extern DECLSPEC SDL_Window * SDLCALL SDL_GetWindowFromID(Uint32 id);


extern DECLSPEC Uint32 SDLCALL SDL_GetWindowFlags(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_SetWindowTitle(SDL_Window * window,
                                                const char *title);


extern DECLSPEC const char *SDLCALL SDL_GetWindowTitle(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_SetWindowIcon(SDL_Window * window,
                                               SDL_Surface * icon);


extern DECLSPEC void* SDLCALL SDL_SetWindowData(SDL_Window * window,
                                                const char *name,
                                                void *userdata);


extern DECLSPEC void *SDLCALL SDL_GetWindowData(SDL_Window * window,
                                                const char *name);


extern DECLSPEC void SDLCALL SDL_SetWindowPosition(SDL_Window * window,
                                                   int x, int y);


extern DECLSPEC void SDLCALL SDL_GetWindowPosition(SDL_Window * window,
                                                   int *x, int *y);


extern DECLSPEC void SDLCALL SDL_SetWindowSize(SDL_Window * window, int w,
                                               int h);


extern DECLSPEC void SDLCALL SDL_GetWindowSize(SDL_Window * window, int *w,
                                               int *h);


extern DECLSPEC int SDLCALL SDL_GetWindowBordersSize(SDL_Window * window,
                                                     int *top, int *left,
                                                     int *bottom, int *right);


extern DECLSPEC void SDLCALL SDL_SetWindowMinimumSize(SDL_Window * window,
                                                      int min_w, int min_h);


extern DECLSPEC void SDLCALL SDL_GetWindowMinimumSize(SDL_Window * window,
                                                      int *w, int *h);


extern DECLSPEC void SDLCALL SDL_SetWindowMaximumSize(SDL_Window * window,
                                                      int max_w, int max_h);


extern DECLSPEC void SDLCALL SDL_GetWindowMaximumSize(SDL_Window * window,
                                                      int *w, int *h);


extern DECLSPEC void SDLCALL SDL_SetWindowBordered(SDL_Window * window,
                                                   SDL_bool bordered);


extern DECLSPEC void SDLCALL SDL_SetWindowResizable(SDL_Window * window,
                                                    SDL_bool resizable);



extern DECLSPEC void SDLCALL SDL_SetWindowAlwaysOnTop(SDL_Window * window,
                                                      SDL_bool on_top);

extern DECLSPEC void SDLCALL SDL_ShowWindow(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_HideWindow(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_RaiseWindow(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_MaximizeWindow(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_MinimizeWindow(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_RestoreWindow(SDL_Window * window);


extern DECLSPEC int SDLCALL SDL_SetWindowFullscreen(SDL_Window * window,
                                                    Uint32 flags);


extern DECLSPEC SDL_Surface * SDLCALL SDL_GetWindowSurface(SDL_Window * window);


extern DECLSPEC int SDLCALL SDL_UpdateWindowSurface(SDL_Window * window);


extern DECLSPEC int SDLCALL SDL_UpdateWindowSurfaceRects(SDL_Window * window,
                                                         const SDL_Rect * rects,
                                                         int numrects);


extern DECLSPEC void SDLCALL SDL_SetWindowGrab(SDL_Window * window,
                                               SDL_bool grabbed);


extern DECLSPEC void SDLCALL SDL_SetWindowKeyboardGrab(SDL_Window * window,
                                                       SDL_bool grabbed);


extern DECLSPEC void SDLCALL SDL_SetWindowMouseGrab(SDL_Window * window,
                                                    SDL_bool grabbed);


extern DECLSPEC SDL_bool SDLCALL SDL_GetWindowGrab(SDL_Window * window);


extern DECLSPEC SDL_bool SDLCALL SDL_GetWindowKeyboardGrab(SDL_Window * window);


extern DECLSPEC SDL_bool SDLCALL SDL_GetWindowMouseGrab(SDL_Window * window);


extern DECLSPEC SDL_Window * SDLCALL SDL_GetGrabbedWindow(void);


extern DECLSPEC int SDLCALL SDL_SetWindowBrightness(SDL_Window * window, float brightness);


extern DECLSPEC float SDLCALL SDL_GetWindowBrightness(SDL_Window * window);


extern DECLSPEC int SDLCALL SDL_SetWindowOpacity(SDL_Window * window, float opacity);


extern DECLSPEC int SDLCALL SDL_GetWindowOpacity(SDL_Window * window, float * out_opacity);


extern DECLSPEC int SDLCALL SDL_SetWindowModalFor(SDL_Window * modal_window, SDL_Window * parent_window);


extern DECLSPEC int SDLCALL SDL_SetWindowInputFocus(SDL_Window * window);


extern DECLSPEC int SDLCALL SDL_SetWindowGammaRamp(SDL_Window * window,
                                                   const Uint16 * red,
                                                   const Uint16 * green,
                                                   const Uint16 * blue);


extern DECLSPEC int SDLCALL SDL_GetWindowGammaRamp(SDL_Window * window,
                                                   Uint16 * red,
                                                   Uint16 * green,
                                                   Uint16 * blue);


typedef enum
{
    SDL_HITTEST_NORMAL,  
    SDL_HITTEST_DRAGGABLE,  
    SDL_HITTEST_RESIZE_TOPLEFT,
    SDL_HITTEST_RESIZE_TOP,
    SDL_HITTEST_RESIZE_TOPRIGHT,
    SDL_HITTEST_RESIZE_RIGHT,
    SDL_HITTEST_RESIZE_BOTTOMRIGHT,
    SDL_HITTEST_RESIZE_BOTTOM,
    SDL_HITTEST_RESIZE_BOTTOMLEFT,
    SDL_HITTEST_RESIZE_LEFT
} SDL_HitTestResult;


typedef SDL_HitTestResult (SDLCALL *SDL_HitTest)(SDL_Window *win,
                                                 const SDL_Point *area,
                                                 void *data);


extern DECLSPEC int SDLCALL SDL_SetWindowHitTest(SDL_Window * window,
                                                 SDL_HitTest callback,
                                                 void *callback_data);


extern DECLSPEC int SDLCALL SDL_FlashWindow(SDL_Window * window, SDL_FlashOperation operation);


extern DECLSPEC void SDLCALL SDL_DestroyWindow(SDL_Window * window);



extern DECLSPEC SDL_bool SDLCALL SDL_IsScreenSaverEnabled(void);


extern DECLSPEC void SDLCALL SDL_EnableScreenSaver(void);


extern DECLSPEC void SDLCALL SDL_DisableScreenSaver(void);





extern DECLSPEC int SDLCALL SDL_GL_LoadLibrary(const char *path);


extern DECLSPEC void *SDLCALL SDL_GL_GetProcAddress(const char *proc);


extern DECLSPEC void SDLCALL SDL_GL_UnloadLibrary(void);


extern DECLSPEC SDL_bool SDLCALL SDL_GL_ExtensionSupported(const char
                                                           *extension);


extern DECLSPEC void SDLCALL SDL_GL_ResetAttributes(void);


extern DECLSPEC int SDLCALL SDL_GL_SetAttribute(SDL_GLattr attr, int value);


extern DECLSPEC int SDLCALL SDL_GL_GetAttribute(SDL_GLattr attr, int *value);


extern DECLSPEC SDL_GLContext SDLCALL SDL_GL_CreateContext(SDL_Window *
                                                           window);


extern DECLSPEC int SDLCALL SDL_GL_MakeCurrent(SDL_Window * window,
                                               SDL_GLContext context);


extern DECLSPEC SDL_Window* SDLCALL SDL_GL_GetCurrentWindow(void);


extern DECLSPEC SDL_GLContext SDLCALL SDL_GL_GetCurrentContext(void);


extern DECLSPEC void SDLCALL SDL_GL_GetDrawableSize(SDL_Window * window, int *w,
                                                    int *h);


extern DECLSPEC int SDLCALL SDL_GL_SetSwapInterval(int interval);


extern DECLSPEC int SDLCALL SDL_GL_GetSwapInterval(void);


extern DECLSPEC void SDLCALL SDL_GL_SwapWindow(SDL_Window * window);


extern DECLSPEC void SDLCALL SDL_GL_DeleteContext(SDL_GLContext context);





#ifdef __cplusplus
}
#endif
#include "close_code.h"

#endif 

//File:include/SDL_events.h




#ifndef SDL_events_h_
#define SDL_events_h_

#include "SDL_stdinc.h"
#include "SDL_error.h"
#include "SDL_video.h"
#include "SDL_keyboard.h"
#include "SDL_mouse.h"
#include "SDL_joystick.h"
#include "SDL_gamecontroller.h"
#include "SDL_quit.h"
#include "SDL_gesture.h"
#include "SDL_touch.h"

#include "begin_code.h"

#ifdef __cplusplus
extern "C" {
#endif


#define SDL_RELEASED    0
#define SDL_PRESSED 1


typedef enum
{
    SDL_FIRSTEVENT     = 0,     

    
    SDL_QUIT           = 0x100, 

    
    SDL_APP_TERMINATING,        
    SDL_APP_LOWMEMORY,          
    SDL_APP_WILLENTERBACKGROUND, 
    SDL_APP_DIDENTERBACKGROUND, 
    SDL_APP_WILLENTERFOREGROUND, 
    SDL_APP_DIDENTERFOREGROUND, 

    SDL_LOCALECHANGED,  

    
    SDL_DISPLAYEVENT   = 0x150,  

    
    SDL_WINDOWEVENT    = 0x200, 
    SDL_SYSWMEVENT,             

    
    SDL_KEYDOWN        = 0x300, 
    SDL_KEYUP,                  
    SDL_TEXTEDITING,            
    SDL_TEXTINPUT,              
    SDL_KEYMAPCHANGED,          

    
    SDL_MOUSEMOTION    = 0x400, 
    SDL_MOUSEBUTTONDOWN,        
    SDL_MOUSEBUTTONUP,          
    SDL_MOUSEWHEEL,             

    
    SDL_JOYAXISMOTION  = 0x600, 
    SDL_JOYBALLMOTION,          
    SDL_JOYHATMOTION,           
    SDL_JOYBUTTONDOWN,          
    SDL_JOYBUTTONUP,            
    SDL_JOYDEVICEADDED,         
    SDL_JOYDEVICEREMOVED,       

    
    SDL_CONTROLLERAXISMOTION  = 0x650, 
    SDL_CONTROLLERBUTTONDOWN,          
    SDL_CONTROLLERBUTTONUP,            
    SDL_CONTROLLERDEVICEADDED,         
    SDL_CONTROLLERDEVICEREMOVED,       
    SDL_CONTROLLERDEVICEREMAPPED,      
    SDL_CONTROLLERTOUCHPADDOWN,        
    SDL_CONTROLLERTOUCHPADMOTION,      
    SDL_CONTROLLERTOUCHPADUP,          
    SDL_CONTROLLERSENSORUPDATE,        

    
    SDL_FINGERDOWN      = 0x700,
    SDL_FINGERUP,
    SDL_FINGERMOTION,

    
    SDL_DOLLARGESTURE   = 0x800,
    SDL_DOLLARRECORD,
    SDL_MULTIGESTURE,

    
    SDL_CLIPBOARDUPDATE = 0x900, 

    
    SDL_DROPFILE        = 0x1000, 
    SDL_DROPTEXT,                 
    SDL_DROPBEGIN,                
    SDL_DROPCOMPLETE,             

    
    SDL_AUDIODEVICEADDED = 0x1100, 
    SDL_AUDIODEVICEREMOVED,        

    
    SDL_SENSORUPDATE = 0x1200,     

    
    SDL_RENDER_TARGETS_RESET = 0x2000, 
    SDL_RENDER_DEVICE_RESET, 

    
    SDL_USEREVENT    = 0x8000,

    
    SDL_LASTEVENT    = 0xFFFF
} SDL_EventType;


typedef struct SDL_CommonEvent
{
    Uint32 type;
    Uint32 timestamp;   
} SDL_CommonEvent;


typedef struct SDL_DisplayEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 display;     
    Uint8 event;        
    Uint8 padding1;
    Uint8 padding2;
    Uint8 padding3;
    Sint32 data1;       
} SDL_DisplayEvent;


typedef struct SDL_WindowEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 windowID;    
    Uint8 event;        
    Uint8 padding1;
    Uint8 padding2;
    Uint8 padding3;
    Sint32 data1;       
    Sint32 data2;       
} SDL_WindowEvent;


typedef struct SDL_KeyboardEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 windowID;    
    Uint8 state;        
    Uint8 repeat;       
    Uint8 padding2;
    Uint8 padding3;
    SDL_Keysym keysym;  
} SDL_KeyboardEvent;

#define SDL_TEXTEDITINGEVENT_TEXT_SIZE (32)

typedef struct SDL_TextEditingEvent
{
    Uint32 type;                                
    Uint32 timestamp;                           
    Uint32 windowID;                            
    char text[SDL_TEXTEDITINGEVENT_TEXT_SIZE];  
    Sint32 start;                               
    Sint32 length;                              
} SDL_TextEditingEvent;


#define SDL_TEXTINPUTEVENT_TEXT_SIZE (32)

typedef struct SDL_TextInputEvent
{
    Uint32 type;                              
    Uint32 timestamp;                         
    Uint32 windowID;                          
    char text[SDL_TEXTINPUTEVENT_TEXT_SIZE];  
} SDL_TextInputEvent;


typedef struct SDL_MouseMotionEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 windowID;    
    Uint32 which;       
    Uint32 state;       
    Sint32 x;           
    Sint32 y;           
    Sint32 xrel;        
    Sint32 yrel;        
} SDL_MouseMotionEvent;


typedef struct SDL_MouseButtonEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 windowID;    
    Uint32 which;       
    Uint8 button;       
    Uint8 state;        
    Uint8 clicks;       
    Uint8 padding1;
    Sint32 x;           
    Sint32 y;           
} SDL_MouseButtonEvent;


typedef struct SDL_MouseWheelEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 windowID;    
    Uint32 which;       
    Sint32 x;           
    Sint32 y;           
    Uint32 direction;   
} SDL_MouseWheelEvent;


typedef struct SDL_JoyAxisEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Uint8 axis;         
    Uint8 padding1;
    Uint8 padding2;
    Uint8 padding3;
    Sint16 value;       
    Uint16 padding4;
} SDL_JoyAxisEvent;


typedef struct SDL_JoyBallEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Uint8 ball;         
    Uint8 padding1;
    Uint8 padding2;
    Uint8 padding3;
    Sint16 xrel;        
    Sint16 yrel;        
} SDL_JoyBallEvent;


typedef struct SDL_JoyHatEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Uint8 hat;          
    Uint8 value;        
    Uint8 padding1;
    Uint8 padding2;
} SDL_JoyHatEvent;


typedef struct SDL_JoyButtonEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Uint8 button;       
    Uint8 state;        
    Uint8 padding1;
    Uint8 padding2;
} SDL_JoyButtonEvent;


typedef struct SDL_JoyDeviceEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Sint32 which;       
} SDL_JoyDeviceEvent;



typedef struct SDL_ControllerAxisEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Uint8 axis;         
    Uint8 padding1;
    Uint8 padding2;
    Uint8 padding3;
    Sint16 value;       
    Uint16 padding4;
} SDL_ControllerAxisEvent;



typedef struct SDL_ControllerButtonEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Uint8 button;       
    Uint8 state;        
    Uint8 padding1;
    Uint8 padding2;
} SDL_ControllerButtonEvent;



typedef struct SDL_ControllerDeviceEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Sint32 which;       
} SDL_ControllerDeviceEvent;


typedef struct SDL_ControllerTouchpadEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Sint32 touchpad;    
    Sint32 finger;      
    float x;            
    float y;            
    float pressure;     
} SDL_ControllerTouchpadEvent;


typedef struct SDL_ControllerSensorEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_JoystickID which; 
    Sint32 sensor;      
    float data[3];      
} SDL_ControllerSensorEvent;


typedef struct SDL_AudioDeviceEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 which;       
    Uint8 iscapture;    
    Uint8 padding1;
    Uint8 padding2;
    Uint8 padding3;
} SDL_AudioDeviceEvent;



typedef struct SDL_TouchFingerEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_TouchID touchId; 
    SDL_FingerID fingerId;
    float x;            
    float y;            
    float dx;           
    float dy;           
    float pressure;     
    Uint32 windowID;    
} SDL_TouchFingerEvent;



typedef struct SDL_MultiGestureEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_TouchID touchId; 
    float dTheta;
    float dDist;
    float x;
    float y;
    Uint16 numFingers;
    Uint16 padding;
} SDL_MultiGestureEvent;



typedef struct SDL_DollarGestureEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_TouchID touchId; 
    SDL_GestureID gestureId;
    Uint32 numFingers;
    float error;
    float x;            
    float y;            
} SDL_DollarGestureEvent;



typedef struct SDL_DropEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    char *file;         
    Uint32 windowID;    
} SDL_DropEvent;



typedef struct SDL_SensorEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Sint32 which;       
    float data[6];      
} SDL_SensorEvent;


typedef struct SDL_QuitEvent
{
    Uint32 type;        
    Uint32 timestamp;   
} SDL_QuitEvent;


typedef struct SDL_OSEvent
{
    Uint32 type;        
    Uint32 timestamp;   
} SDL_OSEvent;


typedef struct SDL_UserEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    Uint32 windowID;    
    Sint32 code;        
    void *data1;        
    void *data2;        
} SDL_UserEvent;


struct SDL_SysWMmsg;
typedef struct SDL_SysWMmsg SDL_SysWMmsg;


typedef struct SDL_SysWMEvent
{
    Uint32 type;        
    Uint32 timestamp;   
    SDL_SysWMmsg *msg;  
} SDL_SysWMEvent;


typedef union SDL_Event
{
    Uint32 type;                            
    SDL_CommonEvent common;                 
    SDL_DisplayEvent display;               
    SDL_WindowEvent window;                 
    SDL_KeyboardEvent key;                  
    SDL_TextEditingEvent edit;              
    SDL_TextInputEvent text;                
    SDL_MouseMotionEvent motion;            
    SDL_MouseButtonEvent button;            
    SDL_MouseWheelEvent wheel;              
    SDL_JoyAxisEvent jaxis;                 
    SDL_JoyBallEvent jball;                 
    SDL_JoyHatEvent jhat;                   
    SDL_JoyButtonEvent jbutton;             
    SDL_JoyDeviceEvent jdevice;             
    SDL_ControllerAxisEvent caxis;          
    SDL_ControllerButtonEvent cbutton;      
    SDL_ControllerDeviceEvent cdevice;      
    SDL_ControllerTouchpadEvent ctouchpad;  
    SDL_ControllerSensorEvent csensor;      
    SDL_AudioDeviceEvent adevice;           
    SDL_SensorEvent sensor;                 
    SDL_QuitEvent quit;                     
    SDL_UserEvent user;                     
    SDL_SysWMEvent syswm;                   
    SDL_TouchFingerEvent tfinger;           
    SDL_MultiGestureEvent mgesture;         
    SDL_DollarGestureEvent dgesture;        
    SDL_DropEvent drop;                     

    
    Uint8 padding[sizeof(void *) <= 8 ? 56 : sizeof(void *) == 16 ? 64 : 3 * sizeof(void *)];
} SDL_Event;


SDL_COMPILE_TIME_ASSERT(SDL_Event, sizeof(SDL_Event) == sizeof(((SDL_Event *)NULL)->padding));





extern DECLSPEC void SDLCALL SDL_PumpEvents(void);


typedef enum
{
    SDL_ADDEVENT,
    SDL_PEEKEVENT,
    SDL_GETEVENT
} SDL_eventaction;


extern DECLSPEC int SDLCALL SDL_PeepEvents(SDL_Event * events, int numevents,
                                           SDL_eventaction action,
                                           Uint32 minType, Uint32 maxType);



extern DECLSPEC SDL_bool SDLCALL SDL_HasEvent(Uint32 type);



extern DECLSPEC SDL_bool SDLCALL SDL_HasEvents(Uint32 minType, Uint32 maxType);


extern DECLSPEC void SDLCALL SDL_FlushEvent(Uint32 type);


extern DECLSPEC void SDLCALL SDL_FlushEvents(Uint32 minType, Uint32 maxType);


extern DECLSPEC int SDLCALL SDL_PollEvent(SDL_Event * event);


extern DECLSPEC int SDLCALL SDL_WaitEvent(SDL_Event * event);


extern DECLSPEC int SDLCALL SDL_WaitEventTimeout(SDL_Event * event,
                                                 int timeout);


extern DECLSPEC int SDLCALL SDL_PushEvent(SDL_Event * event);


typedef int (SDLCALL * SDL_EventFilter) (void *userdata, SDL_Event * event);


extern DECLSPEC void SDLCALL SDL_SetEventFilter(SDL_EventFilter filter,
                                                void *userdata);


extern DECLSPEC SDL_bool SDLCALL SDL_GetEventFilter(SDL_EventFilter * filter,
                                                    void **userdata);


extern DECLSPEC void SDLCALL SDL_AddEventWatch(SDL_EventFilter filter,
                                               void *userdata);


extern DECLSPEC void SDLCALL SDL_DelEventWatch(SDL_EventFilter filter,
                                               void *userdata);


extern DECLSPEC void SDLCALL SDL_FilterEvents(SDL_EventFilter filter,
                                              void *userdata);


#define SDL_QUERY   -1
#define SDL_IGNORE   0
#define SDL_DISABLE  0
#define SDL_ENABLE   1


extern DECLSPEC Uint8 SDLCALL SDL_EventState(Uint32 type, int state);

#define SDL_GetEventState(type) SDL_EventState(type, SDL_QUERY)


extern DECLSPEC Uint32 SDLCALL SDL_RegisterEvents(int numevents);


#ifdef __cplusplus
}
#endif
#include "close_code.h"

#endif 

//File:include/SDL_mouse.h




#ifndef SDL_mouse_h_
#define SDL_mouse_h_

#include "SDL_stdinc.h"
#include "SDL_error.h"
#include "SDL_video.h"

#include "begin_code.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct SDL_Cursor SDL_Cursor;   


typedef enum
{
    SDL_SYSTEM_CURSOR_ARROW,     
    SDL_SYSTEM_CURSOR_IBEAM,     
    SDL_SYSTEM_CURSOR_WAIT,      
    SDL_SYSTEM_CURSOR_CROSSHAIR, 
    SDL_SYSTEM_CURSOR_WAITARROW, 
    SDL_SYSTEM_CURSOR_SIZENWSE,  
    SDL_SYSTEM_CURSOR_SIZENESW,  
    SDL_SYSTEM_CURSOR_SIZEWE,    
    SDL_SYSTEM_CURSOR_SIZENS,    
    SDL_SYSTEM_CURSOR_SIZEALL,   
    SDL_SYSTEM_CURSOR_NO,        
    SDL_SYSTEM_CURSOR_HAND,      
    SDL_NUM_SYSTEM_CURSORS
} SDL_SystemCursor;


typedef enum
{
    SDL_MOUSEWHEEL_NORMAL,    
    SDL_MOUSEWHEEL_FLIPPED    
} SDL_MouseWheelDirection;




extern DECLSPEC SDL_Window * SDLCALL SDL_GetMouseFocus(void);


extern DECLSPEC Uint32 SDLCALL SDL_GetMouseState(int *x, int *y);


extern DECLSPEC Uint32 SDLCALL SDL_GetGlobalMouseState(int *x, int *y);


extern DECLSPEC Uint32 SDLCALL SDL_GetRelativeMouseState(int *x, int *y);


extern DECLSPEC void SDLCALL SDL_WarpMouseInWindow(SDL_Window * window,
                                                   int x, int y);


extern DECLSPEC int SDLCALL SDL_WarpMouseGlobal(int x, int y);


extern DECLSPEC int SDLCALL SDL_SetRelativeMouseMode(SDL_bool enabled);


extern DECLSPEC int SDLCALL SDL_CaptureMouse(SDL_bool enabled);


extern DECLSPEC SDL_bool SDLCALL SDL_GetRelativeMouseMode(void);


extern DECLSPEC SDL_Cursor *SDLCALL SDL_CreateCursor(const Uint8 * data,
                                                     const Uint8 * mask,
                                                     int w, int h, int hot_x,
                                                     int hot_y);


extern DECLSPEC SDL_Cursor *SDLCALL SDL_CreateColorCursor(SDL_Surface *surface,
                                                          int hot_x,
                                                          int hot_y);


extern DECLSPEC SDL_Cursor *SDLCALL SDL_CreateSystemCursor(SDL_SystemCursor id);


extern DECLSPEC void SDLCALL SDL_SetCursor(SDL_Cursor * cursor);


extern DECLSPEC SDL_Cursor *SDLCALL SDL_GetCursor(void);


extern DECLSPEC SDL_Cursor *SDLCALL SDL_GetDefaultCursor(void);


extern DECLSPEC void SDLCALL SDL_FreeCursor(SDL_Cursor * cursor);


extern DECLSPEC int SDLCALL SDL_ShowCursor(int toggle);


#define SDL_BUTTON(X)       (1 << ((X)-1))
#define SDL_BUTTON_LEFT     1
#define SDL_BUTTON_MIDDLE   2
#define SDL_BUTTON_RIGHT    3
#define SDL_BUTTON_X1       4
#define SDL_BUTTON_X2       5
#define SDL_BUTTON_LMASK    SDL_BUTTON(SDL_BUTTON_LEFT)
#define SDL_BUTTON_MMASK    SDL_BUTTON(SDL_BUTTON_MIDDLE)
#define SDL_BUTTON_RMASK    SDL_BUTTON(SDL_BUTTON_RIGHT)
#define SDL_BUTTON_X1MASK   SDL_BUTTON(SDL_BUTTON_X1)
#define SDL_BUTTON_X2MASK   SDL_BUTTON(SDL_BUTTON_X2)


#ifdef __cplusplus
}
#endif
#include "close_code.h"

#endif 

//File:include/SDL_main.h


#ifndef SDL_main_h_
#define SDL_main_h_

#include "SDL_stdinc.h"



#ifndef SDL_MAIN_HANDLED
#if defined(__WIN32__)

#define SDL_MAIN_AVAILABLE

#elif defined(__WINRT__)

#define SDL_MAIN_NEEDED

#elif defined(__IPHONEOS__)

#define SDL_MAIN_NEEDED

#elif defined(__ANDROID__)

#define SDL_MAIN_NEEDED


#define SDLMAIN_DECLSPEC    DECLSPEC

#elif defined(__NACL__)

#define SDL_MAIN_NEEDED

#endif
#endif 

#ifndef SDLMAIN_DECLSPEC
#define SDLMAIN_DECLSPEC
#endif



#if defined(SDL_MAIN_NEEDED) || defined(SDL_MAIN_AVAILABLE)
#define main    SDL_main
#endif

#include "begin_code.h"
#ifdef __cplusplus
extern "C" {
#endif


typedef int (*SDL_main_func)(int argc, char *argv[]);
extern SDLMAIN_DECLSPEC int SDL_main(int argc, char *argv[]);



extern DECLSPEC void SDLCALL SDL_SetMainReady(void);

#ifdef __WIN32__


extern DECLSPEC int SDLCALL SDL_RegisterApp(char *name, Uint32 style, void *hInst);
extern DECLSPEC void SDLCALL SDL_UnregisterApp(void);

#endif 


#ifdef __WINRT__


extern DECLSPEC int SDLCALL SDL_WinRTRunApp(SDL_main_func mainFunction, void * reserved);

#endif 

#if defined(__IPHONEOS__)


extern DECLSPEC int SDLCALL SDL_UIKitRunApp(int argc, char *argv[], SDL_main_func mainFunction);

#endif 


#ifdef __cplusplus
}
#endif
#include "close_code.h"

#endif 

//File:include/SDL_keyboard.h




#ifndef SDL_keyboard_h_
#define SDL_keyboard_h_

#include "SDL_stdinc.h"
#include "SDL_error.h"
#include "SDL_keycode.h"
#include "SDL_video.h"

#include "begin_code.h"

#ifdef __cplusplus
extern "C" {
#endif


typedef struct SDL_Keysym
{
    SDL_Scancode scancode;      
    SDL_Keycode sym;            
    Uint16 mod;                 
    Uint32 unused;
} SDL_Keysym;




extern DECLSPEC SDL_Window * SDLCALL SDL_GetKeyboardFocus(void);


extern DECLSPEC const Uint8 *SDLCALL SDL_GetKeyboardState(int *numkeys);


extern DECLSPEC SDL_Keymod SDLCALL SDL_GetModState(void);


extern DECLSPEC void SDLCALL SDL_SetModState(SDL_Keymod modstate);


extern DECLSPEC SDL_Keycode SDLCALL SDL_GetKeyFromScancode(SDL_Scancode scancode);


extern DECLSPEC SDL_Scancode SDLCALL SDL_GetScancodeFromKey(SDL_Keycode key);


extern DECLSPEC const char *SDLCALL SDL_GetScancodeName(SDL_Scancode scancode);


extern DECLSPEC SDL_Scancode SDLCALL SDL_GetScancodeFromName(const char *name);


extern DECLSPEC const char *SDLCALL SDL_GetKeyName(SDL_Keycode key);


extern DECLSPEC SDL_Keycode SDLCALL SDL_GetKeyFromName(const char *name);


extern DECLSPEC void SDLCALL SDL_StartTextInput(void);


extern DECLSPEC SDL_bool SDLCALL SDL_IsTextInputActive(void);


extern DECLSPEC void SDLCALL SDL_StopTextInput(void);


extern DECLSPEC void SDLCALL SDL_SetTextInputRect(SDL_Rect *rect);


extern DECLSPEC SDL_bool SDLCALL SDL_HasScreenKeyboardSupport(void);


extern DECLSPEC SDL_bool SDLCALL SDL_IsScreenKeyboardShown(SDL_Window *window);


#ifdef __cplusplus
}
#endif
#include "close_code.h"

#endif 

//File:include/SDL_joystick.h




#ifndef SDL_joystick_h_
#define SDL_joystick_h_

#include "SDL_stdinc.h"
#include "SDL_error.h"

#include "begin_code.h"

#ifdef __cplusplus
extern "C" {
#endif




struct _SDL_Joystick;
typedef struct _SDL_Joystick SDL_Joystick;


typedef struct {
    Uint8 data[16];
} SDL_JoystickGUID;


typedef Sint32 SDL_JoystickID;

typedef enum
{
    SDL_JOYSTICK_TYPE_UNKNOWN,
    SDL_JOYSTICK_TYPE_GAMECONTROLLER,
    SDL_JOYSTICK_TYPE_WHEEL,
    SDL_JOYSTICK_TYPE_ARCADE_STICK,
    SDL_JOYSTICK_TYPE_FLIGHT_STICK,
    SDL_JOYSTICK_TYPE_DANCE_PAD,
    SDL_JOYSTICK_TYPE_GUITAR,
    SDL_JOYSTICK_TYPE_DRUM_KIT,
    SDL_JOYSTICK_TYPE_ARCADE_PAD,
    SDL_JOYSTICK_TYPE_THROTTLE
} SDL_JoystickType;

typedef enum
{
    SDL_JOYSTICK_POWER_UNKNOWN = -1,
    SDL_JOYSTICK_POWER_EMPTY,   
    SDL_JOYSTICK_POWER_LOW,     
    SDL_JOYSTICK_POWER_MEDIUM,  
    SDL_JOYSTICK_POWER_FULL,    
    SDL_JOYSTICK_POWER_WIRED,
    SDL_JOYSTICK_POWER_MAX
} SDL_JoystickPowerLevel;


#define SDL_IPHONE_MAX_GFORCE 5.0





extern DECLSPEC void SDLCALL SDL_LockJoysticks(void);



extern DECLSPEC void SDLCALL SDL_UnlockJoysticks(void);


extern DECLSPEC int SDLCALL SDL_NumJoysticks(void);


extern DECLSPEC const char *SDLCALL SDL_JoystickNameForIndex(int device_index);


extern DECLSPEC int SDLCALL SDL_JoystickGetDevicePlayerIndex(int device_index);


extern DECLSPEC SDL_JoystickGUID SDLCALL SDL_JoystickGetDeviceGUID(int device_index);


extern DECLSPEC Uint16 SDLCALL SDL_JoystickGetDeviceVendor(int device_index);


extern DECLSPEC Uint16 SDLCALL SDL_JoystickGetDeviceProduct(int device_index);


extern DECLSPEC Uint16 SDLCALL SDL_JoystickGetDeviceProductVersion(int device_index);


extern DECLSPEC SDL_JoystickType SDLCALL SDL_JoystickGetDeviceType(int device_index);


extern DECLSPEC SDL_JoystickID SDLCALL SDL_JoystickGetDeviceInstanceID(int device_index);


extern DECLSPEC SDL_Joystick *SDLCALL SDL_JoystickOpen(int device_index);


extern DECLSPEC SDL_Joystick *SDLCALL SDL_JoystickFromInstanceID(SDL_JoystickID instance_id);


extern DECLSPEC SDL_Joystick *SDLCALL SDL_JoystickFromPlayerIndex(int player_index);


extern DECLSPEC int SDLCALL SDL_JoystickAttachVirtual(SDL_JoystickType type,
                                                      int naxes,
                                                      int nbuttons,
                                                      int nhats);


extern DECLSPEC int SDLCALL SDL_JoystickDetachVirtual(int device_index);


extern DECLSPEC SDL_bool SDLCALL SDL_JoystickIsVirtual(int device_index);


extern DECLSPEC int SDLCALL SDL_JoystickSetVirtualAxis(SDL_Joystick *joystick, int axis, Sint16 value);


extern DECLSPEC int SDLCALL SDL_JoystickSetVirtualButton(SDL_Joystick *joystick, int button, Uint8 value);


extern DECLSPEC int SDLCALL SDL_JoystickSetVirtualHat(SDL_Joystick *joystick, int hat, Uint8 value);


extern DECLSPEC const char *SDLCALL SDL_JoystickName(SDL_Joystick *joystick);


extern DECLSPEC int SDLCALL SDL_JoystickGetPlayerIndex(SDL_Joystick *joystick);


extern DECLSPEC void SDLCALL SDL_JoystickSetPlayerIndex(SDL_Joystick *joystick, int player_index);


extern DECLSPEC SDL_JoystickGUID SDLCALL SDL_JoystickGetGUID(SDL_Joystick *joystick);


extern DECLSPEC Uint16 SDLCALL SDL_JoystickGetVendor(SDL_Joystick *joystick);


extern DECLSPEC Uint16 SDLCALL SDL_JoystickGetProduct(SDL_Joystick *joystick);


extern DECLSPEC Uint16 SDLCALL SDL_JoystickGetProductVersion(SDL_Joystick *joystick);


extern DECLSPEC const char * SDLCALL SDL_JoystickGetSerial(SDL_Joystick *joystick);


extern DECLSPEC SDL_JoystickType SDLCALL SDL_JoystickGetType(SDL_Joystick *joystick);


extern DECLSPEC void SDLCALL SDL_JoystickGetGUIDString(SDL_JoystickGUID guid, char *pszGUID, int cbGUID);


extern DECLSPEC SDL_JoystickGUID SDLCALL SDL_JoystickGetGUIDFromString(const char *pchGUID);


extern DECLSPEC SDL_bool SDLCALL SDL_JoystickGetAttached(SDL_Joystick *joystick);


extern DECLSPEC SDL_JoystickID SDLCALL SDL_JoystickInstanceID(SDL_Joystick *joystick);


extern DECLSPEC int SDLCALL SDL_JoystickNumAxes(SDL_Joystick *joystick);


extern DECLSPEC int SDLCALL SDL_JoystickNumBalls(SDL_Joystick *joystick);


extern DECLSPEC int SDLCALL SDL_JoystickNumHats(SDL_Joystick *joystick);


extern DECLSPEC int SDLCALL SDL_JoystickNumButtons(SDL_Joystick *joystick);


extern DECLSPEC void SDLCALL SDL_JoystickUpdate(void);


extern DECLSPEC int SDLCALL SDL_JoystickEventState(int state);

#define SDL_JOYSTICK_AXIS_MAX   32767
#define SDL_JOYSTICK_AXIS_MIN   -32768

extern DECLSPEC Sint16 SDLCALL SDL_JoystickGetAxis(SDL_Joystick *joystick,
                                                   int axis);


extern DECLSPEC SDL_bool SDLCALL SDL_JoystickGetAxisInitialState(SDL_Joystick *joystick,
                                                   int axis, Sint16 *state);


#define SDL_HAT_CENTERED    0x00
#define SDL_HAT_UP          0x01
#define SDL_HAT_RIGHT       0x02
#define SDL_HAT_DOWN        0x04
#define SDL_HAT_LEFT        0x08
#define SDL_HAT_RIGHTUP     (SDL_HAT_RIGHT|SDL_HAT_UP)
#define SDL_HAT_RIGHTDOWN   (SDL_HAT_RIGHT|SDL_HAT_DOWN)
#define SDL_HAT_LEFTUP      (SDL_HAT_LEFT|SDL_HAT_UP)
#define SDL_HAT_LEFTDOWN    (SDL_HAT_LEFT|SDL_HAT_DOWN)



extern DECLSPEC Uint8 SDLCALL SDL_JoystickGetHat(SDL_Joystick *joystick,
                                                 int hat);


extern DECLSPEC int SDLCALL SDL_JoystickGetBall(SDL_Joystick *joystick,
                                                int ball, int *dx, int *dy);


extern DECLSPEC Uint8 SDLCALL SDL_JoystickGetButton(SDL_Joystick *joystick,
                                                    int button);


extern DECLSPEC int SDLCALL SDL_JoystickRumble(SDL_Joystick *joystick, Uint16 low_frequency_rumble, Uint16 high_frequency_rumble, Uint32 duration_ms);


extern DECLSPEC int SDLCALL SDL_JoystickRumbleTriggers(SDL_Joystick *joystick, Uint16 left_rumble, Uint16 right_rumble, Uint32 duration_ms);


extern DECLSPEC SDL_bool SDLCALL SDL_JoystickHasLED(SDL_Joystick *joystick);


extern DECLSPEC int SDLCALL SDL_JoystickSetLED(SDL_Joystick *joystick, Uint8 red, Uint8 green, Uint8 blue);


extern DECLSPEC int SDLCALL SDL_JoystickSendEffect(SDL_Joystick *joystick, const void *data, int size);


extern DECLSPEC void SDLCALL SDL_JoystickClose(SDL_Joystick *joystick);


extern DECLSPEC SDL_JoystickPowerLevel SDLCALL SDL_JoystickCurrentPowerLevel(SDL_Joystick *joystick);


#ifdef __cplusplus
}
#endif
#include "close_code.h"

#endif 

//File:include/SDL_scancode.h




#ifndef SDL_scancode_h_
#define SDL_scancode_h_

#include "SDL_stdinc.h"


typedef enum
{
    SDL_SCANCODE_UNKNOWN = 0,

    
    

    SDL_SCANCODE_A = 4,
    SDL_SCANCODE_B = 5,
    SDL_SCANCODE_C = 6,
    SDL_SCANCODE_D = 7,
    SDL_SCANCODE_E = 8,
    SDL_SCANCODE_F = 9,
    SDL_SCANCODE_G = 10,
    SDL_SCANCODE_H = 11,
    SDL_SCANCODE_I = 12,
    SDL_SCANCODE_J = 13,
    SDL_SCANCODE_K = 14,
    SDL_SCANCODE_L = 15,
    SDL_SCANCODE_M = 16,
    SDL_SCANCODE_N = 17,
    SDL_SCANCODE_O = 18,
    SDL_SCANCODE_P = 19,
    SDL_SCANCODE_Q = 20,
    SDL_SCANCODE_R = 21,
    SDL_SCANCODE_S = 22,
    SDL_SCANCODE_T = 23,
    SDL_SCANCODE_U = 24,
    SDL_SCANCODE_V = 25,
    SDL_SCANCODE_W = 26,
    SDL_SCANCODE_X = 27,
    SDL_SCANCODE_Y = 28,
    SDL_SCANCODE_Z = 29,

    SDL_SCANCODE_1 = 30,
    SDL_SCANCODE_2 = 31,
    SDL_SCANCODE_3 = 32,
    SDL_SCANCODE_4 = 33,
    SDL_SCANCODE_5 = 34,
    SDL_SCANCODE_6 = 35,
    SDL_SCANCODE_7 = 36,
    SDL_SCANCODE_8 = 37,
    SDL_SCANCODE_9 = 38,
    SDL_SCANCODE_0 = 39,

    SDL_SCANCODE_RETURN = 40,
    SDL_SCANCODE_ESCAPE = 41,
    SDL_SCANCODE_BACKSPACE = 42,
    SDL_SCANCODE_TAB = 43,
    SDL_SCANCODE_SPACE = 44,

    SDL_SCANCODE_MINUS = 45,
    SDL_SCANCODE_EQUALS = 46,
    SDL_SCANCODE_LEFTBRACKET = 47,
    SDL_SCANCODE_RIGHTBRACKET = 48,
    SDL_SCANCODE_BACKSLASH = 49, 
    SDL_SCANCODE_NONUSHASH = 50, 
    SDL_SCANCODE_SEMICOLON = 51,
    SDL_SCANCODE_APOSTROPHE = 52,
    SDL_SCANCODE_GRAVE = 53, 
    SDL_SCANCODE_COMMA = 54,
    SDL_SCANCODE_PERIOD = 55,
    SDL_SCANCODE_SLASH = 56,

    SDL_SCANCODE_CAPSLOCK = 57,

    SDL_SCANCODE_F1 = 58,
    SDL_SCANCODE_F2 = 59,
    SDL_SCANCODE_F3 = 60,
    SDL_SCANCODE_F4 = 61,
    SDL_SCANCODE_F5 = 62,
    SDL_SCANCODE_F6 = 63,
    SDL_SCANCODE_F7 = 64,
    SDL_SCANCODE_F8 = 65,
    SDL_SCANCODE_F9 = 66,
    SDL_SCANCODE_F10 = 67,
    SDL_SCANCODE_F11 = 68,
    SDL_SCANCODE_F12 = 69,

    SDL_SCANCODE_PRINTSCREEN = 70,
    SDL_SCANCODE_SCROLLLOCK = 71,
    SDL_SCANCODE_PAUSE = 72,
    SDL_SCANCODE_INSERT = 73, 
    SDL_SCANCODE_HOME = 74,
    SDL_SCANCODE_PAGEUP = 75,
    SDL_SCANCODE_DELETE = 76,
    SDL_SCANCODE_END = 77,
    SDL_SCANCODE_PAGEDOWN = 78,
    SDL_SCANCODE_RIGHT = 79,
    SDL_SCANCODE_LEFT = 80,
    SDL_SCANCODE_DOWN = 81,
    SDL_SCANCODE_UP = 82,

    SDL_SCANCODE_NUMLOCKCLEAR = 83, 
    SDL_SCANCODE_KP_DIVIDE = 84,
    SDL_SCANCODE_KP_MULTIPLY = 85,
    SDL_SCANCODE_KP_MINUS = 86,
    SDL_SCANCODE_KP_PLUS = 87,
    SDL_SCANCODE_KP_ENTER = 88,
    SDL_SCANCODE_KP_1 = 89,
    SDL_SCANCODE_KP_2 = 90,
    SDL_SCANCODE_KP_3 = 91,
    SDL_SCANCODE_KP_4 = 92,
    SDL_SCANCODE_KP_5 = 93,
    SDL_SCANCODE_KP_6 = 94,
    SDL_SCANCODE_KP_7 = 95,
    SDL_SCANCODE_KP_8 = 96,
    SDL_SCANCODE_KP_9 = 97,
    SDL_SCANCODE_KP_0 = 98,
    SDL_SCANCODE_KP_PERIOD = 99,

    SDL_SCANCODE_NONUSBACKSLASH = 100, 
    SDL_SCANCODE_APPLICATION = 101, 
    SDL_SCANCODE_POWER = 102, 
    SDL_SCANCODE_KP_EQUALS = 103,
    SDL_SCANCODE_F13 = 104,
    SDL_SCANCODE_F14 = 105,
    SDL_SCANCODE_F15 = 106,
    SDL_SCANCODE_F16 = 107,
    SDL_SCANCODE_F17 = 108,
    SDL_SCANCODE_F18 = 109,
    SDL_SCANCODE_F19 = 110,
    SDL_SCANCODE_F20 = 111,
    SDL_SCANCODE_F21 = 112,
    SDL_SCANCODE_F22 = 113,
    SDL_SCANCODE_F23 = 114,
    SDL_SCANCODE_F24 = 115,
    SDL_SCANCODE_EXECUTE = 116,
    SDL_SCANCODE_HELP = 117,
    SDL_SCANCODE_MENU = 118,
    SDL_SCANCODE_SELECT = 119,
    SDL_SCANCODE_STOP = 120,
    SDL_SCANCODE_AGAIN = 121,   
    SDL_SCANCODE_UNDO = 122,
    SDL_SCANCODE_CUT = 123,
    SDL_SCANCODE_COPY = 124,
    SDL_SCANCODE_PASTE = 125,
    SDL_SCANCODE_FIND = 126,
    SDL_SCANCODE_MUTE = 127,
    SDL_SCANCODE_VOLUMEUP = 128,
    SDL_SCANCODE_VOLUMEDOWN = 129,

    SDL_SCANCODE_KP_COMMA = 133,
    SDL_SCANCODE_KP_EQUALSAS400 = 134,

    SDL_SCANCODE_INTERNATIONAL1 = 135, 
    SDL_SCANCODE_INTERNATIONAL2 = 136,
    SDL_SCANCODE_INTERNATIONAL3 = 137, 
    SDL_SCANCODE_INTERNATIONAL4 = 138,
    SDL_SCANCODE_INTERNATIONAL5 = 139,
    SDL_SCANCODE_INTERNATIONAL6 = 140,
    SDL_SCANCODE_INTERNATIONAL7 = 141,
    SDL_SCANCODE_INTERNATIONAL8 = 142,
    SDL_SCANCODE_INTERNATIONAL9 = 143,
    SDL_SCANCODE_LANG1 = 144, 
    SDL_SCANCODE_LANG2 = 145, 
    SDL_SCANCODE_LANG3 = 146, 
    SDL_SCANCODE_LANG4 = 147, 
    SDL_SCANCODE_LANG5 = 148, 
    SDL_SCANCODE_LANG6 = 149, 
    SDL_SCANCODE_LANG7 = 150, 
    SDL_SCANCODE_LANG8 = 151, 
    SDL_SCANCODE_LANG9 = 152, 

    SDL_SCANCODE_ALTERASE = 153, 
    SDL_SCANCODE_SYSREQ = 154,
    SDL_SCANCODE_CANCEL = 155,
    SDL_SCANCODE_CLEAR = 156,
    SDL_SCANCODE_PRIOR = 157,
    SDL_SCANCODE_RETURN2 = 158,
    SDL_SCANCODE_SEPARATOR = 159,
    SDL_SCANCODE_OUT = 160,
    SDL_SCANCODE_OPER = 161,
    SDL_SCANCODE_CLEARAGAIN = 162,
    SDL_SCANCODE_CRSEL = 163,
    SDL_SCANCODE_EXSEL = 164,

    SDL_SCANCODE_KP_00 = 176,
    SDL_SCANCODE_KP_000 = 177,
    SDL_SCANCODE_THOUSANDSSEPARATOR = 178,
    SDL_SCANCODE_DECIMALSEPARATOR = 179,
    SDL_SCANCODE_CURRENCYUNIT = 180,
    SDL_SCANCODE_CURRENCYSUBUNIT = 181,
    SDL_SCANCODE_KP_LEFTPAREN = 182,
    SDL_SCANCODE_KP_RIGHTPAREN = 183,
    SDL_SCANCODE_KP_LEFTBRACE = 184,
    SDL_SCANCODE_KP_RIGHTBRACE = 185,
    SDL_SCANCODE_KP_TAB = 186,
    SDL_SCANCODE_KP_BACKSPACE = 187,
    SDL_SCANCODE_KP_A = 188,
    SDL_SCANCODE_KP_B = 189,
    SDL_SCANCODE_KP_C = 190,
    SDL_SCANCODE_KP_D = 191,
    SDL_SCANCODE_KP_E = 192,
    SDL_SCANCODE_KP_F = 193,
    SDL_SCANCODE_KP_XOR = 194,
    SDL_SCANCODE_KP_POWER = 195,
    SDL_SCANCODE_KP_PERCENT = 196,
    SDL_SCANCODE_KP_LESS = 197,
    SDL_SCANCODE_KP_GREATER = 198,
    SDL_SCANCODE_KP_AMPERSAND = 199,
    SDL_SCANCODE_KP_DBLAMPERSAND = 200,
    SDL_SCANCODE_KP_VERTICALBAR = 201,
    SDL_SCANCODE_KP_DBLVERTICALBAR = 202,
    SDL_SCANCODE_KP_COLON = 203,
    SDL_SCANCODE_KP_HASH = 204,
    SDL_SCANCODE_KP_SPACE = 205,
    SDL_SCANCODE_KP_AT = 206,
    SDL_SCANCODE_KP_EXCLAM = 207,
    SDL_SCANCODE_KP_MEMSTORE = 208,
    SDL_SCANCODE_KP_MEMRECALL = 209,
    SDL_SCANCODE_KP_MEMCLEAR = 210,
    SDL_SCANCODE_KP_MEMADD = 211,
    SDL_SCANCODE_KP_MEMSUBTRACT = 212,
    SDL_SCANCODE_KP_MEMMULTIPLY = 213,
    SDL_SCANCODE_KP_MEMDIVIDE = 214,
    SDL_SCANCODE_KP_PLUSMINUS = 215,
    SDL_SCANCODE_KP_CLEAR = 216,
    SDL_SCANCODE_KP_CLEARENTRY = 217,
    SDL_SCANCODE_KP_BINARY = 218,
    SDL_SCANCODE_KP_OCTAL = 219,
    SDL_SCANCODE_KP_DECIMAL = 220,
    SDL_SCANCODE_KP_HEXADECIMAL = 221,

    SDL_SCANCODE_LCTRL = 224,
    SDL_SCANCODE_LSHIFT = 225,
    SDL_SCANCODE_LALT = 226, 
    SDL_SCANCODE_LGUI = 227, 
    SDL_SCANCODE_RCTRL = 228,
    SDL_SCANCODE_RSHIFT = 229,
    SDL_SCANCODE_RALT = 230, 
    SDL_SCANCODE_RGUI = 231, 

    SDL_SCANCODE_MODE = 257,    

    

    
    

    SDL_SCANCODE_AUDIONEXT = 258,
    SDL_SCANCODE_AUDIOPREV = 259,
    SDL_SCANCODE_AUDIOSTOP = 260,
    SDL_SCANCODE_AUDIOPLAY = 261,
    SDL_SCANCODE_AUDIOMUTE = 262,
    SDL_SCANCODE_MEDIASELECT = 263,
    SDL_SCANCODE_WWW = 264,
    SDL_SCANCODE_MAIL = 265,
    SDL_SCANCODE_CALCULATOR = 266,
    SDL_SCANCODE_COMPUTER = 267,
    SDL_SCANCODE_AC_SEARCH = 268,
    SDL_SCANCODE_AC_HOME = 269,
    SDL_SCANCODE_AC_BACK = 270,
    SDL_SCANCODE_AC_FORWARD = 271,
    SDL_SCANCODE_AC_STOP = 272,
    SDL_SCANCODE_AC_REFRESH = 273,
    SDL_SCANCODE_AC_BOOKMARKS = 274,

    

    
    

    SDL_SCANCODE_BRIGHTNESSDOWN = 275,
    SDL_SCANCODE_BRIGHTNESSUP = 276,
    SDL_SCANCODE_DISPLAYSWITCH = 277, 
    SDL_SCANCODE_KBDILLUMTOGGLE = 278,
    SDL_SCANCODE_KBDILLUMDOWN = 279,
    SDL_SCANCODE_KBDILLUMUP = 280,
    SDL_SCANCODE_EJECT = 281,
    SDL_SCANCODE_SLEEP = 282,

    SDL_SCANCODE_APP1 = 283,
    SDL_SCANCODE_APP2 = 284,

    

    
    

    SDL_SCANCODE_AUDIOREWIND = 285,
    SDL_SCANCODE_AUDIOFASTFORWARD = 286,

    

    

    SDL_NUM_SCANCODES = 512 
} SDL_Scancode;

#endif 

