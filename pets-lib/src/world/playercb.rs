use godot::engine::{CharacterBody2D, CharacterBody2DVirtual};
use godot::prelude::*;

use crate::{load_pchar_scene_under, prelude::*};

use super::pchar_node::PCharNode;

// Movement physics stuff
const ACCELERATION: f64 = 3000.0;
const FRICTION: f64 = 2500.0;
const MAX_SPEED: f64 = 320.0;

// Distance between party members
const PERSONAL_SPACE: u16 = 15;

/// This scene contains the "player" aka the invisible
/// entity that is moved around with WASD. It also contains
/// party members as scenes, and this script does stuff like
/// running animations on those nodes too.
#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct PlayerCB {
    #[base]
    node: Base<CharacterBody2D>,

    party: Vec<Gd<PCharNode>>,
    past_positions: LimiQ<Vector2>,
    past_rotations: LimiQ<Vector2>,
}

#[godot_api]
impl PlayerCB {
    fn move_chars(&mut self, moving: bool) {
        if self.past_positions.len() == 0 {
            return;
        }

        for (i, ch) in self.party.iter_mut().enumerate() {
            // index of past data limqs
            let nth = i * PERSONAL_SPACE as usize;

            ch.set_global_position(*self.past_positions.get_or_last(nth));

            {
                let mut ch = ch.bind_mut();
                ch.anim_move(moving, *self.past_rotations.get_or_last(nth));
            }
        }
    }
}

#[godot_api]
impl CharacterBody2DVirtual for PlayerCB {
    fn init(node: Base<CharacterBody2D>) -> Self {
        Self {
            node,

            party: vec![],
            past_positions: LimiQ::new(2000),
            past_rotations: LimiQ::new(2000),
        }
    }

    fn ready(&mut self) {
        self.party = vec![
            load_pchar_scene_under!(self, "agent_e"),
            load_pchar_scene_under!(self, "agent_s"),
            load_pchar_scene_under!(self, "agent_t"),
            load_pchar_scene_under!(self, "mira"),
            load_pchar_scene_under!(self, "dubs"),
            load_pchar_scene_under!(self, "yoyo"),
        ];
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let input_vector = input
            .get_vector("left".into(), "right".into(), "up".into(), "down".into())
            .normalized();
        let moving = input_vector != Vector2::ZERO;

        let (toward, deltatimes) = if moving {
            (
                input_vector * real::from_f64(MAX_SPEED),
                delta * ACCELERATION,
            )
        } else {
            (Vector2::ZERO, (delta * FRICTION))
        };

        let velocity = self.node.get_velocity();
        self.node
            .set_velocity(velocity.move_toward(toward, deltatimes as f32));

        self.node.move_and_slide();

        let pos_updated = (self.past_positions.len() == 0)
            || (self.past_positions[0] != self.node.get_position());

        if pos_updated {
            self.past_positions.push(self.node.get_global_position());

            // don't push new input vector if slowing down
            self.past_rotations.push(if moving {
                input_vector
            } else {
                self.past_rotations
                    .get(0)
                    .cloned()
                    .unwrap_or(Vector2 { x: 0.0, y: 0.0 })
            })
        }

        self.move_chars(moving)
    }
}
