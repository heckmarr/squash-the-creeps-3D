use godot::prelude::*;

use godot::classes::Node;
use godot::classes::INode;


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
	}
}


impl Drop for MainNode {
	fn drop(&mut self) {
		godot_print!("Dropping MainNode!");
	}
}
