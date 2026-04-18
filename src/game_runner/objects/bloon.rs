use super::*;


struct map_thingie {sm: u32}

#[repr(u32)]
enum Attribute {
    Regrow = 1 << 0,
    Magic_Immune = 1 << 1,
}

pub struct Bloon<'a>{
	hp: u16,
	location: (f32, f32),
	attr: Attribute,
	map: &'a map_thingie,
}


impl<'a> Bloon<'a> {
	pub fn new(hp: u16, attr: Attribute, location: (f32, f32), map: &'a map_thingie) -> Self {
		Self{hp: 1, location, attr, map}
	} 
}
