#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]


use std::io::Read;

use regex::Regex;

use std::os::raw::*;


//include!("../combined.rs");

fn get_type_as_rust_type(s: &str) -> &str
{
    let result: &str = match s 
    {
        "int" => "i32",
        "Sint32" => "i32",
        "Uint32" => "u32",
        "Uint8" => "u8",
        "Uint16" => "u16",
        "char" => "c_uchar",
        "float" => "f32",
        "double" => "f64",
        "void" => "c_void",
        _ => s// panic!("unknown type: {}", s)
    };
    return result;
}

fn get_function_type_as_rust_type(s: &str) -> &str
{
    let result: &str = match s 
    {
        "int" => "i32",
        "Sint32" => "i32",
        "Uint32" => "u32",
        "Uint16" => "u16",
        "Uint8" => "u8",
        "char" => "c_char",
        "float" => "f32",
        "double" => "f64",
        "void" => "()",
        _ => s
    };
    return result;
}
const FILE_PATH: &'static str = "../../cpp/SDL/";

const FILE_NAMES: &'static [&'static str] = &
[
    "include/SDL.h",

    "include/SDL_video.h",


    "include/SDL_events.h",
    "include/SDL_mouse.h",
    "include/SDL_main.h",

    "include/SDL_keyboard.h",
    "include/SDL_joystick.h",
    "include/SDL_scancode.h",
    "include/SDL_keycode.h",
    "include/SDL_error.h",

];

const TYPEDEF_NAMES: &'static [&'static str] = &
[
    "SDL_Keycode",
    "SDL_JoystickID",
    "SDL_GLContext",
];

const DEFINE_NAMES: &'static [&'static str] = &
[
    "SDL_INIT_VIDEO",
    "SDL_INIT_TIMER",
    "SDL_INIT_EVENTS",
];


const ENUM_NAMES: &'static [&'static str] = &
[
    "SDL_GLattr",
    "SDL_WindowFlags",
    "SDL_EventType",
    "SDL_Scancode",
];

const STRUCT_NAMES: &'static [&'static str] = &
[
    "SDL_Window",
    "SDL_KeyboardEvent",
    "SDL_Keysym",
];

const FUNC_NAMES: &'static [&'static str] = &
[
    "SDL_Init",
    "SDL_InitSubSystem",
    "SDL_QuitSubSystem",
    "SDL_WasInit",
    "SDL_Quit",

    "SDL_CreateWindow",
    "SDL_DestroyWindow",
    "SDL_GL_CreateContext",
    "SDL_GetError",
    "SDL_GL_SetAttribute",
    "SDL_GL_GetSwapInterval",
    "SDL_GL_SetSwapInterval",
    "SDL_PollEvent",
    "SDL_GL_SwapWindow",
    "SDL_PumpEvents",
    "SDL_GL_GetProcAddress",

    "SDL_GetError",
];

fn get_type_or_mutable_type(f: fn(s: &str) -> &str, type_str: &str, ptr_str: &str, ptr_mutable: &str) -> String
{
    let mut return_str = String::new();
    for _ in 0..ptr_str.len()
    {
        return_str.push_str(&format!("*{} ", ptr_mutable)[..]);
    }
    return_str.push_str(f(&type_str));
    return return_str;
}


fn main()
{
    let parsed = strip_comments();


    let mut write_str = String::new();

    for f in &parsed
    {
        write_str.push_str(&format!("//File:{}\r\n{}", &f[0], &f[1])[..]);
    }

    let structs = parse_structs(&parsed);
    let enums = parse_enums(&parsed);
    let funcs = parse_functions(&parsed);
    let defines = parse_defines(&parsed);
    let typedefs = parse_typedefs(&parsed);

    let mut all_types = String::new();
    all_types.push_str(&format!("use std::os::raw::*;\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n", &typedefs, &defines, &enums, &structs, &funcs)[..]);

    std::fs::write("carp_sdl_parser/structs.rs", &structs).expect("Unable to write file");
    std::fs::write("carp_sdl_parser/enums.rs", &enums).expect("Unable to write file");
    std::fs::write("carp_sdl_parser/funcs.rs", &funcs).expect("Unable to write file");
    std::fs::write("carp_sdl_parser/defines.rs", &defines).expect("Unable to write file");
    std::fs::write("carp_sdl_parser/typedefs.rs", &typedefs).expect("Unable to write file");

    std::fs::write("carp_sdl_parser/combined.rs", &all_types).expect("Unable to write file");

    std::fs::write("carp_sdl_parser/stripped_funcs.rs", &write_str).expect("Unable to write file");

}

