use bevy::prelude::*;
use std::f32::consts::{PI, TAU};

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug)]
pub struct Creature {
    pub position: Position,
    pub angle: f32,
    pub max_step: f32,
    pub max_turn: f32,
}

impl Creature {
    pub fn new(position: Position, angle: f32) -> Self {
        Self {
            position,
            angle,
            // Maximum distance a creature can travel in one second.
            // A larger value helps make movement visually noticeable.
            max_step: 100.0,
            max_turn: TAU,
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        let distance = distance.clamp(0.0, self.max_step);
        self.position.x += -self.angle.sin() * distance;
        self.position.y += self.angle.cos() * distance;
    }

    pub fn rotate(&mut self, delta: f32) {
        let delta = delta.clamp(-self.max_turn, self.max_turn);
        self.angle = (self.angle + delta).rem_euclid(2.0 * PI);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_and_rotation_respect_limits() {
        let mut creature = Creature::new(Position::new(0.0, 0.0), 0.0);
        creature.move_forward(200.0);
        assert!(creature.position.x.abs() < 1e-6);
        assert!((creature.position.y - 100.0).abs() < 1e-6);

        creature.rotate(10.0 * TAU);
        assert!(creature.angle.abs() < 1e-6);
    }
}
