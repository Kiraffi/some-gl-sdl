
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;


#[cfg(target_arch = "x86")]
use std::arch::x86::*;




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
    let mut p = s.as_bytes().as_ptr();
    let start = s.as_bytes().as_ptr();
    let end = unsafe { start.offset(s.len() as isize ) };
    while p < end
    {
        unsafe {
        if *p == 'a' as u8
        {
            return (p as isize - start as isize) as usize;
        }
        p = p.add(1);
        };
        
        
    }
    return s.len();
}














fn find_u8_simd_fake(s: &str) -> usize
{
    let mut v = 0u128;
    for i in 0..16
    {
        v += (('a' as u8) as u128) << (i * 8);
    }
    let ss = s.as_bytes();
    let mut counter = 0;
    while counter + 16 <= s.len()
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

    counter = counter - 16;
    while counter < s.len()
    {
        if ss[counter ] == 'a' as u8 { return counter; }
        counter += 1;
    }

    return s.len();
}



fn find_u8_simd_fake2(s: &str) -> usize
{
    let mut v = 0u64;
    for i in 0..8
    {
        v += (('a' as u8) as u64) << (i * 8);
    }
    let ss = s.as_bytes();
    let mut p = ss.as_ptr() as *const u64;
    
    let mut counter = 0;
    while counter + 8 <= s.len()
    {
        let slice = &ss[counter..counter + 8];
        let mut t = unsafe { *p };
 
        t ^= v;
        
        if (t >> (0  * 8)) & 255 == 0 { return counter + 0; }
        if (t >> (1  * 8)) & 255 == 0 { return counter + 1; }
        if (t >> (2  * 8)) & 255 == 0 { return counter + 2; }
        if (t >> (3  * 8)) & 255 == 0 { return counter + 3; }
 
        if (t >> (4  * 8)) & 255 == 0 { return counter + 4; }
        if (t >> (5  * 8)) & 255 == 0 { return counter + 5; }
        if (t >> (6  * 8)) & 255 == 0 { return counter + 6; }
        if (t >> (7  * 8)) & 255 == 0 { return counter + 7; }

        counter += 8;
        p = unsafe { p.add(1) };
    }

    counter = counter - 8;
    while counter < s.len()
    {
        if ss[counter ] == 'a' as u8 { return counter; }
        counter += 1;
    }

    return s.len();
}


fn find_u8_simd_fake3(s: &str) -> usize
{
    let mut v = 0u128;
    for i in 0..16
    {
        v += (('a' as u8) as u128) << (i * 8);
    }
    let ss = s.as_bytes();
    
    let mut p = ss.as_ptr() as *const u128;
    let end = unsafe { p.offset(((s.len() - 15) / 16) as isize ) };
    
    let mut counter = 0;
    while p < end
    {
        let mut t = unsafe{ *p as u128 };
 
        t ^= v;

        if (t >> (0  * 8)) & 255 == 0 { return counter + 0; }
        if (t >> (1  * 8)) & 255 == 0 { return counter + 1; }
        if (t >> (2  * 8)) & 255 == 0 { return counter + 2; }
        if (t >> (3  * 8)) & 255 == 0 { return counter + 3; }
 
        if (t >> (4  * 8)) & 255 == 0 { return counter + 4; }
        if (t >> (5  * 8)) & 255 == 0 { return counter + 5; }
        if (t >> (6  * 8)) & 255 == 0 { return counter + 6; }
        if (t >> (7  * 8)) & 255 == 0 { return counter + 7; }
 
        
        if (t >> (8  * 8)) & 255 == 0 { return counter + 8; }
        if (t >> (9  * 8)) & 255 == 0 { return counter + 9; }
        if (t >> (10 * 8)) & 255 == 0 { return counter + 10; }
        if (t >> (11 * 8)) & 255 == 0 { return counter + 11; }

        if (t >> (12 * 8)) & 255 == 0 { return counter + 12; }
        if (t >> (13 * 8)) & 255 == 0 { return counter + 13; }
        if (t >> (14 * 8)) & 255 == 0 { return counter + 14; }
        if (t >> (15 * 8)) & 255 == 0 { return counter + 15; }

        counter += 16;
        p = unsafe { p.add(1) };
    }
    counter = counter - 16;
    while counter < s.len()
    {
        if ss[counter ] == 'a' as u8 { return counter; }
        counter += 1;
    }


    return s.len();
}


