use super::*;

struct Regular<'a>{
	hp: u16,
	location: (f32, f32),
	map: &'a map_thingie,
}


impl<'a> Enemy<'a> for Regular<'a> {
	fn new(hp: u16, location: (f32, f32), map: &'a map_thingie) -> Self {
		Self{hp: 1, location, map} 
	} 
}

