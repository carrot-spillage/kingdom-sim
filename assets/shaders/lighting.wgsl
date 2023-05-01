#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@group(1) @binding(2)
var<uniform> brightness: f32;

@fragment
fn fragment(
    @builtin(position) position: vec4<f32>,
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let uv = coords_to_viewport_uv(position.xy, view.viewport);

    // Sample the texture using the provided sampler and UV coordinates.
    var output_color = textureSample(texture, our_sampler, uv);

    // Apply a nighttime color shift by blending the output color with the night color.
    output_color *= brightness;

    return output_color;
}