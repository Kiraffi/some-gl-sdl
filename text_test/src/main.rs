
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;


#[cfg(target_arch = "x86")]
use std::arch::x86::*;




fn add_chars(s: &mut std::string::String)
{
    for i in 0..1_000_000
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
    let mut counter = 0;
    let mut p = s.as_bytes().as_ptr();
    while counter < s.len()
    {
        unsafe {
        if *p == 'a' as u8
        {
            return counter;
        }
        p = p.add(1);
        };
        counter += 1;
        
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
    
    
    let mut counter = 0;
    while counter + 8 <= s.len()
    {
        let slice = &ss[counter..counter + 8];
        let mut t = unsafe { *slice.as_ptr() as u64 };
 
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
    
    
    let mut counter = 0;
    while counter + 16 <= s.len()
    {
        let slice = &ss[counter..counter + 16];
        let mut t = unsafe{ *slice.as_ptr() as u128 };
 
        t ^= v;

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












fn find_u8_simd_fake4(s: &str) -> usize
{
    let v = 0x6161_6161_6161_6161_6161_6161_6161_6161;
    let ss = s.as_bytes();
    
    
    let mut counter = 0;
    let mut p = s.as_ptr();
    while counter + 16 <= s.len()
    {
        //let slice = &ss[counter..counter + 16];
        //let mut t = unsafe{ *slice.as_ptr() as u128 };
        let mut t = unsafe{ *p as u128 };
 
        t ^= v;

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

        counter += 16;

        p = unsafe { p.add(16) };
    }

    counter = counter - 16;
    while counter < s.len()
    {
        if ss[counter ] == 'a' as u8 { return counter; }
        counter += 1;
    }


    return s.len();
}




fn find_u8_simd_fake6(s: &str) -> usize
{
    let v = 0x6161_6161_6161_6161_6161_6161_6161_6161;
    let ss = s.as_bytes();
    
    
    let mut counter = 0;
    let mut p = ss.as_ptr();
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
        p = unsafe {p.add(16)};
    }

    counter = counter - 16;
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
    while counter + 16 <= s.len() && u == 0
    {
        let slice = &ss[counter..counter + 16];
        let mut t = unsafe{ *slice.as_ptr() as u128 };
 
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
        
    }
    counter = counter - 16;
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
    print_find(&s, s_len,"using fakesimd 4", find_u8_simd_fake4);
    std::thread::sleep(std::time::Duration::from_millis(100));
    //print_find(&s, s_len,"using fakesimd 5", find_u8_simd_fake5);
    //std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 6", find_u8_simd_fake6);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using fakesimd 7", find_u8_simd_fake7);
   
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using simd 8", find_u8_simd_fake8);
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using simd 9", find_u8_simd_fake9);

    
    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"using find char", find_find_char);


    std::thread::sleep(std::time::Duration::from_millis(100));
    print_find(&s, s_len,"u8 ptr?", find_u8_2);
    
}




