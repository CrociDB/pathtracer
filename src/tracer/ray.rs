use cgmath;
use cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>
}

impl Ray {
    pub fn point_at_parameter(&self, p:f32) -> Vector3<f32> {
        self.origin + (p * self.direction)
    }
}
