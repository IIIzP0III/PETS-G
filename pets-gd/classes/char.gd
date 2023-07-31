class_name PChar
extends Node2D

@onready var sprite			= $Sprite2D
@onready var anim_player	= $AnimationPlayer
@onready var anim_tree		= $AnimationTree
@onready var anim_state		= anim_tree["parameters/playback"]

func _ready():
	anim_tree.active = true

func anim_move(moving: bool, inputs: Vector2):
	anim_tree.set("parameters/Idle/blend_position", inputs)
	anim_tree.set("parameters/Run/blend_position", inputs)
	
	if moving:
		anim_state.travel("Run")
	else:
		anim_state.travel("Idle")
		print(anim_state)