fn parse_typedefs(files: &Vec<[String; 2]>) -> String
{
    let mut out_typedefs = String::new();
    out_typedefs.push_str("//Typedefs \r\n");

    let re = Regex::new(r"typedef\s+(?P<type>[\w_]+)\s*(?P<ptr_type>\*?)\s*(?P<name>[\w_\.]+);").unwrap();
   // let re_fn = Regex::new(r"#define (?P<define>[\w_]+)\((?P<params>.+?)\)\s+(?P<value>.+)").unwrap();

    for file_data in files
    {
        let file_string = &file_data[1];
        out_typedefs.push_str(&format!("//File: {}\r\n", &file_data[0])[..]);
        for caps in re.captures_iter(&file_string[..])
        {
            //println!("{}", &caps[0]);
            if !TYPEDEF_NAMES.contains(&&caps["name"])
            {
                continue;
            }
            let ptr_str = if caps["ptr_type"].len() > 0 { "*const " } else { "" };

            out_typedefs.push_str(&format!("pub type {} = {}{};\r\n", &caps["name"], ptr_str, get_type_as_rust_type(&caps["type"]))[..]);
        }
    }
    return out_typedefs;
}

fn parse_defines(files: &Vec<[String; 2]>) -> String
{
    let mut out_defines = String::new();
    out_defines.push_str("//Defines \r\n");

    let re = Regex::new(r"#define (?P<define>[\w_]+)\s+(?P<value>[\dxa-fA-F.*]*)").unwrap();
   // let re_fn = Regex::new(r"#define (?P<define>[\w_]+)\((?P<params>.+?)\)\s+(?P<value>.+)").unwrap();

    for file_data in files
    {
        let file_string = &file_data[1];
        out_defines.push_str(&format!("//File: {}\r\n", &file_data[0])[..]);
        for caps in re.captures_iter(&file_string[..])
        {
            if !DEFINE_NAMES.contains(&&caps["define"])
            {
                continue;
            }
            out_defines.push_str(&format!("pub const {}: u32 = {} as u32;\r\n", &caps["define"], &caps["value"])[..]);
        }
        /*
        for caps in re_fn.captures_iter(&file_string[..])
        {
            if !DEFINE_NAMES.contains(&&caps["define"])
            {
                continue;
            }
            out_defines.push_str(&format!("pub const fn {}({})) {{ {}; }}\r\n", &caps["define"], &caps["params"], &caps["value"])[..]);
        }
        */
    }
    return out_defines;
}

fn strip_comments() -> Vec<[String; 2]>
{
    let mut out_result: Vec<[String; 2]> = Vec::new();
    for file_name in FILE_NAMES
    {
        let mut s = FILE_PATH.to_string();
        s.push_str(*file_name);
        let file_string = read_file_to_string(&s[..]).unwrap();
        

        let mut parsed_file = String::new();

        let str_file = &file_string[0..];
        let str_len = str_file.len();

        let mut start = false;
        let mut last_copy_pos = 0usize;
        for current_pos in 0 .. str_len - 1
        {
            let cmp_str = &str_file[current_pos..current_pos + 2];            
            if !start && cmp_str.eq("/*")
            {
                if last_copy_pos + 1 < current_pos
                {
                    parsed_file.push_str(&file_string[last_copy_pos..current_pos])
                }
                start = true;
            }
            else if start && cmp_str.eq("*/") && last_copy_pos < current_pos
            {
                last_copy_pos = current_pos + 2;
                start = false;
            }
        }

        if last_copy_pos < last_copy_pos && !start
        {
            parsed_file.push_str(&file_string[last_copy_pos..])
        }
        out_result.push([(*file_name).to_string(), parsed_file] );
    }
    return out_result;
}

