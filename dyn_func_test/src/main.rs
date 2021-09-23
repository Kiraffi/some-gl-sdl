use std::os::raw::c_void;

// This function takes two integers and a function that performs some operation on the two arguments
fn apply_function<T>(a: i32, b: i32, func: T) -> i32 where T: Fn(i32, i32) -> i32 {
    // apply the passed function to arguments a and b
    func(a, b)
}

struct TestState
{
	i1: i32,
	i2: i32,
	i3: i32,
	i4: i32
}


fn test_f1(value: i32, f: fn(i32) -> i32) -> i32
{
	return f(value);
}

fn test_f2(value: i32) -> i32
{
	return value * 2;
}
fn test_f3(value: i32) -> i32
{
	return value * 3;
}


fn my_callback3(data: *mut c_void, p_value: &mut caller_lib::MyStructHere) 
{
    // unsafe is needed because we dereference a raw pointer here
    let data: &mut TestState = unsafe { &mut *(data as *mut TestState) };
    println!("state_cb3: {}, {}, {}, {}", data.i1, data.i2, data.i3, data.i4);
    data.i1 = data.i2 * data.i3;
	p_value.v1 = p_value.v1 * 7i32;
	p_value.v2 = p_value.v2 * 13i32;
}

fn my_callback(data: *mut c_void) {
    // unsafe is needed because we dereference a raw pointer here
    let data: &mut TestState = unsafe { &mut *(data as *mut TestState) };
    println!("state: {}, {}, {}, {}", data.i1, data.i2, data.i3, data.i4);
    data.i4 = data.i3 * data.i4;
}

fn test_ff(data: *mut c_void, f: fn(data: *mut c_void))
{
	return f(data);
}


fn main()
{
	println!("test2 = {}", test_f1(3, test_f2));
	println!("test3 = {}", test_f1(3, test_f3));


	let mut data = TestState { i1: 1, i2: 2, i3: 3, i4: 4 };
	let state_ptr: *mut c_void = &mut data as *mut _ as *mut c_void;
	test_ff(state_ptr, my_callback);
	println!("state2: {}, {}, {}, {}", data.i1, data.i2, data.i3, data.i4);

	let mut st = foo_impl::FooStruct{x: 10, y: 15};
	caller_lib::my_callback_2(&mut st, state_ptr, my_callback3);
	println!("state3: {}, {}, {}, {}", data.i1, data.i2, data.i3, data.i4);

	println!("did change? values: {}, {}", st.x, st.y);
	// let's define three lambdas, each operating on the same parameters
	let sum = |a, b| a + b;
	let product = |a, b| a * b;
	let diff = |a, b| a - b;

	// And now let's pass them to apply_function along with some arbitary values
	println!("3 + 6 = {}", apply_function(3, 6, sum));
	println!("-4 * 9 = {}", apply_function(-4, 9, product));
	println!("7 - (-3) = {}", apply_function(7, -3, diff));

}
