extends CharacterBody2D

"""
This scene contains the "player" aka the invisible
entity that is moved around with WASD. It also contains
party members as scenes, and this script does stuff like
running animations on those nodes too.
"""

const ACCELERATION	:= 3000
const FRICTION		:= 2500
const MAX_SPEED		:= 320

# For i in members, position is (distance * i)th element of queue
var pastPositions = LimitedQueue.new(2000)

func _physics_process(delta):
	var input_vector := Input.get_vector("left", "right", "up", "down").normalized()
	var moving := input_vector != Vector2.ZERO
	var oldPos = position
	
	if moving:
		velocity = velocity.move_toward(input_vector * MAX_SPEED, delta * ACCELERATION)
	else:
		velocity = velocity.move_toward(Vector2.ZERO, delta * FRICTION)
	
	move_and_slide()
	
	if oldPos != position:
		pastPositions.push_front(position)
	
	move_chars()

func move_chars():
	pass
