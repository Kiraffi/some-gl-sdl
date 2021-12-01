



fn main()
{
    let mut carp_window = carp_window::App::new();
    
    if carp_window.create_window(640, 480, "New Title!")
    {
        if gl::load_with(&|s | carp_window.load_fn(s))
        {
            let version;
            match unsafe
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

            let vertex_shader_id = unsafe { gl::glCreateShader(gl::GL_VERTEX_SHADER) };
            let  fragment_shader_id = unsafe { gl::glCreateShader(gl::GL_FRAGMENT_SHADER) };

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

            unsafe 
            {
                gl::glShaderSource(vertex_shader_id, 1, &vert_shader, std::ptr::null());
                gl::glCompileShader(vertex_shader_id);
                let mut success = 0;
                gl::glGetShaderiv(vertex_shader_id, gl::GL_COMPILE_STATUS, &mut success);

                gl::glShaderSource(fragment_shader_id, 1, &frag_shader, std::ptr::null());
                gl::glCompileShader(fragment_shader_id);
                let mut success2 = 0;
                gl::glGetShaderiv(vertex_shader_id, gl::GL_COMPILE_STATUS, &mut success2);


                println!("compile1: {}, compile2: {}", success, success2);
            }
            // Link the program
            println!("Linking program");
            let program_id = unsafe { gl::glCreateProgram() };

            unsafe 
            {
                gl::glAttachShader(program_id, vertex_shader_id);
                gl::glAttachShader(program_id, fragment_shader_id);
                gl::glLinkProgram(program_id);

                gl::glDetachShader(program_id, vertex_shader_id);
                gl::glDetachShader(program_id, fragment_shader_id);

                gl::glDeleteShader(vertex_shader_id);
                gl::glDeleteShader(fragment_shader_id);

                gl::glUseProgram(program_id);
            }

            let mut vertex_array_id: gl::GLuint = 0;
            unsafe 
            {
                gl::glGenVertexArrays(1, &mut vertex_array_id);
                gl::glBindVertexArray(vertex_array_id);
            }
            // An array of 3 vectors which represents 3 vertices
            let g_vertex_buffer_data: [gl::GLfloat; 9] = [
                -1.0f32, -1.0f32, 0.0f32,
                1.0f32, -1.0f32, 0.0f32,
                0.0f32,  1.0f32, 0.0f32,
            ];

            let mut vertexbuffer: gl::GLuint  = 0;
            unsafe 
            {
                gl::glGenBuffers(1, &mut vertexbuffer);
                gl::glBindBuffer(gl::GL_ARRAY_BUFFER, vertexbuffer);
                gl::glBufferData(gl::GL_ARRAY_BUFFER, 4 * 9,
                    g_vertex_buffer_data.as_ptr() as _, gl::GL_STATIC_DRAW);
            }

            carp_window.set_window_title("New title2!");
            let mut now = std::time::Instant::now();
            
            carp_window.set_vsync(false);
            
            while carp_window.running
            {
                carp_window.update();
                unsafe 
                {
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
                }
                carp_window.swap_buffers();


                
                let now2 = std::time::Instant::now();
                let _nanos1 = now2.duration_since(now).as_nanos();
                //println!("Duration {:?}ms", _nanos1 as f32 / 1000_000.0f32);
                now = now2;
                let set_timer = carp_window.set_timer_resolution(1);

                if set_timer == 0
                {
                    std::thread::sleep(std::time::Duration::from_millis(5));
                    let end_timer = carp_window.unset_timer_resolution(1);
                    if end_timer != 0
                    {
                        println!("unset failed: {}", end_timer);
                    }
                }
                else
                {
                    println!("Set timer failed: {}", set_timer);
                }

                let now3 = std::time::Instant::now();
                let _nanos = now3.duration_since(now).as_nanos();
                //println!("Duration sleep: {:?}ms", _nanos as f32 / 1000_000.0f32);
            }
            println!("running over");
            
            unsafe { gl::glDeleteProgram(program_id); }
        }
    }
}