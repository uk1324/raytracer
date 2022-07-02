use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub const UP: Vec3 = Vec3{ x: 0.0, y: 1.0, z: 0.0};

pub type Color = Vec3;
pub type Pt3 = Vec3;

impl Vec3 {
    pub fn all(all: f32) -> Vec3 {
        Vec3{ x: all, y: all, z: all }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{ x, y, z }
    }

    pub fn new_random() -> Vec3 {
        Vec3{ x: rand::thread_rng().gen::<f32>(), y: rand::thread_rng().gen::<f32>(), z: rand::thread_rng().gen::<f32>() }
    }

    pub fn new_random_in_range(min: f32, max: f32) -> Vec3 {
        Vec3{ x: rand::thread_rng().gen_range(min..max), y: rand::thread_rng().gen_range(min..max), z: rand::thread_rng().gen_range(min..max) }
    }


    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Vec3::new_random_in_range(-1.0, 1.0);
            if point.length_squared() < 1.0 {
                break point
            }
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().normalized()
    }

    pub fn apply(&mut self, function: fn(f32) -> f32) -> &Vec3 {
        self.x = function(self.x);
        self.y = function(self.y);
        self.z = function(self.z);
        self
    }

    pub fn applied(&self, function: fn(f32) -> f32) -> Vec3 {
        Vec3{ x: function(self.x), y: function(self.y), z: function(self.z) }
    }
    
    pub fn is_near_zero(&self) -> bool {
        const EPSILON: f32 = 1e-8;
        (self.x.abs() < EPSILON) || (self.y.abs() < EPSILON) || (self.z.abs() < EPSILON)
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalized(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(self, rhs: Vec3) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3{ x: self.y * rhs.z - self.z * rhs.y, y : self.z * rhs.x - self.x * rhs.z, z : self.x * rhs.y - self.y * rhs.x }
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * Vec3::dot(self, normal) * normal
    }

    pub fn refract(self, normal: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = Vec3::dot(-self, normal).min(1.0);
        let perpendicular = etai_over_etat * (self * cos_theta * normal);
        let parallel = -(1.0 - perpendicular.length_squared()).abs().sqrt() * normal;
        perpendicular + parallel
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        Self{ x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z }
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{ x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self{ x: -self.x, y: - self.y, z: -self.z }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range")
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range")
        }
    }
}