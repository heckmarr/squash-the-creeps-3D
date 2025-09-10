use godot::prelude::*;

use godot::classes::Area3D;
use godot::classes::IArea3D;
use godot::classes::Node3D;

use crate::mobtimer::MobTimer;

#[derive(GodotClass)]
#[class(base=Area3D)]
struct MobDetector {
	base: Base<Area3D>
}


#[godot_api]
impl IArea3D for MobDetector {
	fn init(base: Base<Area3D>) -> Self {
		Self {
			base
		}
	}
	fn ready(&mut self) {
		self.signals().body_entered().connect_self(MobDetector::on_hit);
		self.signals().hit().connect_self(MobDetector::stop_timer);
	}
}
#[godot_api]
impl MobDetector {

	#[signal]
	fn hit();

	fn stop_timer(&mut self) {
		let mut timer: Gd<MobTimer> = self.base_mut().get_node_as("/root/Main/MobTimer");
		godot_print!("Stopping mob spawn timer!");
		timer.stop();
	}

	fn on_hit(&mut self, body: Gd<Node3D>) {
		self.signals().hit().emit();
		let name_of_hit = body.get_name();
		godot_print!("Hit an enemy, {name_of_hit}!");
		let mut parent_node = self.base().get_parent().expect("No parent!");
		let parent_name = parent_node.get_name();
		godot_print!("Deleting {parent_name}");
		parent_node.queue_free();
	}
}
