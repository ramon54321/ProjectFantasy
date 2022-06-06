use noise::{NoiseFn, Perlin};

pub const TILING: f64 = 8.0;
pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 1024;

pub struct World {
    pub heights: [[u16; WIDTH]; HEIGHT],
}

impl World {
    pub fn new() -> Self {
        Self {
            heights: [[128; WIDTH]; HEIGHT],
        }
    }

    pub fn generate(&mut self) {
        let perlin = Perlin::new();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let noise_value = perlin.get([
                    x as f64 / (WIDTH as f64 / TILING),
                    y as f64 / (HEIGHT as f64 / TILING),
                ]);
                let noise_value_normalized = (noise_value + 1.0) * 127.5;
                self.heights[x][y] = noise_value_normalized as u16;
            }
        }

        let min = self.heights.iter().flatten().min().unwrap();
        let max = self.heights.iter().flatten().max().unwrap();
        println!("{min} -> {max}");
    }
}
