
pub struct XMLElement
{
    elements: Vec<XMLElement>,
    element_name: String,
    element_text: String,

    param_stack: Vec<(String, String)>,
    level: u32,
}
impl XMLElement
{
    fn new(level: u32) -> Self
    {
        let s = XMLElement {element_name: String::new(), element_text: String::new(), elements: Vec::new(), param_stack: Vec::new(), level };
        return s;
    }
}


fn get_text_from_array(txt: &[u8], start: usize, end: usize) -> Result<String, &'static str>
{
    if start >= end || end > txt.len()
    {
        return Ok( String::new() );
    }

    let s = match String::from_utf8(txt[start..end].to_vec())
    {
        Ok(r) => r,
        Err(_) => return Err("Failed to convert u8 array to string")
    };
    return Ok(s);
}



fn compare_text_arrays(txt: &[u8], current_letter_pos: usize, cmp_text: &[u8]) -> bool
{
    let str_cmp_len = cmp_text.len();
    if current_letter_pos + str_cmp_len > txt.len()
    {
        return false;
    }

    let s = &txt[current_letter_pos..current_letter_pos + str_cmp_len];
    return s.eq(cmp_text);
}
fn compare_text(txt: &[u8], current_letter_pos: usize, cmp_text: &str) -> bool
{
    return compare_text_arrays(txt, current_letter_pos, cmp_text.as_bytes());
}


