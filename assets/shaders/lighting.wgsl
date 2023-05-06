#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> color_distortion: vec3<f32>;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let uv = coords_to_viewport_uv(position.xy, view.viewport);

    // Sample the texture using the provided sampler and UV coordinates.
    var color = vec4<f32>(textureSample(texture, our_sampler, uv));

    var r = color.r * color_distortion.r;
    var g = color.g * color_distortion.g;
    var b = color.b * color_distortion.b;

    return vec4<f32>(select(r, 1.0, r > 1.0), select(g, 1.0, g > 1.0), select(b, 1.0, b > 1.0), color.a);
}