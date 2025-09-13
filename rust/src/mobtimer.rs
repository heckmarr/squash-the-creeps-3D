use godot::prelude::*;

use godot::classes::Timer;
use godot::classes::ITimer;
use godot::classes::PathFollow3D;
use godot::classes::AnimationPlayer;

use godot::global::randf;
use godot::global::randi_range;
use godot::global::randf_range;

use crate::mobs::Mob;
use crate::hopplayer::PlayerHop;

#[derive(GodotClass)]
#[class(base=Timer)]
pub struct MobTimer {
	min_speed: i64,
	max_speed: i64,
	mob_scene: Gd<PackedScene>,
	base: Base<Timer>
}

#[godot_api]
impl ITimer for MobTimer {
	fn init(base: Base<Timer>) -> Self {
		let mobscene = load("res://mob.tscn");
		Self {
			min_speed: 10,
			max_speed: 18,
			mob_scene: mobscene,
			base
		}
	}
	fn ready (&mut self) {
		self.signals().timeout().connect_self(MobTimer::player_pos_and_init);
	}
}

#[godot_api]
impl MobTimer {
	fn player_pos_and_init(&mut self) {
		let mut mob_spawn_location: Gd<PathFollow3D> = self.base().get_node_as("/root/Main/SpawnPath/SpawnLocation");
		let random_value: f32 = randf() as f32;
		mob_spawn_location.set_progress_ratio(random_value);
		let start_position: Vector3 = mob_spawn_location.get_position();
		let play_obj: Gd<PlayerHop> = self.base().get_node_as("/root/Main/Player");
		let player_position = play_obj.get_position();

                let mut mob_node = self.mob_scene.instantiate().unwrap();
                self.base_mut().add_child(&mob_node.clone());
                mob_node.set_owner(&self.base().clone().upcast::<Node>());
                let mob_name = GString::from(mob_node.get_name());
//                godot_print!("creating {mob_name}");
                let mob_obj_node = self.base_mut().find_child(&mob_name).expect("Failed to find Mob!");
                let mob_path = mob_obj_node.get_path();
                let mut mob: Gd<Mob> = mob_obj_node.get_node_as(&mob_path);
                //done casting


                //Look at the player
                mob.look_at_from_position(start_position, player_position);
                //random rotation
                let rot = randf_range((-real_consts::PI / 4.0).into(), (real_consts::PI / 4.0).into()) as f32;
                mob.rotate_y(rot.into());
                let mob_rot = mob.get_rotation();


                //calculate a randomized speed Vector3 relative to the model's front
                let random_speed = randi_range(self.min_speed, self.max_speed) as f32;
                //calculate the vector
                let vel: Vector3 = Vector3::FORWARD * random_speed;
                let v = vel.rotated(Vector3::UP, mob_rot.y);
		let mut anim_path = "/root/Main/MobTimer/".to_owned();
		anim_path.push_str(&mob_node.get_name().to_string());
		anim_path.push_str("/AnimationPlayer");
		let mut anim: Gd<AnimationPlayer> = self.base_mut().get_node_as(&anim_path);
		anim.set_speed_scale(random_speed / self.min_speed as f32);
                mob.set_velocity(v);

	}
}
