



fn main()
{
    let mut carp_window = carp_window::CarpWindow::new();
    unsafe
    {
        if carp_window.create_window(640, 480, b"New Title!\0".as_ptr() as _)
        {
            if gl::load_with(&|s | carp_window.load_fn(s))
            {
                let version;
                match
                {
                    let data = std::ffi::CStr::from_ptr(gl::glGetString(gl::GL_VERSION) as *const _)
                        .to_bytes()
                        .to_vec();
                    String::from_utf8(data)
                }
                {
                    Ok(v) =>
                    {
                        version = v;
                    }
                    Err(e) =>
                    {
                        println!("Error: {}", e);
                        return;
                    }
                }
                println!("OpenGL version {}", version);

                // Create the shaders
                let vertex_shader_id = gl::glCreateShader(gl::GL_VERTEX_SHADER);
                let  fragment_shader_id = gl::glCreateShader(gl::GL_FRAGMENT_SHADER);

                let vert_shader =
                    b"#version 330 core
                    layout(location = 0) in vec3 vertexPosition_modelspace;
                    void main()
                    {
                        gl_Position.xyz = vertexPosition_modelspace;
                        gl_Position.w = 1.0;
                    }\0".as_ptr() as *const i8;

                let frag_shader =
                    b"#version 330 core
                    out vec3 color;

                    void main()
                    {
                        color = vec3(1,0,0);
                    }\0".as_ptr() as *const i8;


                gl::glShaderSource(vertex_shader_id, 1, &vert_shader, std::ptr::null());
                gl::glCompileShader(vertex_shader_id);
                let mut success = 0;
                gl::glGetShaderiv(vertex_shader_id, gl::GL_COMPILE_STATUS, &mut success);

                gl::glShaderSource(fragment_shader_id, 1, &frag_shader, std::ptr::null());
                gl::glCompileShader(fragment_shader_id);
                let mut success2 = 0;
                gl::glGetShaderiv(vertex_shader_id, gl::GL_COMPILE_STATUS, &mut success2);


                println!("compile1: {}, compile2: {}", success, success2);
                // Link the program
                println!("Linking program");
                let program_id = gl::glCreateProgram();
                gl::glAttachShader(program_id, vertex_shader_id);
                gl::glAttachShader(program_id, fragment_shader_id);
                gl::glLinkProgram(program_id);

                gl::glDetachShader(program_id, vertex_shader_id);
                gl::glDetachShader(program_id, fragment_shader_id);

                gl::glDeleteShader(vertex_shader_id);
                gl::glDeleteShader(fragment_shader_id);



                // Use our shader
                gl::glUseProgram(program_id);
                // Draw triangle...


                let mut vertex_array_id: gl::GLuint = 0;
                gl::glGenVertexArrays(1, &mut vertex_array_id);
                gl::glBindVertexArray(vertex_array_id);

                // An array of 3 vectors which represents 3 vertices
                let g_vertex_buffer_data: [gl::GLfloat; 9] = [
                    -1.0f32, -1.0f32, 0.0f32,
                    1.0f32, -1.0f32, 0.0f32,
                    0.0f32,  1.0f32, 0.0f32,
                ];

                // This will identify our vertex buffer
                let mut vertexbuffer: gl::GLuint  = 0;
                // Generate 1 buffer, put the resulting identifier in vertexbuffer
                gl::glGenBuffers(1, &mut vertexbuffer);
                // The following commands will talk about our 'vertexbuffer' buffer
                gl::glBindBuffer(gl::GL_ARRAY_BUFFER, vertexbuffer);
                // Give our vertices to OpenGL.
                gl::glBufferData(gl::GL_ARRAY_BUFFER, 4 * 9,
                    g_vertex_buffer_data.as_ptr() as _, gl::GL_STATIC_DRAW);

                // Use our shader
                gl::glUseProgram(program_id);
                // Draw triangle...

                println!("vao: {}", vertex_array_id);
                println!("vertex_b: {}", vertexbuffer);

                carp_window.set_window_title(b"New title2!\0".as_ptr() as _);
                while carp_window.running
                {
                    carp_window.update();

                    if carp_window.resized
                    {
                        carp_window.resized = false;
                        gl::glViewport(0, 0, carp_window.width, carp_window.height);
                    }

                    gl::glUseProgram(program_id);

                    gl::glClearColor(0.2f32, 1.0f32, 1.0f32, 1.0f32);
                    gl::glClear(gl::GL_COLOR_BUFFER_BIT | gl::GL_DEPTH_BUFFER_BIT);

                    gl::glEnableVertexAttribArray(0);
                    gl::glBindBuffer(gl::GL_ARRAY_BUFFER, vertexbuffer);
                    gl::glVertexAttribPointer(0, 3, gl::GL_FLOAT, gl::GL_FALSE, 0, 0 as _);

                    gl::glDrawArrays(gl::GL_TRIANGLES, 0, 3); // Starting from vertex 0; 3 vertices total -> 1 triangle
                    gl::glDisableVertexAttribArray(0);

                    carp_window.swap_buffers();
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                println!("running over");

                gl::glDeleteProgram(program_id);
            }
        }
    }
}