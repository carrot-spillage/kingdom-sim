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

use crate::{ambience::DayNightColorDistortion, create_world::WorldParams, GameState};

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<DayNightLightingMaterial>::default())
            .add_startup_system(setup)
            .add_system(update_day_night_material.in_set(OnUpdate(GameState::Playing)));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn update_day_night_material(
    mut post_processing_materials: ResMut<Assets<DayNightLightingMaterial>>,
    day_night_color_distortions: Query<&DayNightColorDistortion, Changed<DayNightColorDistortion>>,
) {
    if let Ok(day_night_color_distortion) = day_night_color_distortions.get_single() {
        post_processing_materials
            .iter_mut()
            .next()
            .unwrap()
            .1
            .color_distortion = day_night_color_distortion.0.extend(1.0);
    }
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<DayNightLightingMaterial>>,
    mut images: ResMut<Assets<Image>>,
    day_light_color_distortions: Query<&DayNightColorDistortion>,
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
    let material_handle = post_processing_materials.add(DayNightLightingMaterial {
        source_image: image_handle,
        color_distortion: Vec4::ZERO, // TODO: put an actual initial value: day_light_color_distortions.single().0.extend(1.0)
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
        (
            Camera2dBundle {
                camera: Camera {
                    // renders after the first main camera which has default value: 0.
                    order: 1,
                    ..default()
                },
                ..Camera2dBundle::default()
            },
            post_processing_pass_layer,
        ),
        UiCameraConfig { show_ui: false },
    ));
}

/// Our custom post processing material
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct DayNightLightingMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,

    #[uniform(2)]
    color_distortion: Vec4,
}

impl Material2d for DayNightLightingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/lighting.wgsl".into()
    }
}
