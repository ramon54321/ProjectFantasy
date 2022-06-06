mod graphics;
mod grid_sweep;

use graphics::{GpuApp, GpuFixture, GpuFixtureCreateInfo, GpuInterface, Sweep};
use grid_sweep::GridSweep;
use image::{ImageBuffer, Pixel, RgbImage};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

fn create_sweeps(gpu_interface: &GpuInterface, gpu_fixture: &GpuFixture) -> Vec<Box<dyn Sweep>> {
    let sweep_1 = Box::new(GridSweep::new(&gpu_interface, &gpu_fixture));
    vec![sweep_1]
}

struct World {
    heights: [[u16; 256]; 256],
}

impl World {
    fn new() -> Self {
        Self {
            heights: [[128; 256]; 256],
        }
    }
}

fn main() {
    let mut world = World::new();
    world.heights[128][64] = 0;

    let mut image: RgbImage = ImageBuffer::new(256, 256);
    world
        .heights
        .into_iter()
        .enumerate()
        .for_each(|(x, height_row)| {
            height_row.into_iter().enumerate().for_each(|(y, height)| {
                image.put_pixel(
                    x as u32,
                    y as u32,
                    Pixel::from_channels(height as u8, height as u8, height as u8, 255),
                );
            })
        });
    image.save("world.png").unwrap();

    //let event_loop = EventLoop::new();
    //let gpu_interface = GpuInterface::new(&event_loop);
    //let mut gpu_app = GpuApp::new(gpu_interface.clone(), create_sweeps);

    //let gpu_fixture = GpuFixture::new(&GpuFixtureCreateInfo {}, &gpu_interface);

    //gpu_app.set_fixture(gpu_fixture);

    //gpu_app.on_start(&gpu_interface);
    //event_loop.run(move |event, _window_target, control_flow| {
    //*control_flow = ControlFlow::Poll;
    //match event {
    //Event::WindowEvent {
    //event: WindowEvent::CloseRequested,
    //..
    //} => *control_flow = ControlFlow::Exit,
    //_ => (),
    //};
    //gpu_app.on_event(event, control_flow, &gpu_interface);
    //});
}
