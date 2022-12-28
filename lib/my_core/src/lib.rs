pub struct RandomPCG
{
    rng_seed: u64,
    rng_inc: u64
}

impl RandomPCG
{
    pub fn new(rng_seed: u64) -> RandomPCG
    {
        RandomPCG{ rng_seed, rng_inc: 0xa5dfa5dfu64 }
    }
    // From https://www.pcg-random.org/download.html
    pub fn get_next(&mut self) -> u32
    {
        let old: u64 = self.rng_seed;
        self.rng_seed = self.rng_seed.wrapping_mul(636413622384679005u64);
        self.rng_seed = self.rng_seed.wrapping_add(self.rng_inc | 1u64);
        let xor_shifted: u32  = (((old >> 18u64) ^ old) >> 27u64) as u32;
        let rot: u32 = (old >> 59u64) as u32;
        let result: u32 = (xor_shifted >> rot) | (xor_shifted << ((!rot) & 31));
        return result;
    }
}



pub fn clamp(value: f32, min: f32, max: f32) ->f32
{
    let mut v = if value > min { value } else { min };
    v = if v < max { v } else { max };
    return v;
}

pub fn get_u32_agbr_color(r: f32, g: f32, b: f32, a: f32) -> u32
{
    let r = clamp(r, 0.0f32, 1.0f32);
    let g = clamp(g, 0.0f32, 1.0f32);
    let b = clamp(b, 0.0f32, 1.0f32);
    let a = clamp(a, 0.0f32, 1.0f32);

    let mut v = 0u32;
    v += (r * 255.0f32) as u32;
    v += ((g * 255.0f32) as u32) << 8u32;
    v += ((b * 255.0f32) as u32) << 16u32;
    v += ((a * 255.0f32) as u32) << 24u32;

    return v;
}


pub const fn get_u32_agbr_color_const(r: u8, g: u8, b: u8, a: u8) -> u32
{
    let mut v = 0u32;
    v += r as u32;
    v += (g as u32) << 8u32;
    v += (b as u32) << 16u32;
    v += (a as u32) << 24u32;

    return v;
}
