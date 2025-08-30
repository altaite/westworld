use bevy::prelude::*;
use std::f32::consts::PI;

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
    health: f32,
    satiety: f32,
    water: f32,
    pub max_step: f32,
    pub max_turn: f32,
}

impl Creature {
    pub fn new(position: Position, angle: f32) -> Self {
        Self {
            position,
            angle,
            health: 1.0,
            satiety: 1.0,
            water: 1.0,
            max_step: 1.0,
            max_turn: PI / 4.0,
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        let distance = distance.clamp(0.0, self.max_step);
        self.position.x += self.angle.cos() * distance;
        self.position.y += self.angle.sin() * distance;
    }

    pub fn rotate(&mut self, delta: f32) {
        let delta = delta.clamp(-self.max_turn, self.max_turn);
        self.angle = (self.angle + delta).rem_euclid(2.0 * PI);
    }

    pub fn set_health(&mut self, value: f32) {
        self.health = value.clamp(0.0, 1.0);
    }

    pub fn set_satiety(&mut self, value: f32) {
        self.satiety = value.clamp(0.0, 1.0);
    }

    pub fn set_water(&mut self, value: f32) {
        self.water = value.clamp(0.0, 1.0);
    }

    pub fn health(&self) -> f32 {
        self.health
    }

    pub fn satiety(&self) -> f32 {
        self.satiety
    }

    pub fn water(&self) -> f32 {
        self.water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn movement_and_rotation_respect_limits() {
        let mut creature = Creature::new(Position::new(0.0, 0.0), 0.0);
        creature.move_forward(2.0);
        assert!((creature.position.x - 1.0).abs() < 1e-6);
        assert!(creature.position.y.abs() < 1e-6);

        creature.rotate(PI);
        assert!((creature.angle - PI / 4.0).abs() < 1e-6);
    }

    #[test]
    fn attributes_clamped_between_zero_and_one() {
        let mut creature = Creature::new(Position::new(0.0, 0.0), 0.0);
        creature.set_health(-1.0);
        creature.set_satiety(2.0);
        creature.set_water(0.5);
        assert_eq!(creature.health(), 0.0);
        assert_eq!(creature.satiety(), 1.0);
        assert_eq!(creature.water(), 0.5);
    }
}