fn find_u8_simd_fake3_2(s: &str) -> usize
{
    let mut v = 0u128;
    for i in 0..16
    {
        v += (('a' as u8) as u128) << (i * 8);
    }
    let ss = s.as_bytes();
    
    let mut p = ss.as_ptr() as *const u128;
    let start = ss.as_ptr() as *const u128;
    let end = unsafe { p.offset(((s.len() - 15) / 16) as isize ) };
    
    let mut counter = 0;
    let mut ones = !0;
    while p < end && ones != 0
    {
        let mut t = unsafe{ *p as u128 };
 
        t ^= v;
        ones = 0; 
        ones |= if (t >> (0  * 8)) & 255 == 0 { 1 << 0 } else {0};
        ones |= if (t >> (1  * 8)) & 255 == 0 { 1 << 1 } else {0};
        ones |= if (t >> (2  * 8)) & 255 == 0 { 1 << 2 } else {0};
        ones |= if (t >> (3  * 8)) & 255 == 0 { 1 << 3 } else {0};
 
        ones |= if (t >> (4  * 8)) & 255 == 0 { 1 << 4 } else {0};
        ones |= if (t >> (5  * 8)) & 255 == 0 { 1 << 5 } else {0};
        ones |= if (t >> (6  * 8)) & 255 == 0 { 1 << 6 } else {0};
        ones |= if (t >> (7  * 8)) & 255 == 0 { 1 << 7 } else {0};
 
        
        ones |= if (t >> (8  * 8)) & 255 == 0 { 1 << 8  } else {0};
        ones |= if (t >> (9  * 8)) & 255 == 0 { 1 << 9  } else {0};
        ones |= if (t >> (10 * 8)) & 255 == 0 { 1 << 10 } else {0};
        ones |= if (t >> (11 * 8)) & 255 == 0 { 1 << 11 } else {0};

        ones |= if (t >> (12 * 8)) & 255 == 0 { 1 << 12 } else {0};
        ones |= if (t >> (13 * 8)) & 255 == 0 { 1 << 13 } else {0};
        ones |= if (t >> (14 * 8)) & 255 == 0 { 1 << 14 } else {0};
        ones |= if (t >> (15 * 8)) & 255 == 0 { 1 << 15 } else {0};

        p = unsafe { p.add(1) };
    }
    counter = (p as usize - start as usize) * 16;
    while counter < s.len()
    {
        if ss[counter ] == 'a' as u8 { return counter; }
        counter += 1;
    }


    return s.len();
}











fn find_u8_simd_fake4(s: &str) -> usize
{
    let v = 0x6161_6161_6161_6161_6161_6161_6161_6161;
    
    let mut counter = 0;
    let mut p = s.as_ptr() as *const u128;
    let start = p;
    let end = unsafe { p.offset((s.len() / 16 ) as isize ) };
    while p < end
    {
        //let slice = &ss[counter..counter + 16];
        //let mut t = unsafe{ *slice.as_ptr() as u128 };
        let mut t = unsafe{ *p as u128 };
 
        t ^= v;

        counter = (p as isize - start as isize) as usize;

        if (t & (255 << (0  * 8))) == 0 { return counter + 0; }
        if (t & (255 << (1  * 8))) == 0 { return counter + 1; }
        if (t & (255 << (2  * 8))) == 0 { return counter + 2; }
        if (t & (255 << (3  * 8))) == 0 { return counter + 3; }
 
        if (t & (255 << (4  * 8))) == 0 { return counter + 4; }
        if (t & (255 << (5  * 8))) == 0 { return counter + 5; }
        if (t & (255 << (6  * 8))) == 0 { return counter + 6; }
        if (t & (255 << (7  * 8))) == 0 { return counter + 7; }
 
        
        if (t & (255 << (8  * 8))) == 0 { return counter + 8; }
        if (t & (255 << (9  * 8))) == 0 { return counter + 9; }
        if (t & (255 << (10 * 8))) == 0 { return counter + 10; }
        if (t & (255 << (11 * 8))) == 0 { return counter + 11; }

        if (t & (255 << (12 * 8))) == 0 { return counter + 12; }
        if (t & (255 << (13 * 8))) == 0 { return counter + 13; }
        if (t & (255 << (14 * 8))) == 0 { return counter + 14; }
        if (t & (255 << (15 * 8))) == 0 { return counter + 15; }

        p = unsafe { p.add(1) };
    }

    /*
    counter = counter - 16;
    while counter < s.len()
    {
        if ss[counter ] == 'a' as u8 { return counter; }
        counter += 1;
    }
*/

    return s.len();
}




