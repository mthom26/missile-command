use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::Indices,
        pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
};

use crate::{state::GameState, team::Team, AssetHandles, Velocity};

const VERTEX_SHADER: &str = include_str!("shader.vert");
const FRAGMENT_SHADER: &str = include_str!("shader.frag");

const MAX_LINE_LENGTH: f32 = 100.0;
const LINE_GROWTH_SPEED: f32 = 80.0;
const LINE_WIDTH: f32 = 3.0;

pub struct LineTrail {
    owner: Entity, // The missile that spawned the LineTrail
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
pub struct LineMaterial {
    pub color: Color,
}

// Event to spawn LineTrail
pub struct SpawnLineTrail {
    pub position: Vec3,
    pub velocity: Vec2,
    pub rotation: f32,
    pub owner: Entity,
    pub team: Team,
}

pub struct LineTrailPlugin;
impl Plugin for LineTrailPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnLineTrail>()
            .add_asset::<LineMaterial>()
            .add_startup_system(setup.system())
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(spawn_line_trails.system())
                    .with_system(update_line_scale.system())
                    .with_system(update_line_velocity.system())
                    .with_system(despawn_line_trails.system()),
            );
    }
}

fn setup(
    mut asset_handles: ResMut<AssetHandles>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    render_graph.add_system_node(
        "line_material",
        AssetRenderResourcesNode::<LineMaterial>::new(true),
    );
    render_graph
        .add_node_edge("line_material", base::node::MAIN_PASS)
        .unwrap();

    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, VERTEX_SHADER)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, FRAGMENT_SHADER))),
    }));

    // Quad
    let mut rect = Mesh::new(PrimitiveTopology::TriangleList);
    let vertexes = vec![
        [-0.5 * LINE_WIDTH, -1.0, 0.0],
        [0.5 * LINE_WIDTH, -1.0, 0.0],
        [0.5 * LINE_WIDTH, 0.0, 0.0],
        [-0.5 * LINE_WIDTH, 0.0, 0.0],
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
    asset_handles.player_line_trail_material = materials.add(LineMaterial {
        color: Color::rgb(0.3, 0.9, 0.2),
    });
    asset_handles.enemy_line_trail_material = materials.add(LineMaterial {
        color: Color::rgb(0.9, 0.3, 0.2),
    });
}

fn spawn_line_trails(
    asset_handles: Res<AssetHandles>,
    mut commands: Commands,
    mut events: EventReader<SpawnLineTrail>,
) {
    for e in events.iter() {
        let mat = match e.team {
            Team::Player => asset_handles.player_line_trail_material.clone(),
            Team::Enemy => asset_handles.enemy_line_trail_material.clone(),
        };

        commands
            .spawn_bundle(MeshBundle {
                mesh: asset_handles.line_trail.clone(),
                transform: Transform {
                    translation: e.position,
                    rotation: Quat::from_rotation_z(e.rotation),
                    scale: Vec3::new(1.0, 0.1, 1.0),
                },
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    asset_handles.line_trail_pipeline.clone(),
                )]),
                ..Default::default()
            })
            .insert(LineTrail { owner: e.owner })
            .insert(mat)
            .insert(Velocity(e.velocity));
    }
}

fn despawn_line_trails(mut commands: Commands, query: Query<(Entity, &LineTrail, &Transform)>) {
    for (entity, _, transform) in query.iter() {
        if transform.scale.y == 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_line_scale(time: Res<Time>, mut query: Query<(&LineTrail, &Velocity, &mut Transform)>) {
    for (_, velocity, mut transform) in query.iter_mut() {
        if velocity.0 == Vec2::ZERO {
            transform.scale.y -= time.delta_seconds() * LINE_GROWTH_SPEED;
        } else {
            transform.scale.y += time.delta_seconds() * LINE_GROWTH_SPEED;
        }
        transform.scale.y = transform.scale.y.clamp(0.0, MAX_LINE_LENGTH);
    }
}

fn update_line_velocity(owner_query: Query<Entity>, mut query: Query<(&LineTrail, &mut Velocity)>) {
    for (line_trail, mut velocity) in query.iter_mut() {
        // If the owner has despawned so stop the line
        match owner_query.get(line_trail.owner) {
            Ok(_) => {}
            Err(_) => velocity.0 = Vec2::ZERO,
        }
    }
}
