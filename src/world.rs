use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use image::{
    imageops::{self, resize},
    io::Reader,
    GenericImageView, GrayImage, RgbImage,
};
use nalgebra_glm::smoothstep;
use noise::{
    BasicMulti, Billow, Fbm, HybridMulti, NoiseFn, OpenSimplex, Perlin, Seedable, Select,
    SuperSimplex, Terrace, Turbulence, Value, Worley,
};

pub const TILING: f64 = 8.0;
pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 1024;

fn new_data_grid<T: Clone>(initial_value: T) -> Vec<Vec<T>> {
    println!("Creating new data grid");
    vec![vec![initial_value; HEIGHT]; WIDTH]
}

fn remap(x: f64, min: f64, max: f64, a: f64, b: f64) -> f64 {
    let delta_in = max - min;
    let delta_out = b - a;
    (delta_out * ((x - min) / delta_in)) + a
}

fn min_in(collection: &Vec<Vec<f64>>) -> f64 {
    let mut min_value = f64::INFINITY;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if min_value > collection[x][y] {
                min_value = collection[x][y];
            }
        }
    }
    min_value
}

fn max_in(collection: &Vec<Vec<f64>>) -> f64 {
    let mut max_value = f64::NEG_INFINITY;
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if max_value < collection[x][y] {
                max_value = collection[x][y];
            }
        }
    }
    max_value
}

fn remap_data_grid(collection: &mut Vec<Vec<f64>>, min: f64, max: f64) {
    let min_value = min_in(&collection);
    let max_value = max_in(&collection);
    println!("Remapping: {} - {}", min_value, max_value);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let current_value = collection[x][y];
            collection[x][y] = remap(current_value, min_value, max_value, min, max);
        }
    }
}

fn get_noise<F: NoiseFn<[f64; 2]>>(
    func: F,
    frequency: f64,
    skew: f64,
    stretch: f64,
) -> Vec<Vec<f64>> {
    println!("Getting noise data grid");
    //let mut values = Arc::new(new_data_grid(0.0));
    //println!("Start");

    //let perlin = Perlin::new();

    //let mut handles = Vec::new();

    //for i in 0..8 {
    //let values = values.clone();
    //let x_start = (WIDTH / 8) * i;
    //let x_end = (WIDTH / 8) * i + 1;
    //let handle = thread::spawn(move || unsafe {
    //for x in x_start..x_end {
    //for y in 0..HEIGHT {
    //let x = x as f64;
    //let y = y as f64;
    //let regional_value = perlin.get([
    //(x * TILING * frequency + skew * y) / WIDTH as f64,
    //(y * TILING * frequency / stretch) / HEIGHT as f64,
    //]);
    //values[x as usize][y as usize] = regional_value;
    //}
    //}
    //});
    //handles.push(handle);
    //}
    //for handle in handles {
    //handle.join().unwrap();
    //}

    //println!("Done");
    //*values

    //let thread_handle = thread::spawn(|| {
    //for x in 0..WIDTH {
    //for y in 0..HEIGHT {
    //let x = x as f64;
    //let y = y as f64;
    //let regional_value = perlin.get([
    //(x * TILING * frequency + skew * y) / WIDTH as f64,
    //(y * TILING * frequency / stretch) / HEIGHT as f64,
    //]);
    //values[x as usize][y as usize] = regional_value;
    //}
    //}
    //});

    //let input = Arc::new([1u32, 2, 3, 4]);
    //let output = Arc::new([0; 4]);

    //let mut handles = Vec::new();

    //for t in 0..4 {
    //let inp = input.clone();
    //let out = output.clone();
    //let handle = thread::spawn(move || unsafe {
    //let p = (out.as_ptr() as *mut u32).offset(t as isize);

    //*p = inp[t] + (t as u32 + 1);
    //});

    //handles.push(handle);
    //}

    //for h in handles {
    //h.join().unwrap();
    //}

    //println!("{:?}", output);

    //println!("Start");
    //for x in 0..WIDTH {
    //for y in 0..HEIGHT {
    //let x = x as f64;
    //let y = y as f64;
    //let regional_value = func.get([
    //(x * TILING * frequency + skew * y) / WIDTH as f64,
    //(y * TILING * frequency / stretch) / HEIGHT as f64,
    //]);
    //values[x as usize][y as usize] = regional_value;
    //}
    //}
    //println!("Done");
    //values

    new_data_grid(0.0)
}

pub struct World {
    pub heights: DataGrid,
}

pub struct DataGrid {
    pub values: Vec<Vec<f64>>,
    pub width: usize,
    pub height: usize,
}
impl DataGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![vec![0.0; height]; width],
            width,
            height,
        }
    }
    pub fn from_values(width: usize, height: usize, values: &Vec<Vec<f64>>) -> Self {
        let mut data_grid = Self::new(width, height);
        for x in 0..width {
            for y in 0..height {
                data_grid.values[x][y] = values[x][y];
            }
        }
        data_grid
    }
    pub fn from_image(image: &GrayImage) -> Self {
        let mut data_grid = Self::new(image.width() as usize, image.height() as usize);
        for x in 0..image.width() {
            for y in 0..image.height() {
                unsafe {
                    let pixel_value = image.unsafe_get_pixel(x, y)[0];
                    data_grid.values[x as usize][y as usize] = pixel_value as f64 / 256.0;
                }
            }
        }
        data_grid
    }
    pub fn to_image(&self) -> GrayImage {
        GrayImage::from_fn(self.width as u32, self.height as u32, |x, y| {
            image::Luma([(self.values[x as usize][y as usize] * 256.0) as u8])
        })
    }
    pub fn get(&self, x: usize, y: usize) -> Option<f64> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(self.values[x][y])
    }
    pub fn set(&mut self, x: usize, y: usize, value: f64) {
        if x >= self.width || y >= self.width {
            return;
        }
        self.values[x][y] = value;
    }
    pub fn blend_mut<F: Fn(f64, f64) -> f64>(
        &mut self,
        other: &DataGrid,
        offset_x: isize,
        offset_y: isize,
        blend_func: F,
    ) {
        println!("Blending");
        for x in 0..other.width {
            for y in 0..other.height {
                let top = other.get(x, y).unwrap();
                let x = x as isize + offset_x;
                let y = y as isize + offset_y;
                if x < 0 || y < 0 {
                    continue;
                }
                let bottom = self.get(x as usize, y as usize);
                if bottom.is_none() {
                    continue;
                }
                let new = blend_func(bottom.unwrap(), top);
                self.set(x as usize, y as usize, new);
            }
        }
    }
}