fn find_u8_simd_fake6(s: &str) -> usize
{
    let v = 0x6161_6161_6161_6161_6161_6161_6161_6161;
    let ss = s.as_bytes();
    
    
    let mut counter = 0;
    let mut p = ss.as_ptr() as *const u128;
    while counter + 16 <= s.len()
    {
        //let slice = &ss[counter..counter + 16];
        //let mut t = unsafe{ *slice.as_ptr() as u128 };
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
        p = unsafe {p.add(1)};
    }

    counter = counter - 16;
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}


fn find_u8_simd_fake6_2(s: &str) -> usize
{
    let v = 0x6161_6161_6161_6161_6161_6161_6161_6161;
    let ss = s.as_bytes();
    
    let mut dumb_stuff: [u128; 4] =  [0; 4];

    let mut counter = 0;
    let mut p = ss.as_ptr() as *const u128;
    'boo:
    while counter + 16 * 4 <= s.len()
    {
        //let slice = &ss[counter..counter + 16];
        //let mut t = unsafe{ *slice.as_ptr() as u128 };
        for i in 0 .. 4
        {
            dumb_stuff[i] = !unsafe{ *p.offset(i as isize) } ^ v;
            dumb_stuff[i] &= ((dumb_stuff[i] & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0) >> 4) | ((dumb_stuff[i] & 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f) << 4);
            dumb_stuff[i] &= ((dumb_stuff[i] & 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc) >> 2) | ((dumb_stuff[i] & 0x3333_3333_3333_3333_3333_3333_3333_3333) << 2); // &
            dumb_stuff[i] &= ((dumb_stuff[i] & 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa) >> 1) | ((dumb_stuff[i] & 0x5555_5555_5555_5555_5555_5555_5555_5555) << 1); // &
        }


        let mut pp = dumb_stuff.as_ptr() as *const u128;
        for _ in 0..16 / 4
        {
            if unsafe { *pp != 0} 
            {
                break 'boo;
            }
            counter += 16;
            pp = unsafe { pp.add(1) };
        }

        p = unsafe { p.add(4)};
    }
   
    
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}




fn find_u8_simd_fake7(s: &str) -> usize
{
    let v = 0x6161_6161_6161_6161_6161_6161_6161_6161;
    let ss = s.as_bytes();
    
    
    let mut counter = 0;
    let mut u = 0u128;
    let mut p = ss.as_ptr() as *const u128;
    while counter + 16 <= s.len() && u == 0
    {
        //let slice = &ss[counter..counter + 16];
        //let mut t = unsafe{ *(slice.as_ptr() as *const u128) };
        let mut t = unsafe{ *p };
 
        t ^= v;
        t = !t;
        u = t;

        
        u &= ((u & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f) << 4);
        u &= ((u & 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc) >> 2) | ((u & 0x3333_3333_3333_3333_3333_3333_3333_3333) << 2); // &
        u &= ((u & 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa) >> 1) | ((u & 0x5555_5555_5555_5555_5555_5555_5555_5555) << 1); // &


/*
        t &
        (((t & 0xfefe_fefe_fefe_fefe_fefe_fefe_fefe_fefe) >> 1) | ((t & 0x0101_0101_0101_0101_0101_0101_0101_0101) << 7)) &
        (((t & 0xfcfc_fcfc_fcfc_fcfc_fcfc_fcfc_fcfc_fcfc) >> 2) | ((t & 0x0303_0303_0303_0303_0303_0303_0303_0303) << 6)) &
        (((t & 0xf8f8_f8f8_f8f8_f8f8_f8f8_f8f8_f8f8_f8f8) >> 3) | ((t & 0x0707_0707_0707_0707_0707_0707_0707_0707) << 5)) &

        (((t & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0) >> 4) | ((t & 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f) << 4)) &
        (((t & 0xe0e0_e0e0_e0e0_e0e0_e0e0_e0e0_e0e0_e0e0) >> 5) | ((t & 0x1f1f_1f1f_1f1f_1f1f_1f1f_1f1f_1f1f_1f1f) << 3)) &
        (((t & 0xc0c0_c0c0_c0c0_c0c0_c0c0_c0c0_c0c0_c0c0) >> 6) | ((t & 0x3f3f_3f3f_3f3f_3f3f_3f3f_3f3f_3f3f_3f3f) << 2)) &

        (((t & 0x8080_8080_8080_8080_8080_8080_8080_8080) >> 7) | ((t & 0x7f7f_7f7f_7f7f_7f7f_7f7f_7f7f_7f7f_7f7f) << 1));
*/
        counter += 16;
        p = unsafe { p.add(1) };
    }
    counter = counter - 16;
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}


