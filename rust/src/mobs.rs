use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Mob {
	base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for Mob {
	fn init(base: Base<CharacterBody3D>) -> Self {
	//	godot_print!("Initializing mob");

		Self {
			base
		}
	}
	fn physics_process(&mut self, _delta: f32) {

		self.base_mut().move_and_slide();
	}
	fn ready(&mut self) {
		self.signals().squashed().connect_self(Self::drop_and_roll);
	}
}

#[godot_api]
impl Mob {
	#[signal]
	fn squashed();

	pub fn drop_and_roll(&mut self) {
		self.base_mut().queue_free()
	}
}
