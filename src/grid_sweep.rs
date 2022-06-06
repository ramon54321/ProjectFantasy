use std::sync::Arc;

use crate::graphics::{interface::create_graphics_pipeline, GpuFixture, GpuInterface, Sweep};
use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use vulkano::{
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    impl_vertex,
    pipeline::{GraphicsPipeline, Pipeline, PipelineBindPoint},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
struct GridVertex {
    position: [f32; 2],
}
impl_vertex!(GridVertex, position);

#[repr(C)]
#[derive(Default, Clone, Copy, Zeroable, Pod)]
struct MVP {
    view: [f32; 16],
    projection: [f32; 16],
}
impl MVP {
    fn new(dimensions: [u32; 2]) -> Self {
        let scaled_height = 10.0;
        let scaled_width = scaled_height * (dimensions[0] as f32 / dimensions[1] as f32);
        let projection = Mat4::orthographic_rh(
            -scaled_width / 2.0,
            scaled_width / 2.0,
            -scaled_height / 2.0,
            scaled_height / 2.0,
            0.1,
            100.0,
        );
        let view = Mat4::from_translation(Vec3::new(5.0, 2.0, 0.0));
        Self {
            view: view.to_cols_array(),
            projection: projection.to_cols_array(),
        }
    }
}

pub struct GridSweep {
    graphics_pipeline: Arc<GraphicsPipeline>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[GridVertex]>>,
    descriptor_set: Arc<PersistentDescriptorSet>,
}

impl GridSweep {
    pub fn new(gpu_interface: &GpuInterface, fixture: &GpuFixture) -> Self {
        let graphics_pipeline = create_graphics_pipeline::<GridVertex>(
            gpu_interface.device.clone(),
            fixture.swapchain.clone().image_extent(),
            fixture.render_pass.clone(),
            "data/shader",
            "data/shader",
        );
        let mvp_uniform = MVP::new(fixture.swapchain.image_extent());
        let uniform_buffer = CpuAccessibleBuffer::from_data(
            gpu_interface.device.clone(),
            BufferUsage::uniform_buffer_transfer_destination(),
            false,
            mvp_uniform,
        )
        .expect("Could not create uniform buffer");
        let descriptor_set = PersistentDescriptorSet::new(
            graphics_pipeline
                .layout()
                .set_layouts()
                .get(0)
                .expect("Could not get pipeline descriptor set 0")
                .clone(),
            [WriteDescriptorSet::buffer(0, uniform_buffer)],
        )
        .expect("Could not create descriptor set");
        let verticies: Vec<GridVertex> = (0..4)
            .flat_map(move |x| {
                (0..4).flat_map(move |y| {
                    vec![
                        GridVertex {
                            position: [-0.5 + x as f32, -0.5 + y as f32],
                        },
                        GridVertex {
                            position: [-0.5 + x as f32, 0.5 + y as f32],
                        },
                        GridVertex {
                            position: [0.5 + x as f32, 0.5 + y as f32],
                        },
                        GridVertex {
                            position: [0.5 + x as f32, 0.5 + y as f32],
                        },
                        GridVertex {
                            position: [0.5 + x as f32, -0.5 + y as f32],
                        },
                        GridVertex {
                            position: [-0.5 + x as f32, -0.5 + y as f32],
                        },
                    ]
                })
            })
            .collect();
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            gpu_interface.device.clone(),
            BufferUsage::vertex_buffer(),
            false,
            verticies,
        )
        .expect("Could not create vertex buffer");
        Self {
            graphics_pipeline,
            vertex_buffer,
            descriptor_set,
        }
    }
}

impl Sweep for GridSweep {
    fn on_build(
        &mut self,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        command_buffer_builder
            .bind_pipeline_graphics(self.graphics_pipeline.clone())
            .bind_vertex_buffers(0, self.vertex_buffer.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.graphics_pipeline.layout().clone(),
                0,
                self.descriptor_set.clone(),
            )
            .draw((self.vertex_buffer.size() / 8) as u32, 1, 0, 0)
            .expect("Could not enqueue draw command");
    }
    fn on_event(
        &mut self,
        event: &Event<()>,
        _control_flow: &mut ControlFlow,
        _gpu_interface: &GpuInterface,
    ) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {}
            _ => {}
        }
    }
}