fn find_u8_simd_fake7_32(s: &str) -> usize
{
    let v = 0x6161_6161;
    let ss = s.as_bytes();
    
    
    let mut counter = 0;
    let mut u = 0u32;
    let mut p = ss.as_ptr() as *const u32;
    while counter + 4 <= s.len() && u == 0
    {
        let mut t = unsafe{ *p };
 
        t ^= v;
        t = !t;
        u = t;

        
        u &= ((u & 0xf0f0_f0f0) >> 4) | ((u & 0x0f0f_0f0f) << 4);
        u &= ((u & 0xcccc_cccc) >> 2) | ((u & 0x3333_3333) << 2); // &
        u &= ((u & 0xaaaa_aaaa) >> 1) | ((u & 0x5555_5555) << 1); // &


/*
        t &
        (((t & 0xfefe_fefe_fefe_fefe_fefe_fefe_fefe_fefe) >> 1) | ((t & 0x0101_0101_0101_0101_0101_0101_0101_0101) << 7)) &
        (((t & 0xfcfc_fcfc_fcfc_fcfc_fcfc_fcfc_fcfc_fcfc) >> 2) | ((t & 0x0303_0303_0303_0303_0303_0303_0303_0303) << 6)) &
        (((t & 0xf8f8_f8f8_f8f8_f8f8_f8f8_f8f8_f8f8_f8f8) >> 3) | ((t & 0x0707_0707_0707_0707_0707_0707_0707_0707) << 5)) &

        (((t & 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0) >> 4) | ((t & 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f) << 4)) &
        (((t & 0xe0e0_e0e0_e0e0_e0e0_e0e0_e0e0_e0e0_e0e0) >> 5) | ((t & 0x1f1f_1f1f_1f1f_1f1f_1f1f_1f1f_1f1f_1f1f) << 3)) &
        (((t & 0xc0c0_c0c0_c0c0_c0c0_c0c0_c0c0_c0c0_c0c0) >> 6) | ((t & 0x3f3f_3f3f_3f3f_3f3f_3f3f_3f3f_3f3f_3f3f) << 2)) &

        (((t & 0x8080_8080_8080_8080_8080_8080_8080_8080) >> 7) | ((t & 0x7f7f_7f7f_7f7f_7f7f_7f7f_7f7f_7f7f_7f7f) << 1));
*/
        counter += 4;
        p = unsafe { p.add(1) };
    }
    counter = counter - 4;
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}


