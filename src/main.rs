mod graphics;

use std::sync::Arc;

use crate::graphics::interface::{FixtureCreateInfo, GpuInterface};
use bytemuck::{Pod, Zeroable};
use graphics::{interface::Fixture, WindowEventDriven};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use vulkano::{
    buffer::CpuAccessibleBuffer,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
    },
    impl_vertex,
    pipeline::{graphics::vertex_input::Vertex, GraphicsPipeline},
    render_pass::Framebuffer,
    swapchain::acquire_next_image,
    sync::{now, GpuFuture, NowFuture},
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

struct CheeseTrianglesFixturePass {
    fixture: Fixture<CheeseTrianglesVertex>,
    random_range: ThreadRng,
}
impl CheeseTrianglesFixturePass {
    fn new(gpu_interface: &GpuInterface) -> Self {
        let fixture_create_info = FixtureCreateInfo {
            verticies: vec![
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
            ],
        };
        let fixture = gpu_interface.create_fixture(fixture_create_info);
        let random_range = thread_rng();
        CheeseTrianglesFixturePass {
            fixture,
            random_range,
        }
    }
}

fn add_some_render_commands(
    command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    frame_buffer: Arc<Framebuffer>,
    graphics_pipeline: Arc<GraphicsPipeline>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[CheeseTrianglesVertex]>>,
) {
    command_buffer_builder
        .begin_render_pass(
            frame_buffer.clone(),
            SubpassContents::Inline,
            vec![[1.0, 1.0, 1.0, 1.0].into(), [0.0, 0.0, 0.0, 1.0].into()],
        )
        .expect("Could not begin render pass")
        .bind_pipeline_graphics(graphics_pipeline.clone())
        .bind_vertex_buffers(0, vertex_buffer.clone())
        .draw(6, 1, 0, 0)
        .expect("Could not draw");
    command_buffer_builder
        .end_render_pass()
        .expect("Could not end render pass");
}

impl FixturePass for CheeseTrianglesFixturePass {
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
                    let fixture_create_info = FixtureCreateInfo {
                        verticies: vec![
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
                        ],
                    };
                    self.fixture = gpu_interface.create_fixture(fixture_create_info);
                }
            }
            Event::RedrawEventsCleared => {
                let (image_index, _is_acquired_image_suboptimal, acquire_future) =
                    match acquire_next_image(gpu_interface.swapchain.clone(), None) {
                        Ok(result) => result,
                        Err(e) => panic!("{:?}", e),
                    };

                let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                    gpu_interface.device.clone(),
                    gpu_interface.queue.family(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                .expect("Could not create command buffer builder");

                //command_buffer_builder
                //.begin_render_pass(
                //self.fixture.frame_buffers[image_index].clone(),
                //SubpassContents::Inline,
                //vec![[1.0, 1.0, 1.0, 1.0].into(), [0.0, 0.0, 0.0, 1.0].into()],
                //)
                //.expect("Could not begin render pass")
                //.bind_pipeline_graphics(self.fixture.graphics_pipeline.clone())
                //.bind_vertex_buffers(0, self.fixture.vertex_buffer.clone())
                //.draw(6, 1, 0, 0)
                //.expect("Could not draw");
                //command_buffer_builder
                //.end_render_pass()
                //.expect("Could not end render pass");

                // Can be repeated (the unit of repitition is sort of a render pass with its own
                // graphics pipeline and vertex buffer and draw call). Only at the end the entire
                // command buffer is built and submitted, only once per frame.
                add_some_render_commands(
                    &mut command_buffer_builder,
                    self.fixture.frame_buffers[image_index].clone(),
                    self.fixture.graphics_pipeline.clone(),
                    self.fixture.vertex_buffer.clone(),
                );

                let command_buffer = command_buffer_builder
                    .build()
                    .expect("Could not build command buffer");

                let execution_future = now(gpu_interface.device.clone())
                    .join(acquire_future)
                    .then_execute(gpu_interface.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(
                        gpu_interface.queue.clone(),
                        gpu_interface.swapchain.clone(),
                        image_index,
                    )
                    .then_signal_fence_and_flush();

                execution_future
                    .expect("Execution future was not present")
                    .wait(None)
                    .expect("Execution future could not wait");
            }
            _ => (),
        }
    }
}

trait FixturePass {
    fn on_event(
        &mut self,
        event: &Event<()>,
        _control_flow: &mut ControlFlow,
        gpu_interface: &GpuInterface,
    );
}

struct App {
    previous_frame_end: Option<NowFuture>,
    fixture_passes: Vec<Box<dyn FixturePass>>,
}
impl App {
    fn new() -> Self {
        Self {
            previous_frame_end: None,
            fixture_passes: Vec::new(),
        }
    }
}
impl WindowEventDriven<()> for App {
    fn on_start(&mut self, gpu_interface: &GpuInterface) {
        self.previous_frame_end = Some(now(gpu_interface.device.clone()));
        self.fixture_passes
            .push(Box::new(CheeseTrianglesFixturePass::new(&gpu_interface)));
        //self.fixture_passes
        //.push(Box::new(CheeseTrianglesFixturePass::new(&gpu_interface)));
    }
    fn on_event(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
        gpu_interface: &GpuInterface,
    ) {
        if event == Event::RedrawEventsCleared {
            self.previous_frame_end
                .as_mut()
                .expect("Could not get previous frame end")
                .cleanup_finished();
        }
        self.fixture_passes.iter_mut().for_each(|fixture_pass| {
            fixture_pass.on_event(&event, control_flow, gpu_interface);
        });
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let gpu_interface = GpuInterface::new(&event_loop);
    let mut app = App::new();

    app.on_start(&gpu_interface);
    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        };
        app.on_event(event, control_flow, &gpu_interface);
    });
}
