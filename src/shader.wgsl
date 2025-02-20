struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) colour: vec3<f32>
};

struct InstanceInput {
    @location(3) colour: vec3<f32>,
    @location(4) location: vec2<f32>,
    @location(5) radius: f32,
};

@group(0) @binding(0)
var<uniform> matrix: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput, inst: InstanceInput) -> VertexOutput
{
    var out: VertexOutput;
    out.uv = in.uv - vec2<f32>(0.5);
    out.colour = inst.colour;
    
    var pos = (in.position.xy * vec2<f32>(inst.radius * 2.0));
    pos += inst.location;
    
    out.clip_position = matrix * vec4<f32>(pos, in.position.z, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.uv;
    if ((p.x * p.x + p.y * p.y) > 0.25)
    {
        discard;
    }
    return vec4<f32>(in.colour, 1.0);
}