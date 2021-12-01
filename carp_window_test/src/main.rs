

fn run() -> Result<(), String>
{
    let mut carp_window = carp_window::CarpWindow::init(640, 480, "New Title!")?;
    render_gl::init_gl(&carp_window.window_state, &|s | carp_window.load_fn(s))?;

    let vert_shader_txt = std::ffi::CString::new(
        "#version 330 core
        layout(location = 0) in vec3 vertexPosition_modelspace;
        void main()
        {
            gl_Position.xyz = vertexPosition_modelspace;
            gl_Position.w = 1.0;
        }").unwrap();

    let frag_shader_txt = std::ffi::CString::new(
        "#version 330 core
        out vec3 color;

        void main()
        {
            color = vec3(1,0,0);
        }").unwrap();

    let shader_program;
    {
        let vert_shader = render_gl::Shader::from_vert_source(&vert_shader_txt, &"triangle.vert".to_string())?;
        let frag_shader = render_gl::Shader::from_frag_source(&frag_shader_txt, &"triangle.frag".to_string())?;
        shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader])?;
    }

    shader_program.set_used();


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
    
    carp_window.enable_vsync(false)?;
    
    while !carp_window.window_state.quit
    {
        carp_window.update();
        render_gl::update(&mut carp_window.window_state);
        unsafe 
        {
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

    return Ok(());
}



fn main()
{
    match run()
    {
        Ok(_) => {},
        Err(err) => println!("Error running: {}", &err)
    };
}