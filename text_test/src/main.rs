
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;


#[cfg(target_arch = "x86")]
use std::arch::x86::*;

const  V32: u32  = 0x6161_6161;
const  V64: u64  = 0x6161_6161_6161_6161;
const V128: u128 = 0x6161_6161_6161_6161_6161_6161_6161_6161;



fn add_chars(s: &mut std::string::String)
{
    for i in 0..1_000_007
    {
        let c = (i % 16) as u8 + 'b' as u8;
        s.push(c as char);
    }
}


fn find_find_str(s: &str) -> usize
{
    
    match s.find("a")
    {
        Some(v) => return v,
        None => return s.len()
    }
}

fn find_find_char(s: &str) -> usize
{
    match s.find('a')
    {
        Some(v) => return v,
        None => return s.len()
    }
}

fn find_iter(s: &str) -> usize
{
    let mut counter = 0;
    for c in s.chars()
    {
        if c == 'a'
        {
            return counter;
        }
        counter += 1;
    }
    return s.len();
}

fn find_u8(s: &str) -> usize 
{
    let mut counter = 0;
    let ss = s.as_bytes();
    for c in ss
    {
        if *c == 'a' as u8
        {
            return counter;
        }
        counter += 1;
    }
    return s.len();
}

fn find_u8_2(s: &str) -> usize 
{
    return find_ending(&s.as_bytes(), 0);
}



fn find_ending(ss: &[u8], mut pos: usize) -> usize
{
    let len = ss.len();
    let mut p = unsafe { ss.as_ptr().add(pos) };
    while pos < len
    {
        unsafe 
        {
            if *p == 'a' as u8
            {
                return pos;
            }
            p = p.add(1);
            
        };
        pos += 1;
            
    }
    return len;
}










fn find_u128_slice_return(s: &str) -> usize
{
    let v = V128;    
    let ss = s.as_bytes();    
    let len = ss.len() - 16;

    let mut counter = 0;
    while counter < len
    {
        let slice = &ss[counter..counter + 16];
        let mut t = 0u128;
        t |= (slice[0] as u128) << (0 * 8);
        t |= (slice[1] as u128) << (1 * 8);
        t |= (slice[2] as u128) << (2 * 8);
        t |= (slice[3] as u128) << (3 * 8);

        t |= (slice[4] as u128) << (4 * 8);
        t |= (slice[5] as u128) << (5 * 8);
        t |= (slice[6] as u128) << (6 * 8);
        t |= (slice[7] as u128) << (7 * 8);

        t |= (slice[8]  as u128) << (8 * 8);
        t |= (slice[9]  as u128) << (9 * 8);
        t |= (slice[10] as u128) << (10 * 8);
        t |= (slice[11] as u128) << (11 * 8);

        t |= (slice[12] as u128) << (12 * 8);
        t |= (slice[13] as u128) << (13 * 8);
        t |= (slice[14] as u128) << (14 * 8);
        t |= (slice[15] as u128) << (15 * 8);

        t = t ^ v;

        if (t >> (0  * 8)) & 255 == 0 { return counter + 0; }
        if (t >> (1  * 8)) & 255 == 0 { return counter + 1; }
        if (t >> (2  * 8)) & 255 == 0 { return counter + 2; }
        if (t >> (3  * 8)) & 255 == 0 { return counter + 3; }
 
        if (t >> (4  * 8)) & 255 == 0 { return counter + 4; }
        if (t >> (5  * 8)) & 255 == 0 { return counter + 5; }
        if (t >> (6  * 8)) & 255 == 0 { return counter + 6; }
        if (t >> (7  * 8)) & 255 == 0 { return counter + 7; }
 
        
        if (t >> 8  * 8) & 255 == 0 { return counter + 8; }
        if (t >> 9  * 8) & 255 == 0 { return counter + 9; }
        if (t >> 10 * 8) & 255 == 0 { return counter + 10; }
        if (t >> 11 * 8) & 255 == 0 { return counter + 11; }

        if (t >> 12 * 8) & 255 == 0 { return counter + 12; }
        if (t >> 13 * 8) & 255 == 0 { return counter + 13; }
        if (t >> 14 * 8) & 255 == 0 { return counter + 14; }
        if (t >> 15 * 8) & 255 == 0 { return counter + 15; }

        counter += 16;
    }

    return find_ending(ss, counter);
}











fn find_u32_return(s: &str) -> usize
{
    let v = V32;
    
    let ss = s.as_bytes();
    let len = ss.len() - 4;

    let mut p = ss.as_ptr() as *const u32;
    let mut counter = 0;
    
    while counter < len
    {
        let t = unsafe{ *p } ^ v;

        if ( t & 0x000000ff ) == 0 { return counter + 0; }
        if ( t & 0x0000ff00 ) == 0 { return counter + 1; }
        if ( t & 0x00ff0000 ) == 0 { return counter + 2; }
        if ( t & 0xff000000 ) == 0 { return counter + 3; }

        counter += 4;
        p = unsafe { p.add(1) };
    }

    return find_ending(ss, counter);
}


pub fn find_u32_return_test_assign(s: &str) -> usize
{
    let v = V32;
    
    let ss = s.as_bytes();
    let len = ss.len() - 4;

    let mut p = ss.as_ptr() as *const u32;
    let mut counter = 0;
    let mut v1 = !0u32;
    let mut v2 = !0u32;
    let mut v3 = !0u32;
    let mut v4 = !0u32;
    while counter < len && v1 != 0 && v2 != 0 && v3 != 0 && v4 != 0
    {
        let t = unsafe{ *p } ^ v;

        v1 = t & 0x000000ff;
        v2 = t & 0x0000ff00;
        v3 = t & 0x00ff0000;
        v4 = t & 0xff000000;

        counter += 4;
        p = unsafe { p.add(1) };
    }
    counter -= 4;
    if v1 == 0 { return counter + 0; }
    if v2 == 0 { return counter + 1; }
    if v3 == 0 { return counter + 2; }
    if v4 == 0 { return counter + 3; }

    return find_ending(ss, counter);
}

pub fn find_u32_return_test(s: &str) -> usize
{
    let v = V32;
    
    let ss = s.as_bytes();
    let len = ss.len() - 4;

    let mut p = ss.as_ptr() as *const u32;
    let mut counter = 0;

    let mut test_case = false;
    while counter < len && !test_case
    {
        let t = unsafe{ *p } ^ v;

        test_case  = (t & 0x000000ff) == 0;
        test_case |= (t & 0x0000ff00) == 0;
        test_case |= (t & 0x00ff0000) == 0;
        test_case |= (t & 0xff000000) == 0;

        counter += 4;
        p = unsafe { p.add(1) };
    }
    counter -= 4;

    return find_ending(ss, counter);
}



fn find_u64_return(s: &str) -> usize
{
    let v = V64;
    let ss = s.as_bytes();
    let mut p = ss.as_ptr() as *const u64;
    let len = ss.len() - 8;
    let mut counter = 0;
    while counter < len
    {
        let t = unsafe { *p } ^ v;
        
        if (t & 0x0000_0000_0000_00ff ) == 0 { return counter + 0; }
        if (t & 0x0000_0000_0000_ff00 ) == 0 { return counter + 1; }
        if (t & 0x0000_0000_00ff_0000 ) == 0 { return counter + 2; }
        if (t & 0x0000_0000_ff00_0000 ) == 0 { return counter + 3; }
 
        if (t & 0x0000_00ff_0000_0000 ) == 0 { return counter + 4; }
        if (t & 0x0000_ff00_0000_0000 ) == 0 { return counter + 5; }
        if (t & 0x00ff_0000_0000_0000 ) == 0 { return counter + 6; }
        if (t & 0xff00_0000_0000_0000 ) == 0 { return counter + 7; }

        counter += 8;
        p = unsafe { p.add(1) };
    }

    return find_ending(ss, counter);
}


fn find_u64_return_2(s: &str) -> usize
{
    let v = V64;
    let ss = s.as_bytes();
    let mut p = ss.as_ptr() as *const u64;
    let len = ss.len() - 8 * 4;
    

    let mut test_case = false;

    let mut counter = 0;
    while counter < len && !test_case
    {
        let t1 = unsafe { *p } ^ v;
        let t2 = unsafe { *p.add(1) } ^ v;
        let t3 = unsafe { *p.add(2) } ^ v;
        let t4 = unsafe { *p.add(3) } ^ v;
 
        test_case  = (t1 & 0x0000_0000_0000_00ff) == 0;
        test_case |= (t1 & 0x0000_0000_0000_ff00) == 0;
        test_case |= (t1 & 0x0000_0000_00ff_0000) == 0;
        test_case |= (t1 & 0x0000_0000_ff00_0000) == 0;
 
        test_case |= (t1 & 0x0000_00ff_0000_0000) == 0;
        test_case |= (t1 & 0x0000_ff00_0000_0000) == 0;
        test_case |= (t1 & 0x00ff_0000_0000_0000) == 0;
        test_case |= (t1 & 0xff00_0000_0000_0000) == 0;

        test_case |= (t2 & 0x0000_0000_0000_00ff) == 0;
        test_case |= (t2 & 0x0000_0000_0000_ff00) == 0;
        test_case |= (t2 & 0x0000_0000_00ff_0000) == 0;
        test_case |= (t2 & 0x0000_0000_ff00_0000) == 0;
 
        test_case |= (t2 & 0x0000_00ff_0000_0000) == 0;
        test_case |= (t2 & 0x0000_ff00_0000_0000) == 0;
        test_case |= (t2 & 0x00ff_0000_0000_0000) == 0;
        test_case |= (t2 & 0xff00_0000_0000_0000) == 0;

        test_case |= (t4 & 0x0000_00ff_0000_0000) == 0;
        test_case |= (t4 & 0x0000_ff00_0000_0000) == 0;
        test_case |= (t4 & 0x00ff_0000_0000_0000) == 0;
        test_case |= (t4 & 0xff00_0000_0000_0000) == 0;

        test_case |= (t3 & 0x0000_0000_0000_00ff) == 0;
        test_case |= (t3 & 0x0000_0000_0000_ff00) == 0;
        test_case |= (t3 & 0x0000_0000_00ff_0000) == 0;
        test_case |= (t3 & 0x0000_0000_ff00_0000) == 0;
 
        test_case |= (t3 & 0x0000_00ff_0000_0000) == 0;
        test_case |= (t3 & 0x0000_ff00_0000_0000) == 0;
        test_case |= (t3 & 0x00ff_0000_0000_0000) == 0;
        test_case |= (t3 & 0xff00_0000_0000_0000) == 0;

        test_case |= (t4 & 0x0000_0000_0000_00ff) == 0;
        test_case |= (t4 & 0x0000_0000_0000_ff00) == 0;
        test_case |= (t4 & 0x0000_0000_00ff_0000) == 0;
        test_case |= (t4 & 0x0000_0000_ff00_0000) == 0;
 
        counter += 8 * 4;
        p = unsafe { p.add(4) };
    }

    counter -= 8 * 4;
    return find_ending(ss, counter);
}


