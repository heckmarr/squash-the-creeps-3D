use godot::prelude::*;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::global::randf_range;
use godot::global::randi_range;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Mob {
	min_speed: i64,
	max_speed: i64,
	base: Base<CharacterBody3D>
}

#[godot_api]

impl ICharacterBody3D for Mob {
	fn init(base: Base<CharacterBody3D>) -> Self {
		godot_print!("Initializing mob");

		Self {
			min_speed: 10,
			max_speed: 18,
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
	
		let rand_speed = randi_range(self.min_speed, self.max_speed) as f32;
		self.base_mut().set_velocity( Vector3::FORWARD * rand_speed);
		let vel = self.base().get_velocity();
		let rot = self.base().get_rotation();
		self.base_mut().set_velocity(vel.rotated(Vector3::UP, rot.y));

	}
}
