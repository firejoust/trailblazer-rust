```
Trailblazer - Pathfinding module for Minecraft (NPM/Neon)

Roadmap:
  - create a JSON to Rust compatibility layer for runtime
  - parse chunk data directly from prismarine-chunk
  - detect hitbox collision and line intersection with blocks
  - simulate player physics and block collision
  - implement non-specific pathfinding algorithm below

Pathfinding:
  - linear & parabolic raycasting will elicit nodes
  - the point of vertical collision determines the next node
  - a path formed by consecutive rays will accumulate a cost
  - when conditions are met, a destination node and path are chosen

Nodes:
  - node elicitation will be influenced by the following parameters:
  >   rotations: the amount of yaw angles to be considered
  >   threshold: the maximum amount of paths that are remembered
  >   tolerance: the first x steps that the threshold will be ignored

Movement:
  - the start position of a path will be determined by the "header" node
  - if the header node is too many steps away from the current node, it will destroy its path
  - new paths will be initialised a few steps ahead of the current node
```
