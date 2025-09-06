extends MainNode


func _on_mob_timer_timeout():
#	var mob = mob_scene.instantiate()
	var mob_spawn_location = get_node("SpawnPath/SpawnLocation")
	mob_spawn_location.progress_ratio = randf()
	
	var player_position = $Player.position
	self.initialize(mob_spawn_location.position, player_position)