fn find_u64x4_return(s: &str) -> usize
{
    let v = V64;
    let ss = s.as_bytes();
    let mut p = ss.as_ptr() as *const u64;
    let len = ss.len() - 8 * 4;
    

    let mut counter = 0;
    while counter < len
    {
        let t1 = unsafe { *p } ^ v;
        let t2 = unsafe { *p.add(1) } ^ v;
        let t3 = unsafe { *p.add(2) } ^ v;
        let t4 = unsafe { *p.add(3) } ^ v;

        if (t1 & 0x0000_0000_0000_00ff ) == 0 { return counter + 0; }
        if (t1 & 0x0000_0000_0000_ff00 ) == 0 { return counter + 1; }
        if (t1 & 0x0000_0000_00ff_0000 ) == 0 { return counter + 2; }
        if (t1 & 0x0000_0000_ff00_0000 ) == 0 { return counter + 3; }
 
        if (t1 & 0x0000_00ff_0000_0000 ) == 0 { return counter + 4; }
        if (t1 & 0x0000_ff00_0000_0000 ) == 0 { return counter + 5; }
        if (t1 & 0x00ff_0000_0000_0000 ) == 0 { return counter + 6; }
        if (t1 & 0xff00_0000_0000_0000 ) == 0 { return counter + 7; }

        if (t2 & 0x0000_0000_0000_00ff ) == 0 { return counter + 7; }
        if (t2 & 0x0000_0000_0000_ff00 ) == 0 { return counter + 8; }
        if (t2 & 0x0000_0000_00ff_0000 ) == 0 { return counter + 10; }
        if (t2 & 0x0000_0000_ff00_0000 ) == 0 { return counter + 11; }
 
        if (t2 & 0x0000_00ff_0000_0000 ) == 0 { return counter + 12; }
        if (t2 & 0x0000_ff00_0000_0000 ) == 0 { return counter + 13; }
        if (t2 & 0x00ff_0000_0000_0000 ) == 0 { return counter + 14; }
        if (t2 & 0xff00_0000_0000_0000 ) == 0 { return counter + 15; }

        if (t3 & 0x0000_0000_0000_00ff ) == 0 { return counter + 16; }
        if (t3 & 0x0000_0000_0000_ff00 ) == 0 { return counter + 17; }
        if (t3 & 0x0000_0000_00ff_0000 ) == 0 { return counter + 18; }
        if (t3 & 0x0000_0000_ff00_0000 ) == 0 { return counter + 19; }
 
        if (t3 & 0x0000_00ff_0000_0000 ) == 0 { return counter + 20; }
        if (t3 & 0x0000_ff00_0000_0000 ) == 0 { return counter + 21; }
        if (t3 & 0x00ff_0000_0000_0000 ) == 0 { return counter + 22; }
        if (t3 & 0xff00_0000_0000_0000 ) == 0 { return counter + 23; }

        if (t4 & 0x0000_0000_0000_00ff ) == 0 { return counter + 24; }
        if (t4 & 0x0000_0000_0000_ff00 ) == 0 { return counter + 25; }
        if (t4 & 0x0000_0000_00ff_0000 ) == 0 { return counter + 26; }
        if (t4 & 0x0000_0000_ff00_0000 ) == 0 { return counter + 27; }
 
        if (t4 & 0x0000_00ff_0000_0000 ) == 0 { return counter + 28; }
        if (t4 & 0x0000_ff00_0000_0000 ) == 0 { return counter + 29; }
        if (t4 & 0x00ff_0000_0000_0000 ) == 0 { return counter + 30; }
        if (t4 & 0xff00_0000_0000_0000 ) == 0 { return counter + 31; }

        counter += 8 * 4;
        p = unsafe { p.add(4) };
    }
    
    return len;
}

fn find_u64x4_return_test(s: &str) -> usize
{
    let v = V64;
    let ss = s.as_bytes();
    let mut p = ss.as_ptr() as *const u64;
    let len = ss.len() - 8 * 4;
    
    let mut find_test = false;
    let mut counter = 0;
    while counter < len && !find_test
    {
        let mut t1 = unsafe { *p };
        let mut t2 = unsafe { *p.add(1) };
        let mut t3 = unsafe { *p.add(2) };
        let mut t4 = unsafe { *p.add(3) };
 
        t1 ^= v;
        t2 ^= v;
        t3 ^= v;
        t4 ^= v;

        find_test = false;

        find_test |= (t1 & 0x0000_0000_0000_00ff ) == 0;
        find_test |= (t1 & 0x0000_0000_0000_ff00 ) == 0;
        find_test |= (t1 & 0x0000_0000_00ff_0000 ) == 0;
        find_test |= (t1 & 0x0000_0000_ff00_0000 ) == 0;
        find_test |= (t1 & 0x0000_00ff_0000_0000 ) == 0;
        find_test |= (t1 & 0x0000_ff00_0000_0000 ) == 0;
        find_test |= (t1 & 0x00ff_0000_0000_0000 ) == 0;
        find_test |= (t1 & 0xff00_0000_0000_0000 ) == 0;
        find_test |= (t2 & 0x0000_0000_0000_00ff ) == 0;
        find_test |= (t2 & 0x0000_0000_0000_ff00 ) == 0;
        find_test |= (t2 & 0x0000_0000_00ff_0000 ) == 0;
        find_test |= (t2 & 0x0000_0000_ff00_0000 ) == 0;
        find_test |= (t2 & 0x0000_00ff_0000_0000 ) == 0;
        find_test |= (t2 & 0x0000_ff00_0000_0000 ) == 0;
        find_test |= (t2 & 0x00ff_0000_0000_0000 ) == 0;
        find_test |= (t2 & 0xff00_0000_0000_0000 ) == 0;
        find_test |= (t3 & 0x0000_0000_0000_00ff ) == 0;
        find_test |= (t3 & 0x0000_0000_0000_ff00 ) == 0;
        find_test |= (t3 & 0x0000_0000_00ff_0000 ) == 0;
        find_test |= (t3 & 0x0000_0000_ff00_0000 ) == 0;
        find_test |= (t3 & 0x0000_00ff_0000_0000 ) == 0;
        find_test |= (t3 & 0x0000_ff00_0000_0000 ) == 0;
        find_test |= (t3 & 0x00ff_0000_0000_0000 ) == 0;
        find_test |= (t3 & 0xff00_0000_0000_0000 ) == 0;
        find_test |= (t4 & 0x0000_0000_0000_00ff ) == 0;
        find_test |= (t4 & 0x0000_0000_0000_ff00 ) == 0;
        find_test |= (t4 & 0x0000_0000_00ff_0000 ) == 0;
        find_test |= (t4 & 0x0000_0000_ff00_0000 ) == 0;
        find_test |= (t4 & 0x0000_00ff_0000_0000 ) == 0;
        find_test |= (t4 & 0x0000_ff00_0000_0000 ) == 0;
        find_test |= (t4 & 0x00ff_0000_0000_0000 ) == 0;
        find_test |= (t4 & 0xff00_0000_0000_0000 ) == 0;

        counter += 8 * 4;
        p = unsafe { p.add(4) };
    }
    counter -= 8 * 4;
    return find_ending(ss, counter);
}


fn find_u128_return(s: &str) -> usize
{
    let v = V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16;

    let mut p = ss.as_ptr() as *const u128;
    let mut counter = 0;
    while counter < len
    {
        let mut t = unsafe{ *p as u128 };
        t ^= v;

        if (t & 0x0000_0000_0000_0000_0000_0000_0000_00ff) == 0 { return counter + 0; }
        if (t & 0x0000_0000_0000_0000_0000_0000_0000_ff00) == 0 { return counter + 1; }
        if (t & 0x0000_0000_0000_0000_0000_0000_00ff_0000) == 0 { return counter + 2; }
        if (t & 0x0000_0000_0000_0000_0000_0000_ff00_0000) == 0 { return counter + 3; }
 
        if (t & 0x0000_0000_0000_0000_0000_00ff_0000_0000) == 0 { return counter + 4; }
        if (t & 0x0000_0000_0000_0000_0000_ff00_0000_0000) == 0 { return counter + 5; }
        if (t & 0x0000_0000_0000_0000_00ff_0000_0000_0000) == 0 { return counter + 6; }
        if (t & 0x0000_0000_0000_0000_ff00_0000_0000_0000) == 0 { return counter + 7; }
 
        
        if (t & 0x0000_0000_0000_00ff_0000_0000_0000_0000) == 0 { return counter + 8; }
        if (t & 0x0000_0000_0000_ff00_0000_0000_0000_0000) == 0 { return counter + 9; }
        if (t & 0x0000_0000_00ff_0000_0000_0000_0000_0000) == 0 { return counter + 10; }
        if (t & 0x0000_0000_ff00_0000_0000_0000_0000_0000) == 0 { return counter + 11; }

        if (t & 0x0000_00ff_0000_0000_0000_0000_0000_0000) == 0 { return counter + 12; }
        if (t & 0x0000_ff00_0000_0000_0000_0000_0000_0000) == 0 { return counter + 13; }
        if (t & 0x00ff_0000_0000_0000_0000_0000_0000_0000) == 0 { return counter + 14; }
        if (t & 0xff00_0000_0000_0000_0000_0000_0000_0000) == 0 { return counter + 15; }

        counter += 16;
        p = unsafe { p.add(1) };
    }

    return find_ending(ss, counter);
}


fn find_u128x2_return(s: &str) -> usize
{
    let v = V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 2;

    let mut p = ss.as_ptr() as *const u128;
    let mut counter = 0;
    while counter < len
    {
        let mut t1 = unsafe{ *p };
        let mut t2 = unsafe{ *p.add(1) };
        t1 ^= v;
        t2 ^= v;

        if (t1 & 0x0000_0000_0000_0000_0000_0000_0000_00ff) == 0 { return counter + 0; }
        if (t1 & 0x0000_0000_0000_0000_0000_0000_0000_ff00) == 0 { return counter + 1; }
        if (t1 & 0x0000_0000_0000_0000_0000_0000_00ff_0000) == 0 { return counter + 2; }
        if (t1 & 0x0000_0000_0000_0000_0000_0000_ff00_0000) == 0 { return counter + 3; }
        if (t1 & 0x0000_0000_0000_0000_0000_00ff_0000_0000) == 0 { return counter + 4; }
        if (t1 & 0x0000_0000_0000_0000_0000_ff00_0000_0000) == 0 { return counter + 5; }
        if (t1 & 0x0000_0000_0000_0000_00ff_0000_0000_0000) == 0 { return counter + 6; }
        if (t1 & 0x0000_0000_0000_0000_ff00_0000_0000_0000) == 0 { return counter + 7; }
        if (t1 & 0x0000_0000_0000_00ff_0000_0000_0000_0000) == 0 { return counter + 8; }
        if (t1 & 0x0000_0000_0000_ff00_0000_0000_0000_0000) == 0 { return counter + 9; }
        if (t1 & 0x0000_0000_00ff_0000_0000_0000_0000_0000) == 0 { return counter + 10; }
        if (t1 & 0x0000_0000_ff00_0000_0000_0000_0000_0000) == 0 { return counter + 11; }
        if (t1 & 0x0000_00ff_0000_0000_0000_0000_0000_0000) == 0 { return counter + 12; }
        if (t1 & 0x0000_ff00_0000_0000_0000_0000_0000_0000) == 0 { return counter + 13; }
        if (t1 & 0x00ff_0000_0000_0000_0000_0000_0000_0000) == 0 { return counter + 14; }
        if (t1 & 0xff00_0000_0000_0000_0000_0000_0000_0000) == 0 { return counter + 15; }

        if (t2 & 0x0000_0000_0000_0000_0000_0000_0000_00ff) == 0 { return counter + 16; }
        if (t2 & 0x0000_0000_0000_0000_0000_0000_0000_ff00) == 0 { return counter + 17; }
        if (t2 & 0x0000_0000_0000_0000_0000_0000_00ff_0000) == 0 { return counter + 18; }
        if (t2 & 0x0000_0000_0000_0000_0000_0000_ff00_0000) == 0 { return counter + 19; }
        if (t2 & 0x0000_0000_0000_0000_0000_00ff_0000_0000) == 0 { return counter + 20; }
        if (t2 & 0x0000_0000_0000_0000_0000_ff00_0000_0000) == 0 { return counter + 21; }
        if (t2 & 0x0000_0000_0000_0000_00ff_0000_0000_0000) == 0 { return counter + 22; }
        if (t2 & 0x0000_0000_0000_0000_ff00_0000_0000_0000) == 0 { return counter + 23; }
        if (t2 & 0x0000_0000_0000_00ff_0000_0000_0000_0000) == 0 { return counter + 24; }
        if (t2 & 0x0000_0000_0000_ff00_0000_0000_0000_0000) == 0 { return counter + 25; }
        if (t2 & 0x0000_0000_00ff_0000_0000_0000_0000_0000) == 0 { return counter + 26; }
        if (t2 & 0x0000_0000_ff00_0000_0000_0000_0000_0000) == 0 { return counter + 27; }
        if (t2 & 0x0000_00ff_0000_0000_0000_0000_0000_0000) == 0 { return counter + 28; }
        if (t2 & 0x0000_ff00_0000_0000_0000_0000_0000_0000) == 0 { return counter + 29; }
        if (t2 & 0x00ff_0000_0000_0000_0000_0000_0000_0000) == 0 { return counter + 30; }
        if (t2 & 0xff00_0000_0000_0000_0000_0000_0000_0000) == 0 { return counter + 31; }

        counter += 16 * 2;
        p = unsafe { p.add(2) };
    }
    
    return len;
}



