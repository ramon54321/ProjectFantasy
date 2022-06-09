use noise::{
    BasicMulti, Billow, Fbm, HybridMulti, NoiseFn, OpenSimplex, Perlin, Select, SuperSimplex,
    Terrace, Turbulence, Value, Worley,
};

pub const TILING: f64 = 8.0;
pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 1024;

fn new_data_grid<T: Clone>(initial_value: T) -> Vec<Vec<T>> {
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

fn remap_collection(collection: &mut Vec<Vec<f64>>, min: f64, max: f64) {
    let min_value = min_in(&collection);
    let max_value = max_in(&collection);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let current_value = collection[x][y];
            collection[x][y] = remap(current_value, min_value, max_value, min, max);
        }
    }
}

fn get_noise<F: NoiseFn<[f64; 2]>>(func: F) -> Vec<Vec<f64>> {
    let mut values = new_data_grid(0.0);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let x = x as f64;
            let y = y as f64;
            let regional_value =
                func.get([(x * TILING) / WIDTH as f64, (y * TILING) / HEIGHT as f64]);
            values[x as usize][y as usize] = regional_value;
        }
    }
    values
}

pub struct World {
    pub heights: Vec<Vec<u16>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            heights: new_data_grid(0),
        }
    }

    pub fn generate(&mut self) {
        let perlin = Perlin::new();
        let worley = Worley::new().enable_range(true);

        let mut regional_values = get_noise(perlin);
        remap_collection(&mut regional_values, 0.0, 1.0);

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                self.heights[x][y] = (regional_values[x][y] * 255.0) as u16;
            }
        }

        let min = self.heights.iter().flatten().min().unwrap();
        let max = self.heights.iter().flatten().max().unwrap();
        println!("{min} -> {max}");
    }
}
