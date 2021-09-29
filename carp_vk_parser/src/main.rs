#![allow(dead_code)]

use std::{clone, io::Read};
use carp_xml_parser::{XMLElement, get_text_from_array};

/*
use crate::vk_all::VkInternalAllocationType;

pub mod test_vk;
pub mod vk_all;
*/

fn parse_api_constant(st: &String) -> String
{
    // wrong way....
    let eq_len = st.len();
    if eq_len == 0
    {
        return String::new();
    }
    let mut s = st.clone(); 
    s = s.trim().to_string();
    s = s.replace("(", "");
    s = s.replace(")", "");
    s = s.replace("~", "!");
    s = s.replace(",", ";");

    if s.ends_with("F;")
    {
        s = s.replace("F;", "f32;");
        s = s.replace(" = ", ":f32 = ");
    }
    else if s.ends_with("U;")
    {
        s = s.replace("U;", "u32;");
        s = s.replace(" = ", ":u32 = ");

    }
    else if s.ends_with("ULL;")
    {
        s = s.replace("ULL;", "u64;");
        s = s.replace(" = ", ":u64 = ");

    }

    else
    {
        s = s.replace(" = ", ":usize = ");

    }

    return s;
}


fn parse_api_constant2(st: &String) -> String
{
    // wrong way....
    let eq_len = st.len();
    if eq_len == 0
    {
        return String::new();
    }
    let mut s = st.clone(); 
    s = s.trim().to_string();
    s = s.replace("(", "");
    s = s.replace(")", "");
    s = s.replace("~", "!");

    if s.ends_with("F")
    {
        s = s.replace("F", "f32");
    }
    else if s.ends_with("U")
    {
        s = s.replace("U", "u32");

    }
    else if s.ends_with("ULL")
    {
        s = s.replace("ULL", "u64");

    }

    return s;
}


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

#[derive(Debug)]
struct VulkanHandle
{
    handle_name: String,
    handle_type: String,
    parent: String,
    obj_enum: String,

}


#[derive(Debug)]
struct ReturnType
{
    return_type: String,
    ptrs: u32,
    ptr_mutable: bool,
}
impl ReturnType
{
    fn new(mutable: bool) -> Self
    {
        return Self {return_type: String::new(), ptrs: 0u32, ptr_mutable: mutable };
    }
}
//#[derive(Debug, Clone, PartialEq, Eq)]

#[derive(Debug)]
struct Aliases
{
    original_name: String,
    alias_name: String,
    category: String,
}

#[derive(Debug)]
struct BitMask
{
    name: String,
    flag_bits: u32,
}

#[derive(Debug)]
struct FuncPointer
{
    name: String,
    return_type: ReturnType,
    param_type_names: Vec<String>,
    param_types: Vec<ReturnType>,
}


#[derive(Debug)]
struct EnumType2
{
    enum_name: String,
    param_type_names: Vec<String>,
    param_type_values: Vec<String>,
    is_bitmask: bool,
}


#[derive(Debug)]
struct StructType
{
    struct_name: String,
    s_type_name: String,
    param_names: Vec<String>,
    type_names: Vec<String>,
}

#[derive(Debug)]
struct Define
{
    define_name: String,
    define_value: ReturnType,
}

#[derive(Debug)]
struct StructType2
{
    struct_name: String,
    s_type_value: String,
    struct_extends: String,
    param_names: Vec<String>,
    types: Vec<ReturnType>,
}

impl StructType
{
    pub fn new() -> Self
    {
        Self{ struct_name: String::new(), s_type_name: String::new(), param_names: Vec::new(), type_names: Vec::new() }
    }
}


#[derive(Debug)]
struct ConstantValue
{
    name: String,
    constant_type: String,
    constant_value: String,    
}

struct ParsedArrays
{
    aliases: Vec<Aliases>,
    bitmasks: Vec<BitMask>,
    handles: Vec<VulkanHandle>,
    enums: Vec<EnumType2>,
    func_ptrs: Vec<FuncPointer>,
    structs: Vec<StructType2>,
    defines: Vec<Define>,
    constants: Vec<ConstantValue>,
}

fn check_attributes(elem: &XMLElement, attribute_list: &[(&str, &str)]) -> bool
{
    for attr in &*attribute_list
    {
        let mut found = false;
        for elem_attr in &elem.attributes
        {
            if !elem_attr.0.eq(attr.0)
            {
                continue;
            }
            if attr.1.len() > 0 && !elem_attr.1.eq(attr.1)
            {
                return false;
            }
            found = true;
            break;
        }

        if !found
        {
            return false;
        }
    }

    return true;
}

