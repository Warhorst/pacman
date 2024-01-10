# Pacman
A recreation of the arcade game "Pacman" with rust and the bevy engine.

Despite its age and appearance, Pacman was a quite complex game. Therefore, this project aims to battle test bevy.

[Play the latest WASM build (last updated October 10, 2022)](https://warhorst.github.io/pacman/)

(Use WASD or arrow keys to control pacman. Click into the canvas if it's not working)

## State of the game
The game is on version 1.0.0. As I implemented every feature listed in the pacman dossier (as far as I can tell), I consider the game feature complete.

There are still other things one can do, like:
- the main menu, with an introduction to all ghosts
- level transition cutscenes
- persisted high scores
- bugs from the original game, like Pinky cannot walk upwards or the "death screen"
- a new WASM build (would require an update of asset preload first)

The game was also designed (and therefore overengineered) with a map editor in mind (or at least custom maps). Make this will come to fruition one day.

## Main resources
- the great pacman dossier: (multiple links, because this beautiful article gets nuked frequently)
  - https://pacman.holenet.info 
  - https://web.archive.org/web/20220118071921/https://www.gamedeveloper.com/design/the-pac-man-dossier
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