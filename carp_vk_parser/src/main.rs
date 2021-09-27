#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]

use std::{io::Read, thread::current};

use crate::vk_all::VkInternalAllocationType;

pub mod test_vk;
pub mod vk_all;

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
        "char" => "c_uchar",
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
                
                // wrong way....
                let eq_len = enum_type_param_type_names.len();
                if eq_len == 0
                {
                    continue;
                }
                let mut s = enum_type_param_type_names.clone(); 
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


                vk_enum_str.push_str(&format!("pub const {}\n", s));
            }
        }
    }
    return vk_enum_str;
}

struct XMLStuff
{
    elements: Vec<XMLStuff>,
    element_name: String,
    element_text: String,
}
impl XMLStuff
{
    fn new(name: &str) -> Self
    {
        XMLStuff{element_name: name.to_string(), element_text: "".to_string(), elements: Vec::new() }
    }
}

fn remove_element<'a>(root: &'a mut XMLStuff, current_element: &'a mut &'a mut XMLStuff, element_level: &mut u32, tag_stack: &mut Vec<String>)
{
    tag_stack.remove(tag_stack.len() - 1);
            
    *element_level = *element_level - 1;
    *current_element = root;
    for _ in 0..*element_level
    {
        *current_element = (**current_element).elements.last_mut().unwrap();
    }

}

