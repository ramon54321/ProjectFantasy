mod graphics;

use std::sync::Arc;

use crate::graphics::interface::{FixtureCreateInfo, GpuInterface};
use bytemuck::{Pod, Zeroable};
use graphics::{
    interface::{create_graphics_pipeline, create_vertex_buffer, Fixture},
    WindowEventDriven,
};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use vulkano::{
    buffer::CpuAccessibleBuffer,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
    },
    impl_vertex,
    pipeline::GraphicsPipeline,
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

struct CheeseTrianglesSweep {
    graphics_pipeline: Arc<GraphicsPipeline>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[CheeseTrianglesVertex]>>,
    random_range: ThreadRng,
}

impl CheeseTrianglesSweep {
    fn new(gpu_interface: &GpuInterface, fixture: &Fixture, offset: usize) -> Self {
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
        let vertex_buffer = create_vertex_buffer(gpu_interface.device.clone(), verticies);

        let mut random_range = thread_rng();
        for i in 0..offset {
            let f: f32 = random_range.gen();
        }
        Self {
            graphics_pipeline,
            vertex_buffer,
            random_range,
        }
    }
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
}

//impl FixturePass for CheeseTrianglesFixturePass {
//fn on_render(
//&mut self,
//_event: &Event<()>,
//_control_flow: &mut ControlFlow,
//_gpu_interface: &GpuInterface,
//command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
//frame_buffer_index: usize,
//) {
//command_buffer_builder
//.begin_render_pass(
//self.fixture.frame_buffers[frame_buffer_index].clone(),
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
//}
//fn on_event(
//&mut self,
//event: &Event<()>,
//_control_flow: &mut ControlFlow,
//gpu_interface: &GpuInterface,
//) {
//match event {
//Event::WindowEvent {
//event: WindowEvent::MouseInput { state, button, .. },
//..
//} => {
//if *state == ElementState::Pressed && *button == MouseButton::Left {
//let fixture_create_info = FixtureCreateInfo {
//verticies: vec![
//CheeseTrianglesVertex {
//position: [
//-self.random_range.gen::<f32>(),
//-self.random_range.gen::<f32>(),
//],
//},
//CheeseTrianglesVertex {
//position: [
//-self.random_range.gen::<f32>(),
//self.random_range.gen::<f32>(),
//],
//},
//CheeseTrianglesVertex {
//position: [
//self.random_range.gen::<f32>(),
//self.random_range.gen::<f32>(),
//],
//},
//CheeseTrianglesVertex {
//position: [
//self.random_range.gen::<f32>(),
//self.random_range.gen::<f32>(),
//],
//},
//CheeseTrianglesVertex {
//position: [
//self.random_range.gen::<f32>(),
//-self.random_range.gen::<f32>(),
//],
//},
//CheeseTrianglesVertex {
//position: [
//-self.random_range.gen::<f32>(),
//-self.random_range.gen::<f32>(),
//],
//},
//],
//};
//self.fixture = gpu_interface.create_fixture(fixture_create_info);
//}
//}
//_ => (),
//}
//}
//}

//trait FixturePass {
//fn on_render(
//&mut self,
//event: &Event<()>,
//control_flow: &mut ControlFlow,
//gpu_interface: &GpuInterface,
//command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
//frame_buffer_index: usize,
//);
//fn on_event(
//&mut self,
//event: &Event<()>,
//control_flow: &mut ControlFlow,
//gpu_interface: &GpuInterface,
//);
//}

struct App {
    previous_frame_end: Option<NowFuture>,
    fixture: Fixture,
    sweep: CheeseTrianglesSweep,
}
impl App {
    fn new(gpu_interface: &GpuInterface) -> Self {
        let fixture = Fixture::new(&FixtureCreateInfo {}, &gpu_interface);
        let sweep = CheeseTrianglesSweep::new(&gpu_interface, &fixture, 56);
        Self {
            previous_frame_end: None,
            fixture,
            sweep,
        }
    }
}
impl WindowEventDriven<()> for App {
    fn on_start(&mut self, gpu_interface: &GpuInterface) {
        self.previous_frame_end = Some(now(gpu_interface.device.clone()));
    }
    fn on_event(
        &mut self,
        event: Event<()>,
        _control_flow: &mut ControlFlow,
        gpu_interface: &GpuInterface,
    ) {
        if event == Event::RedrawEventsCleared {
            self.previous_frame_end
                .as_mut()
                .expect("Could not get previous frame end")
                .cleanup_finished();
        }
        match event {
            Event::RedrawEventsCleared => {
                let (frame_buffer_image_index, _is_acquired_image_suboptimal, acquire_future) =
                    match acquire_next_image(self.fixture.swapchain.clone(), None) {
                        Ok(result) => result,
                        Err(e) => panic!("{:?}", e),
                    };

                let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
                    gpu_interface.device.clone(),
                    gpu_interface.queue.family(),
                    CommandBufferUsage::OneTimeSubmit,
                )
                .expect("Could not create command buffer builder");

                command_buffer_builder
                    .begin_render_pass(
                        self.fixture.frame_buffers[frame_buffer_image_index].clone(),
                        SubpassContents::Inline,
                        vec![[1.0, 1.0, 1.0, 1.0].into(), [0.0, 0.0, 0.0, 1.0].into()],
                    )
                    .expect("Could not begin render pass");

                self.sweep.render(&mut command_buffer_builder);

                command_buffer_builder
                    .end_render_pass()
                    .expect("Could not end render pass");

                let command_buffer = command_buffer_builder
                    .build()
                    .expect("Could not build command buffer");

                let execution_future = now(gpu_interface.device.clone())
                    .join(acquire_future)
                    .then_execute(gpu_interface.queue.clone(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(
                        gpu_interface.queue.clone(),
                        self.fixture.swapchain.clone(),
                        frame_buffer_image_index,
                    )
                    .then_signal_fence_and_flush();

                execution_future
                    .expect("Execution future was not present")
                    .wait(None)
                    .expect("Execution future could not wait");
            }
            _ => {
                //self.fixture_passes.iter_mut().for_each(|fixture_pass| {
                //fixture_pass.on_event(&event, control_flow, gpu_interface.clone());
                //});
            }
        };
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let gpu_interface = GpuInterface::new(&event_loop);
    let mut app = App::new(&gpu_interface);

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
