struct VSOut {
    @builtin(position) Position: vec4f,
    @location(0) color: vec3f,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> a: CameraUniform;

@vertex
fn vs_main(@location(0) inPos: vec3f,
           @location(1) inColor: vec3f) -> VSOut {
    var vsOut: VSOut;
    vsOut.Position = vec4f(inPos, 1.0) * a.view_proj;
    vsOut.color = inColor;

    return vsOut;
}

@fragment
fn fs_main(@location(0) inColor: vec3f) -> @location(0) vec4f {
    return vec4f(inColor, 1);
}