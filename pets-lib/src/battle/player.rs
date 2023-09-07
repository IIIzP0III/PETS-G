//!
//! Player icon that moves around n shit during battles
//!

use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

type DirectionalInputNames = [(&'static str, Vector2); 4];

// I spent legit 2 hours trying to find a
// good way to do this at compile-time without
// repetition or leaking as static...
//
// hopefully this'll be fixed later but it's
// still better than running format!() once every
// time process() is called.
const BATTLE_DIRECTIONS: DirectionalInputNames = [
    ("battle_move_up", Vector2::UP),
    ("battle_move_down", Vector2::DOWN),
    ("battle_move_left", Vector2::LEFT),
    ("battle_move_right", Vector2::RIGHT),
];

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleIcon {
    #[base]
    node: Base<Node2D>,

    /// Speed of player icon
    speed: f32,
    acceleration: f32,
    friction: f32,

    /// Current velocity of player icon
    velocity: Vector2,
}

#[godot_api]
impl Node2DVirtual for BattleIcon {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,

            speed: 120.0,
            acceleration: 0.2,
            friction: 0.2,

            velocity: Vector2::new(0.0, 0.0),
        }
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();
        let mut accel_amt = Vector2::new(0.0, 0.0);

        self.velocity *= self.friction;

        for (k, v) in BATTLE_DIRECTIONS.into_iter() {
            if input.is_action_pressed(k.into()) {
                accel_amt += v * self.acceleration;
            }
        }

        // if self.velocity.length() > 0.0 {
        self.velocity += accel_amt;
        self.velocity = self.velocity.normalized() * self.speed;

        let change = self.velocity * real::from_f64(delta);
        let position = self.node.get_global_position() + change;
        let position = Vector2::new(position.x.clamp(0.0, 1920.0), position.y.clamp(0.0, 1080.0));
        self.node.set_global_position(position);
    }
}