fn find_u8_simd_fake7_2(s: &str) -> usize
{
    let v = 0x6161_6161_6161_6161_6161_6161_6161_6161;
    let ss = s.as_bytes();
    
    
    let mut counter = 0;


    let mut t1 = 0u128;
    let mut t2 = 0u128;
    let mut t3 = 0u128;
    let mut t4 = 0u128;

    let mut t5 = 0u128;
    let mut t6 = 0u128;
    let mut t7 = 0u128;
    let mut t8 = 0u128;

    let u0 = 0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0;
    let u1 = 0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc;
    let u2 = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa;

    let u3 = 0x0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f_0f0f;
    let u4 = 0x3333_3333_3333_3333_3333_3333_3333_3333;
    let u5 = 0x5555_5555_5555_5555_5555_5555_5555_5555;


    let mut p = ss.as_ptr() as *const u128;
    while counter + 16 * 8 <= s.len() && t1 == 0 && t2 == 0 && t3 == 0 && t4 == 0 && t5 == 0 && t6 == 0 && t7 == 0 && t8 == 0
    {
        //let slice = &ss[counter..counter + 16];
        t1 = !(unsafe{ *p.offset(0) } ^ v);
        t2 = !(unsafe{ *p.offset(1) } ^ v);
        t3 = !(unsafe{ *p.offset(2) } ^ v);
        t4 = !(unsafe{ *p.offset(3) } ^ v);

        t5 = !(unsafe{ *p.offset(4) } ^ v);
        t6 = !(unsafe{ *p.offset(5) } ^ v);
        t7 = !(unsafe{ *p.offset(6) } ^ v);
        t8 = !(unsafe{ *p.offset(7) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2); // &
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1); // &

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2); // &
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1); // &

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2); // &
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1); // &

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2); // &
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1); // &

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2); // &
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1); // &

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2); // &
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1); // &

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2); // &
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1); // &

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2); // &
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1); // &


        //u = t1 | t2 | t3 | t4 | t5 | t6 | t7 | t8;

        counter += 16 * 8;
        p = unsafe { p.add(8) };
    }
    counter = counter - 16 * 8;
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}

fn find_u8_simd_fake7_2_32(s: &str) -> usize
{
    let v = 0x6161_6161;
    let ss = s.as_bytes();
    
    let mut t1 = 0u32;
    let mut t2 = 0u32;
    let mut t3 = 0u32;
    let mut t4 = 0u32;

    let mut t5 = 0u32;
    let mut t6 = 0u32;
    let mut t7 = 0u32;
    let mut t8 = 0u32;


    let u0 = 0xF0F0_F0F0u32;
    let u1 = 0xCCCC_CCCCu32;
    let u2 = 0xAAAA_AAAAu32;

    let u3 = 0x0F0F_0F0Fu32;
    let u4 = 0x3333_3333u32;
    let u5 = 0x5555_5555u32;

    //let mut u1 = 0u128;
    //let mut u2 = 0u128;
    let mut p = ss.as_ptr() as *const u32;
    let start = p;
    let end = unsafe { start.offset((s.len() / 4) as isize - 8 as isize) };

    while p < end && t1 == 0 && t2 == 0 && t3 == 0 && t4 == 0 && t5 == 0 && t6 == 0 && t7 == 0 && t8 == 0
    {
        t1 = !(unsafe{ *p.offset(0) } ^ v);
        t2 = !(unsafe{ *p.offset(1) } ^ v);
        t3 = !(unsafe{ *p.offset(2) } ^ v);
        t4 = !(unsafe{ *p.offset(3) } ^ v);

        t5 = !(unsafe{ *p.offset(4) } ^ v);
        t6 = !(unsafe{ *p.offset(5) } ^ v);
        t7 = !(unsafe{ *p.offset(6) } ^ v);
        t8 = !(unsafe{ *p.offset(7) } ^ v);

        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2); // &
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1); // &

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2); // &
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1); // &

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2); // &
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1); // &

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2); // &
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1); // &

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2); // &
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1); // &

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2); // &
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1); // &

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2); // &
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1); // &

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2); // &
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1); // &

        p = unsafe { p.add(8) };
    }

    let mut counter = (p as isize - start as isize) as usize - 8 * 4;
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}


