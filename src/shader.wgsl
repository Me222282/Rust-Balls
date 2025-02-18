struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput
{
    var out: VertexOutput;
    out.uv = in.uv;
    out.clip_position = vec4<f32>(in.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let p = in.uv - vec2<f32>(0.5, 0.5);
    if ((p.x * p.x + p.y * p.y) > 0.25)
    {
        discard;
    }
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}