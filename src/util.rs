use num_traits::{Float, Num, NumCast};

pub fn lerp<T: Num + Copy>(x1: T, x2: T, y1: T, y2: T, x: T) -> T {
	let denom = x2 - x1;
	if denom == T::zero() {
		y1 // should never happen
	} else {
		// calculate decimal position of x
		let dx = (x - x1)/denom;
		// use weighted sum method of interpolating
		dx*y2 + (T::one() - dx)*y1
	}
}

pub fn lerp_r<T: Num + Copy>(x1: T, x2: T, y1: T, y2: T, y: T) -> T {
	lerp(y1, y2, x1, x2, y)
}

pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
	if x < min { min } else if x > max { max } else { x }
}

pub fn amp_to_db<T: Float + NumCast>(x: T) -> T {
	T::from(20.).unwrap() * x.log10()
}

pub fn db_to_amp<T: Float + NumCast>(x: T) -> T {
	T::from(10.0).unwrap().powf(x / T::from(20.).unwrap())
}