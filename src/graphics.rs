use winit::{event::Event, event_loop::ControlFlow};

use self::interface::GpuInterface;

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
