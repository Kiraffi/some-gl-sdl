pub struct SdlWindowState
{
	pub window_width: i32,
	pub window_height: i32,
	pub vsync: bool,

	pub timer: core::MyTimer,

	pub key_downs: [u8; 512],
	pub key_downs_previous: [u8; 512],
	pub key_half_count: [u8; 512],

	pub mouse_x: i32,
	pub mouse_y: i32,
	pub mouse_b: i32,

	pub quit: bool,
	pub resized: bool,
}
