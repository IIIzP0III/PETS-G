//!
//! Main menu scene
//! Should work somewhat closely with `savefiles.rs`
//!
//! "Oh, boy! More spaghetti code! I love spaghetti, and I love code!"
//! - Cherry, 2:54 AM, 10/5/2023 | <3
//!

use godot::engine::tween::TransitionType;
use godot::engine::{Control, Node2D, Node2DVirtual, RichTextLabel, Theme};
use godot::prelude::*;

use crate::prelude::*;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const MENU_TWEEN_TIME: f64 = 0.1;
const MENU_TWEEN_TRANS: TransitionType = TransitionType::TRANS_QUAD;
const MENU_WAVE_BBCODE: &str = "[wave amp=100 freq=-6]";

#[derive(Debug, FromPrimitive)]
enum MainMenuChoice {
    Play = 0,
    Options,
    Credits,
    Quit,
    DebugBattle,
}

const CHOICES_COUNT: usize = std::mem::variant_count::<MainMenuChoice>();
pub type Choices = [Gd<RichTextLabel>; CHOICES_COUNT];

#[derive(GodotClass)]
#[class(base=Node2D)]
struct TitleScreen {
    #[base]
    node: Base<Node2D>,
    choices: Option<Choices>,

    // null if game just started (no choice hovered)
    current_choice: Option<u8>,
}

#[godot_api]
impl TitleScreen {
    fn change_menu_choice(&mut self, diff: i16) {
        let old_choice = self.current_choice;

        let res: u8 = if let Some(n) = old_choice {
            (n as i16 + diff).rem_euclid(CHOICES_COUNT as i16) as u8
        } else {
            0
        };

        if let Some(old_choice) = old_choice {
            self.tween_choice_to(false, old_choice);
        }

        self.tween_choice_to(true, res);
        self.current_choice = Some(res);
    }

    fn tween_choice_to(&mut self, is_picked: bool, choice: u8) {
        let target_x = if is_picked { 64.0 } else { 0.0 };

        // assume choices is not null
        let choices = self.choices.as_mut().unwrap();
        let node = &mut choices[choice as usize];

        let theme = load::<Theme>("res://themes/theme_deft.tres");
        let target_col = if is_picked {
            theme.get_color("font_selected_color".into(), "RichTextLabel".into())
        } else {
            theme.get_color("default_color".into(), "RichTextLabel".into())
        };

        let mut x_tween = node.create_tween().unwrap();
        x_tween
            .tween_property(
                node.clone().upcast(),
                "position:x".into(),
                Variant::from(target_x),
                MENU_TWEEN_TIME,
            )
            .unwrap()
            .set_trans(MENU_TWEEN_TRANS);

        let mut color_tween = node.create_tween().unwrap();
        color_tween
            .tween_property(
                node.clone().upcast(),
                "theme_override_colors/default_color".into(),
                Variant::from(target_col),
                MENU_TWEEN_TIME,
            )
            .unwrap()
            .set_trans(MENU_TWEEN_TRANS);

        // set bbcode
        // extremely ugly and hacky solution, but...
        // how else could you work with in-band formatting? :P
        let old_text = node.get_text();
        let new_text = if is_picked {
            format!("{}{}", MENU_WAVE_BBCODE, old_text)
        } else {
            let st: String = old_text.into();
            st[MENU_WAVE_BBCODE.len()..].to_owned()
        };

        node.set_text(new_text.into());
    }

    fn pick_choice(&mut self, choice: MainMenuChoice) {
        use MainMenuChoice::*;

        match choice {
            Play => {
                // TODO should animate the menu boxes flying
                // off into the right, and the camera goes left
                self.node
                    .get_tree()
                    .unwrap()
                    .change_scene_to_file("res://scenes/world.tscn".into());
            }

            Options => {
                // TODO scroll right into the options menu
                // leaving this empty so it won't panic
            }

            Credits => {
                // TODO tween the credits box
                // leaving this empty so it won't panic
            }

            Quit => {
                let mut tree = self.node.get_tree().unwrap();
                tree.quit();
            }

            DebugBattle => {
                self.node
                    .get_tree()
                    .unwrap()
                    .change_scene_to_file("res://scenes/battle_engine.tscn".into());
            }
        }
    }
}

#[godot_api]
impl Node2DVirtual for TitleScreen {
    fn init(node: Base<Node2D>) -> Self {
        Self {
            node,
            choices: None,
            current_choice: None,
        }
    }

    fn process(&mut self, _delta: f64) {
        let input = Input::singleton();

        let going_down = input.is_action_just_pressed("ui_down".into());
        let going_up = input.is_action_just_pressed("ui_up".into());
        let submitting = input.is_action_just_pressed("ui_accept".into());

        if submitting {
            let choice = self.current_choice;

            if let Some(choice) = choice {
                let choice = MainMenuChoice::from_u8(choice).unwrap();

                self.pick_choice(choice);
                return;
            }
        }

        if going_down {
            self.change_menu_choice(1);
        } else if going_up {
            self.change_menu_choice(-1);
        }
    }

    fn ready(&mut self) {
        // The node that contains the text labels for
        // all the main menu options you can pick
        let cont = self.node.get_node_as::<Control>("Background/MenuChoices");

        self.choices = Some([
            cont.get_node_as("Play"),
            cont.get_node_as("Options"),
            cont.get_node_as("Credits"),
            cont.get_node_as("Quit"),
            cont.get_node_as("DebugBattle"),
        ]);
    }
}
