use godot::prelude::*;

use godot::classes::Node;
use godot::classes::INode;
use godot::classes::ColorRect;
use godot::classes::InputEvent;
use godot::classes::AudioStreamPlayer;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainNode {
	base: Base<Node>
}

#[godot_api]
impl INode for MainNode {
	fn init(base: Base<Node>) -> Self {

		Self {
			base
		}
	}
	fn ready(&mut self) {
		let mut ui: Gd<ColorRect> = self.base_mut().get_node_as("/root/Main/UserInterface/Retry");
		ui.hide();
	}
	fn unhandled_input(&mut self, event: Gd<InputEvent>) {
		
                let ui: Gd<ColorRect> = self.base_mut().get_node_as("/root/Main/UserInterface/Retry");
                if ui.is_visible() && event.is_action_pressed("ui_accept")	{
                        self.base_mut().get_tree().expect("Not in a tree!").reload_current_scene();
                }

	}
}


impl Drop for MainNode {
	fn drop(&mut self) {
		godot_print!("Dropping MainNode!");
	}
}
