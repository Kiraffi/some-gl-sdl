#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::{io::Read};



mod vk_all;



//
//type void = std::os::raw::c_void;
//type uint32_t = u32;
//type uint16_t = u16;
//type uint8_t = u8;
//
//type int32_t = i32;
//type int16_t = i16;
//type int8_t = i8;



fn get_type_as_rust_type(s: &str) -> &str
{
    let result: &str = match s 
    {
        "size_t" => "usize",
        "uint64_t" => "u64",
        "uint32_t" => "u32",
        "uint16_t" => "u16",
        "uint8_t" =>   "u8",
        "int64_t" =>  "i64",
        "int32_t" =>  "i32",
        "int16_t" =>  "i16",
        "int8_t" =>    "i8",
        "char" =>  "c_char",
        "float" =>    "f32",
        "double" =>   "f64",
        "void" =>  "c_void",
        _ => s// panic!("unknown type: {}", s)
    };
    return result;
}



fn read_file_to_string(filename: &str) -> Result<String, std::io::Error>
{
    let mut file = std::fs::File::open(filename)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    return Ok(file_content);
}

#[allow(dead_code)]
fn print_attributes(elem: &xmltree::Element)
{
    for attr in &elem.attributes
    {
        println!("name: {}, value: {}", attr.0, attr.1);
    }
}

// The most awkward lambda, having to pass arrays for looking what attributes it contains, and setting possible filter value,
// and returning array of the type.
fn node_children<T>(elem: &xmltree::Element, filter_name: &str, attr_contains: &Vec<(&str, &str)>, f: fn(&xmltree::Element) -> Vec<T>) -> Vec<T>
{
    let mut out_vec: Vec<T> = Vec::new();
    for child in &elem.children
    {
        'check_filter:
        for &child2 in &child.as_element()
        {
            
            if filter_name.len() == 0 || child2.name == filter_name
            {
                for attr_contain in attr_contains
                {
                    if !child2.attributes.contains_key(attr_contain.0) || 
                        (attr_contain.1.len() > 0 && child2.attributes[attr_contain.0] != attr_contain.1.to_string())
                    {
                        continue 'check_filter;
                    }
                }
                out_vec.extend(f(child2));
            }
        }
    }
    return out_vec;
}


// The most awkward lambda, having to pass arrays for looking what attributes it contains, and setting possible filter value,
// and returning array of the type.
fn node_children2<U>(elem: &xmltree::Element, u: &mut U, filter_name: &str, attr_contains: &Vec<(&str, &str)>, f: fn(&xmltree::Element, &mut U))
{
    for child in &elem.children
    {
        'check_filter:
        for &child2 in &child.as_element()
        {
            
            if filter_name.len() == 0 || child2.name == filter_name
            {
                for attr_contain in attr_contains
                {
                    if !child2.attributes.contains_key(attr_contain.0) || 
                        (attr_contain.1.len() > 0 && child2.attributes[attr_contain.0] != attr_contain.1.to_string())
                    {
                        continue 'check_filter;
                    }
                }
                f(child2, u);
            }
        }
    }
}




struct StructType
{
    struct_name: String,
    s_type_name: String,
    param_names: Vec<String>,
    type_names: Vec<String>,
}

impl StructType
{
    pub fn new() -> Self
    {
        Self{ struct_name: String::new(), s_type_name: String::new(), param_names: Vec::new(), type_names: Vec::new() }
    }
}

fn parse_type_name(elem: &xmltree::Element) -> (String, String)
{
    let mut name_str = String::new();
    let mut type_str = String::new();
    
    node_children2(&elem, &mut name_str, "name",&Vec::new(),  |child, name_str|
    {
        name_str.push_str( &match child.get_text()
        {
            Some(v) => v.to_string(),
            None => "".to_string()
        });        
    });
    node_children2(&elem, &mut type_str, "type",&Vec::new(),  |child, type_str|
    {
        type_str.push_str(&match child.get_text()
        {
            Some(v) => v.to_string(),
            None => "".to_string()
        });
    });
    let mut consts = 0;
    let mut ptrs = 0;
    // Counting consts and ptrs.
    for child in &elem.children
    {
        for &child2 in &child.as_text()
        {
            for i in 0..child2.len()
            {
                if i < child2.len()
                {
                    if i + 5 < child2.len() && child2[i..i + 5].eq("const") 
                    { 
                        consts = consts + 1;
                    }
                }

                if child2[i..i + 1].eq("*") 
                { 
                    ptrs = ptrs + 1;
                }
            }   
        }
    }

    let mut const_mut_ptr_string = String::new();
    let c = core::cmp::max(consts, ptrs);
    for _ in 0..c
    {
        if consts > 0
        {
            const_mut_ptr_string.push_str("* const ");
        }
        else 
        {
            const_mut_ptr_string.push_str("* mut ");
        }
    }
    const_mut_ptr_string.push_str(get_type_as_rust_type(&type_str));
    
    return (name_str, const_mut_ptr_string);
}


