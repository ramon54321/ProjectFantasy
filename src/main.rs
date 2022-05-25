mod graphics;

use crate::graphics::interface::{FixtureCreateInfo, GpuInterface};
use bytemuck::{Pod, Zeroable};
use graphics::{interface::Fixture, WindowEventDriven};
use rand::{thread_rng, Rng};
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
struct VertexLocal {
    position: [f32; 2],
}
impl_vertex!(VertexLocal, position);

struct App {
    previous_frame_end: Option<NowFuture>,
    fixture_create_info: Option<FixtureCreateInfo<VertexLocal>>,
    fixture: Option<Fixture<VertexLocal>>,
}
impl App {
    fn new() -> Self {
        Self {
            previous_frame_end: None,
            fixture_create_info: None,
            fixture: None,
        }
    }
}
impl WindowEventDriven<()> for App {
    fn on_start(&mut self, gpu_interface: &GpuInterface) {
        self.previous_frame_end = Some(now(gpu_interface.device.clone()));
        self.fixture_create_info = Some(FixtureCreateInfo {
            verticies: vec![
                VertexLocal {
                    position: [-1.0, -1.0],
                },
                VertexLocal {
                    position: [-1.0, 1.0],
                },
                VertexLocal {
                    position: [1.0, 1.0],
                },
                VertexLocal {
                    position: [1.0, 1.0],
                },
                VertexLocal {
                    position: [1.0, -1.0],
                },
                VertexLocal {
                    position: [-1.0, -1.0],
                },
            ],
        });
        self.fixture = Some(
            gpu_interface.create_fixture(
                self.fixture_create_info
                    .as_ref()
                    .expect("Could not get fixture create info")
                    .clone(),
            ),
        );
    }
    fn on_event(
        &mut self,
        event: Event<()>,
        _control_flow: &mut ControlFlow,
        gpu_interface: &GpuInterface,
    ) {
        let mut random_range = thread_rng();
        match event {
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if state == ElementState::Pressed && button == MouseButton::Left {
                    let fixture_create_info = FixtureCreateInfo {
                        verticies: vec![
                            VertexLocal {
                                position: [-random_range.gen::<f32>(), -random_range.gen::<f32>()],
                            },
                            VertexLocal {
                                position: [-random_range.gen::<f32>(), random_range.gen::<f32>()],
                            },
                            VertexLocal {
                                position: [random_range.gen::<f32>(), random_range.gen::<f32>()],
                            },
                            VertexLocal {
                                position: [random_range.gen::<f32>(), random_range.gen::<f32>()],
                            },
                            VertexLocal {
                                position: [random_range.gen::<f32>(), -random_range.gen::<f32>()],
                            },
                            VertexLocal {
                                position: [-random_range.gen::<f32>(), -random_range.gen::<f32>()],
                            },
                        ],
                    };
                    self.fixture = Some(gpu_interface.create_fixture(fixture_create_info.clone()));
                }
            }
            Event::RedrawEventsCleared => {
                self.previous_frame_end
                    .as_mut()
                    .expect("Could not get previous frame end")
                    .cleanup_finished();

                let command_buffers = gpu_interface
                    .create_command_buffers(&self.fixture.as_ref().expect("Could not get fixture"));

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
