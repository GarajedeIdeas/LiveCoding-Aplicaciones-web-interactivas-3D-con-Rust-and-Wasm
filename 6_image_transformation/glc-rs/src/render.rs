use bevy::core_pipeline::Transparent3d;
use bevy::ecs::query::QueryItem;
use bevy::ecs::system::lifetimeless::{Read, SQuery, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::pbr::{MeshPipeline, MeshPipelineKey, SetMeshBindGroup, SetMeshViewBindGroup};
use bevy::prelude::*;
use bevy::render::mesh::GpuBufferInfo;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::render_phase::{
    AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
    SetItemPipeline, TrackedRenderPass,
};
use bevy::render::render_resource::*;
use bevy::render::renderer::RenderDevice;
use bevy::render::view::ExtractedView;
use bevy::render::{RenderApp, RenderStage};
use bytemuck::{Pod, Zeroable};

pub struct GlcRenderingPlugin;

impl Plugin for GlcRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<InstancedMesh>::default());
        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, GlcDraw>()
            .init_resource::<GlcRenderingPipeline>()
            .init_resource::<SpecializedPipelines<GlcRenderingPipeline>>()
            .add_system_to_stage(RenderStage::Queue, glc_render_queue)
            .add_system_to_stage(RenderStage::Prepare, prepare_instance_buffer);
    }
}

/*
 * Custom component for storing instancing data
 */
#[derive(Component, Debug)]
pub struct InstancedMesh(pub Vec<InstanceData>);
impl ExtractComponent for InstancedMesh {
    type Query = &'static InstancedMesh;
    type Filter = ();

    fn extract_component(item: QueryItem<Self::Query>) -> Self {
        InstancedMesh(item.0.clone())
    }
}

#[derive(Clone, Copy, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct InstanceData {
    pub position: Vec3,
    pub scale: f32,
    pub color: [f32; 4],
}

/*
 * Custom rendering pipeline for simple instanced meshes
 */

pub struct GlcRenderingPipeline {
    shader: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
}

impl FromWorld for GlcRenderingPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        // Load the shader
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        asset_server.watch_for_changes().unwrap();
        let shader = asset_server.load("shaders/glc_shader.wgsl");

        // Get the mesh pipeline
        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        GlcRenderingPipeline {
            shader,
            mesh_pipeline: mesh_pipeline.clone(),
        }
    }
}

impl SpecializedPipeline for GlcRenderingPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader.clone();
        descriptor.vertex.buffers.push(VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceData>() as u64,
            step_mode: VertexStepMode::Instance,
            attributes: vec![
                // Position, Normal and UV take up locations 0-2
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 3,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: VertexFormat::Float32x4.size(),
                    shader_location: 4,
                },
            ],
        });
        descriptor.fragment.as_mut().unwrap().shader = self.shader.clone();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
        ]);
        descriptor
    }
}

/*
 * Custom drawing
 */
#[derive(Component)]
pub struct InstanceBuffer {
    buffer: Buffer,
    length: usize,
}

type GlcDraw = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    GlcDrawmMeshInstanced,
);

pub struct GlcDrawmMeshInstanced;
impl EntityRenderCommand for GlcDrawmMeshInstanced {
    type Param = (
        SRes<RenderAssets<Mesh>>,
        SQuery<Read<Handle<Mesh>>>,
        SQuery<Read<InstanceBuffer>>,
    );

    #[inline]
    fn render<'w>(
        _view: Entity,
        item: Entity,
        (meshes, mesh_query, instance_buffer_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let mesh_handle = mesh_query.get(item).unwrap();
        let instance_buffer = instance_buffer_query.get(item).unwrap();

        let gpu_mesh = match meshes.into_inner().get(mesh_handle) {
            Some(gpu_mesh) => gpu_mesh,
            None => return RenderCommandResult::Failure,
        };

        pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, instance_buffer.buffer.slice(..));
        match &gpu_mesh.buffer_info {
            GpuBufferInfo::Indexed {
                buffer,
                count,
                index_format,
            } => {
                pass.set_index_buffer(buffer.slice(..), 0, *index_format);
                pass.draw_indexed(0..*count, 0, 0..instance_buffer.length as u32);
            }
            GpuBufferInfo::NonIndexed { vertex_count } => {
                pass.draw_indexed(0..*vertex_count, 0, 0..instance_buffer.length as u32);
            }
        }
        RenderCommandResult::Success
    }
}

/*
 * Prepare instance buffer for rendering
 */
fn prepare_instance_buffer(
    mut commands: Commands,
    query: Query<(Entity, &InstancedMesh)>,
    render_device: Res<RenderDevice>,
) {
    for (entity, instanced_mesh) in query.iter() {
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("instance buffer"),
            contents: bytemuck::cast_slice(instanced_mesh.0.as_slice()),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        commands.entity(entity).remove::<InstanceBuffer>();
        commands.entity(entity).insert(InstanceBuffer {
            buffer,
            length: instanced_mesh.0.len(),
        });
    }
}

/*
 * Custom render queue
 */
fn glc_render_queue(
    draw_functions: Res<DrawFunctions<Transparent3d>>,
    rendering_pipeline: Res<GlcRenderingPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedPipelines<GlcRenderingPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    query: Query<Entity, (With<Handle<Mesh>>, With<InstancedMesh>)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_function = draw_functions.read().get_id::<GlcDraw>().unwrap();
    let key = MeshPipelineKey::from_msaa_samples(msaa.samples)
        | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);
    let pipeline = pipelines.specialize(&mut pipeline_cache, &rendering_pipeline, key);

    for (_view, mut phase) in views.iter_mut() {
        for entity in query.iter() {
            phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function,
                distance: 0.0,
            });
        }
    }
}