fn print_tags(txt: &str)
{
    let mut root = XMLStuff::new("");
    let mut current_element = &mut root;
    let mut element_level = 0u32;

    let now = std::time::Instant::now();

    let mut tag_stack: Vec<String> = Vec::new();
    let mut param_stack: Vec<(String, String)> = Vec::new();


    let mut prev_char = '\0';
    let mut tag_text = String::new();
    let mut param_name = String::new();
    let mut param_value = String::new();

    
    const TagStart: u32 =  1u32 << 0u32;
    const TagEnd: u32 =  1u32 << 1u32;
    
    const BackSlash: u32 = 1u32 << 2u32;

    const InsideTag: u32 = 1u32 << 3u32;
    const InsideTagParamValue: u32 = 1u32 << 4u32;

    const InisdeComment: u32 = 1u32 << 5u32;

    let mut current_state = 0u32;


    let mut line_number = 0u32;
    
    let mut inside_param_value_quote_count = 0u32;

    let iter = txt.chars();
    for c in iter
    {
        if (c == '-' || c == '>') && (current_state & (InisdeComment | BackSlash)) == InisdeComment
        {
            tag_text.push(c);
            if tag_text.eq("-->")
            {
                current_state = current_state & !(InisdeComment);
                tag_text.clear();
            }
        }
        else if (current_state & InisdeComment) == InisdeComment
        {
            tag_text.clear();
        }
        else if c == '<'
        {
            current_state = current_state | TagStart | InsideTag; 
            //tag_text.clear();
        }
        else if (current_state & TagStart) == TagStart
        {
            if c.is_alphanumeric() || c == '_' || c == '!' || c == '-' || (c == '?' && tag_text.len() == 0)
            { 
                tag_text.push(c); 
            }
            else if c.is_whitespace() && tag_text.len() == 0 {}
            else if c == '/' && prev_char == '<' 
            {
                current_state = current_state & !(TagStart | InsideTag);
                current_state = current_state | TagEnd;
            }
            else if tag_text.len() >= 3 && tag_text[0..3].eq("!--")
            {
                current_state = current_state & !( TagStart );
                current_state = current_state | InisdeComment;
                tag_text.clear();
            }
            else
            {
                (*current_element).elements.push(XMLStuff::new(&tag_text));
                current_element = (*current_element).elements.last_mut().unwrap();
                element_level = element_level + 1;


                tag_stack.push(tag_text.clone());
                //println!("Tag: {}", &tag_text);
                current_state = current_state & !( TagStart );
                tag_text.clear();
            }
        }
        else if (current_state & TagEnd) == TagEnd
        {
            if c.is_alphanumeric() || c == '_' || (c == '?' && tag_text.len() == 0)
            { 
                tag_text.push(c); 
            }
            else if c.is_whitespace() && tag_text.len() == 0 {}
            else 
            {
                current_state = current_state & !(TagEnd);
               
                if tag_text.eq(tag_stack.last().unwrap())
                {
                    remove_element(&mut root, &mut &mut current_element, &mut element_level, &mut &mut tag_stack);
                }
                else
                {
                    println!("Unmatched tags! {} vs {}", &tag_text, tag_stack.last().unwrap() );
                }
                tag_text.clear();
            }
        }
        else if (current_state & (TagEnd | TagStart | BackSlash) == 0) && prev_char == '/' && c == '>'
        {
            remove_element(&mut root, &mut current_element, &mut element_level, &mut &mut tag_stack);
        }
        else if c == '>' && prev_char == '?' &&  tag_stack.last().unwrap().eq("?xml")
        {
            remove_element(&mut root, &mut current_element, &mut element_level, &mut &mut tag_stack);
        }

        else if current_state & InsideTag == InsideTag
        {
            if (c.is_alphanumeric() || c == '_') && (current_state & InsideTagParamValue == 0)
            {
                param_name.push(c);
            }
            else if current_state & InsideTagParamValue == 0 && c == '='
            {
                current_state = current_state | InsideTagParamValue;
                inside_param_value_quote_count = 0;
            }
            else if current_state & InsideTagParamValue == InsideTagParamValue && c == '"'
            {
                inside_param_value_quote_count = inside_param_value_quote_count + 1;
                if inside_param_value_quote_count == 2
                {
                    inside_param_value_quote_count = 0;

                    param_stack.push((param_name.clone(), param_value.clone()));
                    param_name.clear();
                    param_value.clear();

                    current_state = current_state & !(InsideTagParamValue);
                }
            }
            else if current_state & InsideTagParamValue == InsideTagParamValue && inside_param_value_quote_count == 1
            {
                param_value.push(c);
            }
        }
        else if current_state & (InsideTag | TagStart | TagEnd) == 0u32
        {

        }

        if c == '\\' 
        { 
            let is_black_slash = (current_state & BackSlash) == BackSlash;
            current_state = current_state & (!BackSlash);
            if !is_black_slash
            {
                current_state = current_state | BackSlash;
            }
        }
        else if (current_state & BackSlash) == BackSlash
        {
            current_state = current_state & !(BackSlash);
        }

        if c == '>' &&  (current_state & BackSlash) == 0u32
        {
            current_state = current_state & !(InsideTag);


            if current_state & InsideTagParamValue == InsideTagParamValue
            {
                param_stack.push((param_name.clone(), param_value.clone()));
            }
            let ssss = ("category".to_string(), "struct".to_string());

            if param_stack.len() > 0 && tag_stack.len() >= 3 && tag_stack[2].eq("type") && param_stack[0].eq(&ssss)
            {
                let mut ss = String::new();
                for a in &param_stack
                {
                    ss.push_str(&a.0);
                    ss.push_str(": ");
                    ss.push_str(&a.1);
                    ss.push_str(" ");
                }
                println!("Line: {} - {}", line_number, ss);
            }
            
            param_name.clear();
            param_value.clear();

            param_stack.clear();
        }
        
        if c == '\n'
        {
            line_number = line_number + 1;
        }

        prev_char = c;
        
    }
    for tag in &tag_stack
    {
        println!("tag left: {}", tag);
    }
    println!("print tags: {}", now.elapsed().as_micros());
    return ;
}



fn main() 
{

    let vk_xml = match read_file_to_string("../vk.xml") 
    {
        Ok(v) => v,
        Err(_) => { println!("Failed to load vk.xml"); return; }
    };

    print_tags(&vk_xml);

    let now = std::time::Instant::now();
    let root = xmltree::Element::parse((&vk_xml).as_bytes()).unwrap();
    println!("parse elements: {}", now.elapsed().as_micros());

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


    std::fs::write("carp_vk_parser/vk_handles.rs", &handles_str).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_enums.rs", &vk_enum_str).expect("Unable to write file");
    std::fs::write("carp_vk_parser/vk_structs.rs", &vk_structs).expect("Unable to write file");
    std::fs::write("carp_vk_parser/src/vk_all.rs", &vk_all).expect("Unable to write file");
    std::fs::write("carp_vk_parser/commands.rs", &command_str).expect("Unable to write file");
    
    let mut carp_lib_loader =  carp_lib_loader::CarpLibLoader::new();
    match test_vk::Vulkan::new(&mut carp_lib_loader)
    {
        Ok(_) => (),
        Err(err) => println!("Loader error: {}", &err)
    };

}











