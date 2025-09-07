use godot::prelude::*;
use godot::classes::ICharacterBody3D;
use godot::classes::CharacterBody3D;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct PlayerHop {
	base: Base<CharacterBody3D>
}


#[godot_api]
impl ICharacterBody3D for PlayerHop {
	fn init(base: Base<CharacterBody3D>) -> Self {
		godot_print!("Initializing hopping player");

		Self {
			base

		}
	}


}
