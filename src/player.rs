use macroquad::experimental::animation::*;
use macroquad::prelude::*;

use crate::{get_tile_size, GRID_COLS_COUNT, GRID_ROWS_COUNT};

pub struct Player {
    pub position: Vec2,
    pub texture: Texture2D,
    pub sprite: AnimatedSprite,
    speed: f32,
    pub reach: f32,
    pub score: f32,
    pub flip_x: bool,
}

impl Player {
    pub fn new(
        position: Vec2,
        texture: Texture2D,
        sprite: AnimatedSprite,
        speed: f32,
        reach: f32,
    ) -> Self {
        Self {
            position,
            texture,
            sprite,
            speed,
            reach,
            score: 0.0,
            flip_x: false,
        }
    }

    pub fn update(&mut self, direction: &Vec2) {
        self.flip_x = direction.x < 0.0;
        self.position += *direction * (self.speed * get_frame_time());
        self.position = self.position.clamp(
            Vec2::new(0.0, 0.0),
            Vec2::new(GRID_COLS_COUNT as f32, GRID_ROWS_COUNT as f32),
        );
        match direction {
            Vec2 { x: 0.0, y: 0.0 } => {
                self.sprite.set_animation(0);
            }
            Vec2 { x: 0.0, y: 1.0 } => {
                self.sprite.set_animation(1);
            }
            Vec2 { x: 0.0, y: -1.0 } => {
                self.sprite.set_animation(3);
            }
            _ => {
                self.sprite.set_animation(2);
            }
        }
        self.sprite.update();
    }
}
