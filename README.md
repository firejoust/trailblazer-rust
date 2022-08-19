```
Trailblazer - Pathfinding module for Minecraft (NPM/Neon)

Roadmap:
> Parsing of chunk data from minecraft-protocol
> Block detection with raycasting & hitbox collision
> Player physics and velocity handling
> Pathfinding???
> Movement concurrency??? (real time pathfinding)
and more!

Pathfinding (Raycast):
- linear & parabolic trajectory raycasting will elicit nodes
- nodes are created from the block where the raycast collides
- consecutive raycasts accumulate a cost which is distributed per node
- a node's path will be replaced if the new path has a lower cost
```
