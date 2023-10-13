//!
//! Dialog box class for menus and dialogue text
//!

use godot::engine::{PanelContainer, PanelContainerVirtual, RichTextLabel};
use godot::prelude::*;

use crate::prelude::*;

#[derive(GodotClass)]
#[class(base=PanelContainer)]
struct DialogBox {
    #[base]
    node: Base<PanelContainer>,

    // Stat Interface reference-counted
    // This is fine to keep a ref to cuz they won't be dropped anyway
    si: Gd<StatsInterface>,
}

#[godot_api]
impl DialogBox {
    #[func]
    fn do_draw(&mut self) {
        self.spk_txt().set_text("Cherry".into());
        self.msg_txt()
            .set_text("[wave amp=50 freq=6]Hello, World![/wave]".into());
    }

    /// Get the speaker name label
    fn spk_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("SpeakerName/Text")
    }

    /// Get the message text label
    fn msg_txt(&self) -> Gd<RichTextLabel> {
        self.node.get_node_as::<RichTextLabel>("Content/Text")
    }
}

#[godot_api]
impl PanelContainerVirtual for DialogBox {
    fn init(node: Base<PanelContainer>) -> Self {
        Self {
            node,
            si: StatsInterface::singleton(),
        }
    }

    fn ready(&mut self) {
        self.do_draw();
    }
}