fn find_u8_simd_fake7_2_64(s: &str) -> usize
{
    let v = 0x6161_6161;
    let ss = s.as_bytes();
    
    let mut t1 = 0u32;
    let mut t2 = 0u32;
    let mut t3 = 0u32;
    let mut t4 = 0u32;

    let mut t5 = 0u32;
    let mut t6 = 0u32;
    let mut t7 = 0u32;
    let mut t8 = 0u32;

    let mut r1 = 0u32;
    let mut r2 = 0u32;
    let mut r3 = 0u32;
    let mut r4 = 0u32;

    let mut r5 = 0u32;
    let mut r6 = 0u32;
    let mut r7 = 0u32;
    let mut r8 = 0u32;

    let u0 = 0xF0F0_F0F0u32;
    let u1 = 0xCCCC_CCCCu32;
    let u2 = 0xAAAA_AAAAu32;

    let u3 = 0x0F0F_0F0Fu32;
    let u4 = 0x3333_3333u32;
    let u5 = 0x5555_5555u32;

    let mut p = ss.as_ptr() as *const u32;
    let start = p;
    let end = unsafe { start.offset((s.len() / 4) as isize - 16 as isize) };

    while p < end && 
        t1 == 0 && t2 == 0 && t3 == 0 && t4 == 0 && t5 == 0 && t6 == 0 && t7 == 0 && t8 == 0 &&
        r1 == 0 && r2 == 0 && r3 == 0 && r4 == 0 && r5 == 0 && r6 == 0 && r7 == 0 && r8 == 0
    {
        t1 = !(unsafe{ *p.offset(0) } ^ v);
        t2 = !(unsafe{ *p.offset(1) } ^ v);
        t3 = !(unsafe{ *p.offset(2) } ^ v);
        t4 = !(unsafe{ *p.offset(3) } ^ v);

        t5 = !(unsafe{ *p.offset(4) } ^ v);
        t6 = !(unsafe{ *p.offset(5) } ^ v);
        t7 = !(unsafe{ *p.offset(6) } ^ v);
        t8 = !(unsafe{ *p.offset(7) } ^ v);

        r1 = !(unsafe{ *p.offset(8) } ^ v);
        r2 = !(unsafe{ *p.offset(9) } ^ v);
        r3 = !(unsafe{ *p.offset(10) } ^ v);
        r4 = !(unsafe{ *p.offset(11) } ^ v);

        r5 = !(unsafe{ *p.offset(12) } ^ v);
        r6 = !(unsafe{ *p.offset(13) } ^ v);
        r7 = !(unsafe{ *p.offset(14) } ^ v);
        r8 = !(unsafe{ *p.offset(15) } ^ v);


        t1 &= ((t1 & u0) >> 4) | ((t1 & u3) << 4);
        t1 &= ((t1 & u1) >> 2) | ((t1 & u4) << 2); // &
        t1 &= ((t1 & u2) >> 1) | ((t1 & u5) << 1); // &

        t2 &= ((t2 & u0) >> 4) | ((t2 & u3) << 4);
        t2 &= ((t2 & u1) >> 2) | ((t2 & u4) << 2); // &
        t2 &= ((t2 & u2) >> 1) | ((t2 & u5) << 1); // &

        t3 &= ((t3 & u0) >> 4) | ((t3 & u3) << 4);
        t3 &= ((t3 & u1) >> 2) | ((t3 & u4) << 2); // &
        t3 &= ((t3 & u2) >> 1) | ((t3 & u5) << 1); // &

        t4 &= ((t4 & u0) >> 4) | ((t4 & u3) << 4);
        t4 &= ((t4 & u1) >> 2) | ((t4 & u4) << 2); // &
        t4 &= ((t4 & u2) >> 1) | ((t4 & u5) << 1); // &

        t5 &= ((t5 & u0) >> 4) | ((t5 & u3) << 4);
        t5 &= ((t5 & u1) >> 2) | ((t5 & u4) << 2); // &
        t5 &= ((t5 & u2) >> 1) | ((t5 & u5) << 1); // &

        t6 &= ((t6 & u0) >> 4) | ((t6 & u3) << 4);
        t6 &= ((t6 & u1) >> 2) | ((t6 & u4) << 2); // &
        t6 &= ((t6 & u2) >> 1) | ((t6 & u5) << 1); // &

        t7 &= ((t7 & u0) >> 4) | ((t7 & u3) << 4);
        t7 &= ((t7 & u1) >> 2) | ((t7 & u4) << 2); // &
        t7 &= ((t7 & u2) >> 1) | ((t7 & u5) << 1); // &

        t8 &= ((t8 & u0) >> 4) | ((t8 & u3) << 4);
        t8 &= ((t8 & u1) >> 2) | ((t8 & u4) << 2); // &
        t8 &= ((t8 & u2) >> 1) | ((t8 & u5) << 1); // &


        r1 &= ((r1 & u0) >> 4) | ((r1 & u3) << 4);
        r1 &= ((r1 & u1) >> 2) | ((r1 & u4) << 2); // &
        r1 &= ((r1 & u2) >> 1) | ((r1 & u5) << 1); // &

        r2 &= ((r2 & u0) >> 4) | ((r2 & u3) << 4);
        r2 &= ((r2 & u1) >> 2) | ((r2 & u4) << 2); // &
        r2 &= ((r2 & u2) >> 1) | ((r2 & u5) << 1); // &

        r3 &= ((r3 & u0) >> 4) | ((r3 & u3) << 4);
        r3 &= ((r3 & u1) >> 2) | ((r3 & u4) << 2); // &
        r3 &= ((r3 & u2) >> 1) | ((r3 & u5) << 1); // &

        r4 &= ((r4 & u0) >> 4) | ((r4 & u3) << 4);
        r4 &= ((r4 & u1) >> 2) | ((r4 & u4) << 2); // &
        r4 &= ((r4 & u2) >> 1) | ((r4 & u5) << 1); // &

        r5 &= ((r5 & u0) >> 4) | ((r5 & u3) << 4);
        r5 &= ((r5 & u1) >> 2) | ((r5 & u4) << 2); // &
        r5 &= ((r5 & u2) >> 1) | ((r5 & u5) << 1); // &

        r6 &= ((r6 & u0) >> 4) | ((r6 & u3) << 4);
        r6 &= ((r6 & u1) >> 2) | ((r6 & u4) << 2); // &
        r6 &= ((r6 & u2) >> 1) | ((r6 & u5) << 1); // &

        r7 &= ((r7 & u0) >> 4) | ((r7 & u3) << 4);
        r7 &= ((r7 & u1) >> 2) | ((r7 & u4) << 2); // &
        r7 &= ((r7 & u2) >> 1) | ((r7 & u5) << 1); // &

        r8 &= ((r8 & u0) >> 4) | ((r8 & u3) << 4);
        r8 &= ((r8 & u1) >> 2) | ((r8 & u4) << 2); // &
        r8 &= ((r8 & u2) >> 1) | ((r8 & u5) << 1); // &

        p = unsafe { p.add(16) };

  
    }

    let mut counter = (p as isize - start as isize) as usize - 16 * 4;
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}



