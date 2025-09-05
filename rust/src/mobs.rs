use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::global::randf_range;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Mob {
	min_speed: f32,
	max_speed: f32,
	base: Base<CharacterBody3D>
}

#[godot_api]

impl ICharacterBody3D for Mob {
	fn init(base: Base<CharacterBody3D>) -> Self {
		godot_print!("Initializing mob");

		Self {
			min_speed: 10.0,
			max_speed: 18.0,
			base
		}
	}


	fn physics_process(&mut self, _delta: f32) {
		self.base_mut().move_and_slide();
	}
}

impl Mob {
	fn initialize_mob(&mut self, start_position: Vector3, player_position: Vector3) {
		self.base_mut().look_at_from_position(start_position, player_position);
		let rand = randf_range(((-real_consts::PI)/4.0).into(), (real_consts::PI/4.0).into())
			as f32;
		self.base_mut().rotate_y(rand);
	}
}
