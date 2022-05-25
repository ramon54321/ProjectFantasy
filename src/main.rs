mod graphics;

use crate::graphics::interface::{FixtureCreateInfo, GpuInterface};
use bytemuck::{Pod, Zeroable};
use graphics::{interface::Fixture, WindowEventDriven};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use vulkano::{
    impl_vertex,
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
                let command_buffers = gpu_interface.create_command_buffers(&self.fixture);

                let (image_index, _is_acquired_image_suboptimal, acquire_future) =
                    match acquire_next_image(gpu_interface.swapchain.clone(), None) {
                        Ok(result) => result,
                        Err(e) => panic!("{:?}", e),
                    };

                let execution_future = now(gpu_interface.device.clone())
                    .join(acquire_future)
                    .then_execute(
                        gpu_interface.queue.clone(),
                        command_buffers[image_index].clone(),
                    )
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
        self.fixture_passes
            .push(Box::new(CheeseTrianglesFixturePass::new(&gpu_interface)));
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
