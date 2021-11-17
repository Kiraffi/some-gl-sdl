use std::io::Read;
use carp_xml_parser::{XMLElement, get_text_from_array};

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


#[derive(Debug)]
struct CommandValue
{
    name: String,
    return_type: ReturnType,

    params: Vec<(String, ReturnType)>,
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
    commands: Vec<CommandValue>,
    required_types: Vec<String>,
    required_commands: Vec<String>,
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

fn get_attribute<'a>(elem: &XMLElement<'a>, attr: &'static str) -> &'a str
{
    for attribute in &elem.attributes
    {
        if attribute.0.eq(attr)
        {
            return attribute.1;
        }
    }

    return &"";
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


fn parse_type_name_(elem: &XMLElement) -> (String, ReturnType)
{
    let mut name_str = String::new();
    let mut type_str_other = String::new();
    let mut return_type = ReturnType::new(true);

    for elem_child_child in &*elem.elements
    {
        if elem_child_child.element_name == "name"
        {
            name_str = elem_child_child.elements[0].element_text.to_string();
        }
        else 
        {
            type_str_other.push_str(elem_child_child.element_text);
            
            for elem_child_child_child in &*elem_child_child.elements
            {
                return_type.return_type.push_str(get_type_as_rust_type(elem_child_child_child.element_text));
            }
        }
        
    }


    // Counting consts and ptrs.
    let consts = count_sub_strings_from_str(&type_str_other, "const");
    let ptrs = count_sub_strings_from_str(&type_str_other, "*");

    return_type.ptr_mutable = consts > 0;
    return_type.ptrs = ptrs;


    let mut const_mut_ptr_string = String::new();
    let c = core::cmp::max(consts, ptrs);

    let found = type_str_other.contains("[") && type_str_other.contains("]");
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
            type_str_other = type_str_other.replace("[", "");
            type_str_other = type_str_other.replace("]", "");
            return_type.return_type = format!("[{}; {}]", &return_type.return_type, &type_str_other);
        }
    }    
    const_mut_ptr_string.push_str(&return_type.return_type);
    return_type.return_type = const_mut_ptr_string;

