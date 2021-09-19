use std::io::Read;

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

// The most awkward lambda, having to pass arrays for looking what attributes it contains, and setting possible filter value....
fn node_children<T>(elem: &xmltree::Element, filter_name: &str, attr_contains: &Vec<(&str, &str)>, f: fn(&xmltree::Element) -> Vec<T>) -> Vec<T>
{
    let mut out_string: Vec<T> = Vec::new();
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
                out_string.extend(f(child2));
            }
        }
    }
    return out_string;
}











fn parse_vk_structs(root: &xmltree::Element) -> String
{
    let mut s = String::new();
    let mut s_vec: Vec<String> = Vec::new();
    s_vec.extend(node_children(&root, "types", &Vec::new(), |child|
    {
        node_children(&child, "type",&vec![("category", "struct"), ("name", "")],  |child2|
        {
            let mut s_vec: Vec<String> = Vec::new();
            s_vec.push(format!("pub struct {}\r\n{{\r\n", child2.attributes["name"]));
            
            s_vec.extend(node_children(&child2, "member",&Vec::new(),  |child3|
            {
                if child3.attributes.contains_key("values")
                {
                    println!("Values: {}", child3.attributes["values"]);
                }
                let mut s_vec: Vec<String> = Vec::new();
                s_vec.push("    ".to_string());
                s_vec.extend(node_children::<String>(&child3, "name",&Vec::new(),  |child4|
                {
                    match child4.get_text()
                    {
                        Some(v) => vec![v.to_string()],
                        None => vec![String::new()]
                    }
                }));

                s_vec.push(": ".to_string());

                s_vec.extend(node_children::<String>(&child3, "type",&Vec::new(),  |child4|
                {
                    match child4.get_text()
                    {
                        Some(v) => vec![v.to_string()],
                        None => vec![String::new()]
                    }
                }));
                s_vec.push((";\r\n").to_string());
                return s_vec;
            }));


            s_vec.push(("}\r\n\r\n").to_string());
            return s_vec;
        })
    }));

    
    for strings in &s_vec
    {
        s.push_str(&strings[..]);
    }
    return s;
}



fn parse_vk_enums(root: &xmltree::Element) -> String
{
    let mut s = String::new();
    let mut s_vec: Vec<String> = Vec::new();
    
    s_vec.extend(node_children(&root, "enums", &vec![("type", "enum"), ("name", "")], |child|
    {
        let mut s_vec: Vec<String> = Vec::new();
        s_vec.push(format!("pub enum {}\r\n{{\r\n", child.attributes["name"]));
        s_vec.extend(node_children(&child, "enum",&vec![("value", ""), ("name", "")],  |child2|
        {
            return vec![format!("    {}: i32 = {},\r\n", child2.attributes["name"], child2.attributes["value"])];
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


fn main() 
{
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