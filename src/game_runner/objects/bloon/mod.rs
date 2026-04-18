mod regular;
//use super::super::maps;

struct map_thingie {
	nah: u32,
}

pub trait Enemy<'a> {
	fn new(hp: u16, location: (f32, f32), map: &'a map_thingie) -> Self;
/*
	fn draw(&mut self);
	fn move_enemy(&mut self);
	fn damage(&mut self, dmg_amt: u32);
*/
}
