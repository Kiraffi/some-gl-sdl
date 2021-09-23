pub struct FooStruct
{
	pub x: i32,
	pub y: i32
}


impl base_lib::TestTrait for FooStruct 
{
	fn foo_man(&mut self) 
	{ 
		println!("calling FooStruct foo_man. Values {} {}", self.x, self.y);
		
		self.x = self.x * self.y;
	}

}
