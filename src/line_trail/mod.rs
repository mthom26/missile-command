use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::{state::GameState, AssetHandles};

const VERTEX_SHADER: &str = include_str!("shader.vert");
const FRAGMENT_SHADER: &str = include_str!("shader.frag");

struct LineTrail;

// Event to spawn LineTrail
pub struct SpawnLineTrail {
    pub position: Vec3,
    pub rotation: f32,
}

pub struct LineTrailPlugin;
impl Plugin for LineTrailPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnLineTrail>()
            .add_startup_system(setup.system())
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(spawn_line_trails.system()),
            );
    }
}

fn setup(
    mut asset_handles: ResMut<AssetHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
) {
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    // Quad
    let (scale_x, scale_y) = (5.0, 80.0);
    let mut rect = Mesh::new(PrimitiveTopology::TriangleList);
    let vertexes = vec![
        [-1.0 * scale_x, -1.0 * scale_y, 0.0],
        [1.0 * scale_x, -1.0 * scale_y, 0.0],
        [1.0 * scale_x, 1.0 * scale_y, 0.0],
        [-1.0 * scale_x, 1.0 * scale_y, 0.0],
    ];
    rect.set_attribute(Mesh::ATTRIBUTE_POSITION, vertexes);
    let colors = vec![
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 1.0, 0.0],
    ];
    rect.set_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    let indices = vec![0, 1, 2, 0, 2, 3];
    rect.set_indices(Some(Indices::U32(indices)));
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [1.0, 1.0]];
    rect.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    asset_handles.line_trail = meshes.add(rect);
    asset_handles.line_trail_pipeline = pipeline_handle;
}

fn spawn_line_trails(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut events: EventReader<SpawnLineTrail>,
) {
    for e in events.iter() {
        commands
            .spawn_bundle(MeshBundle {
                mesh: asset_handles.line_trail.clone(),
                transform: Transform {
                    translation: e.position,
                    rotation: Quat::from_rotation_z(e.rotation),
                    ..Default::default()
                },
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    asset_handles.line_trail_pipeline.clone(),
                )]),
                ..Default::default()
            })
            .insert(LineTrail);
    }
}