fn find_u32_rotate_7(s: &str) -> usize
{
    // take invert
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4;
    
    let mut counter = 0;
    let mut u = 0u32;
    let mut p = ss.as_ptr() as *const u32;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;

        u = t;
        u &=
        (((u & 0xfefe_fefe) >> 1) | ((u & 0x0101_0101) << 7)) &
        (((u & 0xfcfc_fcfc) >> 2) | ((u & 0x0303_0303) << 6)) &
        (((u & 0xf8f8_f8f8) >> 3) | ((u & 0x0707_0707) << 5)) &

        (((u & 0xf0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f) << 4)) &
        (((u & 0xe0e0_e0e0) >> 5) | ((u & 0x1f1f_1f1f) << 3)) &
        (((u & 0xc0c0_c0c0) >> 6) | ((u & 0x3f3f_3f3f) << 2)) &

        (((u & 0x8080_8080) >> 7) | ((u & 0x7f7f_7f7f) << 1));

        counter += 4;
        p = unsafe { p.add(1) };
    }
    counter = counter - 4;
    return find_ending(ss, counter);
}


fn find_u64_rotate_7(s: &str) -> usize
{
    // take invert
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8;
    
    let mut counter = 0;
    let mut u = 0u64;
    let mut p = ss.as_ptr() as *const u64;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;

        u = t;
        u &=
        (((u & 0xfefe_fefe_fefe_fefe) >> 1) | ((u & 0x0101_0101_0101_0101) << 7)) &
        (((u & 0xfcfc_fcfc_fcfc_fcfc) >> 2) | ((u & 0x0303_0303_0303_0303) << 6)) &
        (((u & 0xf8f8_f8f8_f8f8_f8f8) >> 3) | ((u & 0x0707_0707_0707_0707) << 5)) &

        (((u & 0xf0f0_f0f0_f0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f_0f0f_0f0f) << 4)) &
        (((u & 0xe0e0_e0e0_e0e0_e0e0) >> 5) | ((u & 0x1f1f_1f1f_1f1f_1f1f) << 3)) &
        (((u & 0xc0c0_c0c0_c0c0_c0c0) >> 6) | ((u & 0x3f3f_3f3f_3f3f_3f3f) << 2)) &

        (((u & 0x8080_8080_8080_8080) >> 7) | ((u & 0x7f7f_7f7f_7f7f_7f7f) << 1));

        counter += 8;
        p = unsafe { p.add(1) };
    }
    counter = counter - 8;
    return find_ending(ss, counter);
}


fn find_u128_rotate_7(s: &str) -> usize
{
    // take invert
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16;
    
    let mut counter = 0;
    let mut u = 0u128;
    let mut p = ss.as_ptr() as *const u128;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;

        u = t;
        u &=
        (((u & 0xfefe_fefe_fefe_fefe_fefe_fefe_fefe_fefe) >> 1) | ((u & 0x0101_0101_0101_0101_0101_0101_0101_0101) << 7)) &
        (((u & 0xfcfc_fcfc_fcfc_fcfc_fcfc_fcfc_fcfc_fcfc) >> 2) | ((u & 0x0303_0303_0303_0303_0303_0303_0303_0303) << 6)) &
        (((u & 0xf8f8_f8f8_f8f8_f8f8_f8f8_f8f8_f8f8_f8f8) >> 3) | ((u & 0x0707_0707_0707_0707_0707_0707_0707_0707) << 5)) &

        (((u & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f) << 4)) &
        (((u & 0xe0e0_e0e0_e0e0_e0e0_e0e0_e0e0_e0e0_e0e0) >> 5) | ((u & 0x1f1f_1f1f_1f1f_1f1f_1f1f_1f1f_1f1f_1f1f) << 3)) &
        (((u & 0xc0c0_c0c0_c0c0_c0c0_c0c0_c0c0_c0c0_c0c0) >> 6) | ((u & 0x3f3f_3f3f_3f3f_3f3f_3f3f_3f3f_3f3f_3f3f) << 2)) &

        (((u & 0x8080_8080_8080_8080_8080_8080_8080_8080) >> 7) | ((u & 0x7f7f_7f7f_7f7f_7f7f_7f7f_7f7f_7f7f_7f7f) << 1));

        counter += 16;
        p = unsafe { p.add(1) };
    }
    counter = counter - 16;
    return find_ending(ss, counter);
}


fn find_u32_rotate_3(s: &str) -> usize
{
    // take invert so that xor with value will be all ones
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4;
    
    let mut counter = 0;
    let mut u = 0u32;
    let mut p = ss.as_ptr() as *const u32;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;
 
        u = t;
        u &= ((u & 0xf0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f) << 4);
        u &= ((u & 0xcccc_cccc) >> 2) | ((u & 0x3333_3333) << 2);
        u &= ((u & 0xaaaa_aaaa) >> 1) | ((u & 0x5555_5555) << 1);

        counter += 4;
        p = unsafe { p.add(1) };
    }
    counter = counter - 4;
    return find_ending(ss, counter);
}

fn find_u64_rotate_3(s: &str) -> usize
{
    // take invert
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8;
    
    let mut counter = 0;
    let mut u = 0u64;
    let mut p = ss.as_ptr() as *const u64;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;

        u = t;
        u &= ((u & 0xf0f0_f0f0_f0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f_0f0f_0f0f) << 4);
        u &= ((u & 0xcccc_cccc_cccc_cccc) >> 2) | ((u & 0x3333_3333_3333_3333) << 2);
        u &= ((u & 0xaaaa_aaaa_aaaa_aaaa) >> 1) | ((u & 0x5555_5555_5555_5555) << 1);

        counter += 8;
        p = unsafe { p.add(1) };
    }
    counter = counter - 8;
    return find_ending(ss, counter);
}

fn find_u128_rotate_3(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16;
    
    let mut counter = 0;
    let mut u = 0u128;
    let mut p = ss.as_ptr() as *const u128;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;

        u = t;
        // first mask 4 bits, for 4 bit swap, then 2 bits for every 2 bit sequence swap and finally one bit swapping next to each other.
        // In case the one or another is 0, the & operation will 'copy' the 0 to the other one and thus the 0 will 'spread'.
        u &= ((u & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f) << 4);
        u &= ((u & 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc) >> 2) | ((u & 0x3333_3333_3333_3333_3333_3333_3333_3333) << 2);
        u &= ((u & 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa) >> 1) | ((u & 0x5555_5555_5555_5555_5555_5555_5555_5555) << 1);

        counter += 16;
        p = unsafe { p.add(1) };
    }
    counter = counter - 16;
    return find_ending(ss, counter);
}




fn find_u32x2_rotate_3(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 2;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0;
    let u1 = 0xcccc_cccc;
    let u2 = 0xaaaa_aaaa;

    let u3 = 0x0f0f_0f0f;
    let u4 = 0x3333_3333;
    let u5 = 0x5555_5555;


    let mut p = ss.as_ptr() as *const u32;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        if (t1 | t2) != 0
        {
            break;
        }        

        counter += 4 * 2;
        p = unsafe { p.add(2) };
    }

    return find_ending(ss, counter);
}

fn find_u64x2_rotate_3(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u64;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        if (t1 | t2) != 0
        {
            break;
        }        

        counter += 8 * 2;
        p = unsafe { p.add(2) };
    }

    return find_ending(ss, counter);
}

fn find_u128x2_rotate_3(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 2;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555_5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u128;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        if (t1 | t2) != 0
        {
            break;
        }        

        counter += 16 * 2;
        p = unsafe { p.add(2) };
    }

    return find_ending(ss, counter);
}



fn find_u32x4_rotate_3(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0;
    let u1 = 0xcccc_cccc;
    let u2 = 0xaaaa_aaaa;

    let u3 = 0x0f0f_0f0f;
    let u4 = 0x3333_3333;
    let u5 = 0x5555_5555;


    let mut p = ss.as_ptr() as *const u32;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        if (t1 | t2 | t3 | t4) != 0
        {
            break;
        }        

        counter += 4 * 4;
        p = unsafe { p.add(4) };
    }

    return find_ending(ss, counter);
}

fn find_u64x4_rotate_3(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u64;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        if (t1 | t2 | t3 | t4) != 0
        {
            break;
        }        

        counter += 8 * 4;
        p = unsafe { p.add(4) };
    }

    return find_ending(ss, counter);
}

fn find_u128x4_rotate_3(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555_5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u128;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);


        if (t1 | t2 | t3 | t4) != 0
        {
            break;
        }        

        counter += 16 * 4;
        p = unsafe { p.add(4) };
    }

    return find_ending(ss, counter);
}



fn find_u32x8_rotate_3(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0;
    let u1 = 0xcccc_cccc;
    let u2 = 0xaaaa_aaaa;

    let u3 = 0x0f0f_0f0f;
    let u4 = 0x3333_3333;
    let u5 = 0x5555_5555;


    let mut p = ss.as_ptr() as *const u32;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);


        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8) != 0
        {
            break;
        }        

        counter += 4 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}

fn find_u64x8_rotate_3(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u64;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);


        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8) != 0
        {
            break;
        }        

        counter += 8 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}

fn find_u128x8_rotate_3(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555_5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u128;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);


        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8) != 0
        {
            break;
        }        

        counter += 16 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}


