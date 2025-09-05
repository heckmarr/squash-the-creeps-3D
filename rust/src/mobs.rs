use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::global::randf_range;
use godot::global::randi_range;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Mob {
	base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for Mob {
	fn init(base: Base<CharacterBody3D>) -> Self {
		godot_print!("Initializing mob");

		Self {
			base
		}
	}
	fn physics_process(&mut self, _delta: f32) {

		self.base_mut().move_and_slide();
	}
}

