//!
//! Manages the interaction zones in the world.
//! Shows the input prompt and handles the action if pressed.
//!

use godot::prelude::*;

use godot::engine::{Engine, Object, ObjectVirtual};

use crate::world::interaction::zone::InteractionZone;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct InteractionManager {
    #[base]
    node: Base<Object>,
}

#[godot_api]
impl InteractionManager {
    // #[signal]
    fn register_zone(obj: Gd<InteractionZone>) {
        {
            let obj = obj.bind();
            obj.get_name()
        };
    }

    pub fn singleton() -> Gd<InteractionManager> {
        Engine::singleton()
            .get_singleton("Interactions".into())
            .unwrap()
            .cast()
    }
}

#[godot_api]
impl ObjectVirtual for InteractionManager {
    fn init(node: Base<Object>) -> Self {
        Self { node }
    }
}