fn find_u32x16_rotate_3(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 16;

    let u0 = 0xF0F0_F0F0u32;
    let u1 = 0xCCCC_CCCCu32;
    let u2 = 0xAAAA_AAAAu32;

    let u3 = 0x0F0F_0F0Fu32;
    let u4 = 0x3333_3333u32;
    let u5 = 0x5555_5555u32;

    let mut p = ss.as_ptr() as *const u32;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);


        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);


        r1 &= ((r1 & u0) >> 4) | ((r1 & u3) << 4);
        r1 &= ((r1 & u1) >> 2) | ((r1 & u4) << 2);
        r1 &= ((r1 & u2) >> 1) | ((r1 & u5) << 1);

        r2 &= ((r2 & u0) >> 4) | ((r2 & u3) << 4);
        r2 &= ((r2 & u1) >> 2) | ((r2 & u4) << 2);
        r2 &= ((r2 & u2) >> 1) | ((r2 & u5) << 1);

        r3 &= ((r3 & u0) >> 4) | ((r3 & u3) << 4);
        r3 &= ((r3 & u1) >> 2) | ((r3 & u4) << 2);
        r3 &= ((r3 & u2) >> 1) | ((r3 & u5) << 1);

        r4 &= ((r4 & u0) >> 4) | ((r4 & u3) << 4);
        r4 &= ((r4 & u1) >> 2) | ((r4 & u4) << 2);
        r4 &= ((r4 & u2) >> 1) | ((r4 & u5) << 1);

        r5 &= ((r5 & u0) >> 4) | ((r5 & u3) << 4);
        r5 &= ((r5 & u1) >> 2) | ((r5 & u4) << 2);
        r5 &= ((r5 & u2) >> 1) | ((r5 & u5) << 1);

        r6 &= ((r6 & u0) >> 4) | ((r6 & u3) << 4);
        r6 &= ((r6 & u1) >> 2) | ((r6 & u4) << 2);
        r6 &= ((r6 & u2) >> 1) | ((r6 & u5) << 1);

        r7 &= ((r7 & u0) >> 4) | ((r7 & u3) << 4);
        r7 &= ((r7 & u1) >> 2) | ((r7 & u4) << 2);
        r7 &= ((r7 & u2) >> 1) | ((r7 & u5) << 1);

        r8 &= ((r8 & u0) >> 4) | ((r8 & u3) << 4);
        r8 &= ((r8 & u1) >> 2) | ((r8 & u4) << 2);
        r8 &= ((r8 & u2) >> 1) | ((r8 & u5) << 1);

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8) != 0
           {
               break;
           }

        p = unsafe { p.add(16) };

        counter += 4 * 16;
    }

    return find_ending(ss, counter);
}

fn find_u64x16_rotate_3(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 16;

    let u0 = 0xF0F0_F0F0_F0F0_F0F0;
    let u1 = 0xCCCC_CCCC_CCCC_CCCC;
    let u2 = 0xAAAA_AAAA_AAAA_AAAA;

    let u3 = 0x0F0F_0F0F_0F0F_0F0F;
    let u4 = 0x3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555;

    let mut p = ss.as_ptr() as *const u64;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);


        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);


        r1 &= ((r1 & u0) >> 4) | ((r1 & u3) << 4);
        r1 &= ((r1 & u1) >> 2) | ((r1 & u4) << 2);
        r1 &= ((r1 & u2) >> 1) | ((r1 & u5) << 1);

        r2 &= ((r2 & u0) >> 4) | ((r2 & u3) << 4);
        r2 &= ((r2 & u1) >> 2) | ((r2 & u4) << 2);
        r2 &= ((r2 & u2) >> 1) | ((r2 & u5) << 1);

        r3 &= ((r3 & u0) >> 4) | ((r3 & u3) << 4);
        r3 &= ((r3 & u1) >> 2) | ((r3 & u4) << 2);
        r3 &= ((r3 & u2) >> 1) | ((r3 & u5) << 1);

        r4 &= ((r4 & u0) >> 4) | ((r4 & u3) << 4);
        r4 &= ((r4 & u1) >> 2) | ((r4 & u4) << 2);
        r4 &= ((r4 & u2) >> 1) | ((r4 & u5) << 1);

        r5 &= ((r5 & u0) >> 4) | ((r5 & u3) << 4);
        r5 &= ((r5 & u1) >> 2) | ((r5 & u4) << 2);
        r5 &= ((r5 & u2) >> 1) | ((r5 & u5) << 1);

        r6 &= ((r6 & u0) >> 4) | ((r6 & u3) << 4);
        r6 &= ((r6 & u1) >> 2) | ((r6 & u4) << 2);
        r6 &= ((r6 & u2) >> 1) | ((r6 & u5) << 1);

        r7 &= ((r7 & u0) >> 4) | ((r7 & u3) << 4);
        r7 &= ((r7 & u1) >> 2) | ((r7 & u4) << 2);
        r7 &= ((r7 & u2) >> 1) | ((r7 & u5) << 1);

        r8 &= ((r8 & u0) >> 4) | ((r8 & u3) << 4);
        r8 &= ((r8 & u1) >> 2) | ((r8 & u4) << 2);
        r8 &= ((r8 & u2) >> 1) | ((r8 & u5) << 1);

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8) != 0
           {
               break;
           }

        p = unsafe { p.add(16) };

        counter += 8 * 16;
    }

    return find_ending(ss, counter);
}

fn find_u128x16_rotate_3(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 16;

    let u0 = 0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0;
    let u1 = 0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC;
    let u2 = 0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA;

    let u3 = 0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F;
    let u4 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555_5555_5555_5555_5555;

    let mut p = ss.as_ptr() as *const u128;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);


        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);


        r1 &= ((r1 & u0) >> 4) | ((r1 & u3) << 4);
        r1 &= ((r1 & u1) >> 2) | ((r1 & u4) << 2);
        r1 &= ((r1 & u2) >> 1) | ((r1 & u5) << 1);

        r2 &= ((r2 & u0) >> 4) | ((r2 & u3) << 4);
        r2 &= ((r2 & u1) >> 2) | ((r2 & u4) << 2);
        r2 &= ((r2 & u2) >> 1) | ((r2 & u5) << 1);

        r3 &= ((r3 & u0) >> 4) | ((r3 & u3) << 4);
        r3 &= ((r3 & u1) >> 2) | ((r3 & u4) << 2);
        r3 &= ((r3 & u2) >> 1) | ((r3 & u5) << 1);

        r4 &= ((r4 & u0) >> 4) | ((r4 & u3) << 4);
        r4 &= ((r4 & u1) >> 2) | ((r4 & u4) << 2);
        r4 &= ((r4 & u2) >> 1) | ((r4 & u5) << 1);

        r5 &= ((r5 & u0) >> 4) | ((r5 & u3) << 4);
        r5 &= ((r5 & u1) >> 2) | ((r5 & u4) << 2);
        r5 &= ((r5 & u2) >> 1) | ((r5 & u5) << 1);

        r6 &= ((r6 & u0) >> 4) | ((r6 & u3) << 4);
        r6 &= ((r6 & u1) >> 2) | ((r6 & u4) << 2);
        r6 &= ((r6 & u2) >> 1) | ((r6 & u5) << 1);

        r7 &= ((r7 & u0) >> 4) | ((r7 & u3) << 4);
        r7 &= ((r7 & u1) >> 2) | ((r7 & u4) << 2);
        r7 &= ((r7 & u2) >> 1) | ((r7 & u5) << 1);

        r8 &= ((r8 & u0) >> 4) | ((r8 & u3) << 4);
        r8 &= ((r8 & u1) >> 2) | ((r8 & u4) << 2);
        r8 &= ((r8 & u2) >> 1) | ((r8 & u5) << 1);

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8) != 0
           {
               break;
           }

        p = unsafe { p.add(16) };

        counter += 16 * 16;
    }

    return find_ending(ss, counter);
}








