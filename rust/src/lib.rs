use godot::prelude::*;

struct SpinnyBot;

#[gdextension]
unsafe impl ExtensionLibrary for SpinnyBot {}

mod creepnotifier;
mod hopplayer;
mod mobs;
mod mainnode;
mod mobtimer;
