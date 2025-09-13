use godot::prelude::*;
use godot::classes::ICharacterBody3D;
use godot::classes::CharacterBody3D;
use godot::classes::Input;
use godot::classes::AnimationPlayer;


#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct PlayerHop {
	score: i64,
	jump_impulse: f32,
	speed: f32,
	fall_acceleration: f32,
	bounce_impulse: f32,
	target_velocity: Vector3,
	base: Base<CharacterBody3D>
}
use crate::mobs::Mob;
use crate::scorelabel::ScoreLabel;

#[godot_api]
impl PlayerHop {
	#[signal]
	fn squashed();
	fn on_squashed(&mut self) {
		let mut lab: Gd<ScoreLabel> = self.base_mut().get_node_as("/root/Main/UserInterface/ScoreLabel");
		self.score += 1;
                let text = format!("Score: {0}", self.score);
                godot_print!("Setting text...");
                lab.set_text(&text);


	}

}

#[godot_api]
impl ICharacterBody3D for PlayerHop {
	fn init(base: Base<CharacterBody3D>) -> Self {
		godot_print!("Initializing hopping player");

		Self {
			score: 0,
			jump_impulse: 20.0,
			speed: 14.0,
			bounce_impulse: 16.0,
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
			let mut anim: Gd<AnimationPlayer> = self.base_mut().get_node_as("/root/Main/Player/AnimationPlayer");
			anim.set_speed_scale(4.0);
		} else {
			let mut anim: Gd<AnimationPlayer> = self.base_mut().get_node_as("/root/Main/Player/AnimationPlayer");
			anim.set_speed_scale(1.0);
		}
		

		self.target_velocity.x = direction.x * self.speed;
		self.target_velocity.z = direction.z * self.speed;
		
		if self.base().is_on_floor() && go.is_action_just_pressed("jump") {
			self.target_velocity.y = self.jump_impulse;
		}
		if self.base().is_on_floor() != true {
			self.target_velocity.y = self.target_velocity.y - (self.fall_acceleration * delta);
		
		}
		for index in 0..self.base().get_slide_collision_count() {
			let collision = self.base_mut().get_slide_collision(index).expect("Null collision!");
			let col_type = collision.get_collider();
			match col_type {
				Some(_) => {
					let col_class_name = col_type.expect("Not valid collision!").get_class();
					let col_string: String = col_class_name.to_string();
		//			godot_print!("{col_string}");
		//			let mob_str = String::from("Mob");
					match col_string.contains("Mob") {
						true => {
							let mut mob: Gd<Mob> = collision.get_collider().expect("Null collision!").try_cast::<Mob>()
							.expect("Not a mob!");
							godot_print!("Successfully cast a mob!");

							if Vector3::UP.dot(collision.get_normal()) > 0.1 {
								self.target_velocity.y = self.bounce_impulse;
		//						mob.bind_mut().squash();
								self.signals().squashed().emit();
								mob.bind_mut().drop_and_roll();
								break;
							}
						
						}
						
						_ => {
							//pass
						}
					}

				}
				None => {continue;}
			}
		}
		let targ_vel = self.target_velocity;
		self.base_mut().set_velocity(targ_vel);
		self.base_mut().move_and_slide();
		let mut piv: Gd<Node3D> = self.base_mut().get_node_as("/root/Main/Player/Pivot");
		let mut piv_trans = piv.get_rotation();
		piv_trans.x = real_consts::PI / (6.0 * targ_vel.y) / self.jump_impulse;
		piv.set_rotation(piv_trans);

	}
	fn ready(&mut self) {
		self.signals().squashed().connect_self(PlayerHop::on_squashed);
	}
}


