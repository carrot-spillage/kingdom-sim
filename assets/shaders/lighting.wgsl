#import bevy_pbr::utils::{coords_to_viewport_uv}
#import bevy_sprite::mesh2d_view_bindings::{view}
#import bevy_sprite::mesh2d_vertex_output::{VertexOutput}

@group(2) @binding(0)
var texture: texture_2d<f32>;

@group(2) @binding(1)
var our_sampler: sampler;

@group(2) @binding(2)
var<uniform> color_distortion: vec4<f32>;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let uv = coords_to_viewport_uv(mesh.position.xy, view.viewport);

    // Sample the texture using the provided sampler and UV coordinates.
    var color = vec4<f32>(textureSample(texture, our_sampler, uv));

    var r = color.r * color_distortion.r;
    var g = color.g * color_distortion.g;
    var b = color.b * color_distortion.b;

    return vec4<f32>(select(r, 1.0, r > 1.0), select(g, 1.0, g > 1.0), select(b, 1.0, b > 1.0), color.a);
}