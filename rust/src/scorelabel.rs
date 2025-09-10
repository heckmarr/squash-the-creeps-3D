use godot::prelude::*;

use godot::classes::Label;
use godot::classes::ILabel;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct ScoreLabel {
	score: i64,
	text: String,
	base: Base<Label>
}

#[godot_api]
impl ILabel for ScoreLabel {
	fn init(base: Base<Label>) -> Self {
		let sl = 0;
		let txt = String::from("Score: 0");
		Self {
			text: txt,
			score: sl,
			base
		}
	}
}

#[godot_api]
impl ScoreLabel {
	
	#[signal]
	fn squashed();


}