/*
fn find_u32x32_rotate_3(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 32;

    let u0 = 0xF0F0_F0F0;
    let u1 = 0xCCCC_CCCC;
    let u2 = 0xAAAA_AAAA;

    let u3 = 0x0F0F_0F0F;
    let u4 = 0x3333_3333;
    let u5 = 0x5555_5555;

    let mut p = ss.as_ptr() as *const u32;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);

        let mut q1 = (unsafe{ *p.offset(16) } ^ v);
        let mut q2 = (unsafe{ *p.offset(17) } ^ v);
        let mut q3 = (unsafe{ *p.offset(18) } ^ v);
        let mut q4 = (unsafe{ *p.offset(19) } ^ v);

        let mut q5 = (unsafe{ *p.offset(20) } ^ v);
        let mut q6 = (unsafe{ *p.offset(21) } ^ v);
        let mut q7 = (unsafe{ *p.offset(22) } ^ v);
        let mut q8 = (unsafe{ *p.offset(23) } ^ v);

        let mut w1 = (unsafe{ *p.offset(24) } ^ v);
        let mut w2 = (unsafe{ *p.offset(25) } ^ v);
        let mut w3 = (unsafe{ *p.offset(26) } ^ v);
        let mut w4 = (unsafe{ *p.offset(27) } ^ v);

        let mut w5 = (unsafe{ *p.offset(28) } ^ v);
        let mut w6 = (unsafe{ *p.offset(29) } ^ v);
        let mut w7 = (unsafe{ *p.offset(30) } ^ v);
        let mut w8 = (unsafe{ *p.offset(31) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);

        r1 &= ((r1 & u0) >> 4) | ((r1 & u3) << 4);
        r1 &= ((r1 & u1) >> 2) | ((r1 & u4) << 2);
        r1 &= ((r1 & u2) >> 1) | ((r1 & u5) << 1);

        r2 &= ((r2 & u0) >> 4) | ((r2 & u3) << 4);
        r2 &= ((r2 & u1) >> 2) | ((r2 & u4) << 2);
        r2 &= ((r2 & u2) >> 1) | ((r2 & u5) << 1);

        r3 &= ((r3 & u0) >> 4) | ((r3 & u3) << 4);
        r3 &= ((r3 & u1) >> 2) | ((r3 & u4) << 2);
        r3 &= ((r3 & u2) >> 1) | ((r3 & u5) << 1);

        r4 &= ((r4 & u0) >> 4) | ((r4 & u3) << 4);
        r4 &= ((r4 & u1) >> 2) | ((r4 & u4) << 2);
        r4 &= ((r4 & u2) >> 1) | ((r4 & u5) << 1);

        r5 &= ((r5 & u0) >> 4) | ((r5 & u3) << 4);
        r5 &= ((r5 & u1) >> 2) | ((r5 & u4) << 2);
        r5 &= ((r5 & u2) >> 1) | ((r5 & u5) << 1);

        r6 &= ((r6 & u0) >> 4) | ((r6 & u3) << 4);
        r6 &= ((r6 & u1) >> 2) | ((r6 & u4) << 2);
        r6 &= ((r6 & u2) >> 1) | ((r6 & u5) << 1);

        r7 &= ((r7 & u0) >> 4) | ((r7 & u3) << 4);
        r7 &= ((r7 & u1) >> 2) | ((r7 & u4) << 2);
        r7 &= ((r7 & u2) >> 1) | ((r7 & u5) << 1);

        r8 &= ((r8 & u0) >> 4) | ((r8 & u3) << 4);
        r8 &= ((r8 & u1) >> 2) | ((r8 & u4) << 2);
        r8 &= ((r8 & u2) >> 1) | ((r8 & u5) << 1);

        q1 &= ((q1 & u0) >> 4) | ((q1 & u3) << 4);
        q1 &= ((q1 & u1) >> 2) | ((q1 & u4) << 2);
        q1 &= ((q1 & u2) >> 1) | ((q1 & u5) << 1);

        q2 &= ((q2 & u0) >> 4) | ((q2 & u3) << 4);
        q2 &= ((q2 & u1) >> 2) | ((q2 & u4) << 2);
        q2 &= ((q2 & u2) >> 1) | ((q2 & u5) << 1);

        q3 &= ((q3 & u0) >> 4) | ((q3 & u3) << 4);
        q3 &= ((q3 & u1) >> 2) | ((q3 & u4) << 2);
        q3 &= ((q3 & u2) >> 1) | ((q3 & u5) << 1);

        q4 &= ((q4 & u0) >> 4) | ((q4 & u3) << 4);
        q4 &= ((q4 & u1) >> 2) | ((q4 & u4) << 2);
        q4 &= ((q4 & u2) >> 1) | ((q4 & u5) << 1);

        q5 &= ((q5 & u0) >> 4) | ((q5 & u3) << 4);
        q5 &= ((q5 & u1) >> 2) | ((q5 & u4) << 2);
        q5 &= ((q5 & u2) >> 1) | ((q5 & u5) << 1);

        q6 &= ((q6 & u0) >> 4) | ((q6 & u3) << 4);
        q6 &= ((q6 & u1) >> 2) | ((q6 & u4) << 2);
        q6 &= ((q6 & u2) >> 1) | ((q6 & u5) << 1);

        q7 &= ((q7 & u0) >> 4) | ((q7 & u3) << 4);
        q7 &= ((q7 & u1) >> 2) | ((q7 & u4) << 2);
        q7 &= ((q7 & u2) >> 1) | ((q7 & u5) << 1);

        q8 &= ((q8 & u0) >> 4) | ((q8 & u3) << 4);
        q8 &= ((q8 & u1) >> 2) | ((q8 & u4) << 2);
        q8 &= ((q8 & u2) >> 1) | ((q8 & u5) << 1);

        w1 &= ((w1 & u0) >> 4) | ((w1 & u3) << 4);
        w1 &= ((w1 & u1) >> 2) | ((w1 & u4) << 2);
        w1 &= ((w1 & u2) >> 1) | ((w1 & u5) << 1);

        w2 &= ((w2 & u0) >> 4) | ((w2 & u3) << 4);
        w2 &= ((w2 & u1) >> 2) | ((w2 & u4) << 2);
        w2 &= ((w2 & u2) >> 1) | ((w2 & u5) << 1);

        w3 &= ((w3 & u0) >> 4) | ((w3 & u3) << 4);
        w3 &= ((w3 & u1) >> 2) | ((w3 & u4) << 2);
        w3 &= ((w3 & u2) >> 1) | ((w3 & u5) << 1);

        w4 &= ((w4 & u0) >> 4) | ((w4 & u3) << 4);
        w4 &= ((w4 & u1) >> 2) | ((w4 & u4) << 2);
        w4 &= ((w4 & u2) >> 1) | ((w4 & u5) << 1);

        w5 &= ((w5 & u0) >> 4) | ((w5 & u3) << 4);
        w5 &= ((w5 & u1) >> 2) | ((w5 & u4) << 2);
        w5 &= ((w5 & u2) >> 1) | ((w5 & u5) << 1);

        w6 &= ((w6 & u0) >> 4) | ((w6 & u3) << 4);
        w6 &= ((w6 & u1) >> 2) | ((w6 & u4) << 2);
        w6 &= ((w6 & u2) >> 1) | ((w6 & u5) << 1);

        w7 &= ((w7 & u0) >> 4) | ((w7 & u3) << 4);
        w7 &= ((w7 & u1) >> 2) | ((w7 & u4) << 2);
        w7 &= ((w7 & u2) >> 1) | ((w7 & u5) << 1);

        w8 &= ((w8 & u0) >> 4) | ((w8 & u3) << 4);
        w8 &= ((w8 & u1) >> 2) | ((w8 & u4) << 2);
        w8 &= ((w8 & u2) >> 1) | ((w8 & u5) << 1);

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 |
           q1 | q2 | q3 | q4 | q5 | q6 | q7 | q8 |
           w1 | w2 | w3 | w4 | w5 | w6 | w7 | w8) != 0
           {
               break;
           }

        p = unsafe { p.add(32) };

        counter += 4 * 32;
    }

    return find_ending(ss, counter);
}

fn find_u64x32_rotate_3(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 32;

    let u0 = 0xF0F0_F0F0_F0F0_F0F0;
    let u1 = 0xCCCC_CCCC_CCCC_CCCC;
    let u2 = 0xAAAA_AAAA_AAAA_AAAA;

    let u3 = 0x0F0F_0F0F_0F0F_0F0F;
    let u4 = 0x3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555;

    let mut p = ss.as_ptr() as *const u64;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);

        let mut q1 = (unsafe{ *p.offset(16) } ^ v);
        let mut q2 = (unsafe{ *p.offset(17) } ^ v);
        let mut q3 = (unsafe{ *p.offset(18) } ^ v);
        let mut q4 = (unsafe{ *p.offset(19) } ^ v);

        let mut q5 = (unsafe{ *p.offset(20) } ^ v);
        let mut q6 = (unsafe{ *p.offset(21) } ^ v);
        let mut q7 = (unsafe{ *p.offset(22) } ^ v);
        let mut q8 = (unsafe{ *p.offset(23) } ^ v);

        let mut w1 = (unsafe{ *p.offset(24) } ^ v);
        let mut w2 = (unsafe{ *p.offset(25) } ^ v);
        let mut w3 = (unsafe{ *p.offset(26) } ^ v);
        let mut w4 = (unsafe{ *p.offset(27) } ^ v);

        let mut w5 = (unsafe{ *p.offset(28) } ^ v);
        let mut w6 = (unsafe{ *p.offset(29) } ^ v);
        let mut w7 = (unsafe{ *p.offset(30) } ^ v);
        let mut w8 = (unsafe{ *p.offset(31) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);

        r1 &= ((r1 & u0) >> 4) | ((r1 & u3) << 4);
        r1 &= ((r1 & u1) >> 2) | ((r1 & u4) << 2);
        r1 &= ((r1 & u2) >> 1) | ((r1 & u5) << 1);

        r2 &= ((r2 & u0) >> 4) | ((r2 & u3) << 4);
        r2 &= ((r2 & u1) >> 2) | ((r2 & u4) << 2);
        r2 &= ((r2 & u2) >> 1) | ((r2 & u5) << 1);

        r3 &= ((r3 & u0) >> 4) | ((r3 & u3) << 4);
        r3 &= ((r3 & u1) >> 2) | ((r3 & u4) << 2);
        r3 &= ((r3 & u2) >> 1) | ((r3 & u5) << 1);

        r4 &= ((r4 & u0) >> 4) | ((r4 & u3) << 4);
        r4 &= ((r4 & u1) >> 2) | ((r4 & u4) << 2);
        r4 &= ((r4 & u2) >> 1) | ((r4 & u5) << 1);

        r5 &= ((r5 & u0) >> 4) | ((r5 & u3) << 4);
        r5 &= ((r5 & u1) >> 2) | ((r5 & u4) << 2);
        r5 &= ((r5 & u2) >> 1) | ((r5 & u5) << 1);

        r6 &= ((r6 & u0) >> 4) | ((r6 & u3) << 4);
        r6 &= ((r6 & u1) >> 2) | ((r6 & u4) << 2);
        r6 &= ((r6 & u2) >> 1) | ((r6 & u5) << 1);

        r7 &= ((r7 & u0) >> 4) | ((r7 & u3) << 4);
        r7 &= ((r7 & u1) >> 2) | ((r7 & u4) << 2);
        r7 &= ((r7 & u2) >> 1) | ((r7 & u5) << 1);

        r8 &= ((r8 & u0) >> 4) | ((r8 & u3) << 4);
        r8 &= ((r8 & u1) >> 2) | ((r8 & u4) << 2);
        r8 &= ((r8 & u2) >> 1) | ((r8 & u5) << 1);

        q1 &= ((q1 & u0) >> 4) | ((q1 & u3) << 4);
        q1 &= ((q1 & u1) >> 2) | ((q1 & u4) << 2);
        q1 &= ((q1 & u2) >> 1) | ((q1 & u5) << 1);

        q2 &= ((q2 & u0) >> 4) | ((q2 & u3) << 4);
        q2 &= ((q2 & u1) >> 2) | ((q2 & u4) << 2);
        q2 &= ((q2 & u2) >> 1) | ((q2 & u5) << 1);

        q3 &= ((q3 & u0) >> 4) | ((q3 & u3) << 4);
        q3 &= ((q3 & u1) >> 2) | ((q3 & u4) << 2);
        q3 &= ((q3 & u2) >> 1) | ((q3 & u5) << 1);

        q4 &= ((q4 & u0) >> 4) | ((q4 & u3) << 4);
        q4 &= ((q4 & u1) >> 2) | ((q4 & u4) << 2);
        q4 &= ((q4 & u2) >> 1) | ((q4 & u5) << 1);

        q5 &= ((q5 & u0) >> 4) | ((q5 & u3) << 4);
        q5 &= ((q5 & u1) >> 2) | ((q5 & u4) << 2);
        q5 &= ((q5 & u2) >> 1) | ((q5 & u5) << 1);

        q6 &= ((q6 & u0) >> 4) | ((q6 & u3) << 4);
        q6 &= ((q6 & u1) >> 2) | ((q6 & u4) << 2);
        q6 &= ((q6 & u2) >> 1) | ((q6 & u5) << 1);

        q7 &= ((q7 & u0) >> 4) | ((q7 & u3) << 4);
        q7 &= ((q7 & u1) >> 2) | ((q7 & u4) << 2);
        q7 &= ((q7 & u2) >> 1) | ((q7 & u5) << 1);

        q8 &= ((q8 & u0) >> 4) | ((q8 & u3) << 4);
        q8 &= ((q8 & u1) >> 2) | ((q8 & u4) << 2);
        q8 &= ((q8 & u2) >> 1) | ((q8 & u5) << 1);

        w1 &= ((w1 & u0) >> 4) | ((w1 & u3) << 4);
        w1 &= ((w1 & u1) >> 2) | ((w1 & u4) << 2);
        w1 &= ((w1 & u2) >> 1) | ((w1 & u5) << 1);

        w2 &= ((w2 & u0) >> 4) | ((w2 & u3) << 4);
        w2 &= ((w2 & u1) >> 2) | ((w2 & u4) << 2);
        w2 &= ((w2 & u2) >> 1) | ((w2 & u5) << 1);

        w3 &= ((w3 & u0) >> 4) | ((w3 & u3) << 4);
        w3 &= ((w3 & u1) >> 2) | ((w3 & u4) << 2);
        w3 &= ((w3 & u2) >> 1) | ((w3 & u5) << 1);

        w4 &= ((w4 & u0) >> 4) | ((w4 & u3) << 4);
        w4 &= ((w4 & u1) >> 2) | ((w4 & u4) << 2);
        w4 &= ((w4 & u2) >> 1) | ((w4 & u5) << 1);

        w5 &= ((w5 & u0) >> 4) | ((w5 & u3) << 4);
        w5 &= ((w5 & u1) >> 2) | ((w5 & u4) << 2);
        w5 &= ((w5 & u2) >> 1) | ((w5 & u5) << 1);

        w6 &= ((w6 & u0) >> 4) | ((w6 & u3) << 4);
        w6 &= ((w6 & u1) >> 2) | ((w6 & u4) << 2);
        w6 &= ((w6 & u2) >> 1) | ((w6 & u5) << 1);

        w7 &= ((w7 & u0) >> 4) | ((w7 & u3) << 4);
        w7 &= ((w7 & u1) >> 2) | ((w7 & u4) << 2);
        w7 &= ((w7 & u2) >> 1) | ((w7 & u5) << 1);

        w8 &= ((w8 & u0) >> 4) | ((w8 & u3) << 4);
        w8 &= ((w8 & u1) >> 2) | ((w8 & u4) << 2);
        w8 &= ((w8 & u2) >> 1) | ((w8 & u5) << 1);

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 |
           q1 | q2 | q3 | q4 | q5 | q6 | q7 | q8 |
           w1 | w2 | w3 | w4 | w5 | w6 | w7 | w8) != 0
           {
               break;
           }

        p = unsafe { p.add(32) };

        counter += 8 * 32;
    }

    return find_ending(ss, counter);
}


fn find_u128x32_rotate_3(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 32;

    let u0 = 0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0;
    let u1 = 0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC;
    let u2 = 0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA;

    let u3 = 0x0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F_0F0F;
    let u4 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555_5555_5555_5555_5555;

    let mut p = ss.as_ptr() as *const u128;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);

        let mut q1 = (unsafe{ *p.offset(16) } ^ v);
        let mut q2 = (unsafe{ *p.offset(17) } ^ v);
        let mut q3 = (unsafe{ *p.offset(18) } ^ v);
        let mut q4 = (unsafe{ *p.offset(19) } ^ v);

        let mut q5 = (unsafe{ *p.offset(20) } ^ v);
        let mut q6 = (unsafe{ *p.offset(21) } ^ v);
        let mut q7 = (unsafe{ *p.offset(22) } ^ v);
        let mut q8 = (unsafe{ *p.offset(23) } ^ v);

        let mut w1 = (unsafe{ *p.offset(24) } ^ v);
        let mut w2 = (unsafe{ *p.offset(25) } ^ v);
        let mut w3 = (unsafe{ *p.offset(26) } ^ v);
        let mut w4 = (unsafe{ *p.offset(27) } ^ v);

        let mut w5 = (unsafe{ *p.offset(28) } ^ v);
        let mut w6 = (unsafe{ *p.offset(29) } ^ v);
        let mut w7 = (unsafe{ *p.offset(30) } ^ v);
        let mut w8 = (unsafe{ *p.offset(31) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2);
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1);

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2);
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1);

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2);
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1);

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2);
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1);

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2);
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1);

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2);
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1);

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2);
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1);

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2);
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1);

        r1 &= ((r1 & u0) >> 4) | ((r1 & u3) << 4);
        r1 &= ((r1 & u1) >> 2) | ((r1 & u4) << 2);
        r1 &= ((r1 & u2) >> 1) | ((r1 & u5) << 1);

        r2 &= ((r2 & u0) >> 4) | ((r2 & u3) << 4);
        r2 &= ((r2 & u1) >> 2) | ((r2 & u4) << 2);
        r2 &= ((r2 & u2) >> 1) | ((r2 & u5) << 1);

        r3 &= ((r3 & u0) >> 4) | ((r3 & u3) << 4);
        r3 &= ((r3 & u1) >> 2) | ((r3 & u4) << 2);
        r3 &= ((r3 & u2) >> 1) | ((r3 & u5) << 1);

        r4 &= ((r4 & u0) >> 4) | ((r4 & u3) << 4);
        r4 &= ((r4 & u1) >> 2) | ((r4 & u4) << 2);
        r4 &= ((r4 & u2) >> 1) | ((r4 & u5) << 1);

        r5 &= ((r5 & u0) >> 4) | ((r5 & u3) << 4);
        r5 &= ((r5 & u1) >> 2) | ((r5 & u4) << 2);
        r5 &= ((r5 & u2) >> 1) | ((r5 & u5) << 1);

        r6 &= ((r6 & u0) >> 4) | ((r6 & u3) << 4);
        r6 &= ((r6 & u1) >> 2) | ((r6 & u4) << 2);
        r6 &= ((r6 & u2) >> 1) | ((r6 & u5) << 1);

        r7 &= ((r7 & u0) >> 4) | ((r7 & u3) << 4);
        r7 &= ((r7 & u1) >> 2) | ((r7 & u4) << 2);
        r7 &= ((r7 & u2) >> 1) | ((r7 & u5) << 1);

        r8 &= ((r8 & u0) >> 4) | ((r8 & u3) << 4);
        r8 &= ((r8 & u1) >> 2) | ((r8 & u4) << 2);
        r8 &= ((r8 & u2) >> 1) | ((r8 & u5) << 1);

        q1 &= ((q1 & u0) >> 4) | ((q1 & u3) << 4);
        q1 &= ((q1 & u1) >> 2) | ((q1 & u4) << 2);
        q1 &= ((q1 & u2) >> 1) | ((q1 & u5) << 1);

        q2 &= ((q2 & u0) >> 4) | ((q2 & u3) << 4);
        q2 &= ((q2 & u1) >> 2) | ((q2 & u4) << 2);
        q2 &= ((q2 & u2) >> 1) | ((q2 & u5) << 1);

        q3 &= ((q3 & u0) >> 4) | ((q3 & u3) << 4);
        q3 &= ((q3 & u1) >> 2) | ((q3 & u4) << 2);
        q3 &= ((q3 & u2) >> 1) | ((q3 & u5) << 1);

        q4 &= ((q4 & u0) >> 4) | ((q4 & u3) << 4);
        q4 &= ((q4 & u1) >> 2) | ((q4 & u4) << 2);
        q4 &= ((q4 & u2) >> 1) | ((q4 & u5) << 1);

        q5 &= ((q5 & u0) >> 4) | ((q5 & u3) << 4);
        q5 &= ((q5 & u1) >> 2) | ((q5 & u4) << 2);
        q5 &= ((q5 & u2) >> 1) | ((q5 & u5) << 1);

        q6 &= ((q6 & u0) >> 4) | ((q6 & u3) << 4);
        q6 &= ((q6 & u1) >> 2) | ((q6 & u4) << 2);
        q6 &= ((q6 & u2) >> 1) | ((q6 & u5) << 1);

        q7 &= ((q7 & u0) >> 4) | ((q7 & u3) << 4);
        q7 &= ((q7 & u1) >> 2) | ((q7 & u4) << 2);
        q7 &= ((q7 & u2) >> 1) | ((q7 & u5) << 1);

        q8 &= ((q8 & u0) >> 4) | ((q8 & u3) << 4);
        q8 &= ((q8 & u1) >> 2) | ((q8 & u4) << 2);
        q8 &= ((q8 & u2) >> 1) | ((q8 & u5) << 1);

        w1 &= ((w1 & u0) >> 4) | ((w1 & u3) << 4);
        w1 &= ((w1 & u1) >> 2) | ((w1 & u4) << 2);
        w1 &= ((w1 & u2) >> 1) | ((w1 & u5) << 1);

        w2 &= ((w2 & u0) >> 4) | ((w2 & u3) << 4);
        w2 &= ((w2 & u1) >> 2) | ((w2 & u4) << 2);
        w2 &= ((w2 & u2) >> 1) | ((w2 & u5) << 1);

        w3 &= ((w3 & u0) >> 4) | ((w3 & u3) << 4);
        w3 &= ((w3 & u1) >> 2) | ((w3 & u4) << 2);
        w3 &= ((w3 & u2) >> 1) | ((w3 & u5) << 1);

        w4 &= ((w4 & u0) >> 4) | ((w4 & u3) << 4);
        w4 &= ((w4 & u1) >> 2) | ((w4 & u4) << 2);
        w4 &= ((w4 & u2) >> 1) | ((w4 & u5) << 1);

        w5 &= ((w5 & u0) >> 4) | ((w5 & u3) << 4);
        w5 &= ((w5 & u1) >> 2) | ((w5 & u4) << 2);
        w5 &= ((w5 & u2) >> 1) | ((w5 & u5) << 1);

        w6 &= ((w6 & u0) >> 4) | ((w6 & u3) << 4);
        w6 &= ((w6 & u1) >> 2) | ((w6 & u4) << 2);
        w6 &= ((w6 & u2) >> 1) | ((w6 & u5) << 1);

        w7 &= ((w7 & u0) >> 4) | ((w7 & u3) << 4);
        w7 &= ((w7 & u1) >> 2) | ((w7 & u4) << 2);
        w7 &= ((w7 & u2) >> 1) | ((w7 & u5) << 1);

        w8 &= ((w8 & u0) >> 4) | ((w8 & u3) << 4);
        w8 &= ((w8 & u1) >> 2) | ((w8 & u4) << 2);
        w8 &= ((w8 & u2) >> 1) | ((w8 & u5) << 1);

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8 |
           q1 | q2 | q3 | q4 | q5 | q6 | q7 | q8 |
           w1 | w2 | w3 | w4 | w5 | w6 | w7 | w8) != 0
           {
               break;
           }

        p = unsafe { p.add(32) };

        counter += 16 * 32;
    }

    return find_ending(ss, counter);
}
*/







