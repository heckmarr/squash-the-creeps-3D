use godot::prelude::*;

use godot::classes::VisibleOnScreenNotifier3D;
use godot::classes::IVisibleOnScreenNotifier3D;

#[derive(GodotClass)]
#[class(base=VisibleOnScreenNotifier3D)]
pub struct CreepNotifier {
	base: Base<VisibleOnScreenNotifier3D>
}

#[godot_api]
impl CreepNotifier {
	pub fn dorp(&mut self) {
		godot_print!("Dropping creep!");
                self.base_mut().queue_free();
        }
}

#[godot_api]
impl IVisibleOnScreenNotifier3D for CreepNotifier {

	fn init(base: Base<VisibleOnScreenNotifier3D>) -> Self {

		Self {
			base
		}
	}
	fn ready(&mut self) {
		self.signals().screen_exited().connect_self(CreepNotifier::dorp);
	}


}