    return (name_str, return_type);
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
                        "alias" => alias.original_name = attr.1.to_string(),
                        "category" => alias.category = attr.1.to_string(),
                        "name" => alias.alias_name = attr.1.to_string(),
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
                                "name" => defi.define_name = child2.elements[0].element_text.to_string(),
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
                                bitmask.name = child2.elements[0].element_text.to_string();
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
                                "parent" => { p = true; handle.parent = attr.1.to_string(); },
                                "objtypeenum" => { founds += 1; handle.obj_enum = attr.1.to_string(); },
                                _ => ()
                            };
                        }

                        for child2 in &child.elements
                        {
                            let s: &str = &child2.element_name;
                            match s 
                            {
                                "type" =>  { founds += 1; handle.handle_type = child2.elements[0].element_text.to_string(); },
                                "name" => { founds += 1; handle.handle_name = child2.elements[0].element_text.to_string(); },
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
                                "name" => { founds += 1; e.enum_name = attr.1.to_string(); },
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
                                f.name = child2.elements[0].element_text.to_string();
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

                                    f.param_type_names.push(get_text_from_array(s, c, d).unwrap().to_string());

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

                                    f.param_type_names.push(get_text_from_array(s, c, d).unwrap().to_string());
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
                                s.struct_name = attr.1.to_string();
                                name_found = true;
                            }
                            else if attr.0 == "structextends"
                            {
                                s.struct_extends = attr.1.to_string();
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
                                    s.s_type_value = attr.1.to_string();
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
                                    return_name = child3.elements[0].element_text.to_string();
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
    let mut parsed_arrays = ParsedArrays { 
        aliases: Vec::new(), bitmasks: Vec::new(), defines: Vec::new(), enums: Vec::new(),
        func_ptrs: Vec::new(), handles: Vec::new(), structs: Vec::new(), constants: Vec::new(),
        commands: Vec::new(), required_types: Vec::new(), required_commands: Vec::new(), 
    };
    for child in &elem.elements
    {
        if child.element_name != "registry"
        {
            continue;
        }

        for child2 in &child.elements
        {
            match child2.element_name
            {
                "types" => parse_types(child2, &mut parsed_arrays)?,
                "enums" => 
                {
                    let mut enum_name = String::new();
                    let mut enum_bitmask = -1;
                    //let mut bit_width = 32;

                    let mut enums: Vec<(String, String)> = Vec::new();

                    for attr in &child2.attributes
                    {
                        let s: &str = &attr.0;
                        match s
                        {
                            "name" => enum_name = attr.1.to_string(),
                            "type" => enum_bitmask = if attr.1 == "bitmask" { 1 } else { 0 },
                            //"bitwidth" => bit_width = attr.1.parse().unwrap(),
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
                                    "type" => constant.constant_type = attr.1.to_string(),
                                    "value" => constant.constant_value = attr.1.to_string(),
                                    "name" => constant.name = attr.1.to_string(),
                                    "alias" => alias = attr.1.to_string(),
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
                            match attr.0
                            {
                                "value" => bit_value = attr.1.to_string(),
                                "bitpos" => bit_value = (1i128 << attr.1.parse::<i128>().unwrap()).to_string(),
                                "name" => enum_enum_name = attr.1.to_string(),
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
                },

                "commands" => 
                {
                    'command_children:
                    for elem in &child2.elements
                    {
                        if elem.element_name != "command"
                        {
                            continue 'command_children;
                        }

                        for attr in &elem.attributes
                        {
                            if attr.0 == "alias"
                            {
                                continue 'command_children;
                            }
                        }
                        

                        let mut command = CommandValue {name: String::new(), return_type: ReturnType::new(false), params: Vec::new() };
                        
                        for elem_child in &*elem.elements
                        {
                            //let mut command_name = String::new();
                            //let mut command_name_type = String::new();

                            match elem_child.element_name
                            {
                                "proto" =>
                                {
                                    let (command_name, return_type) = parse_type_name_(elem_child);
                                    command.name = command_name;
                                    command.return_type = return_type;
                                },
                                "param" => 
                                {
                                    let (param_name, param_return_type) = parse_type_name_(elem_child);
                                    command.params.push((param_name, param_return_type));
                                },
                                _ => ()
                            }

                        }
                        if command.name.is_empty()
                        {
                            return Err("command has no name!");
                        }

                        parsed_arrays.commands.push(command);
                    }
                },
                "feature" => 
                {
                    for elem in &child2.elements
                    {
                        if elem.element_name != "require"
                        {
                            continue;
                        }

                        for elem_child in &*elem.elements
                        {
                            //let mut command_name = String::new();
                            //let mut command_name_type = String::new();

                            match elem_child.element_name
                            {
                                "type" => parsed_arrays.required_types.push(elem_child.attributes[0].1.to_string()),
                                "command" => parsed_arrays.required_commands.push(elem_child.attributes[0].1.to_string()),
                                "enum" =>
                                {
                                    let mut name: &str = "";
                                    let mut extend_name: &str = "";

                                    let mut is_bitpos = false;

                                    let mut bit_pos = 0i128;
                                    let mut extnumb = 0i128;
                                    let mut offset = 0i128;

                                    for attr in &elem_child.attributes
                                    {
                                        match attr.0
                                        {
                                            "name" => name = attr.1,
                                            "extends" => extend_name = attr.1,
                                            "extnumber" => extnumb = attr.1.parse().unwrap(),
                                            "offset" => offset = attr.1.parse().unwrap(),
                                            "bitpos" => 
                                            {                                            
                                                is_bitpos = true;
                                                bit_pos = attr.1.parse().unwrap();
                                            },
                                            _ => ()
                                        }
                                    }
                                    parsed_arrays.required_types.push(name.to_string());
                                    if name.is_empty()
                                    {
                                        return Err("Empty name for extending enum");
                                    }
                                    if extend_name.is_empty()
                                    {
                                        continue;
                                    }
                                    let mut found = false;
                                    for ind in &mut parsed_arrays.enums
                                    {
                                        if ind.enum_name == extend_name
                                        {
                                            found = true;
                                            if extnumb > 1
                                            { 
                                                extnumb -= 1;
                                            }
                                            let parsed_value = if is_bitpos { 1i128 << bit_pos } else { 1_000_000_000i128 + extnumb * 1000 + offset };
                                            ind.param_type_names.push(name.to_string());
                                            ind.param_type_values.push(parsed_value.to_string());

                                            break;
                                        }
                                    }

                                    if !found
                                    {
                                        println!("failed to extend: name {}, extend {}", name, extend_name);
                                        return Err("Didn't find existing enum to extend.");
                                    }

                                    // extend enum
                                },
                                "comment" => (),
                                _ => 
                                {
                                    println!("Hmm: {}", elem_child.element_name);
                                    return Err("was not found!")
                                }
                            }

                        }
                    }
                },
                _ => ()
            }
        }
    }
    dbg!(&parsed_arrays.required_commands);

    return Ok(parsed_arrays);
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
    let mut result;
    match parse_vk(&root)
    {
        Ok(res) => result = res,
        Err(e) => { println!("Error: {}", &e); return; }
    };
    println!("carp parse vk: {}", now.elapsed().as_micros());
    
    /*
    for enums in &result.enums
    {
        if !result.required_types.contains(&enums.enum_name)
        {
            continue;
        }
        println!("{}", enums.enum_name);
        for i in 0..enums.param_type_names.len()
        {
            println!("   {} : {}", enums.param_type_names[i], enums.param_type_values[i]);

        }
    }


    println!("carp parse vk after print: {}", now.elapsed().as_micros());
    */
    

}











