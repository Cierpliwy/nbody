use cgmath::Vector3;
use rand::{Rand, Rng};

#[derive(Copy, Clone)]
pub struct Body {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
}

impl Body {
    pub fn radius(&self) -> f32 {
        0.1
    }
}

impl Rand for Body {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Body {
            position: rng.gen::<Vector3<f32>>() * 2.0 - Vector3::new(1.0, 1.0, 1.0),
            color: rng.gen::<Vector3<f32>>(),
        }
    }
}