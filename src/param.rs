use num_traits::clamp;
use asprim::AsPrim;

use util::*;

pub struct ParamDef {
	pub name: &'static str,
	pub min: f32,
	pub max: f32,
	pub default: f32,
}

impl ParamDef {
	pub fn new(name: &'static str, min: f32, max: f32, default: f32) -> Self {
		Self {
			name: name,
			min: min,
			max: max,
			default: default,
		}
	}
}

pub struct Param {
	pub def: ParamDef,
	pub val: f32,
}

impl Param {
	pub fn new(param: ParamDef) -> Self {
		Self {
			val: param.default,
			def: param,
		}
	}

	pub fn norm(&self) -> f32 { // TODO: support non-linear relationships
		lerp_r(0.0, 1.0, self.def.min.as_(), self.def.max.as_(), self.val.as_()).as_f32()
	}

	pub fn set(&mut self, val: f32) {
		self.val = clamp(val, self.def.min, self.def.max);
	}
	pub fn user_sets_norm(&mut self, val: f32) {
		let val = lerp(0.0.as_(), 1.0.as_(), self.def.min.as_(), self.def.max.as_(), val.as_());
		self.set(val);
	}
}