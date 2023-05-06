//! A custom post processing effect, using two cameras, with one reusing the render texture of the first one.
//! Here a chromatic aberration is applied to a 3d scene containing a rotating cube.
//! This example is useful to implement your own post-processing effect such as
//! edge detection, blur, pixelization, vignette... and countless others.

use std::ops::Range;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_pancam::PanCam;

use crate::{create_world::WorldParams, environment_hud::SunAltitude, GameState};

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<TimeLightingMaterial>::default())
            .add_startup_system(setup)
            .add_system(main_camera_cube_rotator_system)
            .add_systems((update_lighting,).in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

/// Marks the first camera cube (rendered to a texture.)
#[derive(Component)]
struct MainCube;

fn update_lighting(
    query: Query<&SunAltitude, Changed<SunAltitude>>,
    mut materials: ResMut<Assets<TimeLightingMaterial>>,
) {
    if let Ok(sun_altitude) = query.get_single() {
        if let Some(mut material) = materials.iter_mut().next() {
            material.1.color_distortion = get_color_distortion(sun_altitude.0).extend(1.0);
        }
    }
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<TimeLightingMaterial>>,
    mut images: ResMut<Assets<Image>>,
    world_params: Res<WorldParams>,
) {
    // This assumes we only have a single window
    let window = windows.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let mut camera_bundle = Camera2dBundle::new_with_far(world_params.half_max_isometric_z * 2.0);
    camera_bundle.projection.scale = 2.0;
    camera_bundle.camera.target = RenderTarget::Image(image_handle.clone());

    commands
        // .spawn(Camera2dBundle::new_with_far(
        //     world_params.half_max_isometric_z * 2.0,
        // ))
        .spawn(camera_bundle)
        .insert(PanCam {
            grab_buttons: vec![MouseButton::Left, MouseButton::Middle], // which buttons should drag the camera
            enabled: true, // when false, controls are disabled. See toggle example.
            zoom_to_cursor: true, // whether to zoom towards the mouse or the center of the screen
            min_scale: 1., // prevent the camera from zooming too far in
            max_scale: Some(40.), // prevent the camera from zooming too far out
            ..PanCam::default()
        });

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(TimeLightingMaterial {
        color_distortion: Vec4::ONE,
        source_image: image_handle,
    });

    // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 100.0),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
    ));

    // The post-processing pass camera.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                order: 1,
                ..default()
            },
            ..Camera2dBundle::default()
        },
        post_processing_pass_layer,
    ));
}

/// Rotates the cube rendered by the main camera
fn main_camera_cube_rotator_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCube>>,
) {
    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}

static DUSK_SPAN: f32 = 0.2;
static SUNRISE_SPAN: f32 = 0.2;
static DUSK_RANGE: Range<f32> = 0.0..DUSK_SPAN;
static SUNRISE_RANGE: Range<f32> = DUSK_RANGE.end..(DUSK_RANGE.end + SUNRISE_SPAN);
static MAX_SUNRISE_DISTORTION: Vec3 = Vec3::new(0.3, 0.1, -0.3);
static MAX_DUSK_DISTORTION: Vec3 = Vec3::new(-0.5, -0.4, -0.3);

fn get_color_distortion(sun_position: f32) -> Vec3 {
    if sun_position < DUSK_RANGE.start {
        Vec3::ONE + MAX_DUSK_DISTORTION
    } else if SUNRISE_RANGE.contains(&sun_position) {
        let distortion_scale = 1.0 - (sun_position - SUNRISE_RANGE.start) / SUNRISE_SPAN;
        Vec3::ONE + (MAX_SUNRISE_DISTORTION * distortion_scale)
    } else if DUSK_RANGE.contains(&sun_position) {
        let distortion_scale = 1.0 - (sun_position - DUSK_RANGE.start) / DUSK_SPAN;
        Vec3::ONE
            + (((MAX_DUSK_DISTORTION * distortion_scale)
                + (MAX_SUNRISE_DISTORTION * (1.0 - distortion_scale)))
                / 2.0)
    } else {
        Vec3::ONE
    }
}

// Region below declares of the custom material handling post processing effect

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct TimeLightingMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,

    #[uniform(2)]
    color_distortion: Vec4,
}

impl Material2d for TimeLightingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lighting.wgsl".into()
    }
}
