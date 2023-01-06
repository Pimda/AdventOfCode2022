use crate::vectors::Vec2D;

pub fn get_adjecent_directions() -> [Vec2D; 4] {
    [
        Vec2D::new(-1, 0),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(0, 1),
    ]
}

pub fn get_adjecent_directions_including_self() -> [Vec2D; 5] {
    [
        Vec2D::new(-1, 0),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(0, 1),
        Vec2D::new(0, 0),
    ]
}

pub fn get_all_surrounding_directions() -> [Vec2D; 8] {
    [
        Vec2D::new(-1, 0),
        Vec2D::new(1, 0),
        Vec2D::new(0, -1),
        Vec2D::new(0, 1),
        Vec2D::new(-1, -1),
        Vec2D::new(1, -1),
        Vec2D::new(1, 1),
        Vec2D::new(-1, 1),
    ]
}