fn parse_vk_structs(root: &xmltree::Element, enum_types: &mut Vec<EnumType>) -> String
{
    let mut string_out = String::new();
    let mut struct_vec: Vec<StructType> = Vec::new();
    node_children2(&root, &mut struct_vec, "types", &Vec::new(), |child, struct_vec|
    {
        node_children2(&child, struct_vec, "type",&vec![("category", "struct"), ("name", "")],  |child2, struct_vec|
        {
            /*
            if child2.attributes.contains_key("structextends")
            {
                println!("Struct name: {}, extends: {}", child2.attributes["name"], child2.attributes["structextends"]);
                return;
            }
*/
            let check_list = ["EXT", "KHR", "NV", "NVX", "NN", "AMD", "AMDX", "INTEL", "FB", 
                "FUCHSIA", "QCOM", "HUAWEI", "VALVE", "GGP", "ANDROID", "GOOGLE", "MKV", "QNX",
                "MVK"];

            for extension in check_list
            {
                if child2.attributes["name"].ends_with(extension)
                {
                    return;
                }
            }
            //println!("Normal struct: {}", child2.attributes["name"]);

            let mut new_struct = StructType::new();
            new_struct.struct_name = child2.attributes["name"].clone();
            node_children2(&child2, &mut new_struct, "member",&Vec::new(),  |child3, new_struct|
            {
                if child3.attributes.contains_key("values")
                {
                    new_struct.s_type_name = child3.attributes["values"].to_string();
                }


                let ans = parse_type_name(child3);

                new_struct.param_names.push(ans.0.clone());
                new_struct.type_names.push(ans.1.clone());
                return;
            });
            
            struct_vec.push(new_struct);
            return;
        })
    });

    let mut hset: std::collections::HashSet<String> = std::collections::HashSet::new();

    // Adding names, and changing Flags to FlagBits to match enum name, maybe should do this other way around.
    // And name the enum as Flags like 
    for struct_type in &mut struct_vec
    {
        for i in 0..struct_type.type_names.len()
        {
            if struct_type.type_names[i].find("Flags") != None
            {
                struct_type.type_names[i] = struct_type.type_names[i].replace("Flags", "FlagBits");
                hset.insert(struct_type.type_names[i].clone());
            }
            

        }
        for i in 0..struct_type.type_names.len()
        {
            if struct_type.param_names[i].eq("type")
            {
                struct_type.param_names[i] = "type_type".to_string();
            }
        }
    }

    for value in &hset
    {
        let mut last_space = 0;
        for i in 0..value.len()
        {
            if value[i..i+1].eq(" ")
            {
                last_space = i + 1;
            }
        }
        let value_str = &value[last_space..];
        let mut found = false;
        for enum_type in &*enum_types // ???????????????????????????wtf....
        {
            if enum_type.enum_name.eq(value_str)
            {
                found = true;
                break;
            }
        }
    
        if !found
        {
            enum_types.push(EnumType{enum_name: value.to_string(), bit_width: "32".to_string(), param_type_names: Vec::new()});
        }
    }

    for struct_type in &struct_vec
    {
        string_out.push_str(&format!("#[repr(C)]\r\n#[derive(Copy, Clone)]\r\npub struct {}\r\n{{\r\n", struct_type.struct_name));
        assert!(struct_type.param_names.len() == struct_type.type_names.len());
        for i in 0..struct_type.param_names.len()
        {
            string_out.push_str(&format!("\t{}: {},\r\n", struct_type.param_names[i], struct_type.type_names[i]));
        }
        let mut s_type_str = String::new();
        if struct_type.s_type_name.len() > 0
        {
            s_type_str.push_str(&format!("s.sType = VkStructureType::{};\r\n", struct_type.s_type_name));
        }
        string_out.push_str("}\r\n");

        string_out.push_str(&format!(
r"impl {}
{{
    fn new() -> Self
    {{
        let mut s: Self = unsafe {{ mem::zeroed() }};
        {}
        return s;
    }}
}}

", struct_type.struct_name, s_type_str));       
    }
    
    return string_out;
}

struct EnumType
{
    enum_name: String,
    bit_width: String,
    param_type_names: Vec<String>,
}


fn parse_vk_enums(root: &xmltree::Element) -> Vec<EnumType>
{
    let mut enums: Vec<EnumType> = Vec::new();

    node_children2(&root, &mut enums, "enums", &vec![("type", ""), ("name", "")], |child, enums|
    {
        if !(child.attributes["type"] == "bitmask" || child.attributes["type"] == "enum")
        {
            return;
        }

        let mut bit_width = "32"; 
        if child.attributes.contains_key("bitwidth") 
        { 
            bit_width = &child.attributes["bitwidth"];
        }
        
        
        let mut enum_type = EnumType{enum_name: child.attributes["name"].to_string(), bit_width: bit_width.to_string(), param_type_names: Vec::new()};
        //s_vec.push(format!("#[repr(i{})]\r\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\r\npub enum {}\r\n{{\r\n", bit_width, child.attributes["name"]));
        node_children2(&child, &mut enum_type.param_type_names, "enum",&vec![("name", "")],  |child2, params|
        {
            if child2.attributes.contains_key("bitpos")
            {
                let bitpos:u64 = child2.attributes["bitpos"].parse().unwrap();
                let bitvalue:u64 = 1 << bitpos;
                params.push(format!("\t{} = {},\r\n", child2.attributes["name"], bitvalue));
                return;
            }
            else if child2.attributes.contains_key("value")
            {
                params.push(format!("\t{} = {},\r\n", child2.attributes["name"], child2.attributes["value"]));
                return;
            }
            params.push("".to_string());
            return;            
        });
       
        enums.push(enum_type);
    });


    return enums;
}





fn parse_type_types(root: &xmltree::Element) -> &str
{
    let mut s_vec: Vec<(String, String)> = Vec::new();
    s_vec.extend(node_children(&root, "types", &Vec::new(), |child|
    {
        node_children(&child, "type",&vec![("category", "")],  |child2|
        {

            let mut strings: Vec<(String, String)> = Vec::new();
            //println!("{}: {};", child2.attributes["name"], child2.attributes["category"]);
            if child2.attributes.contains_key("name")
            {
                strings.push((child2.attributes["name"].clone(), child2.attributes["category"].clone()));
            }
            else 
            {
                strings.extend(node_children(&child2, "name", &Vec::new(),  |child3|
                {
                    let mut txt = String::new();

                    for child4 in &child3.children
                    {

                        for child5 in &child4.as_text()
                        {
                            txt.push_str(&child5);
                        }
                    }

                    vec![(txt.clone(), "unknown".to_string())]
                }));
            }

            for s in &mut strings
            {
                s.1 = child2.attributes["category"].clone();
            }
            //vec!(child2.attributes["category"].clone())
            return strings;
        })
    }));

    return "";

}

fn parse_vk_enum_extensions_helper(child: &xmltree::Element, enum_types: &mut Vec<EnumType>)
{
    
    node_children2(&child, enum_types,"", &Vec::new(), |child2, enum_types|
    {

/*
        print!("{}", child2.name);
        for attr in &child2.attributes
        {
            print!(", {} = {}", attr.0, attr.1);
        }
        println!("");
*/
        node_children2(&child2, enum_types,"", &Vec::new(), |child3, enum_types|
        {
            /*
            print!("\t{}", child3.name);
            for attr in &child3.attributes
            {
                print!(", {} = {}", attr.0, attr.1);
            }
            println!("");
*/
            if child3.name == "enum"
            {
                let mut find_string = child3.attributes["name"].to_string();
                find_string.push_str(" ");
    
                if child3.attributes.contains_key("extends") && child3.attributes.contains_key("offset") &&
                 child3.attributes.contains_key("extnumber") && child3.attributes.contains_key("name")
                {
                    for s in enum_types
                    {
                        if s.enum_name == child3.attributes["extends"]
                        {
                            for t in &s.param_type_names
                            {
                                if t.find(&find_string) != None
                                {
                                    return;
                                }
                            }
                            let mut v = 1_000_000_000u64;
                            let mut ext_num: u64 = child3.attributes["extnumber"].parse::<u64>().unwrap();
                            if ext_num > 1
                            { 
                                ext_num = ext_num - 1;
                            }
                            v = v + child3.attributes["offset"].parse::<u64>().unwrap();
                            v = v + ext_num * 1000;

                            s.param_type_names.push(format!("\t{} = {},\r\n", child3.attributes["name"], v));
                            return;
                        }
                    }
                }
                else if child3.attributes.contains_key("extends") && child3.attributes.contains_key("value") &&
                    child3.attributes.contains_key("name")
               {
                   for s in enum_types
                   {
                       if s.enum_name == child3.attributes["extends"]
                       {
                            for t in &s.param_type_names
                            {
                                if t.find(&find_string) != None
                                {
                                    return;
                                }
                            }
                            
                           let v = child3.attributes["value"].parse::<u64>().unwrap();
                           s.param_type_names.push(format!("\t{} = {},\r\n", child3.attributes["name"], v));
                           return;
                       }
                   }
               }
            }
        })
    });
}


fn parse_vk_enum_extensions(root: &xmltree::Element, enum_types: &mut Vec<EnumType>)
{
    //let t = enum_types;
    node_children2(&root,  enum_types, "", &Vec::new(), |child, enum_types |
    //node_children2(&root,  enum_types, "feature", &vec![("api", "")], |child, enum_types |
    {

        if child.name == "feature"
        {
            parse_vk_enum_extensions_helper(child, enum_types);            
        }
        else if child.name == "extensions"
        {
            node_children2(&child,  enum_types, "", &Vec::new(), |child2, enum_types |
            {
                parse_vk_enum_extensions_helper(child2, enum_types);
            });
        }
        //println!("api: {}, namme: {}, number: {}, comment: {}", child.attributes["api"], child.attributes["name"], child.attributes["number"], child.attributes["comment"]);
        
    });

}

fn parse_handles(root: &xmltree::Element) -> String
{
    //let t = enum_types;
    let mut string = String::new();

    node_children2(&root, &mut string, "types", &Vec::new(), |child, string|
    {
        node_children2(&child, string, "type", &vec![("category", "handle")],  |child2, string|
        {
            node_children2(&child2, string, "name", &Vec::new(), |child3, string|
            {
                string.push_str(&format!("type {} = *mut c_void;\r\n", child3.get_text().unwrap()));
            });
        });
    });
    string.push_str("\r\n\r\n");
    return string;
}



fn main() 
{

    let vk_xml = match read_file_to_string("carp_vk_parser/vk.xml") 
    {
        Ok(v) => v,
        Err(_) => { println!("Failed to load vk.xml"); return; }
    };
    let root = xmltree::Element::parse((&vk_xml).as_bytes()).unwrap();

    //parse_vk_structs2(&root);
    let mut vk_enums = parse_vk_enums(&root);
    parse_vk_enum_extensions(&root, &mut vk_enums);
    let vk_structs = parse_vk_structs(&root, &mut vk_enums);

    let handles_str = parse_handles(&root);

    let mut vk_enum_str = String::new();
    for enum_type in vk_enums
    {
        if enum_type.param_type_names.len() == 0
        {
            vk_enum_str.push_str(&format!("type {} = u{};\r\n\r\n", enum_type.enum_name, enum_type.bit_width));
            continue;
        }
        vk_enum_str.push_str(&format!("#[repr(i{})]\r\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\r\npub enum {}\r\n{{\r\n", enum_type.bit_width, enum_type.enum_name));
        for enum_type_param_type_names in enum_type.param_type_names
        {
            vk_enum_str.push_str(&format!("{}", enum_type_param_type_names));
        }
        vk_enum_str.push_str("}\r\n\r\n");
    }
    


    parse_type_types(&root);

    let mut vk_all = 
r"use std::mem;
use std::os::raw::*;

type VkSampleMask = u32;
type VkBool32 = u32;
type VkFlags = u32;
type VkFlags64 = u64;
type VkDeviceSize = u64;
type VkDeviceAddress = u64;

fn vk_make_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32
{
    return (variant << 29) | (major << 22) | (minor << 12) | patch; 
}     

".to_string();
    vk_all.push_str(&handles_str);
    vk_all.push_str(&vk_enum_str);
    vk_all.push_str(&vk_structs);
    
    std::fs::write("carp_vk_parser/vk_handles.rs", &handles_str).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_enums.rs", &vk_enum_str).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_structs.rs", &vk_structs).expect("Unable to write file");
    std::fs::write("carp_vk_parser/src/vk_all.rs", &vk_all).expect("Unable to write file");
}











