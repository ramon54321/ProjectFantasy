use bytemuck::{Pod, Zeroable};
use rand::{thread_rng, Rng};
use vulkano::{
    impl_vertex,
    swapchain::acquire_next_image,
    sync::{now, GpuFuture},
};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::graphics::{create_command_buffers, FixtureCreateInfo, Vertex};

mod graphics;

fn main() {
    let event_loop = EventLoop::new();
    let app = graphics::App::new(&event_loop);

    let mut previous_frame_end = Some(now(app.device.clone()));
    #[repr(C)]
    #[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
    struct VertexLocal {
        position: [f32; 2],
    }
    impl Vertex for VertexLocal {}
    impl_vertex!(VertexLocal, position);
    let fixture_create_info = FixtureCreateInfo {
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
    };
    let mut random_range = thread_rng();
    let mut fixture = app.create_fixture(fixture_create_info.clone());
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
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
                    fixture = app.create_fixture(fixture_create_info.clone());
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawEventsCleared => {
                previous_frame_end
                    .as_mut()
                    .expect("Could not get previous frame end")
                    .cleanup_finished();

                let command_buffers = create_command_buffers(
                    app.device.clone(),
                    app.queue.clone(),
                    &fixture.frame_buffers,
                    fixture.graphics_pipeline.clone(),
                    fixture.vertex_buffer.clone(),
                );

                let (image_index, _is_acquired_image_suboptimal, acquire_future) =
                    match acquire_next_image(app.swapchain.clone(), None) {
                        Ok(result) => result,
                        Err(e) => panic!("{:?}", e),
                    };

                let execution_future = now(app.device.clone())
                    .join(acquire_future)
                    .then_execute(app.queue.clone(), command_buffers[image_index].clone())
                    .unwrap()
                    .then_swapchain_present(app.queue.clone(), app.swapchain.clone(), image_index)
                    .then_signal_fence_and_flush();

                execution_future
                    .expect("Execution future was not present")
                    .wait(None)
                    .expect("Execution future could not wait");
            }
            _ => (),
        }
    });
}
