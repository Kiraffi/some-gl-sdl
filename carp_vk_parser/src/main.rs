#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]


use std::{io::Read, mem};




type void = std::os::raw::c_void;
type uint32_t = u32;
type uint16_t = u16;
type uint8_t = u8;

type int32_t = i32;
type int16_t = i16;
type int8_t = i8;


type char = std::os::raw::c_char;






















































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




fn parse_vk_structs(root: &xmltree::Element) -> String
{
    let mut string_out = String::new();
    let mut s_vec: Vec<StructType> = Vec::new();
    s_vec.extend(node_children(&root, "types", &Vec::new(), |child|
    {
        node_children(&child, "type",&vec![("category", "struct"), ("name", "")],  |child2|
        {
            let mut new_struct = StructType::new();
            new_struct.struct_name = child2.attributes["name"].clone();
            
            let s_vec: Vec<StructType> = node_children(&child2, "member",&Vec::new(),  |child3|
            {
                let mut new_struct = StructType::new();
                if child3.attributes.contains_key("values")
                {
                    new_struct.s_type_name = child3.attributes["values"].to_string();
                }
                let mut s_vec: Vec<StructType> = Vec::new();
                s_vec.extend(node_children(&child3, "name",&Vec::new(),  |child4|
                {
                    let mut new_struct = StructType::new();
                    let s= match child4.get_text()
                    {
                        Some(v) => v.to_string(),
                        None => "".to_string()
                    };
                    new_struct.param_names.push(s);
                    return vec![new_struct];
                }));


                s_vec.extend(node_children(&child3, "type",&Vec::new(),  |child4|
                {
                    let mut new_struct = StructType::new();
                    let s= match child4.get_text()
                    {
                        Some(v) => v.to_string(),
                        None => "".to_string()
                    };
                    new_struct.type_names.push(s);
                    return vec![new_struct];
                }));
                new_struct.param_names.push(s_vec[0].param_names[0].clone());
                new_struct.type_names.push(s_vec[1].type_names[0].clone());
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

    for strings in &s_vec
    {
        s.push_str(&strings[..]);
    }
    return s;
}

#[repr(C)]
struct SetSt
{
    poo: u64,
    foo: i32,
    faa: u32,
}

fn get_set() -> SetSt
{
    let poo: SetSt = unsafe { mem::zeroed() };
    return poo;
}

fn main() 
{
    let poo = get_set();
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

    std::fs::write("carp_vk_parser/vk_enums.rs", &vk_enums).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_structs.rs", &vk_structs).expect("Unable to write file");
}











