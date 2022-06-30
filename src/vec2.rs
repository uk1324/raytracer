#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self{ x, y }
    }

    pub fn applied(&self, function: fn(f32) -> f32) -> Self {
        Self{ x: function(self.x), y: function(self.y) }
    }

    pub fn apply(&mut self, function: fn(f32) -> f32) -> &Self {
        self.x = function(self.x);
        self.y = function(self.y);
        self
    }

}