#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::{io::Read, mem};


//mod vk_all;



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
        "uint32_t" => "u32",
        "uint16_t" => "u16",
        "uint8_t" =>   "u8",
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
    let name_str;
    let mut type_str = String::new();
    
    let mut s_vec: Vec<String> = Vec::new();
    s_vec.extend(node_children(&elem, "name",&Vec::new(),  |child|
    {
        let s= match child.get_text()
        {
            Some(v) => v.to_string(),
            None => "".to_string()
        };
        return vec![s];
    }));
    s_vec.extend(node_children(&elem, "type",&Vec::new(),  |child|
    {
        let s= match child.get_text()
        {
            Some(v) => v.to_string(),
            None => "".to_string()
        };
        return vec![s];
    }));
    let mut consts = 0;
    let mut ptrs = 0;
    
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
    let c = core::cmp::max(consts, ptrs);
    for _ in 0..c
    {
        //if i < ptrs
        {
            type_str.push_str("* ");
        }
        //if i < consts
        {
            type_str.push_str("const ");                    
        }
    }

    name_str = s_vec[0].clone();
    type_str.push_str(&get_type_as_rust_type(&s_vec[1])[0..]);
    return (name_str, type_str);
}


fn parse_vk_structs(root: &xmltree::Element) -> String
{
    let mut string_out = String::new();
    let mut s_vec: Vec<StructType> = Vec::new();
    s_vec.extend(node_children(&root, "types", &Vec::new(), |child|
    {
        node_children(&child, "type",&vec![("category", "struct"), ("name", "")],  |child2|
        {
            if child2.attributes.contains_key("structextends")
            {
                return Vec::new();
            }
            let mut new_struct = StructType::new();
            new_struct.struct_name = child2.attributes["name"].clone();
            let s_vec: Vec<StructType> = node_children(&child2, "member",&Vec::new(),  |child3|
            {
                let mut new_struct = StructType::new();
                if child3.attributes.contains_key("values")
                {
                    new_struct.s_type_name = child3.attributes["values"].to_string();
                }


                let ans = parse_type_name(child3);

                new_struct.param_names.push(ans.0.clone());
                new_struct.type_names.push(ans.1.clone());
                return vec![new_struct];
            });
            
            for struct_type in &s_vec
            {
                new_struct.param_names.push(struct_type.param_names[0].clone());
                new_struct.type_names.push(struct_type.type_names[0].clone());
                if struct_type.s_type_name.len() > 0
                {
                    new_struct.s_type_name = struct_type.s_type_name.clone();
                }          
            }
            return vec![new_struct];
        })
    }));

    
    for struct_type in &s_vec
    {
        string_out.push_str(&format!("#[repr(C)]\r\n#[derive(Copy, Clone)]\r\npub struct {}\r\n{{\r\n", struct_type.struct_name)[..]);
        assert!(struct_type.param_names.len() == struct_type.type_names.len());
        for i in 0..struct_type.param_names.len()
        {
            string_out.push_str(&format!("\t{}: {},\r\n", struct_type.param_names[i], struct_type.type_names[i])[..]);
        }
        let mut s_type_str = String::new();
        if struct_type.s_type_name.len() > 0
        {
            s_type_str.push_str(&format!("s.sType = VkStructureType::{};\r\n", struct_type.s_type_name)[..]);
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

", struct_type.struct_name, s_type_str)[..] );       
    }
    
    return string_out;
}



fn parse_vk_enums(root: &xmltree::Element) -> String
{
    let mut s = String::new();
    let mut s_vec: Vec<String> = Vec::new();
    
    s_vec.extend(node_children(&root, "enums", &vec![("type", "enum"), ("name", "")], |child|
    {
        let mut s_vec: Vec<String> = Vec::new();
        s_vec.push(format!("#[repr(i32)]\r\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\r\npub enum {}\r\n{{\r\n", child.attributes["name"]));
        s_vec.extend(node_children(&child, "enum",&vec![("value", ""), ("name", "")],  |child2|
        {
            return vec![format!("\t{} = {},\r\n", child2.attributes["name"], child2.attributes["value"])];
        }));
        s_vec.push(("}\r\n\r\n").to_string());
        return s_vec;
    }));


    s_vec.extend(node_children(&root, "enums", &vec![("type", "bitmask"), ("name", "")], |child|
    {
        let mut s_vec: Vec<String> = Vec::new();
        s_vec.push(format!("#[repr(i32)]\r\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\r\npub enum {}\r\n{{\r\n", child.attributes["name"]));
        s_vec.extend(node_children(&child, "enum",&vec![("name", "")],  |child2|
        {
            if child2.attributes.contains_key("bitpos")
            {
                let bitpos:u64 = child2.attributes["bitpos"].parse().unwrap();
                let bitvalue:u64 = 1 << bitpos;
                println!("bitpos: {}, value {}, str: {}", bitpos, bitvalue, child2.attributes["bitpos"]);
                return vec![format!("\t{} = {},\r\n", child2.attributes["name"], bitvalue)];
            }
            else if child2.attributes.contains_key("value")
            {
                return vec![format!("\t{} = {},\r\n", child2.attributes["name"], child2.attributes["value"])];
            }
            return vec!["".to_string()];
            
        }));
        s_vec.push(("}\r\n\r\n").to_string());
        return s_vec;
    }));
    for strings in &s_vec
    {
        s.push_str(&strings[..]);
    }
    return s;
}





fn parse_type_types(root: &xmltree::Element) -> &str
{
    //let mut result = HashMap<String, String>::new();

    let mut string_out = String::new();
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
                            txt.push_str(&child5[..]);
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

    let mut s_vec_unique: Vec<String> = Vec::new();

    let mut s_vec2: Vec<(String, String)> = Vec::new();

/*
    for s in &s_vec
    {
        if !s_vec_unique.contains(&s.1)
        {
            println!("{}", &s.1);
            s_vec_unique.push(s.1.to_string());

        }
    }
    for s in &s_vec
    {
        let mut c = 0;
        for t in &s_vec
        {
            if s.0 == t.0
            {
                c = c + 1;
            }
        }
//        if c > 1
        {
            println!("{}: {} count: {}", &s.0, &s.1, c);
        }
    }
*/
    return "";

}




#[repr(C)]
struct SetSt
{
    poo: u64,
    foo: i32,
    faa: u32,
}

impl SetSt
{
    fn new() -> Self
    {
        let poo: Self = unsafe { mem::zeroed() };
        return poo;
    }
}

fn main() 
{
    let poo = SetSt::new();
    println!("{}, {}, {}", poo.poo, poo.foo, poo.faa);


    let vk_xml = match read_file_to_string("carp_vk_parser/vk.xml") 
    {
        Ok(v) => v,
        Err(_) => { println!("Failed to load vk.xml"); return; }
    };
    let root = xmltree::Element::parse((&vk_xml[..]).as_bytes()).unwrap();

    //parse_vk_structs2(&root);
    let vk_enums = parse_vk_enums(&root);
    let vk_structs = parse_vk_structs(&root);
    parse_type_types(&root);

    let mut vk_all = "use std::{io::Read, mem};\r\nuse std::os::raw::*;\r\ntype u32 = VkBool32;\r\n\r\n".to_string();
    vk_all.push_str(&vk_enums[..]);
    vk_all.push_str(&vk_structs[..]);
    
    std::fs::write("carp_vk_parser/vk_enums.rs", &vk_enums).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_structs.rs", &vk_structs).expect("Unable to write file");
    std::fs::write("carp_vk_parser/src/vk_all.rs", &vk_all).expect("Unable to write file");
}











