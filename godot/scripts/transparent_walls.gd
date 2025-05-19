extends MeshInstance3D

func _process(_delta: float) -> void:
	# Получаем путь к игроку (в file[2] содержится имя сцены, а игрок
	# находится по пути: /root/SceneName/Player).
	var file = String(get_path()).split("/")
	var player_path = '/' + "root" + '/' + file[2] + "/Player"	

	# Получаем путь к родителю путем отсекания своего имени и слэша.
	var parent_mass = String(get_path()).split('/')
	parent_mass[-1] = ""
	
	var parent_path = "/".join(parent_mass)
	parent_path[-1] = ""
	
	# Получаем родителя и игрока.
	var player: Player = get_node(player_path)
	var parent: StaticBody3D = get_node(parent_path)	

	# Получаем альфу.
	var alpha = (
		max(player.position.z, parent.position.z) - 
		min(player.position.z, parent.position.z) - 1.6
	)

	# Устанавливаем альфу для шейдера (подразумевается шейдер transparent_wall.gdshader).
	mesh.material.set_shader_parameter("alpha", alpha)

