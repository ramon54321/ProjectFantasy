mod graphics;
mod grid_sweep;
mod world;

use graphics::{GpuApp, GpuFixture, GpuFixtureCreateInfo, GpuInterface, Sweep};
use grid_sweep::GridSweep;
use image::{ImageBuffer, Pixel, RgbImage};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use world::{World, HEIGHT, WIDTH};

fn create_sweeps(gpu_interface: &GpuInterface, gpu_fixture: &GpuFixture) -> Vec<Box<dyn Sweep>> {
    let sweep_1 = Box::new(GridSweep::new(&gpu_interface, &gpu_fixture));
    vec![sweep_1]
}

fn main() {
    let mut world = World::new();
    world.generate();

    let mut image: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let height = world.heights[x][y];
            image.put_pixel(
                x as u32,
                y as u32,
                Pixel::from_channels(height as u8, height as u8, height as u8, 255),
            );
        }
    }
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
