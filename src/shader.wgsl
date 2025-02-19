struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct Uniform {
    matrix: mat4x4<f32>,
    colour: vec3<f32>
};
@group(0) @binding(0)
var<uniform> uni: Uniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput
{
    var out: VertexOutput;
    out.uv = in.uv;
    out.clip_position = uni.matrix * vec4<f32>(in.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.uv - vec2<f32>(0.5, 0.5);
    if ((p.x * p.x + p.y * p.y) > 0.25)
    {
        discard;
    }
    return vec4<f32>(uni.colour, 1.0);
}