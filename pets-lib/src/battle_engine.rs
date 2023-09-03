use godot::engine::{Node2D, Node2DVirtual};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct BattleEngine {
    #[base]
    node: Base<Node2D>,
}

#[godot_api]
impl Node2DVirtual for BattleEngine {
    fn init(node: Base<Node2D>) -> Self {
        // Prints to the Godot console
        godot_print!("Hello, world!");
        Self { node }
    }

    fn process(&mut self, delta: f64) {
        // godot_print!("Hello, world!");
    }
}