impl Clone for DataGrid {
    fn clone(&self) -> Self {
        DataGrid::from_values(self.width, self.height, &self.values)
    }
}

fn generate_layer_0() -> DataGrid {
    let perlin = Perlin::new().set_seed(1);

    let mut global_1_values = get_noise(perlin, 0.2, 0.0, 1.0);
    let mut regional_1_values = get_noise(perlin, 0.45, 2.0, 1.0);
    let mut regional_2_values = get_noise(perlin, 0.45, -0.5, 3.5);
    let mut perlin_1_values = get_noise(perlin, 1.0, 0.0, 1.0);
    let mut perlin_2_values = get_noise(perlin, 2.0, 0.0, 1.0);
    let mut perlin_3_values = get_noise(perlin, 4.0, 0.0, 1.0);
    let mut perlin_4_values = get_noise(perlin, 8.0, 0.0, 1.0);
    let mut perlin_5_values = get_noise(perlin, 16.0, 0.0, 1.0);
    let mut perlin_6_values = get_noise(perlin, 24.0, 0.0, 1.0);
    let mut perlin_mask_values = get_noise(perlin, 0.3, 6.0, 2.0);
    let mut perlin_values = new_data_grid(0.0);

    remap_data_grid(&mut global_1_values, 0.0, 1.0);
    remap_data_grid(&mut regional_1_values, 0.0, 0.25);
    remap_data_grid(&mut regional_2_values, 0.0, 0.25);
    remap_data_grid(&mut perlin_1_values, 0.0, 0.5);
    remap_data_grid(&mut perlin_2_values, 0.0, 0.25);
    remap_data_grid(&mut perlin_3_values, 0.0, 0.125);
    remap_data_grid(&mut perlin_4_values, 0.0, 0.0625);
    remap_data_grid(&mut perlin_5_values, 0.0, 0.03);
    remap_data_grid(&mut perlin_6_values, 0.0, 0.015);
    remap_data_grid(&mut perlin_mask_values, 0.3, 1.0);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let perlin_value = (perlin_1_values[x][y]
                + perlin_2_values[x][y]
                + perlin_3_values[x][y]
                + perlin_4_values[x][y]
                + perlin_5_values[x][y]
                + perlin_6_values[x][y])
                * perlin_mask_values[x][y];
            perlin_values[x][y] = perlin_value;
        }
    }
    remap_data_grid(&mut perlin_values, 0.0, 0.15);
    let mut output_values = new_data_grid(0.0);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let regional_value = regional_1_values[x][y] + regional_2_values[x][y];
            let perlin_value = perlin_values[x][y];
            let output_value = global_1_values[x][y] * (regional_value + perlin_value);
            output_values[x][y] = output_value;
        }
    }
    remap_data_grid(&mut output_values, 0.0, 1.0);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            output_values[x][y] = smoothstep(0.35, 0.65, output_values[x][y]);
        }
    }
    remap_data_grid(&mut output_values, 0.25, 0.75);
    DataGrid::from_values(WIDTH, HEIGHT, &output_values)
}

impl World {
    pub fn new() -> Self {
        Self {
            heights: DataGrid::new(WIDTH, HEIGHT),
        }
    }

    pub fn generate(&mut self) {
        use rand::prelude::*;
        let mut random = rand::rngs::StdRng::seed_from_u64(1);

        let crater = Reader::open("resources/crater.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_luma8();

        let mut layer_1 = generate_layer_0();
        for i in 0..5 {
            let crater_size = (32.0 + random.gen::<f64>() * 128.0) as u32;
            let crater = resize(
                &crater,
                crater_size,
                crater_size,
                imageops::FilterType::CatmullRom,
            );
            let crater = imageproc::geometric_transformations::rotate_about_center(
                &crater,
                1.8,
                imageproc::geometric_transformations::Interpolation::Bicubic,
                image::Luma([128]),
            );
            let crater = DataGrid::from_image(&crater);
            let offset_x = -(crater.width as isize) + (random.gen::<f64>() * WIDTH as f64) as isize;
            let offset_y =
                -(crater.height as isize) + (random.gen::<f64>() * HEIGHT as f64) as isize;
            let crater_dim = 1.0 + random.gen::<f64>() * 4.0;
            layer_1.blend_mut(&crater, offset_x, offset_y, |a, b| {
                blend_overlay(a, ((b - 0.5) / crater_dim) + 0.5)
            });
        }

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.heights.set(x, y, layer_1.get(x, y).unwrap());
            }
        }
    }
}

fn blend_overlay(bottom: f64, top: f64) -> f64 {
    if bottom > 0.5 {
        let unit = (1.0 - bottom) / 0.5;
        let min = bottom - (1.0 - bottom);
        top * unit + min
    } else {
        let unit = bottom / 0.5;
        top * unit
    }
}
