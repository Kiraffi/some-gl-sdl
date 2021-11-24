
#[cfg(windows)]
pub mod win32_win;
#[cfg(windows)]
use win32_win::*;

use std::os::raw::*;


#[cfg(target_os = "linux")]
pub mod linux_win;
#[cfg(target_os = "linux")]
use linux_win::*;

type GLenum = c_uint;
type GLboolean = c_uchar;
type GLbitfield = c_uint;
type GLvoid = c_void;
type GLbyte = c_schar;
type GLshort = c_short;
type GLubyte = c_uchar;
type GLushort = c_ushort;
type GLuint = c_uint;
type GLsizei = c_int;
type GLfloat = f32;
type GLclampf = f32;
type GLdouble = f64;
type GLclampd = f64;

const GL_TRIANGLES: u32 = 4;
const GL_COLOR_BUFFER_BIT: u32 = 16384;

extern "C"
{
    fn glClearColor(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf);
    fn glBegin(mode: GLenum);
    fn glEnd();
    fn glVertex2d(x: GLdouble, y: GLdouble);
    fn glVertex2f(x: GLfloat, y: GLfloat);
    fn glVertex2i(x: GLint, y: GLint);
    fn glVertex2s(x: GLshort, y: GLshort);
    fn glVertex3d(x: GLdouble, y: GLdouble, z: GLdouble);
    fn glVertex3f(x: GLfloat, y: GLfloat, z: GLfloat);
    fn glColor3f(red: GLfloat, green: GLfloat, blue: GLfloat);
    fn glClear(mask: GLbitfield);
    fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}

fn main()
{
    #[cfg(windows)]
    win_main();

    #[cfg(target_os = "linux")]
    {
        let mut carp_window = CarpWindow::new();
        unsafe
        {
            if carp_window.create_window()
            {
                while carp_window.running
                {
                    carp_window.update();

                    // Present frame
                    glClearColor(0.3f32, 0.7f32, 0.3f32, 1.0f32);
                            // OpenGL Rendering
                    glClear(GL_COLOR_BUFFER_BIT);

                    glBegin(GL_TRIANGLES);
                        glColor3f(  1.0f32,  0.0f32, 0.0f32);
                        glVertex3f( 0.0f32, -1.0f32, 0.0f32);
                        glColor3f(  0.0f32,  1.0f32, 0.0f32);
                        glVertex3f(-1.0f32,  1.0f32, 0.0f32);
                        glColor3f(  0.0f32,  0.0f32, 1.0f32);
                        glVertex3f( 1.0f32,  1.0f32, 0.0f32);
                    glEnd();

                    carp_window.swap_buffers();
                }
            }
        }
        linux_main();
    }
}