fn find_u32_rotate_3_half(s: &str) -> usize
{
    // take invert so that xor with value will be all ones
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4;
    
    let mut counter = 0;
    let mut u = 0u32;
    let mut p = ss.as_ptr() as *const u32;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;
 
        u = t;
        u &= (u & 0xf0f0_f0f0) >> 4;
        u &= (u & 0xcccc_cccc) >> 2;
        u &= (u & 0xaaaa_aaaa) >> 1;

        counter += 4;
        p = unsafe { p.add(1) };
    }
    counter = counter - 4;
    return find_ending(ss, counter);
}

fn find_u64_rotate_3_half(s: &str) -> usize
{
    // take invert
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8;
    
    let mut counter = 0;
    let mut u = 0u64;
    let mut p = ss.as_ptr() as *const u64;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;

        u = t;
        u &= (u & 0xf0f0_f0f0_f0f0_f0f0) >> 4;
        u &= (u & 0xcccc_cccc_cccc_cccc) >> 2;
        u &= (u & 0xaaaa_aaaa_aaaa_aaaa) >> 1;

        counter += 8;
        p = unsafe { p.add(1) };
    }
    counter = counter - 8;
    return find_ending(ss, counter);
}
 
fn find_u128_rotate_3_half(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16;
    
    let mut counter = 0;
    let mut u = 0u128;
    let mut p = ss.as_ptr() as *const u128;
    while counter < len && u == 0
    {
        let t = unsafe{ *p } ^ v;

        u = t;
        // first mask 4 bits, for 4 bit swap, then 2 bits for every 2 bit sequence swap and finally one bit swapping next to each other.
        // In case the one or another is 0, the & operation will 'copy' the 0 to the other one and thus the 0 will 'spread'.
        u &= (u & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0) >> 4;
        u &= (u & 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc) >> 2;
        u &= (u & 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa) >> 1;

        counter += 16;
        p = unsafe { p.add(1) };
    }
    counter = counter - 16;
    return find_ending(ss, counter);
}






fn find_u32x2_rotate_3_half(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 2;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0;
    let u1 = 0xcccc_cccc;
    let u2 = 0xaaaa_aaaa;

    let mut p = ss.as_ptr() as *const u32;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;

        if (t1 | t2) != 0
        {
            break;
        }        

        counter += 4 * 2;
        p = unsafe { p.add(2) };
    }

    return find_ending(ss, counter);
}

fn find_u64x2_rotate_3_half(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa;

    let mut p = ss.as_ptr() as *const u64;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);


        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;


        if (t1 | t2) != 0
        {
            break;
        }        

        counter += 8 * 2;
        p = unsafe { p.add(2) };
    }

    return find_ending(ss, counter);
}

fn find_u128x2_rotate_3_half(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 2;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let mut p = ss.as_ptr() as *const u128;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;


        if (t1 | t2) != 0
        {
            break;
        }        

        counter += 16 * 2;
        p = unsafe { p.add(2) };
    }

    return find_ending(ss, counter);
}



