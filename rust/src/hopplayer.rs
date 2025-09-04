use godot::prelude::*;
use godot::classes::ICharacterBody3D;
use godot::classes::CharacterBody3D;
use godot::classes::Input;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct PlayerHop {
	speed: f32,
	fall_acceleration: f32,
	target_velocity: Vector3,
	base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for PlayerHop {
	fn init(base: Base<CharacterBody3D>) -> Self {
		godot_print!("Initializing hopping player");

		Self {
			speed: 14.0,
			fall_acceleration: 75.0,
			target_velocity: Vector3::ZERO,
			base

		}
	}

	fn physics_process(&mut self, delta: f32) {
		let mut direction = Vector3::ZERO;
		let go = Input::singleton();

		if go.is_action_pressed("move_right") {
			direction.x += 1.0;
		}
		if go.is_action_pressed("move_left") {
			direction.x -= 1.0;
		}
		if go.is_action_pressed("move_back") {
			direction.z += 1.0;
		}
		if go.is_action_pressed("move_forward") {
			direction.z -= 1.0;
		}
		if direction != Vector3::ZERO {
			direction = direction.normalized();
			let pivot_node = self.base().find_child("Pivot").expect("No Pivot in tree!");
			let pivot_path = pivot_node.get_path();
			let mut pivot_obj: Gd<Node3D> = pivot_node.get_node_as(&pivot_path);
			pivot_obj.set_basis(Basis::looking_at(direction, Vector3::UP, true));
		}

		self.target_velocity.x = direction.x * self.speed;
		self.target_velocity.z = direction.z * self.speed;
		
		if self.base().is_on_floor() != true {
			self.target_velocity.y = self.target_velocity.y - (self.fall_acceleration * delta);
		}
		let targ_vel = self.target_velocity;
		self.base_mut().set_velocity(targ_vel);
		self.base_mut().move_and_slide();
		
	}


}
