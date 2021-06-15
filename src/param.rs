pub struct ParamDef {
	pub name: &'static str,
	pub min: f32,
	pub max: f32,
	pub default: f32,
}

impl ParamDef {
	pub const fn new(name: &'static str, min: f32, max: f32, default: f32) -> Self {
		Self { name, min, max, default }
	}
}
