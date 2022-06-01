use std::sync::Arc;

use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
    },
    device::{Device, Queue},
    image::SwapchainImage,
    instance::Instance,
    render_pass::{Framebuffer, RenderPass},
    swapchain::{acquire_next_image, Surface, Swapchain},
    sync::{now, GpuFuture, NowFuture},
};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use self::interface::{
    create_device_and_queue, create_frame_buffers, create_instance, create_render_pass,
    create_surface, create_swapchain_and_images,
};

pub mod interface;

#[derive(Clone)]
pub struct GpuInterface {
    pub instance: Arc<Instance>,
    pub surface: Arc<Surface<Window>>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl GpuInterface {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let instance = create_instance();
        let surface = create_surface(instance.clone(), event_loop);
        let (device, queue) = create_device_and_queue(instance.clone(), surface.clone());
        Self {
            instance,
            surface,
            device,
            queue,
        }
    }
}

#[derive(Clone)]
pub struct GpuFixtureCreateInfo {}

pub struct GpuFixture {
    pub render_pass: Arc<RenderPass>,
    pub frame_buffers: Vec<Arc<Framebuffer>>,
    pub swapchain: Arc<Swapchain<Window>>,
    pub swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
}

impl GpuFixture {
    pub fn new(_fixture_create_info: &GpuFixtureCreateInfo, gpu_interface: &GpuInterface) -> Self {
        let (swapchain, swapchain_images) = create_swapchain_and_images(
            gpu_interface.device.clone(),
            gpu_interface.surface.clone(),
        );
        let render_pass = create_render_pass(gpu_interface.device.clone());
        let frame_buffers = create_frame_buffers(
            gpu_interface.device.clone(),
            &swapchain_images,
            render_pass.clone(),
            swapchain.image_extent(),
        );
        Self {
            swapchain,
            swapchain_images,
            render_pass,
            frame_buffers,
        }
    }
}

pub trait Sweep {
    fn on_build(
        &mut self,
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
    pub fn on_start(&mut self, gpu_interface: &GpuInterface) {
        self.previous_frame_end = Some(now(gpu_interface.device.clone()));
    }
    pub fn on_event(
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
                    .for_each(|sweep| sweep.on_build(&mut command_buffer_builder));

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