fn get_attribute(elem: &XMLElement, attr: &'static str) -> String
{
    for attribute in &elem.attributes
    {
        if attribute.0.eq(attr)
        {
            return attribute.1.clone();
        }
    }

    return String::new();
}


fn parse_types(type_elemnt: &XMLElement, parsed_arrays: &mut ParsedArrays) -> Result<(), &'static str>
{
    let aliases = &mut parsed_arrays.aliases;
    let bitmasks = &mut parsed_arrays.bitmasks;
    let handles = &mut parsed_arrays.handles;
    let enums = &mut parsed_arrays.enums;
    let func_ptrs = &mut parsed_arrays.func_ptrs;
    let structs = &mut parsed_arrays.structs;
    let defines = &mut parsed_arrays.defines;

    for child in &type_elemnt.elements
    {
        if child.element_name.eq("type")
        {
            // Aliases
            if check_attributes(child, &[("category", ""), ("name", ""), ("alias", "")])
            {
                let mut alias = Aliases { alias_name: String::new(), category: String::new(), original_name: String::new() };
                for attr in &child.attributes
                {
                    let s: &str = &attr.0;
                    match s
                    {
                        "alias" => alias.original_name = attr.1.clone(),
                        "category" => alias.category = attr.1.clone(),
                        "name" => alias.alias_name = attr.1.clone(),
                        _ => ()
                    }
                }
                aliases.push(alias);
            }
            else if check_attributes(child, &[("category", "")])
            {
                let category = &get_attribute(child, "category")[..];
                
                match category
                {
                    "basetype" =>
                    {
                        let mut defi = Define { define_name: String::new(), define_value: ReturnType::new(true) };

                        for child2 in &child.elements
                        {
                            let s: &str = &child2.element_name;
                            match s
                            {
                                "type" => defi.define_value.return_type = get_type_as_rust_type(&child2.elements[0].element_text).to_string(),
                                "name" => defi.define_name = child2.elements[0].element_text.clone(),
                                _ => ()
                            }
                            if child2.element_text.len() > 0
                            {
                                if child2.element_text.contains("const")
                                {
                                    defi.define_value.ptr_mutable = false;
                                }

                                let s = child2.element_text.as_bytes();
                                for c in s
                                {
                                    if *c as char == '*'
                                    {
                                        defi.define_value.ptrs += 1;
                                    }
                                }
                            }
                        }

                        if defi.define_name.len() > 0 && defi.define_value.return_type.len() > 0
                        {
                            defines.push(defi);
                        }

                    },
                    "bitmask" => 
                    {
                        let mut bitmask = BitMask{name: String::new(), flag_bits: 32};
                        let mut founds = 0u32;
                        for child2 in &child.elements
                        {
                            if child2.element_name == "type"
                            {
                                founds += 1;
                                let s: &str = &child2.elements[0].element_text;
                                bitmask.flag_bits = match s
                                {
                                    "VkFlags" => 32,
                                    "VkFlags64" => 64,
                                    _ => return Err("Bitmask has unknown type, not VkFlags or VkFlags64")
                                };
                            }
                            else if child2.element_name == "name"
                            {
                                founds += 1;
                                bitmask.name = child2.elements[0].element_text.clone();
                            }
                        }
                        if founds != 2
                        {
                            return Err("Didn't find all the required types for bitmask, name and type.");
                        }
                        bitmasks.push(bitmask);
    
                    },
                    "handle" =>
                    {
                        let mut handle = VulkanHandle { handle_name: String::new(), handle_type: String::new(), obj_enum: String::new(), parent: String::new() };
                        let mut founds = 0u32;
                        let mut p = false;
                        for attr in &child.attributes
                        {
                            let s: &str = &attr.0;
                            match s
                            {
                                "parent" => { p = true; handle.parent = attr.1.clone(); },
                                "objtypeenum" => { founds += 1; handle.obj_enum = attr.1.clone(); },
                                _ => ()
                            };
                        }

                        for child2 in &child.elements
                        {
                            let s: &str = &child2.element_name;
                            match s 
                            {
                                "type" =>  { founds += 1; handle.handle_type = child2.elements[0].element_text.clone(); },
                                "name" => { founds += 1; handle.handle_name = child2.elements[0].element_text.clone(); },
                                _ => ()
                            }
                        }

                        if founds != 3 || !(p || handle.handle_name == "VkInstance")
                        {
                            return Err("Failed to parse handle!");
                        }
                        handles.push(handle);

                    },
                    "enum" => 
                    {
                        let mut e = EnumType2{ enum_name: String::new(), param_type_names: Vec::new(), param_type_values: Vec::new(), is_bitmask: false };
                        let mut founds = 0u32;
                        for attr in &child.attributes
                        {
                            let s: &str = &attr.0;
                            match s
                            {
                                "name" => { founds += 1; e.enum_name = attr.1.clone(); },
                                _ => ()
                            };
                        }

                        if founds != 1
                        {
                            return Err("Failed to parse enum types");
                        }

                        enums.push(e);
                    }
                    "funcpointer" =>
                    {
                        let mut f = FuncPointer { name: String::new(), return_type: ReturnType::new(false), param_type_names: Vec::new(), param_types: Vec::new() };
                        let mut return_found = false;
                        let mut name_found = false;

                        let mut param_type = ReturnType::new(true);

                        for child2 in &child.elements
                        {
                            //println!("text: {}", child2.element_text);
                            if !return_found && (child2.element_text.len() == 0 || !child2.element_text.starts_with("typedef "))
                            {
                                println!("found:{} len:{}, text:{}", return_found, child2.element_text.len(), child2.element_text);
                                return Err("Failed to parse return value of funcpointer");
                            }
                            else if !return_found
                            {
                                let constness = child2.element_text.find(" const ");
                                let mut start_value = 8usize;
                                if constness.is_some()
                                {
                                    start_value = constness.unwrap() + 7;
                                }
                                let s = child2.element_text[start_value..].as_bytes();
                                let mut start_char = 0usize;
                                while (s[start_char] as char).is_whitespace()
                                {
                                    start_char += 1;
                                }

                                let mut empty_char = start_char;

                                while !(s[empty_char] as char).is_whitespace() && s[empty_char] as char != '*'
                                {
                                    empty_char += 1;
                                }

                                let mut ptrs = 0u32;

                                for c in s
                                {
                                    if *c as char == '*'
                                    {
                                        ptrs += 1;
                                    }
                                }
                                
                                

                                f.return_type.return_type = get_type_as_rust_type(&carp_xml_parser::get_text_from_array(s, start_char, empty_char)?).to_string();
                                f.return_type.ptrs = ptrs;
                                f.return_type.ptr_mutable = constness.is_none();

                                return_found = true;
                            }
                            else if child2.element_name == "name"
                            {
                                f.name = child2.elements[0].element_text.clone();
                                name_found = true;
                            }
                            else if name_found && return_found
                            {
                                let s = child2.element_text[..].as_bytes();
                                if child2.element_text == ")(void);"
                                {

                                }
                                else if child2.element_text.contains(",")
                                {
                                    let d = child2.element_text.find(",").unwrap();
                                    let mut c = d - 1;
                                    while c > 0 && (s[c - 1].is_ascii_alphanumeric() || s[c - 1] as char == '_')
                                    {
                                        c -= 1;
                                    }

                                    f.param_type_names.push(get_text_from_array(s, c, d).unwrap());

                                    for i in 0..d
                                    {
                                        if s[i] as char == '*'
                                        {
                                            param_type.ptrs += 1;
                                        }                                        
                                    }
                                    f.param_types.push(param_type);

                                    param_type = ReturnType::new(true);
                                    
                                }
                                else if child2.element_text.contains(");")
                                {
                                    let d = child2.element_text.find(");").unwrap();
                                    let mut c = d - 1;
                                    while c > 0 && (s[c - 1].is_ascii_alphanumeric() || s[c - 1] as char == '_')
                                    {
                                        c -= 1;
                                    }

                                    f.param_type_names.push(get_text_from_array(s, c, d).unwrap());
                                    f.param_types.push(param_type);

                                    param_type = ReturnType::new(true);
                                }
                                else if child2.element_text.contains("const")
                                {
                                    param_type.ptr_mutable = false;
                                }
                                else if child2.element_name == "type"
                                {
                                    param_type.return_type = get_type_as_rust_type(&child2.elements[0].element_text).to_string();
                                }
                            }
                            
                        }


                        func_ptrs.push(f);

                    },
                    "struct" =>
                    {
                        let mut s = StructType2 { struct_name: String::new(), s_type_value: String::new(), struct_extends: String::new(),
                            param_names: Vec::new(), types: Vec::new() };
                        let mut name_found = false;
                        for attr in &child.attributes
                        {
                            if attr.0 == "name"
                            {
                                s.struct_name = attr.1.clone();
                                name_found = true;
                            }
                            else if attr.0 == "structextends"
                            {
                                s.struct_extends = attr.1.clone();
                            }
                        }
                        for child2 in &child.elements
                        {
                            if child2.element_name == "comment"
                            {
                                continue;
                            }
                            if child2.element_name != "member"
                            {
                                println!("{}", child2.element_name);
                                return Err("Found non-member/non-comment type in struct");
                            }

                           
                            
                            let mut return_type = ReturnType::new(true);
                            let mut return_name = String::new();
                            for attr in &child2.attributes
                            {
                                if attr.0 == "values"
                                {
                                    s.s_type_value = attr.1.clone();
                                }
                            }
                            for child3 in &child2.elements
                            {
                                if child3.element_text.len() > 0
                                {
                                    if child3.element_text.contains("const")
                                    {
                                        return_type.ptr_mutable = false;
                                    }
                                    let s = child3.element_text[..].as_bytes();
                                    for i in 0..s.len()
                                    {
                                        if s[i] as char == '*'
                                        {
                                            return_type.ptrs += 1;
                                        }
                                    }
                                }
                                else if child3.element_name == "type"
                                {
                                    return_type.return_type = get_type_as_rust_type(&child3.elements[0].element_text).to_string();
                                }
                                else if child3.element_name == "name"
                                {
                                    return_name = child3.elements[0].element_text.clone();
                                }
                            }
                            s.param_names.push(return_name);
                            s.types.push(return_type);
                            
                        }
                        if !name_found
                        {
                            return Err("No name for struct found!");
                        }
                        structs.push(s);
                    },
                    _ => () //println!("cat: {}", category) // include, define basetype, union
    
                }
            }
        }
    }

    //dbg!(defines);
    return Ok(());
}


fn parse_vk(elem: &XMLElement) -> Result<ParsedArrays, &'static str>
{
    let mut parsed_arrays = ParsedArrays{ 
        aliases: Vec::new(), bitmasks: Vec::new(), defines: Vec::new(), enums: Vec::new(),
        func_ptrs: Vec::new(), handles: Vec::new(), structs: Vec::new(), constants: Vec::new(),
    };
    for child in &elem.elements
    {
        if child.element_name != "registry"
        {
            continue;
        }

        for child2 in &child.elements
        {
            if child2.element_name == "types"
            {
                parse_types(child2, &mut parsed_arrays)?;
            }
            else if child2.element_name == "enums"
            {
                let mut enum_name = String::new();
                let mut enum_bitmask = -1;
                let mut bit_width = 32;

                let mut enums: Vec<(String, String)> = Vec::new();

                for attr in &child2.attributes
                {
                    let s: &str = &attr.0;
                    match s
                    {
                        "name" => enum_name = attr.1.clone(),
                        "type" => enum_bitmask = if attr.1 == "bitmask" { 1 } else { 0 },
                        "bitwidth" => bit_width = attr.1.parse().unwrap(),
                        _ => ()
                    };
                    
                }
                if enum_name == "API Constants"
                {
                    for c in &child2.elements
                    {
                        let mut constant = ConstantValue{ name: String::new(), constant_type: String::new(), constant_value: String::new()  };
                        let mut alias = String::new();
                        for attr in &c.attributes
                        {
                            let s: &str = &attr.0;
                            match s 
                            {
                                "type" => constant.constant_type = attr.1.clone(),
                                "value" => constant.constant_value = attr.1.clone(),
                                "name" => constant.name = attr.1.clone(),
                                "alias" => alias = attr.1.clone(),
                                _ => ()
                            };
                        }

                        constant.constant_type = get_type_as_rust_type(&constant.constant_type).to_string();
                        constant.constant_value = parse_api_constant2(&constant.constant_value);

                        if alias.len() != 0
                        {
                            for alias_constant in &parsed_arrays.constants
                            {
                                if alias_constant.name == alias
                                {
                                    constant.constant_value = alias_constant.constant_value.clone();
                                    constant.constant_type = alias_constant.constant_type.clone();
                                    break;
                                }
                            }
                        }




                        if constant.name.len() == 0 || constant.constant_value.len() == 0 || constant.constant_type.len() == 0
                        {
                            println!("{}, {}, {}", constant.name, constant.constant_value, constant.constant_type);
                            return Err("Failed to parse apiconstants!");
                        }
                        parsed_arrays.constants.push(constant);
                    }
                    // continue with the upper for loop
                    continue;
                }
                else if enum_name.len() == 0 || enum_bitmask == -1
                {
                    println!("name; {}, bit: {}", enum_name, enum_bitmask);
                    return Err("Failed to parse enums!");
                }


                'enum_loop:
                for c in &child2.elements
                {
                    if c.element_name != "enum"
                    {
                        continue;
                    }
                    let mut bit_value = String::new();
                    let mut enum_enum_name = String::new();
                    for attr in &c.attributes
                    {
                        let s: &str = &attr.0;
                        match s
                        {
                            "value" => bit_value = attr.1.clone(),
                            "bitpos" => bit_value = (1i128 << attr.1.parse::<i128>().unwrap()).to_string(),
                            "name" => enum_enum_name = attr.1.clone(),
                            "alias" => continue 'enum_loop,
                            _ => ()
                        }                        
                    };
                    enums.push((enum_enum_name, bit_value));

                }
                let mut found = false;
                for ind in &mut parsed_arrays.enums
                {
                    if ind.enum_name == enum_name
                    {
                        found = true;
                        break;
                    }
                }
                if !found 
                {
                    parsed_arrays.enums.push( EnumType2{ enum_name: enum_name.clone(), is_bitmask: false,  param_type_names: Vec::new(), param_type_values: Vec::new() } );
                }

                for ind in &mut parsed_arrays.enums
                {
                    if ind.enum_name == enum_name
                    {
                        for e in &enums
                        {
                            ind.is_bitmask = if enum_bitmask == 1 { true } else { false };
                            ind.param_type_names.push(e.0.clone());
                            ind.param_type_values.push(e.1.clone());
                        }
                    }
                }
            }
        }
    }
    //dbg!(&parsed_arrays.enums);

    return Ok(parsed_arrays);
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





fn count_sub_strings_from_str(count_from: &str, what_to_count: &str) -> u32
{
    let mut count = 0u32;
    let count_len = what_to_count.len();
    let count_from_len = count_from.len();
    
    if count_from_len < count_len
    {
        return 0u32;
    }
    let end_ind = count_from_len - count_len; 

    for i in 0..end_ind
    {
        if count_from[i..i + count_len].eq(what_to_count)
        {
            count = count + 1;
        }
    } 

    return count;
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

    if name_str.eq("type")
    {
        name_str = "name_type".to_string();
    }
    type_str = get_type_as_rust_type(&type_str).to_string();


    let mut text = String::new();
    for child in &elem.children
    {
        
        for &child2 in &child.as_text()
        {
            text.push_str(child2);
        }
        for &child2 in &child.as_element()
        {
            if child2.name == "name" || child2.name == "type" || child2.name == "comment"
            {
                continue;
            }

            for child3 in &child2.children
            {
                for &child4 in &child3.as_text()
                {
                    text.push_str(child4);
                }
            }

        }

    }

    // Counting consts and ptrs.
    let consts = count_sub_strings_from_str(&text, "const");
    let ptrs = count_sub_strings_from_str(&text, "*");


    let mut const_mut_ptr_string = String::new();
    let c = core::cmp::max(consts, ptrs);
    let found = text.contains("[") && text.contains("]");
    if ptrs > 0
    {
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
    }
    else if consts > 0
    {
        const_mut_ptr_string.push_str("const ");

    }
    else
    {
        if found 
        {
            text = text.replace("[", "");
            text = text.replace("]", "");
            type_str = format!("[{}; {}]", &type_str, &text);
        }
    }    

    const_mut_ptr_string.push_str(&type_str);

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
            let check_list = ["EXT", "NV", "NVX", "NN", "AMD", "AMDX", "INTEL", "FB", "KHR", 
                "FUCHSIA", "QCOM", "HUAWEI", "VALVE", "GGP", "ANDROID", "GOOGLE", "MKV", "QNX",
                "MVK"];

            for extension in check_list
            {
                if child2.attributes["name"].ends_with(extension)
                {
                    return;
                }
            }

            if child2.attributes["name"].eq("VkAllocationCallbacks")
            {
                return;
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
                struct_type.param_names[i] = "name_type".to_string();
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
        for enum_type in &*enum_types // mutable dereference-borrow to get reference not move
        {
            if enum_type.enum_name.eq(value_str)
            {
                found = true;
                break;
            }
        }
    
        if !found
        {
            enum_types.push(EnumType{enum_name: value.to_string(), bit_width: "32".to_string(), param_type_names: Vec::new(), is_enum: true});
        }
    }

    for struct_type in &struct_vec
    {
        string_out.push_str(&format!("#[repr(C)]\r\n#[derive(Copy, Clone)]\r\npub struct {}\r\n{{\r\n", struct_type.struct_name));
        assert!(struct_type.param_names.len() == struct_type.type_names.len());
        for i in 0..struct_type.param_names.len()
        {
            string_out.push_str(&format!("\tpub {}: {},\r\n", struct_type.param_names[i], struct_type.type_names[i]));
        }

        let mut s_type_mut_string = "let s";
        let mut s_type_str = String::new();
        if struct_type.s_type_name.len() > 0
        {
            s_type_mut_string = "let mut s";
            s_type_str.push_str(&format!("s.sType = VkStructureType::{};\r\n", struct_type.s_type_name));
        }
        string_out.push_str("}\r\n");

        string_out.push_str(&format!(
r"impl {}
{{
    pub fn new() -> Self
    {{
        {}: Self = unsafe {{ mem::zeroed() }};
        {}
        return s;
    }}
}}

", struct_type.struct_name, s_type_mut_string, s_type_str));       
    }
    
    return string_out;
}

struct EnumType
{
    enum_name: String,
    bit_width: String,
    param_type_names: Vec<String>,
    is_enum: bool,
}


fn parse_vk_enums(root: &xmltree::Element) -> Vec<EnumType>
{
    let mut enums: Vec<EnumType> = Vec::new();

    node_children2(&root, &mut enums, "enums", &vec![("name", "")], |child, enums|
    {
        let mut name_string = child.attributes["name"].to_string();
        let mut is_enum = true;
        if child.attributes["name"] == "API Constants"
        {
            name_string = "APIConstants".to_string();
            is_enum = false;
        }
        else if !(child.attributes.contains_key("type") && (child.attributes["type"] == "bitmask" || child.attributes["type"] == "enum"))
        {
            return;
        }

        let mut bit_width = "32"; 
        if child.attributes.contains_key("bitwidth") 
        { 
            bit_width = &child.attributes["bitwidth"];
        }
        
        
        let mut enum_type = EnumType{enum_name: name_string, bit_width: bit_width.to_string(),
            param_type_names: Vec::new(), is_enum};
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
                string.push_str(&format!("pub type {} = u64;\r\n", child3.get_text().unwrap()));
            });
        });
    });
    string.push_str("\r\n\r\n");
    return string;
}


