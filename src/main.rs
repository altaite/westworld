mod creature;

use creature::{Creature, Position};

fn main() {
    let mut c = Creature::new(Position::new(0.0, 0.0), 0.0);
    c.move_forward(0.5);
    c.rotate(0.1);
    println!("{:?}", c);
}
