# Pacman
A recreation of the arcade game "Pacman" with rust and the bevy engine.

Despite its age and appearance, Pacman was a quite complex game. Therefore, this project aims to battle test bevy.

[Play the latest WASM build (last updated October 10, 2022)](https://warhorst.github.io/pacman/)

(Use WASD or arrow keys to control pacman. Click into the canvas if it's not working)

## Main resources
- the great pacman dossier: https://www.gamedeveloper.com/design/the-pac-man-dossier
- the game in action: https://www.youtube.com/watch?v=-CbyAk3Sn9I
- longplay until the game glitches out: https://www.youtube.com/watch?v=AuoH0vz3Mqk

### Font
- Press Start 2P Font: https://fonts.google.com/specimen/Press+Start+2P

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

Did some cutting with [LosslessCut](https://github.com/mifi/lossless-cut) and https://mp3cut.net

(Tip: Don't even try to download the sounds from other sides than YouTube. The quality is trash and the tracks are incomplete)


## TODOS
Things that are required for 1.0
- save a highscore for the current session and play a sound when beaten
- ghosts only change direction at specific points, like in the real game
- create a 1.0 WASM build

### Current Bugs
- the music does not resume after the player restarted the game after a game over

## Ideas
Some ideas for future development
- a real map editor (maybe using the bevy scenes format)
- main menu with persistent highscores and intro sequence
- cutscenes
- savegames
- adding the other pacman games