struct CommandStruct
{
    command_string: String,
    command_string_return: String,

    command_parms: Vec<String>,
}

fn parse_commands(root: &xmltree::Element) -> String
{
    //let t = enum_types;
    let mut commands: Vec<CommandStruct> = Vec::new();

    node_children2(&root, &mut commands, "commands", &Vec::new(), |child, commands|
    {
        node_children2(&child, commands, "command", &Vec::new(),  |child2, commands|
        {
            let mut command_struct = CommandStruct{command_string: String::new(), command_string_return: String::new(), command_parms: Vec::new() };
            node_children2(&child2, &mut command_struct, "", &Vec::new(), |child3, command_struct|
            {
                if child3.name.eq("proto")
                {
                    let ans = parse_type_name(child3);
                    command_struct.command_string = ans.0;
                    command_struct.command_string_return = ans.1;
                    //println!("fn {}, ret: {}", ans.0, ans.1);
                }
                else if child3.name.eq("param")
                {
                    let ans = parse_type_name(child3);
                    //println!("\tparam: {}: {}", ans.0, ans.1);
                    command_struct.command_parms.push(format!("{}: {}", ans.0, ans.1));
                }
            });
            if command_struct.command_string.len() > 0
            {
                commands.push(command_struct);
            }
        });
    });

    let mut string = String::new();

    for command in &commands
    {
        string.push_str(&format!("fn {}(", &command.command_string));
        for params in &command.command_parms
        {
            string.push_str(&format!("{}, ", &params));
        }
        string.push_str(&format!(") -> {};\r\n", &command.command_string_return));
    }
    string.push_str("\r\n");

    return string;
}

