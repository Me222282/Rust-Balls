use cgmath::{Vector2, Vector3, Vector4};

#[allow(non_camel_case_types)]
pub type real = f32;
pub type Vec2 = Vector2<real>;
pub type Vec3 = Vector3<real>;
pub type Vec4 = Vector4<real>;
pub type Colour = Vector3<u8>;

pub const fn vec3(data: [real; 3]) -> Vec3
{
    return Vec3::new(data[0], data[1], data[2]);
}
pub const fn colour(data: [u8; 3]) -> Colour
{
    return Colour::new(data[0], data[1], data[2]);
}

trait Cast<T, const N: usize> {
    fn call(self) -> [T; N];
}

impl Cast<real, 3> for Vec3 {
    fn call(self) -> [real; 3] {
        return [self.x, self.y, self.z];
    }
}
impl Cast<real, 2> for Vec2 {
    fn call(self) -> [real; 2] {
        return [self.x, self.y];
    }
}
impl Cast<real, 4> for Vec4 {
    fn call(self) -> [real; 4] {
        return [self.x, self.y, self.z, self.w];
    }
}
const CON_V: f32 = 1.0 / 255.0;
impl Cast<real, 3> for Colour {
    fn call(self) -> [real; 3] {
        return [self.x as f32 * CON_V,
            self.y as f32 * CON_V,
            self.z as f32 * CON_V];
    }
}