fn find_u8_simd_fake7_2_64_2(s: &str) -> usize
{
    let v = 0x6161_6161;
    let ss = s.as_bytes();
    

    let mut t1: [u32; 128] = [0; 128];
    let mut comb = 0u32;

    let u0 = 0xF0F0_F0F0u32;
    let u1 = 0xCCCC_CCCCu32;
    let u2 = 0xAAAA_AAAAu32;

    let u3 = 0x0F0F_0F0Fu32;
    let u4 = 0x3333_3333u32;
    let u5 = 0x5555_5555u32;

    let mut p = ss.as_ptr() as *const u32;
    let start = p;
    let end = unsafe { start.offset((s.len() / 4) as isize - 128 as isize) };
    
    while p < end && comb == 0 
    {
        comb = 0u32;
        for i in 0..128 
        {
            let tmp = &mut t1[i];
            *tmp = !(unsafe{ *p.offset(i as isize) } ^ v);
            
            *tmp &= ((*tmp & u0) >> 4) | ((*tmp & u3) << 4);
            *tmp &= ((*tmp & u1) >> 2) | ((*tmp & u4) << 2); // &
            *tmp &= ((*tmp & u2) >> 1) | ((*tmp & u5) << 1); // &
            comb |= *tmp;
        }
        p = unsafe { p.add(128) };
    }

    let mut counter = (p as isize - start as isize) as usize - 128 * 4;
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}













fn find_u8_simd_fake8(s: &str) -> usize
{
    let v = unsafe { _mm_set1_epi8(0x61i8) };
    let ss = s.as_bytes();
    
    
    let mut counter = 0;
    let zero_u = unsafe { _mm_set1_epi8(0) } ;

    while counter + 16 <= s.len()
    {
        let slice = &ss[counter..counter + 16];
        //let t = unsafe{ _mm_loadl_epi64(slice.as_ptr() as *const __m128i ) };
        let t = unsafe{ _mm_loadl_epi64(slice.as_ptr() as *const __m128i ) };
        //let a: __m256 = unsafe { _mm256_load_epi32(slice.as_ptr() as *const i32 ) };
        
        let u = unsafe { _mm_cmpeq_epi8(t, v) };
        counter += 16;

        if unsafe{ _mm_test_all_zeros(u, zero_u) } != 0
        {
            break;
        }
    }

    counter = counter - 16;
    
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}




