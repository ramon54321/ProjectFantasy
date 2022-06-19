mod graphics;
mod grid_sweep;
mod world;

use fast_poisson::Poisson2D;
use graphics::{GpuApp, GpuFixture, GpuFixtureCreateInfo, GpuInterface, Sweep};
use grid_sweep::GridSweep;
use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use line_drawing::{Bresenham, Point};
use spade::{DelaunayTriangulation, Point2, Triangulation};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use world::{World, HEIGHT, WIDTH};

fn create_sweeps(gpu_interface: &GpuInterface, gpu_fixture: &GpuFixture) -> Vec<Box<dyn Sweep>> {
    let sweep_1 = Box::new(GridSweep::new(&gpu_interface, &gpu_fixture));
    vec![sweep_1]
}

fn draw_line(image: &mut RgbImage, a: Point<i32>, b: Point<i32>) {
    for (x, y) in Bresenham::new(a, b) {
        image.put_pixel(x as u32, y as u32, Pixel::from_channels(0, 255, 0, 0));
    }
}

fn main() {
    //let mut world = World::new();
    //world.generate();

    //let mut image: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
    //for x in 0..WIDTH {
    //for y in 0..HEIGHT {
    //let height = world.heights.get(x, y).unwrap();
    ////let pixel: Rgb<u8> = match height {
    ////h if h < 0.4 => Pixel::from_channels(77, 156, 218, 255),
    ////_ => Pixel::from_channels(119, 250, 106, 255),
    ////};
    //let value = (height * 255.0) as u8;
    //let pixel = Pixel::from_channels(value, value, value, 255);
    //image.put_pixel(x as u32, y as u32, pixel);
    //}
    //}

    //let points: Vec<[f64; 2]> = Poisson2D::new()
    //.with_dimensions([WIDTH as f64, HEIGHT as f64], 35.0)
    //.iter()
    //.take(1000)
    //.collect();

    //points.iter().for_each(|point| {
    //image.put_pixel(
    //point[0] as u32,
    //point[1] as u32,
    //Pixel::from_channels(255, 0, 0, 0),
    //);
    //});

    //let mut triangulation: DelaunayTriangulation<_> = DelaunayTriangulation::new();
    //points.iter().for_each(|point| {
    //triangulation
    //.insert(Point2::new(point[0], point[1]))
    //.unwrap();
    //});

    //let face = triangulation.voronoi_faces().take(18).last().unwrap();
    //face.

    //for x in 0..WIDTH {
    //for y in 0..HEIGHT {
    //let distance = f64::sqrt(face.distance_2(Point2::new(x as f64, y as f64))) * 5.0;
    //image.put_pixel(
    //x as u32,
    //y as u32,
    //Pixel::from_channels(distance as u8, distance as u8, distance as u8, 255),
    //);
    //}
    //}

    //for edge in triangulation.directed_edges() {
    //let a = edge.from().position();
    //let b = edge.to().position();
    ////if a.is_none() || b.is_none() {
    ////continue;
    ////}
    ////let a = a.unwrap();
    ////let b = b.unwrap();
    //if a.x >= WIDTH as f64 - 2.0 || a.x < 0.0 || a.y >= HEIGHT as f64 - 2.0 || a.y < 0.0 {
    //continue;
    //}
    //if b.x >= WIDTH as f64 - 2.0 || b.x < 0.0 || b.y >= HEIGHT as f64 - 2.0 || b.y < 0.0 {
    //continue;
    //}
    //draw_line(
    //&mut image,
    //(a.x as i32, a.y as i32),
    //(b.x as i32, b.y as i32),
    //);
    //}

    //image.save("world.png").unwrap();

    let event_loop = EventLoop::new();
    let gpu_interface = GpuInterface::new(&event_loop);
    let mut gpu_app = GpuApp::new(gpu_interface.clone(), create_sweeps);

    let gpu_fixture = GpuFixture::new(&GpuFixtureCreateInfo {}, &gpu_interface);

    gpu_app.set_fixture(gpu_fixture);

    gpu_app.on_start(&gpu_interface);
    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        };
        gpu_app.on_event(event, control_flow, &gpu_interface);
    });
}