fn get_enums_as_string(enum_types: &Vec<EnumType>) -> String
{
    let mut vk_enum_str = String::new();
    for enum_type in &*enum_types
    {
        if enum_type.param_type_names.len() == 0
        {
            vk_enum_str.push_str(&format!("type {} = u{};\r\n\r\n", enum_type.enum_name, enum_type.bit_width));
            continue;
        }
        else if enum_type.is_enum
        {
            vk_enum_str.push_str(&format!("#[repr(i{})]\r\n#[derive(Debug, Copy, Clone, PartialEq, Eq)]\r\npub enum {}\r\n{{\r\n", enum_type.bit_width, enum_type.enum_name));
            for enum_type_param_type_names in &enum_type.param_type_names
            {
                vk_enum_str.push_str(&format!("{}", enum_type_param_type_names));
            }
            vk_enum_str.push_str("}\r\n\r\n");
        }
        else 
        {
            for enum_type_param_type_names in &enum_type.param_type_names
            {
                let s = parse_api_constant(&enum_type_param_type_names);
                if s.len() == 0
                {
                    continue;
                }
                vk_enum_str.push_str(&format!("pub const {}\n", s));
            }
        }
    }
    return vk_enum_str;
}

fn main() 
{
    let now = std::time::Instant::now();
    let vk_xml = match read_file_to_string("../vk.xml") 
    {
        Ok(v) => v,
        Err(_) => { println!("Failed to load vk.xml"); return; }
    };
    println!("read file: {}", now.elapsed().as_micros());


    let now = std::time::Instant::now();
    let root = match carp_xml_parser::parse(&vk_xml)
    {
        Ok(v) => v,
        Err(v) => { println!("Error parsing: {}", v); return; }
    };
    println!("carp parse xml: {}", now.elapsed().as_micros());


    let now = std::time::Instant::now();
    match parse_vk(&root)
    {
        Ok(_) => (),
        Err(e) => println!("Error: {}", &e)
    };
    println!("carp parse vk: {}", now.elapsed().as_micros());
/*
    let now = std::time::Instant::now();
    let _root = xmltree::Element::parse((&vk_xml).as_bytes()).unwrap();
    println!("parse elements: {}", now.elapsed().as_micros());
*/
/*

    let now = std::time::Instant::now();
    //parse_vk_structs2(&root);
    let mut vk_enums = parse_vk_enums(&root);
    parse_vk_enum_extensions(&root, &mut vk_enums);
    let vk_structs = parse_vk_structs(&root, &mut vk_enums);

    let handles_str = parse_handles(&root);
    let command_str = parse_commands(&root);

    let vk_enum_str = get_enums_as_string(&vk_enums);


    


    parse_type_types(&root);

    let mut vk_all = 
r"#!#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::mem;
use std::os::raw::*;

pub type VkSampleMask = u32;
pub type VkBool32 = u32;
pub type VkFlags = u32;
pub type VkFlags64 = u64;
pub type VkDeviceSize = u64;
pub type VkDeviceAddress = u64;

pub type VkClearValue = f32;

pub fn vk_make_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32
{
    return (variant << 29) | (major << 22) | (minor << 12) | patch; 
}     

".to_string();

    vk_all.push_str(&handles_str);
    vk_all.push_str(&vk_enum_str);
    vk_all.push_str(&vk_structs);
    println!("parse xml into strings: {}", now.elapsed().as_micros());


    let now = std::time::Instant::now();
    std::fs::write("carp_vk_parser/vk_handles.rs", &handles_str).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_enums.rs", &vk_enum_str).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_structs.rs", &vk_structs).expect("Unable to write file");
    std::fs::write("carp_vk_parser/src/vk_all.rs", &vk_all).expect("Unable to write file");
    std::fs::write("carp_vk_parser/commands.rs", &command_str).expect("Unable to write file");
    println!("write xml files: {}", now.elapsed().as_micros());

    let now = std::time::Instant::now();

    let mut carp_lib_loader =  carp_lib_loader::CarpLibLoader::new();
    match test_vk::Vulkan::new(&mut carp_lib_loader)
    {
        Ok(_) => (),
        Err(err) => println!("Loader error: {}", &err)
    };
    println!("instantiate vulkan: {}", now.elapsed().as_micros());
*/
}











