# Pacman
A recreation of the arcade game "Pacman" with rust and the bevy engine.

Despite its age and appearance, Pacman was a quite complex game. Therefore, this project aims to battle test bevy.

[Play the latest build (WIP)](https://warhorst.github.io/pacman/)

(Use WASD or arrow keys to control pacman. Click into the canvas if it's not working)

## Main resources
- the great pacman dossier: https://www.gamedeveloper.com/design/the-pac-man-dossier
- the game in action: https://www.youtube.com/watch?v=-CbyAk3Sn9I
- longplay until the game glitches out: https://www.youtube.com/watch?v=AuoH0vz3Mqk

### Sound
Thanks to all these nice people for providing the sound effects:
- all in one: https://www.youtube.com/watch?v=SPjEhbRFTUk (contains cutscene in good quality)
- start: https://www.youtube.com/watch?v=HwyAwPLHqnM
- waka: https://www.youtube.com/watch?v=EPv04IRMjiA
- siren 1: https://www.youtube.com/watch?v=hGluoDwbWJs
- siren 2: https://www.youtube.com/watch?v=J2FnGBJ1jo4
- fruit eaten: https://www.youtube.com/watch?v=-hXMlrXdkrk
- ghosts scared: https://www.youtube.com/watch?v=cGCz6Zbjuuo
- ghost returns to house: https://www.youtube.com/watch?v=pJtnQasS5ak
- ghost eaten: https://www.youtube.com/shorts/hwM76RZ77ZE
- pacman dying: https://www.youtube.com/watch?v=NxSj2T2vx7M
- highscore: https://www.youtube.com/watch?v=LO9x-jQ5WCA

Downloaded with [yt-dlp](https://github.com/yt-dlp/yt-dlp)

Did some cutting with [LosslessCut](https://github.com/mifi/lossless-cut)

(Tip: Don't even try to download the sounds from other sides than YouTube. The quality is trash and the tracks are incomplete)


## TODOs
(This is a WIP list of things to implement. Will be updated frequently)

### Pacman
- sound when eating dots

### Graphics
- return the correct z coordinate from the board (or only x and y)

### Appearance
- sound (this one might be split later)

### UI
- main menu to start the game (and later access the high score)
- pause menu to quit the game

### Other
- the memory consumption keeps rising when doing nothing (from 90MB initial to up to 500MB after an hour) -> the app might create some resources infinitely