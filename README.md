# Pacman
A recreation of the arcade game "Pacman" with rust and the bevy engine.

Despite its age and appearance, Pacman was a quite complex game. Therefore, this project aims to battle test bevy.

## TODOs
(This is a WIP list of things to implement. Will be updated frequently)

### Pacman
- "waka waka" animation + sound when eating dots (very important)

### Lifecycle
- the game ends if pacman dies without remaining lives
- reset ghosts when pacman dies

### Tunnel
- (Maybe) remove the tunnel component and only use the board resource

### Fruit
- enable fruit spawn
- fruit points and appearance change based on the current level
- eating the fruits gives pacman points

### Graphics
- animations for pacman when walking around
- animations for pacman when he dies
- animations for ghost when walking around
- animations for ghost when frightened
- blinking effect for frightened ghost when the frightened state is almost over
- sprites for dots
- sprites for energizer
- sprites for inner walls
- sprites for the ghost house entrance
- sprites for ghost house walls
- return the correct z coordinate from the board (or only x and y)
- render points for eating a ghost
- render points for eating a fruit

### Appearance
- proper game start
- dramatic pauses when specific actions happen (like eating ghosts)
- sound (this one might be split later)

### UI
- main menu to start the game (and later access the high score)
- pause menu to quit the game

### Refactoring
- add every map element (walls, tunnels, ghost house) as an entity
- (Maybe) change the point to a field struct to remove these stupid getters
- (Maybe) remove the Position component (but keep using it in systems). It can be generated quickly from a transform. This way only the transform needs to be updated.