fn parse_enums(files: &Vec<[String; 2]>) -> String
{
    let mut out_enums = String::new();

    let re = Regex::new(r"(?s)typedef\s+enum\s*(\w)*\s*\{(?P<inside_enum>.*?\})\s*(?P<enum_name>\w*);").unwrap();
    //let re3 = Regex::new(r"(?s)\s*(?P<enum_in>.*?)(,\s+|\s+\})").unwrap();
    let re3 = Regex::new(r"(?s)\n\s*(?P<enum_in>SDL_(\w+|\w+\s*=\s*[\d\.xA-Fa-f]+))(,|\s+\})").unwrap();

    out_enums.push_str("//Enums \r\n");
    for file_data in files
    {
        let file_string = &file_data[1];
        out_enums.push_str(&format!("//File: {}\r\n", &file_data[0])[..]);
        for caps in re.captures_iter(&file_string[..])
        {
            
            //println!("Found:{}\r\nenum:{}", &caps[0], &caps["enum_name"]);
            if !ENUM_NAMES.contains(&&caps["enum_name"])
            {
                continue;
            }
            
            out_enums.push_str(&format!("#[repr(i32)]\r\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\r\npub enum {}\r\n{{\r\n", &caps["enum_name"])[..]);
            for sub_cap in re3.captures_iter(&caps["inside_enum"])
            {
                out_enums.push_str(&format!("\t{},\r\n", &sub_cap["enum_in"])[..]);    
            }
            out_enums.push_str("}\r\n\r\n");
            
        }
    }

    return out_enums;
}



fn parse_structs(files: &Vec<[String; 2]>) -> String
{
    let mut out_structs = String::new();

    let re = Regex::new(r"(?s)typedef\s+struct\s*(\w)*\s*(\{(?P<inside_struct>.*?)\})?\s*(?P<struct_name>\w*);").unwrap();
    let re2 = Regex::new(r"\s*(?P<param_type>.+?)\s*(?P<param_ptr>\**)?\s*(?P<param_name>\w+);").unwrap();

    out_structs.push_str("//Structs \r\n");

    // Special case? 64bytes?
    out_structs.push_str("#[repr(C)]\r\n#[derive(Copy, Clone)]\r\npub struct SDL_Event\r\n{\r\n    pub sdl_type: SDL_EventType,\r\n    pub sdl_timestamp: u32,\r\n    pub _padding: [u8; 56]\r\n}\r\n\r\n");

    for file_data in files
    {
        let file_string = &file_data[1];
        out_structs.push_str(&format!("//File: {}\r\n", &file_data[0])[..]);
        for caps in re.captures_iter(&file_string[..])
        {
            
            if !STRUCT_NAMES.contains(&&caps["struct_name"])
            {
                continue;
            }
            
            //println!("Found:{}\r\nstruct:{}", &caps[0], &caps["struct_name"]);
            out_structs.push_str(&format!("#[repr(C)]\r\n#[derive(Copy, Clone)]\r\npub struct {}\r\n{{\r\n", &caps["struct_name"])[..]);
            if caps.name("inside_struct") != None && caps["inside_struct"].len() > 0
            {
                for sub_cap in re2.captures_iter(&caps["inside_struct"])
                {
                    let param_name = if &sub_cap["param_name"] == "type" { "type_name" } else { &sub_cap["param_name"] };
                    let param_name2 = if param_name == "mod" { "mod_name" } else { param_name };
                    //println!("full: {} - name: {}: type: {}", &sub_cap[0], &sub_cap["param_name"], &sub_cap["param_type"]);
                    if sub_cap.name("param_ptr") != None && sub_cap["param_ptr"].len() > 0
                    {
                        out_structs.push_str(&format!("\tpub {}: {}mut {},\r\n", param_name2, &sub_cap["param_ptr"], get_type_as_rust_type(&sub_cap["param_type"]))[..]);
                    }
                    else 
                    {
                        out_structs.push_str(&format!("\tpub {}: {},\r\n", param_name2, get_type_as_rust_type(&sub_cap["param_type"]))[..]);
                    }
                }
            }
            else
            {
                out_structs.push_str(&format!("\tpub _something: [u8; 0]\r\n")[..]);
            }
            out_structs.push_str("}\r\n\r\n");
        }
    }

    return out_structs;
}

