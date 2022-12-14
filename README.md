# Bubble Rustle

## Authors
- Karol Kuźniak (@karqz on GitHub)

## Description
Bubble Rustle is going to be a platformer-shooter game similar to Bubble Trouble/Struggle.
Link to the youtube video of gameplay: <https://youtu.be/V5HAxc5RSwQ>. The game will not be a browser game.

## Features
- level generator
- shooting
- bubble physics
- game state saving and loading
- scores
- timer
- two player mode (optional)
- obstacles (optional)
- different guns (optional)

## Plan
In the first part we're going to implement the most important things: player movement, bubble physics and shooting. The bubbles will double/triple/quadruple and change size after being shot. There will be only one level for only one player.

In the second part we're going to add random level generator, scores and timer. In addition to this there will be two player mode, obstacles or different guns.

## Libraries
- Bevy (optionally ggez)
- Serde (only for development)
- kira (maybe for game audio)
- log
- rand
