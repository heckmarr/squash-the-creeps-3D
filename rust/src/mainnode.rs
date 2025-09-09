use godot::prelude::*;

use godot::classes::Node;
use godot::classes::INode;
use godot::classes::Timer;

use godot::global::randi_range;
use godot::global::randf_range;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MainNode {
	min_speed: i64,
	max_speed: i64,
	mob_scene: Gd<PackedScene>,
	base: Base<Node>
}

#[godot_api]
impl INode for MainNode {
	fn init(base: Base<Node>) -> Self {
		let mobscene = load("res://mob.tscn");

		Self {
			min_speed: 10,
			max_speed: 18,
			mob_scene: mobscene,
			base
		}
	}
	fn ready(&mut self) {
	}
}

use crate::mobs::Mob;
#[godot_api]
impl MainNode {
	fn spawn_and_get_player(&mut self) {
		godot_print!("Spawn a mob!");
	}

	#[func]
	fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
//		godot_print!("Timer went off! Spawn a mob!");
//		//cast to the newly instantiated mob scene
//		//Technically this spawns the mob directly under the center of the
//		//main scene, which was causing the character to "hop" and then
//		//get banished to another dimension unless you moved.
//		//Translating the mob .tscn root by a few ticks horizontally
//		//fixes it
//		let mut mob_node = self.mob_scene.instantiate().unwrap();
//		self.base_mut().add_child(&mob_node.clone());
//		mob_node.set_owner(&self.base().clone().upcast::<Node>());
//		let mob_name = GString::from(mob_node.get_name());
//		godot_print!("creating {mob_name}");
//		let mob_obj_node = self.base_mut().find_child(&mob_name).expect("Failed to find Mob!");
//		let mob_path = mob_obj_node.get_path();
//		let mut mob: Gd<Mob> = mob_obj_node.get_node_as(&mob_path);
//		//done casting
//		
//
//		//Look at the player
//		mob.look_at_from_position(start_position, player_position);
//		//random rotation
//		let rot = randf_range((-real_consts::PI / 4.0).into(), (real_consts::PI / 4.0).into()) as f32;
//		mob.rotate_y(rot.into());
//		let mob_rot = mob.get_rotation();
//
//
//		//calculate a randomized speed Vector3 relative to the model's front
//		let random_speed = randi_range(self.min_speed, self.max_speed) as f32;
//		//calculate the vector
//		let vel: Vector3 = Vector3::FORWARD * random_speed;
//		let v = vel.rotated(Vector3::UP, mob_rot.y);
//		mob.set_velocity(v);
	}
}

impl Drop for MainNode {
	fn drop(&mut self) {
		godot_print!("Dropping MainNode!");
	}
}