fn find_u8_simd_fake9(s: &str) -> usize
{
    let v = unsafe { _mm_set1_epi8(0x61i8) };
    let ss = s.as_bytes();
    
    
    let mut counter = 0;
    let zero_u = unsafe { _mm_set1_epi8(0) } ;

    let mut p = ss.as_ptr();
    
    while counter + 16 <= s.len()
    {
        unsafe {
        //let slice = &ss[counter..counter + 16];
        //let t = unsafe{ _mm_loadl_epi64(slice.as_ptr() as *const __m128i ) };
            let t = _mm_loadl_epi64(p as *const __m128i );
            
            
            let u = _mm_cmpeq_epi8(t, v);
            counter += 16;
            p =p.add(16);
            if _mm_test_all_zeros(u, zero_u) != 0
            {
                break;
            }
        }
    }

    counter = counter - 16;
    
    while counter < s.len()
    {
        if ss[counter ] == 0x61 { return counter; }
        counter += 1;
    }


    return s.len();
}




fn find_u8_simd_fake5(s: &str) -> usize
{
    let v = 0x61616161;
    
    let ss = s.as_bytes();
    
    let mut counter = 0;
    while counter < s.len()
    {
        let slice = &ss[counter..counter + 4];
        let mut t = unsafe{ *slice.as_ptr() as u32 };
 
        t ^= v;

        if ( t & 0x000000ff ) == 0 { return counter + 0; }
        if ( t & 0x0000ff00 ) == 0 { return counter + 1; }
        if ( t & 0x00ff0000 ) == 0 { return counter + 2; }
        if ( t & 0xff000000 ) == 0 { return counter + 3; }

        counter += 4;
    }

    counter = counter - 4;
    while counter < s.len()
    {
        if ss[counter ] == 'a' as u8 { return counter; }
        counter += 1;
    }


    return s.len();
}





fn print_find(s: &str, s_len: usize, type_str: &str, f: fn(s: &str) -> usize)
{
    
    for _ in 0..10
    {
        let timer = std::time::Instant::now();
        let pos = f(s);
        let t = timer.elapsed().as_micros();

        if pos == s_len
        {
            println!("Failed to find character in {}, duration {}", type_str, t);
        }
        else
        {
            println!("Found character in {}, pos: {}, duration {}", type_str, pos, t);
        }
    }
    println!("");
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


    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"iter", find_iter);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"u8 array", find_u8);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using find char", find_find_char);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using find str", find_find_str);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd", find_u8_simd_fake);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 2", find_u8_simd_fake2);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 3", find_u8_simd_fake3);
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    
    print_find(&s, s_len,"using fakesimd 3_2", find_u8_simd_fake3_2);
    std::thread::sleep(std::time::Duration::from_millis(100));
    

    print_find(&s, s_len,"using fakesimd 4", find_u8_simd_fake4);
    std::thread::sleep(std::time::Duration::from_millis(100));
    //print_find(&s, s_len,"using fakesimd 5", find_u8_simd_fake5);
    //std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 6", find_u8_simd_fake6);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 6_2", find_u8_simd_fake6_2);
    std::thread::sleep(std::time::Duration::from_millis(100));
    


    print_find(&s, s_len,"using fakesimd 7", find_u8_simd_fake7);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 7_2", find_u8_simd_fake7_2);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 7_32",  find_u8_simd_fake7_32);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 7_2_32",  find_u8_simd_fake7_2_32);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 7_2_64",  find_u8_simd_fake7_2_64);

    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 7_2_64_2",  find_u8_simd_fake7_2_64_2);
    
   

    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using simd 8", find_u8_simd_fake8);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using simd 9", find_u8_simd_fake9);

    
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using find char", find_find_char);


    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"u8 ptr?", find_u8_2);
    
}