fn parse_functions(files: &Vec<[String; 2]>) -> String
{
    let mut out_funcs = String::new();

    //let re = Regex::new(r"(?s)extern DECLSPEC (.+?) SDLCALL (.+?)\((.+?)\);").unwrap();

    let re = Regex::new(r"(?s)extern\s+DECLSPEC\s+(?P<fb_return_const>const)?\s*(?P<fn_ret_type>.+?)\s*(?P<fn_return_ptr>\**)\s*SDLCALL (?P<fn_name>.+?)\((?P<fn_params>.+?)\);").unwrap();
    let re2 = Regex::new(r"(?s)\s*(?P<const_param_type>const)?\s*(?P<param_type>\w+)\s+(?P<param_type_ptr>\**)\s*(?P<param_name>[\w_]+)").unwrap();
    //let re = Regex::new(r"(extern.+\);)").unwrap();

    out_funcs.push_str("//Classes \r\nextern \"C\"\r\n{\r\n");
    for file_data in files
    {
        let file_string = &file_data[1];
        out_funcs.push_str(&format!("\t//File: {}\r\n", &file_data[0])[..]);
        for caps in re.captures_iter(&file_string[..])
        {
            //println!("Found:{}\r\nfn:{}, ptr:{} return:{}, func params:{}", &caps[0], &caps["fn_name"], &caps["fn_return_ptr"], &caps["fn_ret_type"], &caps["fn_params"]);
            let mut names: Vec<String> = Vec::new();
            let mut types: Vec<String> = Vec::new();
            let mut new_string = String::new();

           
            if !FUNC_NAMES.contains(&&caps["fn_name"])
            {
                continue;
            }

            for fn_param in caps["fn_params"].split(",")
            {
                //println!("   Found param:{}", &fn_param);
                for sub_cap in re2.captures_iter(&fn_param)
                {
                    //println!("   Found param:{}: type:{} ptr: {}, name:{}", &sub_cap[0], &sub_cap["param_type"], &sub_cap["param_type_ptr"], &sub_cap["param_name"]);
                    if sub_cap["param_name"].eq("type")
                    {
                        names.push("type_name".to_string());
                    }
                    else
                    {
                        names.push(sub_cap["param_name"].to_string());
                    }
                    let mut ptr_mutable_type = "mut";
                    if sub_cap.name("const_param_type") != None
                    {
                        ptr_mutable_type = "const";
                    }
                    types.push(get_type_or_mutable_type(get_type_as_rust_type, &sub_cap["param_type"], &sub_cap["param_type_ptr"], &ptr_mutable_type));
                }
            }

            out_funcs.push_str(&format!("\tpub fn {}(", &caps["fn_name"]));
            for i in 0..names.len()
            {
                out_funcs.push_str(&format!("{}: {}, ", &names[i], &types[i]));
            }
            new_string.push_str(") -> ");

            
            let mut ptr_return_mutable = "mut";
            if caps.name("fb_return_const") != None
            {
                ptr_return_mutable = "const";
            }
            new_string.push_str(&get_type_or_mutable_type(get_function_type_as_rust_type, &caps["fn_ret_type"], &caps["fn_return_ptr"], ptr_return_mutable)[..]);
            new_string.push_str(";");
            out_funcs.push_str(&format!("\t{}\r\n", new_string));
            //println!("{}", &new_string[..]);
        }
        out_funcs.push_str("\r\n");
    }
    out_funcs.push_str("}");
    return out_funcs;
}

fn read_file_to_string(filename: &str) -> Result<String, std::io::Error>
{
    let mut file = std::fs::File::open(filename)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    return Ok(file_content);
}
