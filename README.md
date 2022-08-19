# Pacman
A recreation of the arcade game "Pacman" with rust and the bevy engine.

Despite its age and appearance, Pacman was a quite complex game. Therefore, this project aims to battle test bevy.

[Play the latest build (WIP)](https://warhorst.github.io/pacman/)

(Use WASD or arrow keys to control pacman. Click into the canvas if it's not working)

## Main resources
- the great pacman dossier: https://www.gamedeveloper.com/design/the-pac-man-dossier
- the game in action: https://www.youtube.com/watch?v=-CbyAk3Sn9I
- longplay until the game glitches out: https://www.youtube.com/watch?v=AuoH0vz3Mqk

## TODOs
(This is a WIP list of things to implement. Will be updated frequently)

### Pacman
- sound when eating dots

### Ghosts
- adapt Blinkys speed based on remaining dots

### Graphics
- return the correct z coordinate from the board (or only x and y)
- render points for eating a ghost
- render points for eating a fruit

### Appearance
- dramatic pauses when specific actions happen (like eating ghosts)
- sound (this one might be split later)

### UI
- main menu to start the game (and later access the high score)
- pause menu to quit the game

### Other
- the memory consumption keeps rising when doing nothing (from 90MB initial to up to 500MB after an hour) -> the app might create some resources infinitely