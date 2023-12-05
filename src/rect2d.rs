use crate::vec2d::Vec2D;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rect2D {
    pub topleft: Vec2D,
    pub dimensions: Vec2D,
}

impl Rect2D {
    /// Range inclusive
    pub fn contains(&self, coord: &Vec2D) -> bool {
        let range_x = self.topleft.x..=(self.topleft.x + self.dimensions.x);
        let range_y = self.topleft.y..=(self.topleft.y + self.dimensions.y);
        range_x.contains(&coord.x) && range_y.contains(&coord.y)
    }
}
