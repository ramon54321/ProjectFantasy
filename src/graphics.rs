use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
    },
    swapchain::acquire_next_image,
    sync::{now, GpuFuture, NowFuture},
};
use winit::{event::Event, event_loop::ControlFlow};

use self::interface::{GpuFixture, GpuInterface};

pub mod interface;

pub trait WindowEventDriven<E> {
    fn on_start(&mut self, gpu_interface: &GpuInterface);
    fn on_event(
        &mut self,
        event: Event<E>,
        control_flow: &mut ControlFlow,
        gpu_interface: &GpuInterface,
    );
}

pub trait Sweep {
    fn render(
        &self,
        command_buffer_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    );
    fn on_event(
        &mut self,
        event: &Event<()>,
        control_flow: &mut ControlFlow,
        gpu_interface: &GpuInterface,
    );
}

pub struct GpuApp {
    gpu_interface: GpuInterface,
    gpu_fixture: Option<GpuFixture>,
    sweeps: Vec<Box<dyn Sweep>>,
    create_sweeps: fn(&GpuInterface, &GpuFixture) -> Vec<Box<dyn Sweep>>,
    previous_frame_end: Option<NowFuture>,
}
impl GpuApp {
    pub fn new(
        gpu_interface: GpuInterface,
        create_sweeps: fn(&GpuInterface, &GpuFixture) -> Vec<Box<dyn Sweep>>,
    ) -> Self {
        Self {
            gpu_interface,
            gpu_fixture: None,
            sweeps: Vec::new(),
            create_sweeps,
            previous_frame_end: None,
        }
    }
    pub fn set_fixture(&mut self, gpu_fixture: GpuFixture) {
        self.sweeps = (self.create_sweeps)(&self.gpu_interface, &gpu_fixture);
        self.gpu_fixture = Some(gpu_fixture);
    }
}
impl WindowEventDriven<()> for GpuApp {
    fn on_start(&mut self, gpu_interface: &GpuInterface) {
        self.previous_frame_end = Some(now(gpu_interface.device.clone()));
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
        match event {
            Event::RedrawEventsCleared => {
                if self.gpu_fixture.is_none() {
                    return;
                }

                let (frame_buffer_image_index, _is_acquired_image_suboptimal, acquire_future) =
                    match acquire_next_image(
                        self.gpu_fixture.as_ref().unwrap().swapchain.clone(),
                        None,
                    ) {
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
                        self.gpu_fixture.as_ref().unwrap().frame_buffers[frame_buffer_image_index]
                            .clone(),
                        SubpassContents::Inline,
                        vec![[1.0, 1.0, 1.0, 1.0].into(), [0.0, 0.0, 0.0, 1.0].into()],
                    )
                    .expect("Could not begin render pass");

                self.sweeps
                    .iter_mut()
                    .for_each(|sweep| sweep.render(&mut command_buffer_builder));

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
                        self.gpu_fixture.as_ref().unwrap().swapchain.clone(),
                        frame_buffer_image_index,
                    )
                    .then_signal_fence_and_flush();

                execution_future
                    .expect("Execution future was not present")
                    .wait(None)
                    .expect("Execution future could not wait");
            }
            _ => {
                self.sweeps
                    .iter_mut()
                    .for_each(|sweep| sweep.on_event(&event, control_flow, &gpu_interface));
            }
        };
    }
}
