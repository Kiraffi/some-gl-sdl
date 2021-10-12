
struct  Poo <'a>
{
    elements: Vec<Poo<'a>>,

    txt: &'a [u8],
    i: i32,
}


fn three<'a>(poo: &mut Poo<'a>, txt: &'a [u8] )
{
    poo.elements.push(Poo{elements: Vec::new(), txt: &txt[2..3], i: 49 });
}


fn two<'a>(poo: &mut Poo<'a>, txt: &'a [u8] )
{
    poo.i = 12;

    let mut p2 = Poo{elements: Vec::new(), txt: &txt[1..2], i: 58 };
    three(&mut p2, txt);
    poo.elements.push(p2);
}

fn get_something<'a>(txt: &'a [u8]) -> Poo
{
    let mut foo = Poo {elements: Vec::new(), txt: &txt[0..3], i: 8};
    two(&mut foo, txt);
    return foo;
}

fn print_poo(p: &Poo)
{
    println!("texti: {:#?} v:{}", p.txt, p.i);
    for c in &p.elements
    {
        print_poo(c);
    }
}

fn check_contains(txt: &[u8], find: &[u8]) -> bool
{
    if txt.len() < find.len()
    {
        return false;
    }

    let mut found_chars = 0;

    for c in txt
    {
        if *c == find[found_chars]
        {
            found_chars += 1;
            if found_chars == find.len()
            {
                return true;
            }
        }
        else 
        {
            found_chars = 0;    
        }
    }

    return false;

}

fn main() {
    let p = get_something(b"atexti");
    if p.txt.contains(&b'e')
    {
        println!("has t");
    }
    if check_contains(p.txt, b"at")
    {
        println!("has te");
    }
    print_poo(&p);
    println!("Hello, world!");
}
