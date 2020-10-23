use num_traits::Float;

pub fn lerp<F: Float>(x1: F, x2: F, y1: F, y2: F, x: F) -> F {
	let denom = x2 - x1;
	if denom == F::zero() {
		y1 // should not ever happen
	} else {
		// calculate decimal position of x
		let dx = (x - x1) / denom;
		// use weighted sum method of interpolating
		dx * y2 + (F::one() - dx) * y1
	}
}

#[inline]
pub fn lerp_r<F: Float>(x1: F, x2: F, y1: F, y2: F, y: F) -> F {
	lerp(y1, y2, x1, x2, y)
}
