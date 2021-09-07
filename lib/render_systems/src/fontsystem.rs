extern crate render_gl;
use std::ffi::CString;


const MAX_LETTERS: usize = 65536usize;

pub struct LetterData
{
	_pos_x: f32,
	_pos_y: f32,
	_size_x: f32,
	_size_y: f32,

	_uv_x: f32,
	_uv_y: f32,

    _col: u32,

    _tmp: f32,
}

pub struct FontSystem 
{
    letter_buffer: render_gl::ShaderBuffer,


    letter_datas: Vec<LetterData>,
    tex_handle: gl::types::GLuint,
	shader_textured_program: render_gl::Program,
    vao: gl::types::GLuint,
    canvas_width: f32,
    canvas_height: f32
}


impl FontSystem
{
    
    pub fn init() -> Result<FontSystem, String>
    {
        let shader_textured_program;
        {
            let textured_vert_shader = render_gl::Shader::from_vert_source(
                &CString::new(include_str!("textured_triangle.vert")).unwrap(), &"textured_triangle.vert".to_string()
            )?;
    
            let textured_frag_shader = render_gl::Shader::from_frag_source(
                &CString::new(include_str!("textured_triangle.frag")).unwrap(), &"textured_triangle.frag".to_string()
            )?;
    
            shader_textured_program = render_gl::Program::from_shaders(
                &[textured_vert_shader, textured_frag_shader]
            ).unwrap();
        }
    
    
        let tex: Vec<u8> = include_bytes!("../../../new_font.dat").to_vec();

        let mut tex_handle: gl::types::GLuint = 0;
        {
            let mut font_tex: Vec<u8> = Vec::new();

            for y in 0..12
            {
                for l in 0 .. (128-32)
                {
                    let val = tex[(11 - y) + l * 12];
                    for x in 0..8
                    {
                        let v = ((val >> x) & 1) * 255;
                        font_tex.push(v);
                        font_tex.push(v);
                        font_tex.push(v);
                        font_tex.push(v);
                    }
                }
            }

            unsafe
            {
                let texture_width = 8*(128-32);
                let texture_height = 12;

                gl::GenTextures(1, &mut tex_handle);
                gl::BindTexture(gl::TEXTURE_2D, tex_handle);
                gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA8, texture_width, texture_height);
                gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, texture_width, texture_height, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);

                //gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_BASE_LEVEL, 0);
                //gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAX_LEVEL, 0);
                //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, texture_width, texture_height, 0, gl::BGRA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);

                //gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                //gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8, texture_width, texture_height, 0, gl::RGBA, gl::UNSIGNED_BYTE, font_tex.as_ptr() as *const gl::types::GLvoid);
                // 0 mips, 0 baselevel

                //gl::GenerateMipmap(gl::TEXTURE_2D);

            }
        }
        

        
        // Fill board for shader
        let mut letter_datas: Vec<LetterData> = Vec::new();
        for _x in 0..MAX_LETTERS
        {
            letter_datas.push(LetterData{_pos_x: 0.0f32, _pos_y: 0.0f32, _size_x: 0.0f32, _size_y: 0.0f32,
                _uv_x: 0.032, _uv_y: 0.0f32, _col: 0, _tmp: 0.0f32});
        }


        let letter_buffer: render_gl::ShaderBuffer = render_gl::ShaderBuffer::new_with_data(
            gl::SHADER_STORAGE_BUFFER,
            letter_datas.len() * std::mem::size_of::<LetterData>(),
            letter_datas.as_ptr() as *const gl::types::GLvoid
        );
        letter_datas.clear();
    
        let mut vao: gl::types::GLuint = 0;
        unsafe
        {
            gl::GenVertexArrays(1, &mut vao);
        }

       Ok(Self{ letter_buffer, letter_datas, tex_handle, shader_textured_program, vao, canvas_width: 0.0f32, canvas_height: 0.0f32 })
    }


    pub fn draw_string(&mut self, s: &String, pos_x: f32, pos_y: f32, letter_size_x: f32, letter_size_y: f32, col: u32)
    {
        let mut px = pos_x + letter_size_y * 0.5f32;
        let py = self.canvas_height - pos_y - letter_size_y * 0.5f32;
        for x in 0..s.len()
        {
            let l: u8 = s.as_bytes()[x] - 32u8;
            let tmp_pos_x: f32 = l as f32;
            
            if self.letter_datas.len() < MAX_LETTERS
            {
                self.letter_datas.push(LetterData{_pos_x: px, _pos_y: py,  _size_x: letter_size_x, _size_y: letter_size_y,
                    _uv_x: tmp_pos_x, _uv_y: 0.5f32, _col: col, _tmp: 0.0f32});
            }
            px += letter_size_x + 1.0f32;
        }
    }
    

    pub fn update(&mut self, canvas_width: f32, canvas_height: f32)
    {
        self.canvas_width = canvas_width;
        self.canvas_height = canvas_height;
        
        if self.letter_datas.len() > 0
        {
            self.letter_buffer.write_data(0, self.letter_datas.len() * std::mem::size_of::<LetterData>(),
                self.letter_datas.as_ptr() as *const gl::types::GLvoid);
        }
    }

    pub fn draw(&mut self)
    {
        if self.letter_datas.len() > 0
        {
            unsafe
    		{
                gl::BindVertexArray(self.vao);

                self.shader_textured_program.set_used();
                self.letter_buffer.bind(0);
                
                gl::Uniform2f(0, self.canvas_width as f32, self.canvas_height as f32);

                gl::BindTexture(gl::TEXTURE_2D, self.tex_handle);
                gl::Enable(gl::BLEND);
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);


                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // starting index in the enabled arrays
                    6 * self.letter_datas.len() as i32// number of indices to be rendered
                );
            }
        }
        self.letter_datas.clear();
    }
}