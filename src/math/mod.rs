pub use nalgebra_glm::*;

pub fn radians_f32(i: f32)->f32{
    return *radians(&vec1(i)).index((0,0));
}
