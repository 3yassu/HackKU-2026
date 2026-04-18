mod regular;
//use super::super::maps;

struct map_thingie {
	nah: u32,
}

pub trait Enemy {
	fn new(hp: u16) -> Self;
	fn draw(&mut self);
	fn enemy_set_path(&mut self, map: &map_thingie);
	fn move_enemy(&mut self);
	fn damage(&mut self, dmg_amt: u32);
}
