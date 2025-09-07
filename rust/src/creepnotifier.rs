use godot::prelude::*;

use godot::classes::VisibleOnScreenNotifier3D;
use godot::classes::IVisibleOnScreenNotifier3D;

#[derive(GodotClass)]
#[class(base=VisibleOnScreenNotifier3D)]
pub struct CreepNotifier {
	base: Base<VisibleOnScreenNotifier3D>

}

use crate::mobs::Mob;

#[godot_api]
impl CreepNotifier {
	pub fn dorp(&mut self) {
		//get the parent
		let creep_parent = self.base().get_parent().expect("Highest node!");
		let creep_path = creep_parent.get_path();
		let mut creep_mob: Gd<Mob> = creep_parent.get_node_as(&creep_path);
		//Get it's name for posterity's sake
		let name = creep_mob.get_name();
		//print said name
//		godot_print!("Dropping {name}");
		//queue free the parent
		creep_mob.queue_free();
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
