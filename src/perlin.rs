use rand::Rng;

use crate::vec3::Vec3;

const VALUE_COUNT: usize = 256;

pub struct Perlin {
    random: [f32; VALUE_COUNT],
    permutations_x: [usize; VALUE_COUNT],
    permutations_y: [usize; VALUE_COUNT],
    permutations_z: [usize; VALUE_COUNT]
}

impl Perlin {
    pub fn new() -> Self {
        let mut random = [0.0f32; VALUE_COUNT];
        for item in &mut random {
            *item = rand::thread_rng().gen_range(0.0..1.0);
        }

        let generate_permutations = || {
            let mut permutations = [0; VALUE_COUNT];
            for item in &mut permutations {
                *item = rand::thread_rng().gen_range(0..VALUE_COUNT);
            }
            for i in 0..permutations.len() {
                let target = rand::thread_rng().gen_range(0..VALUE_COUNT);
                permutations.swap(i, target);
            }
            permutations
        };
        Perlin{
            random, 
            permutations_x: generate_permutations(),
            permutations_y: generate_permutations(),
            permutations_z: generate_permutations()
        }
    }

    pub fn get(&self, p: Vec3) -> f32 {
        const SCALE: f32 = 4.0;
        // If the language allows conversion from float to unsigned int that allows underflow 
        // a bitmask can be used because it will do modulo without the need for abs.
        unsafe {
            *self.random.get_unchecked(
                self.permutations_x.get_unchecked((SCALE * p.x.abs()) as usize % VALUE_COUNT) ^
                self.permutations_y.get_unchecked((SCALE * p.y.abs()) as usize % VALUE_COUNT) ^
                self.permutations_z.get_unchecked((SCALE * p.z.abs()) as usize % VALUE_COUNT)
            )
        }
    }
}