use std::os::raw::c_void;

pub struct MyStructHere
{
    pub v1: i32,
    pub v2: i32
}

pub fn my_callback_2( tr:&mut dyn base_lib::TestTrait, data: *mut c_void, f: fn(*mut c_void, &mut MyStructHere))
{
    let mut p = MyStructHere{v1: 10, v2: 20};
    f(data, &mut p);
    println!("p after call: {}, {}", p.v1, p.v2);

    tr.foo_man();

}
