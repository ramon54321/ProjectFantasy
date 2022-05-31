mod graphics;

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use graphics::{
    interface::create_graphics_pipeline, GpuApp, GpuFixture, GpuFixtureCreateInfo, GpuInterface,
    Sweep,
};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    impl_vertex,
    pipeline::GraphicsPipeline,
};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
struct CheeseTrianglesVertex {
    position: [f32; 2],
}
impl_vertex!(CheeseTrianglesVertex, position);

struct CheeseTrianglesSweep {
    graphics_pipeline: Arc<GraphicsPipeline>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[CheeseTrianglesVertex]>>,
    random_range: ThreadRng,
}

impl CheeseTrianglesSweep {
    fn new(gpu_interface: &GpuInterface, fixture: &GpuFixture) -> Self {
        let graphics_pipeline = create_graphics_pipeline::<CheeseTrianglesVertex>(
            gpu_interface.device.clone(),
            fixture.swapchain.clone().image_extent(),
            fixture.render_pass.clone(),
        );
        let verticies = vec![
            CheeseTrianglesVertex {
                position: [-1.0, -1.0],
            },
            CheeseTrianglesVertex {
                position: [-1.0, 1.0],
            },
            CheeseTrianglesVertex {
                position: [1.0, 1.0],
            },
            CheeseTrianglesVertex {
                position: [1.0, 1.0],
            },
            CheeseTrianglesVertex {
                position: [1.0, -1.0],
            },
            CheeseTrianglesVertex {
                position: [-1.0, -1.0],
            },
        ];
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            gpu_interface.device.clone(),
            BufferUsage::vertex_buffer(),
            false,
            verticies,
        )
        .expect("Could not create vertex buffer");
        let random_range = thread_rng();
        Self {
            graphics_pipeline,
            vertex_buffer,
            random_range,
        }
    }
}

impl Sweep for CheeseTrianglesSweep {
    fn render(
        &self,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        command_buffer_builder
            .bind_pipeline_graphics(self.graphics_pipeline.clone())
            .bind_vertex_buffers(0, self.vertex_buffer.clone())
            .draw(6, 1, 0, 0)
            .expect("Could not enqueue draw command");
    }
    fn on_event(
        &mut self,
        event: &Event<()>,
        _control_flow: &mut ControlFlow,
        gpu_interface: &GpuInterface,
    ) {
        match event {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if *state == ElementState::Pressed && *button == MouseButton::Left {
                    let verticies = vec![
                        CheeseTrianglesVertex {
                            position: [
                                -self.random_range.gen::<f32>(),
                                -self.random_range.gen::<f32>(),
                            ],
                        },
                        CheeseTrianglesVertex {
                            position: [
                                -self.random_range.gen::<f32>(),
                                self.random_range.gen::<f32>(),
                            ],
                        },
                        CheeseTrianglesVertex {
                            position: [
                                self.random_range.gen::<f32>(),
                                self.random_range.gen::<f32>(),
                            ],
                        },
                        CheeseTrianglesVertex {
                            position: [
                                self.random_range.gen::<f32>(),
                                self.random_range.gen::<f32>(),
                            ],
                        },
                        CheeseTrianglesVertex {
                            position: [
                                self.random_range.gen::<f32>(),
                                -self.random_range.gen::<f32>(),
                            ],
                        },
                        CheeseTrianglesVertex {
                            position: [
                                -self.random_range.gen::<f32>(),
                                -self.random_range.gen::<f32>(),
                            ],
                        },
                    ];
                    self.vertex_buffer = CpuAccessibleBuffer::from_iter(
                        gpu_interface.device.clone(),
                        BufferUsage::vertex_buffer(),
                        false,
                        verticies,
                    )
                    .expect("Could not create vertex buffer");
                }
            }
            _ => {}
        }
    }
}

fn create_sweeps(gpu_interface: &GpuInterface, gpu_fixture: &GpuFixture) -> Vec<Box<dyn Sweep>> {
    let sweep_0 = Box::new(CheeseTrianglesSweep::new(&gpu_interface, &gpu_fixture));
    vec![sweep_0]
}

fn main() {
    let event_loop = EventLoop::new();
    let gpu_interface = GpuInterface::new(&event_loop);
    let mut gpu_app = GpuApp::new(gpu_interface.clone(), create_sweeps);

    let gpu_fixture = GpuFixture::new(&GpuFixtureCreateInfo {}, &gpu_interface);

    gpu_app.set_fixture(gpu_fixture);

    gpu_app.on_start(&gpu_interface);
    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        };
        gpu_app.on_event(event, control_flow, &gpu_interface);
    });
}
