use cgmath::{InnerSpace, Vector2, Vector3, Vector4};

#[allow(non_camel_case_types)]
pub type real = f32;
pub type Vec2 = Vector2<real>;
pub type Vec3 = Vector3<real>;
pub type Vec4 = Vector4<real>;
pub type Colour = Vector3<u8>;

pub const fn size_bounds(width: real, height: real) -> Vec4
{
    let hw = width * 0.5;
    let hh = height * 0.5;
    return vec4(-hw, hw, hh, -hh);
}

pub const fn vec4(x: real, y: real, z: real, w: real) -> Vec4
{   
    return Vec4::new(x, y, z, w);
}
pub const fn vec3(x: real, y: real, z: real) -> Vec3
{
    return Vec3::new(x, y, z);
}
pub const fn vec2(x: real, y: real) -> Vec2
{
    return Vec2::new(x, y);
}
pub const fn colour(r: u8, g: u8, b: u8) -> Colour
{
    return Colour::new(r, g, b);
}
const CON_V: f32 = 1.0 / 255.0;
pub const fn c_to_v(colour: Colour) -> Vec3
{
    return vec3(colour.x as f32 * CON_V,
        colour.y as f32 * CON_V,
        colour.z as f32 * CON_V);
}

// trait Cast<T, const N: usize> {
//     fn call(self) -> [T; N];
// }

// impl Cast<real, 3> for Vec3 {
//     fn call(self) -> [real; 3] {
//         return [self.x, self.y, self.z];
//     }
// }
// impl Cast<real, 2> for Vec2 {
//     fn call(self) -> [real; 2] {
//         return [self.x, self.y];
//     }
// }
// impl Cast<real, 4> for Vec4 {
//     fn call(self) -> [real; 4] {
//         return [self.x, self.y, self.z, self.w];
//     }
// }
// impl Cast<real, 3> for Colour {
//     fn call(self) -> [real; 3] {
//         return [self.x as f32 * CON_V,
//             self.y as f32 * CON_V,
//             self.z as f32 * CON_V];
//     }
// }