fn find_u32x4_rotate_3_half(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0;
    let u1 = 0xcccc_cccc;
    let u2 = 0xaaaa_aaaa;

    let mut p = ss.as_ptr() as *const u32;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;


        if (t1 | t2 | t3 | t4) != 0
        {
            break;
        }        

        counter += 4 * 4;
        p = unsafe { p.add(4) };
    }

    return find_ending(ss, counter);
}

fn find_u64x4_rotate_3_half(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa;


    let mut p = ss.as_ptr() as *const u64;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;


        if (t1 | t2 | t3 | t4) != 0
        {
            break;
        }        

        counter += 8 * 4;
        p = unsafe { p.add(4) };
    }

    return find_ending(ss, counter);
}

fn find_u128x4_rotate_3_half(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 4;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let mut p = ss.as_ptr() as *const u128;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;



        if (t1 | t2 | t3 | t4) != 0
        {
            break;
        }        

        counter += 16 * 4;
        p = unsafe { p.add(4) };
    }

    return find_ending(ss, counter);
}



fn find_u32x8_rotate_3_half(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0;
    let u1 = 0xcccc_cccc;
    let u2 = 0xaaaa_aaaa;

    let mut p = ss.as_ptr() as *const u32;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;
        t5 &= (t5 & u0) >> 4;
        t5 &= (t5 & u1) >> 2;
        t5 &= (t5 & u2) >> 1;
        t6 &= (t6 & u0) >> 4;
        t6 &= (t6 & u1) >> 2;
        t6 &= (t6 & u2) >> 1;
        t7 &= (t7 & u0) >> 4;
        t7 &= (t7 & u1) >> 2;
        t7 &= (t7 & u2) >> 1;
        t8 &= (t8 & u0) >> 4;
        t8 &= (t8 & u1) >> 2;
        t8 &= (t8 & u2) >> 1;



        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8) != 0
        {
            break;
        }        

        counter += 4 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}

fn find_u64x8_rotate_3_half(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa;

    let mut p = ss.as_ptr() as *const u64;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;
        t5 &= (t5 & u0) >> 4;
        t5 &= (t5 & u1) >> 2;
        t5 &= (t5 & u2) >> 1;
        t6 &= (t6 & u0) >> 4;
        t6 &= (t6 & u1) >> 2;
        t6 &= (t6 & u2) >> 1;
        t7 &= (t7 & u0) >> 4;
        t7 &= (t7 & u1) >> 2;
        t7 &= (t7 & u2) >> 1;
        t8 &= (t8 & u0) >> 4;
        t8 &= (t8 & u1) >> 2;
        t8 &= (t8 & u2) >> 1;



        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8) != 0
        {
            break;
        }        

        counter += 8 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}

fn find_u128x8_rotate_3_half(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let mut p = ss.as_ptr() as *const u128;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;
        t5 &= (t5 & u0) >> 4;
        t5 &= (t5 & u1) >> 2;
        t5 &= (t5 & u2) >> 1;
        t6 &= (t6 & u0) >> 4;
        t6 &= (t6 & u1) >> 2;
        t6 &= (t6 & u2) >> 1;
        t7 &= (t7 & u0) >> 4;
        t7 &= (t7 & u1) >> 2;
        t7 &= (t7 & u2) >> 1;
        t8 &= (t8 & u0) >> 4;
        t8 &= (t8 & u1) >> 2;
        t8 &= (t8 & u2) >> 1;


        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8) != 0
        {
            break;
        }        

        counter += 16 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}


fn find_u32x16_rotate_3_half(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 16;

    let u0 = 0xF0F0_F0F0u32;
    let u1 = 0xCCCC_CCCCu32;
    let u2 = 0xAAAA_AAAAu32;

    let mut p = ss.as_ptr() as *const u32;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);


        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;
        t5 &= (t5 & u0) >> 4;
        t5 &= (t5 & u1) >> 2;
        t5 &= (t5 & u2) >> 1;
        t6 &= (t6 & u0) >> 4;
        t6 &= (t6 & u1) >> 2;
        t6 &= (t6 & u2) >> 1;
        t7 &= (t7 & u0) >> 4;
        t7 &= (t7 & u1) >> 2;
        t7 &= (t7 & u2) >> 1;
        t8 &= (t8 & u0) >> 4;
        t8 &= (t8 & u1) >> 2;
        t8 &= (t8 & u2) >> 1;
        r1 &= (r1 & u0) >> 4;
        r1 &= (r1 & u1) >> 2;
        r1 &= (r1 & u2) >> 1;
        r2 &= (r2 & u0) >> 4;
        r2 &= (r2 & u1) >> 2;
        r2 &= (r2 & u2) >> 1;
        r3 &= (r3 & u0) >> 4;
        r3 &= (r3 & u1) >> 2;
        r3 &= (r3 & u2) >> 1;
        r4 &= (r4 & u0) >> 4;
        r4 &= (r4 & u1) >> 2;
        r4 &= (r4 & u2) >> 1;
        r5 &= (r5 & u0) >> 4;
        r5 &= (r5 & u1) >> 2;
        r5 &= (r5 & u2) >> 1;
        r6 &= (r6 & u0) >> 4;
        r6 &= (r6 & u1) >> 2;
        r6 &= (r6 & u2) >> 1;
        r7 &= (r7 & u0) >> 4;
        r7 &= (r7 & u1) >> 2;
        r7 &= (r7 & u2) >> 1;
        r8 &= (r8 & u0) >> 4;
        r8 &= (r8 & u1) >> 2;
        r8 &= (r8 & u2) >> 1;

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8) != 0
           {
               break;
           }

        p = unsafe { p.add(16) };

        counter += 4 * 16;
    }

    return find_ending(ss, counter);
}

fn find_u64x16_rotate_3_half(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 16;

    let u0 = 0xF0F0_F0F0_F0F0_F0F0;
    let u1 = 0x0C0C_0C0C_0C0C_0C0C;
    let u2 = 0x0202_0202_0202_0202;

    let mut p = ss.as_ptr() as *const u64;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);


        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;
        t5 &= (t5 & u0) >> 4;
        t5 &= (t5 & u1) >> 2;
        t5 &= (t5 & u2) >> 1;
        t6 &= (t6 & u0) >> 4;
        t6 &= (t6 & u1) >> 2;
        t6 &= (t6 & u2) >> 1;
        t7 &= (t7 & u0) >> 4;
        t7 &= (t7 & u1) >> 2;
        t7 &= (t7 & u2) >> 1;
        t8 &= (t8 & u0) >> 4;
        t8 &= (t8 & u1) >> 2;
        t8 &= (t8 & u2) >> 1;
        r1 &= (r1 & u0) >> 4;
        r1 &= (r1 & u1) >> 2;
        r1 &= (r1 & u2) >> 1;
        r2 &= (r2 & u0) >> 4;
        r2 &= (r2 & u1) >> 2;
        r2 &= (r2 & u2) >> 1;
        r3 &= (r3 & u0) >> 4;
        r3 &= (r3 & u1) >> 2;
        r3 &= (r3 & u2) >> 1;
        r4 &= (r4 & u0) >> 4;
        r4 &= (r4 & u1) >> 2;
        r4 &= (r4 & u2) >> 1;
        r5 &= (r5 & u0) >> 4;
        r5 &= (r5 & u1) >> 2;
        r5 &= (r5 & u2) >> 1;
        r6 &= (r6 & u0) >> 4;
        r6 &= (r6 & u1) >> 2;
        r6 &= (r6 & u2) >> 1;
        r7 &= (r7 & u0) >> 4;
        r7 &= (r7 & u1) >> 2;
        r7 &= (r7 & u2) >> 1;
        r8 &= (r8 & u0) >> 4;
        r8 &= (r8 & u1) >> 2;
        r8 &= (r8 & u2) >> 1;

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8) != 0
           {
               break;
           }

        p = unsafe { p.add(16) };

        counter += 8 * 16;
    }

    return find_ending(ss, counter);
}

fn find_u128x16_rotate_3_half(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 16;

    let u0 = 0xF0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0_F0F0;
    let u1 = 0xCCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC_CCCC;
    let u2 = 0xAAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA_AAAA;

    let mut p = ss.as_ptr() as *const u128;
    
    let mut counter = 0;
    while counter < len        
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let mut t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let mut t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let mut t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let mut t8 = (unsafe{ *p.offset(7) } ^ v);

        let mut r1 = (unsafe{ *p.offset(8) } ^ v);
        let mut r2 = (unsafe{ *p.offset(9) } ^ v);
        let mut r3 = (unsafe{ *p.offset(10) } ^ v);
        let mut r4 = (unsafe{ *p.offset(11) } ^ v);

        let mut r5 = (unsafe{ *p.offset(12) } ^ v);
        let mut r6 = (unsafe{ *p.offset(13) } ^ v);
        let mut r7 = (unsafe{ *p.offset(14) } ^ v);
        let mut r8 = (unsafe{ *p.offset(15) } ^ v);


        t1 &= (t1 & u0) >> 4;
        t1 &= (t1 & u1) >> 2;
        t1 &= (t1 & u2) >> 1;
        t2 &= (t2 & u0) >> 4;
        t2 &= (t2 & u1) >> 2;
        t2 &= (t2 & u2) >> 1;
        t3 &= (t3 & u0) >> 4;
        t3 &= (t3 & u1) >> 2;
        t3 &= (t3 & u2) >> 1;
        t4 &= (t4 & u0) >> 4;
        t4 &= (t4 & u1) >> 2;
        t4 &= (t4 & u2) >> 1;
        t5 &= (t5 & u0) >> 4;
        t5 &= (t5 & u1) >> 2;
        t5 &= (t5 & u2) >> 1;
        t6 &= (t6 & u0) >> 4;
        t6 &= (t6 & u1) >> 2;
        t6 &= (t6 & u2) >> 1;
        t7 &= (t7 & u0) >> 4;
        t7 &= (t7 & u1) >> 2;
        t7 &= (t7 & u2) >> 1;
        t8 &= (t8 & u0) >> 4;
        t8 &= (t8 & u1) >> 2;
        t8 &= (t8 & u2) >> 1;
        r1 &= (r1 & u0) >> 4;
        r1 &= (r1 & u1) >> 2;
        r1 &= (r1 & u2) >> 1;
        r2 &= (r2 & u0) >> 4;
        r2 &= (r2 & u1) >> 2;
        r2 &= (r2 & u2) >> 1;
        r3 &= (r3 & u0) >> 4;
        r3 &= (r3 & u1) >> 2;
        r3 &= (r3 & u2) >> 1;
        r4 &= (r4 & u0) >> 4;
        r4 &= (r4 & u1) >> 2;
        r4 &= (r4 & u2) >> 1;
        r5 &= (r5 & u0) >> 4;
        r5 &= (r5 & u1) >> 2;
        r5 &= (r5 & u2) >> 1;
        r6 &= (r6 & u0) >> 4;
        r6 &= (r6 & u1) >> 2;
        r6 &= (r6 & u2) >> 1;
        r7 &= (r7 & u0) >> 4;
        r7 &= (r7 & u1) >> 2;
        r7 &= (r7 & u2) >> 1;
        r8 &= (r8 & u0) >> 4;
        r8 &= (r8 & u1) >> 2;
        r8 &= (r8 & u2) >> 1;

        if (t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8 |
           r1 | r2 | r3 | r4 | r5 | r6 | r7 | r8) != 0
           {
               break;
           }

        p = unsafe { p.add(16) };

        counter += 16 * 16;
    }

    return find_ending(ss, counter);
}







