use std::fs::File;
use std::io::Read;

use regex::Regex;

fn get_type_as_rust_type(s: &str) -> &str
{
    let result: &str = match s 
    {
        "int" => "i32",
        "Uint32" => "u32",
        "Uint8" => "u8",
        "char" => "c_char",
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
        "Uint32" => "u32",
        "Uint8" => "u8",
        "char" => "c_char",
        "float" => "f32",
        "double" => "f64",
        "void" => "()",
        _ => s
    };
    return result;
}


const FILE_NAMES: &'static [&'static str] = &
[
    "include/SDL.h",

    "include/SDL_video.h",


    "include/SDL_events.h",
    "include/SDL_mouse.h",
    "include/SDL_main.h",

    "include/SDL_keyboard.h",
    "include/SDL_keycode.h",

];

fn get_type_or_mutable_type(f: fn(s: &str) -> &str, type_str: &str, ptr_str: &str, ptr_mutable: &str) -> String
{
    let mut return_str = String::new();
    for _ in 0..ptr_str.len()
    {
        return_str.push_str("*");
        return_str.push_str(ptr_mutable);
        return_str.push_str(" ");
    }
    return_str.push_str(f(&type_str));
    return return_str;
}

fn main()
{

    let mut out_classes = String::new();

    //let re = Regex::new(r"(?s)extern DECLSPEC (.+?) SDLCALL (.+?)\((.+?)\);").unwrap();

    let re = Regex::new(r"(?s)extern\s+DECLSPEC\s+(?P<fb_return_const>const)?\s*(?P<fn_ret_type>.+?)\s*(?P<fn_return_ptr>\**)\s*SDLCALL (?P<fn_name>.+?)\((?P<fn_params>.+?)\);").unwrap();
    let re2 = Regex::new(r"(?s)\s*(?P<const_param_type>const)?\s*(?P<param_type>\w+)\s+(?P<param_type_ptr>\**)\s*(?P<param_name>[\w_]+)").unwrap();
    //let re = Regex::new(r"(extern.+\);)").unwrap();

    let path = "../../cpp/SDL/";
    out_classes.push_str("extern \"C\"\r\n{\r\n");
    for file_name in FILE_NAMES
    {
        let mut s = path.to_string();
        s.push_str(*file_name);
        let file_string = read_file_to_string(&s[..]).unwrap();
        out_classes.push_str("\t// File: ");
        out_classes.push_str(*file_name);
        out_classes.push_str("\r\n");
        for caps in re.captures_iter(&file_string[..])
        {
            println!("Found:{}\r\nfn:{}, ptr:{} return:{}, func params:{}", &caps[0], &caps["fn_name"], &caps["fn_return_ptr"], &caps["fn_ret_type"], &caps["fn_params"]);
            let mut names: Vec<String> = Vec::new();
            let mut types: Vec<String> = Vec::new();
            let mut new_string = String::new();

            for fn_param in caps["fn_params"].split(",")
            {
                println!("   Found param:{}", &fn_param);
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

            new_string.push_str("fn ");
            new_string.push_str(&caps["fn_name"].to_string());
            new_string.push_str("(");
            for i in 0..names.len()
            {
                new_string.push_str(&names[i]);
                new_string.push_str(": ");
                new_string.push_str(&types[i]);
                if i + 1< names.len()
                {
                    new_string.push_str(", ");
                }
            }
            new_string.push_str(") -> ");

            
            let mut ptr_return_mutable = "mut";
            if caps.name("fb_return_const") != None
            {
                ptr_return_mutable = "const";
            }
            new_string.push_str(&get_type_or_mutable_type(get_function_type_as_rust_type, &caps["fn_ret_type"], &caps["fn_return_ptr"], ptr_return_mutable)[..]);

            
            new_string.push_str(";");
            out_classes.push_str("\t");
            out_classes.push_str(&new_string[..]);
            out_classes.push_str("\r\n");
            //println!("{}", &new_string[..]);
        }
        out_classes.push_str("\r\n");
    }
    out_classes.push_str("}");
    std::fs::write("carp_sdl_parser/classes.rs", out_classes).expect("Unable to write file");
}

fn read_file_to_string(filename: &str) -> Result<String, std::io::Error>
{
    let mut file = File::open(filename)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    return Ok(file_content);
}