fn parse_element(txt: &[u8], txt_len: usize, current_element: &mut XMLElement, letter_pos: &mut usize) -> Result<(), &'static str>
{
    let mut tag_text = (0usize, 0usize);
    let mut param_name = (0usize, 0usize);
    let mut param_value = (0usize, 0usize);
    let mut elem_text = (0usize, 0usize);

    
    const TAG_START: u32 =            1u32 << 0u32;

    const INSIDE_TAG: u32 =           1u32 << 1u32;
    const INSIDE_TAG_PARAM_VALUE: u32 = 1u32 << 2u32;
    
    const BACK_SLASH: u32 =           1u32 << 3u32;
    
    const INSIDE_COMMENT_BLOCK: u32 =       1u32 << 4u32;

    const PARSING_TEXT_BLOCK: u32 =         1u32 << 5u32;
    
    
    let mut current_state: u32;
    
    //if txt[*letter_pos..*letter_pos + 1].eq("<") && current_element.level > 0
    let c = txt[*letter_pos] as char;
    if c == '<' && current_element.level > 0
    {
        *letter_pos = *letter_pos + 1;
        current_state = TAG_START | INSIDE_TAG;
        tag_text.0 = *letter_pos;
        tag_text.1 = *letter_pos;
    }
    else 
    {
        current_state = PARSING_TEXT_BLOCK;
        elem_text.0 = *letter_pos;
        elem_text.1 = *letter_pos;
    }

    let mut inside_param_value_quote_count = 0u32;

    while *letter_pos < txt_len
    {
        let u = txt[*letter_pos];
        let c = u as char;
        let mut current_letter_pos = *letter_pos;


        if (current_state & INSIDE_COMMENT_BLOCK) == INSIDE_COMMENT_BLOCK && (c == '-') && compare_text(txt, current_letter_pos, "-->")
        {
            current_state = current_state & !(INSIDE_COMMENT_BLOCK);
            *letter_pos = current_letter_pos;
        }
        else if (current_state & INSIDE_COMMENT_BLOCK) == INSIDE_COMMENT_BLOCK
        {
            *letter_pos = *letter_pos + 1;           
            continue;
        }
        else if (current_state & PARSING_TEXT_BLOCK) == PARSING_TEXT_BLOCK && current_element.level > 0
        {
            if c == '<' && current_state & BACK_SLASH == 0
            {
                elem_text.1 = current_letter_pos;
                current_element.element_text = get_text_from_array(txt, elem_text.0, elem_text.1)?;
                *letter_pos = *letter_pos - 1;
                return Ok(());
            }

            if c.is_whitespace() && elem_text.0 == current_letter_pos
            {
                elem_text.0 = current_letter_pos + 1;
                elem_text.1 = current_letter_pos + 1;
            }
        }
        else if c == '<' && (current_state & BACK_SLASH) == 0
        {
            if compare_text(txt, current_letter_pos, "<!--")
            {
                current_state = current_state | INSIDE_COMMENT_BLOCK;                
            }
            else if compare_text(txt, current_letter_pos, "</")
            {
                current_letter_pos += 2;
                let mut tmp = txt[current_letter_pos] as char;
                while tmp.is_whitespace()
                {
                    current_letter_pos += 1;                 
                    if current_letter_pos >= txt_len
                    {
                        return Err("No matching ending tag!");
                    }
                    tmp = txt[current_letter_pos] as char;
                }
                let cmp = &txt[tag_text.0..tag_text.1];
                
                if compare_text_arrays(txt, current_letter_pos, cmp)
                {
                    current_letter_pos += cmp.len();
                    tmp = txt[current_letter_pos] as char;
                    while tmp.is_whitespace()
                    {
                        current_letter_pos += 1;                 
                        if current_letter_pos >= txt_len
                        {
                            return Err("No matching ending tag!");
                        }
                        tmp = txt[current_letter_pos] as char;
                    }
                    if tmp != '>'
                    {
                        return Err("Failed to find matching end!");
                    }
                    *letter_pos = current_letter_pos;
                    return Ok(());
                }

            }
            else
            {
                let mut child = XMLElement::new(current_element.level + 1);
                parse_element(txt, txt_len, &mut child, letter_pos)?;
                current_element.elements.push(child);
            }
        }
        else if (current_state & TAG_START) == TAG_START
        {
            if c.is_alphanumeric() || c == '_' || (c == '?'&& tag_text.0 == tag_text.1)
            {
                tag_text.1 = tag_text.1 + 1;
            }
            else if c.is_whitespace() && tag_text.0 == tag_text.1
            {
                tag_text.0 = tag_text.0 + 1;
                tag_text.1 = tag_text.0;
            }
            else
            {
                current_element.element_name =  get_text_from_array(txt, tag_text.0, tag_text.1)?;
                current_state = current_state & !( TAG_START );

                param_name.0 = current_letter_pos + 1;
                param_name.1 = current_letter_pos + 1;
            }
        }
       
        else if (current_state & (INSIDE_TAG | BACK_SLASH) == INSIDE_TAG) && c == '/' && 
            compare_text(txt, current_letter_pos, "/>")
        {
            *letter_pos = *letter_pos + 1;
            return Ok(());
        }
        else if c == '?' && compare_text(txt, current_letter_pos, "?>")
        {
            *letter_pos = *letter_pos + 1;
            return Ok(());
        }

        else if current_state & INSIDE_TAG == INSIDE_TAG
        {
            if (c.is_alphanumeric() || c == '_') && (current_state & INSIDE_TAG_PARAM_VALUE) == 0
            {
            }
            else if (current_state & INSIDE_TAG_PARAM_VALUE) == 0 && c == '='
            {
                current_state = current_state | INSIDE_TAG_PARAM_VALUE;
                inside_param_value_quote_count = 0;
                param_name.1 = current_letter_pos - 1;
            }
            else if (current_state & INSIDE_TAG_PARAM_VALUE) == 0 && param_name.0 <= param_name.1
            {
                param_name.0 = current_letter_pos + 1;                
            }
            else if (current_state & INSIDE_TAG_PARAM_VALUE) == INSIDE_TAG_PARAM_VALUE && c == '"'
            {
                inside_param_value_quote_count = inside_param_value_quote_count + 1;
                if inside_param_value_quote_count == 2
                {
                    inside_param_value_quote_count = 0;
                    param_value.1 = core::cmp::max(param_value.0, current_letter_pos);

                    
                    current_element.param_stack.push(
                         (get_text_from_array(txt,param_name.0, param_name.1)?,
                         get_text_from_array(txt, param_value.0, param_value.1)?)
                    );
                    

                    current_state = current_state & !(INSIDE_TAG_PARAM_VALUE);

                    param_name.0 = current_letter_pos + 1;
                    param_name.1 = current_letter_pos + 1;
                }
                else
                {
                    param_value.0 = current_letter_pos + 1;
                    param_value.1 = current_letter_pos + 1;
                }
            }
        }
        else if (current_state & (INSIDE_TAG | PARSING_TEXT_BLOCK)) == 0 && tag_text.0 != tag_text.1
        {
            let mut child = XMLElement::new(current_element.level + 1);                 
            parse_element(txt, txt_len, &mut child, letter_pos)?;
            if child.element_text.len() > 0
            {
                current_element.elements.push(child);
            }
        }
        
        if c == '\\' 
        { 
            let is_black_slash = (current_state & BACK_SLASH) == BACK_SLASH;
            current_state = current_state & (!BACK_SLASH);
            if !is_black_slash
            {
                current_state = current_state | BACK_SLASH;
            }
        }
        else 
        {
            current_state = current_state & !(BACK_SLASH);
        }

        if c == '>' &&  (current_state & BACK_SLASH) == 0
        {
            current_state = current_state & !(INSIDE_TAG);
        }

        *letter_pos = *letter_pos + 1;
    }
    return Ok(());
}


pub fn parse(txt: &str) -> Result<XMLElement, &'static str>
{
    let mut root = XMLElement::new(0);

    let mut letter_pos = 0;
    let str_vec = txt.as_bytes();
    match parse_element(&str_vec,  str_vec.len(), &mut root, &mut letter_pos)
    {
        Ok(_) => (),
        Err(v) => println!("Error parsing xml: {}", v)
    };

    return Ok(root);
}