fn find_u32x8_rotate_3_parallel(s: &str) -> usize
{
    let v = !V32;
    let ss = s.as_bytes();
    let len = ss.len() - 4 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0;
    let u1 = 0xcccc_cccc;
    let u2 = 0xaaaa_aaaa;

    let u3 = 0x0f0f_0f0f;
    let u4 = 0x3333_3333;
    let u5 = 0x5555_5555;


    let mut p = ss.as_ptr() as *const u32;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 = (t1 & ( (t1 & u0) >> 4)) | (((t2 & u3) << 4) & t2);
        t3 = (t3 & ( (t3 & u0) >> 4)) | (((t4 & u3) << 4) & t4);
        t5 = (t5 & ( (t5 & u0) >> 4)) | (((t6 & u3) << 4) & t6);
        t7 = (t7 & ( (t7 & u0) >> 4)) | (((t8 & u3) << 4) & t8);




        t1 = (t1 & ((t1 & u1) >> 2)) | (((t3 & u4) << 2) & t3);
        t5 = (t5 & ((t5 & u1) >> 2)) | (((t7 & u4) << 2) & t7);
        
        t1 = (t1 & ((t1 & u2) >> 1)) | (((t5 & u5) << 1) & t5);

        if t1 != 0
        {
            break;
        }        

        counter += 4 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}







fn find_u64x8_rotate_3_parallel(s: &str) -> usize
{
    let v = !V64;
    let ss = s.as_bytes();
    let len = ss.len() - 8 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u64;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 = (t1 & ( (t1 & u0) >> 4)) | (((t2 & u3) << 4) & t2);
        t3 = (t3 & ( (t3 & u0) >> 4)) | (((t4 & u3) << 4) & t4);
        t5 = (t5 & ( (t5 & u0) >> 4)) | (((t6 & u3) << 4) & t6);
        t7 = (t7 & ( (t7 & u0) >> 4)) | (((t8 & u3) << 4) & t8);

        t1 = (t1 & ((t1 & u1) >> 2)) | (((t3 & u4) << 2) & t3);
        t5 = (t5 & ((t5 & u1) >> 2)) | (((t7 & u4) << 2) & t7);
        
        t1 = (t1 & ((t1 & u2) >> 1)) | (((t5 & u5) << 1) & t5);

        if t1 != 0
        {
            break;
        }        

        counter += 8 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}




fn find_u128x8_rotate_3_parallel(s: &str) -> usize
{
    let v = !V128;
    let ss = s.as_bytes();
    let len = ss.len() - 16 * 8;
    
    let mut counter = 0;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555_5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u128;
    while counter < len
    {
        let mut t1 = (unsafe{ *p.offset(0) } ^ v);
        let t2 = (unsafe{ *p.offset(1) } ^ v);
        let mut t3 = (unsafe{ *p.offset(2) } ^ v);
        let t4 = (unsafe{ *p.offset(3) } ^ v);

        let mut t5 = (unsafe{ *p.offset(4) } ^ v);
        let t6 = (unsafe{ *p.offset(5) } ^ v);
        let mut t7 = (unsafe{ *p.offset(6) } ^ v);
        let t8 = (unsafe{ *p.offset(7) } ^ v);

        t1 = (t1 & ( (t1 & u0) >> 4)) | (((t2 & u3) << 4) & t2);
        t3 = (t3 & ( (t3 & u0) >> 4)) | (((t4 & u3) << 4) & t4);
        t5 = (t5 & ( (t5 & u0) >> 4)) | (((t6 & u3) << 4) & t6);
        t7 = (t7 & ( (t7 & u0) >> 4)) | (((t8 & u3) << 4) & t8);

        t1 = (t1 & ((t1 & u1) >> 2)) | (((t3 & u4) << 2) & t3);
        t5 = (t5 & ((t5 & u1) >> 2)) | (((t7 & u4) << 2) & t7);
        
        t1 = (t1 & ((t1 & u2) >> 1)) | (((t5 & u5) << 1) & t5);

        if t1 != 0
        {
            break;
        }        

        counter += 16 * 8;
        p = unsafe { p.add(8) };
    }

    return find_ending(ss, counter);
}






fn find_simd(s: &str) -> usize
{
    let v = unsafe { _mm_set1_epi8(0x61i8) };
    let ss = s.as_bytes();
    let len = ss.len() - 16;
    
    let mut counter = 0;
    let one_u = unsafe { _mm_set1_epi8(-1) } ;
    let mut p = ss.as_ptr() as *const __m128i;
    while counter < len
    {
        unsafe 
        {
            let t =  _mm_load_si128(p);
            let u = _mm_cmpeq_epi8(t, v);

            if _mm_test_all_zeros(u, one_u) == 0
            {
                break;
            }

            counter += 16;
            p = p.add(1);
        }
    }

    return find_ending(ss, counter);
}


/*
fn find_simd_256(s: &str) -> usize
{
    let v = unsafe { _mm256_set1_epi8(0x61i8) };
    let ss = s.as_bytes();
    let len = ss.len() - 32;
    
    let mut counter = 0;
    let one_u = unsafe { _mm256_set1_epi8(-1) } ;
    let mut p = ss.as_ptr() as *const __m256i;
    while counter < len
    {
        unsafe 
        {
            let t =  _mm256_load_si256(p);
            let u = _mm256_cmpeq_epi8(t, v);

            if _mm256_testz_si256(u, one_u) == 0
            {
                break;
            }

            counter += 32;
            p = p.add(1);
        }
    }
    
    return find_ending(ss, counter);
}
*/






fn print_find(s: &str, s_len: usize, type_str: &str, f: fn(s: &str) -> usize)
{
    let mut sum = 0u128;
    let mut max_sum = 0u128;
    let mut min_sum = !0u128;
    let mut amount = 0;
    for _ in 0..1000
    {
        amount += 1;
        let timer = std::time::Instant::now();
        let pos = f(s);
        let t = timer.elapsed().as_micros();
        sum += t;
        max_sum = core::cmp::max(max_sum, t);
        min_sum = core::cmp::min(min_sum, t);
        if pos == s_len || pos != 1_000_007
        {
            println!("Failed to find character in {}, duration {}", type_str, t);
        }
        /*
        else
        {
            println!("Found character in {}, pos: {}, duration {}", type_str, pos, t);
        }
        */
    }

    let sum_avg = sum / amount;

    println!("avg:{},  \tmin:{},  \tmax:{}  \tin {}", sum_avg, min_sum, max_sum, type_str);
}


fn main() 
{
    println!("Hello, world!");


    let mut s = String::new();


    {
        let timer = std::time::Instant::now();
        add_chars(&mut s);
        s.push_str("aaaa");
        add_chars(&mut s);
        println!("Duration adding chars: {}", timer.elapsed().as_micros());
    }

    let s_len = s.len();


    print_find(&s, s_len,"iter", find_iter);
    print_find(&s, s_len,"u8 array", find_u8);
    print_find(&s, s_len,"u8 ptr?", find_u8_2);

    print_find(&s, s_len,"using find char", find_find_char);
    print_find(&s, s_len,"using find str", find_find_str);

    print_find(&s, s_len,"using u128_slice_return", find_u128_slice_return);

    print_find(&s, s_len,"using u32_return", find_u32_return);
    print_find(&s, s_len,"using u32_return_test_assign", find_u32_return_test_assign);
    print_find(&s, s_len,"using u32_return_test", find_u32_return_test);
    
    
    print_find(&s, s_len,"using u64_return", find_u64_return);
    print_find(&s, s_len,"using u64_return_2", find_u64_return_2);
    print_find(&s, s_len,"using u64x4_return", find_u64x4_return);
    print_find(&s, s_len,"using u64x4_return_test", find_u64x4_return_test);
    
    print_find(&s, s_len,"using u128_return", find_u128_return);
    print_find(&s, s_len,"using u128x2_return", find_u128x2_return);
    
    
    
    print_find(&s, s_len,"using u32_rotate_7_times", find_u32_rotate_7);
    print_find(&s, s_len,"using u64_rotate_7_times", find_u64_rotate_7);
    print_find(&s, s_len,"using u128_rotate_7_times", find_u128_rotate_7);

    print_find(&s, s_len,"using u32_rotate_3_times", find_u32_rotate_3);
    print_find(&s, s_len,"using u64_rotate_3_times", find_u64_rotate_3);
    print_find(&s, s_len,"using u128_rotate_3_times", find_u128_rotate_3);


    
    print_find(&s, s_len,"using u32x2_rotate_3_times", find_u32x2_rotate_3);
    print_find(&s, s_len,"using u64x2_rotate_3_times", find_u64x2_rotate_3);
    print_find(&s, s_len,"using u128x2_rotate_3_times", find_u128x2_rotate_3);
    print_find(&s, s_len,"using u32x4_rotate_3_times", find_u32x4_rotate_3);
    print_find(&s, s_len,"using u64x4_rotate_3_times", find_u64x4_rotate_3);
    print_find(&s, s_len,"using u128x4_rotate_3_times", find_u128x4_rotate_3);
    print_find(&s, s_len,"using u32x8_rotate_3_times", find_u32x8_rotate_3);
    print_find(&s, s_len,"using u64x8_rotate_3_times", find_u64x8_rotate_3);
    print_find(&s, s_len,"using u128x8_rotate_3_times", find_u128x8_rotate_3);
    print_find(&s, s_len,"using u32x16_rotate_3_times", find_u32x16_rotate_3);
    print_find(&s, s_len,"using u64x16_rotate_3_times", find_u64x16_rotate_3);
    print_find(&s, s_len,"using u128x16_rotate_3_times", find_u128x16_rotate_3);


    print_find(&s, s_len,"using u32_rotate_3_times_half", find_u32_rotate_3_half);
    print_find(&s, s_len,"using u64_rotate_3_times_half", find_u64_rotate_3_half);
    print_find(&s, s_len,"using u128_rotate_3_times_half", find_u128_rotate_3_half);

    print_find(&s, s_len,"using u32x2_rotate_3_times_half", find_u32x2_rotate_3_half);
    print_find(&s, s_len,"using u64x2_rotate_3_times_half", find_u64x2_rotate_3_half);
    print_find(&s, s_len,"using u128x2_rotate_3_times_half", find_u128x2_rotate_3_half);
    print_find(&s, s_len,"using u32x4_rotate_3_times_half", find_u32x4_rotate_3_half);
    print_find(&s, s_len,"using u64x4_rotate_3_times_half", find_u64x4_rotate_3_half);
    print_find(&s, s_len,"using u128x4_rotate_3_times_half", find_u128x4_rotate_3_half);
    print_find(&s, s_len,"using u32x8_rotate_3_times_half", find_u32x8_rotate_3_half);
    print_find(&s, s_len,"using u64x8_rotate_3_times_half", find_u64x8_rotate_3_half);
    print_find(&s, s_len,"using u128x8_rotate_3_times_half", find_u128x8_rotate_3_half);
    print_find(&s, s_len,"using u32x16_rotate_3_times_half", find_u32x16_rotate_3_half);
    print_find(&s, s_len,"using u64x16_rotate_3_times_half", find_u64x16_rotate_3_half);
    print_find(&s, s_len,"using u128x16_rotate_3_times_half", find_u128x16_rotate_3_half);

    print_find(&s, s_len,"using simd", find_simd);
    //print_find(&s, s_len,"using simd256", find_simd_256);


    print_find(&s, s_len,"using find_u32x8_rotate_3_parallel", find_u32x8_rotate_3_parallel);

    print_find(&s, s_len,"using find_u64x8_rotate_3_parallel", find_u64x8_rotate_3_parallel);
    print_find(&s, s_len,"using find_u128x8_rotate_3_parallel", find_u128x8_rotate_3_parallel);
    
    
    
}




