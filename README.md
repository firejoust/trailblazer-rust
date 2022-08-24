```
Trailblazer - Pathfinding module for Minecraft (NPM/Neon)

Roadmap:
  - create a JSON to Rust compatibility layer during runtime
  - parse chunk data directly from prismarine-chunk
  - detect hitbox collision and line intersection with blocks
  - simulate player physics and block collision
  - implement non-specific pathfinding algorithm

Pathfinding:
  - linear & parabolic raycasting will elicit nodes
  - the point of vertical collision will determine the next node
  - the path formed by consecutive rays will accumulate a cost
  - when conditions are met, a path is chosen to follow

Movement:
  - the start position of a path will be determined by the "header" node
  - if the header node is unreachable from the current node, its path will be destroyed
  - new paths will be initialised a few steps ahead of the current node

Cost:
  - The cost of the new path can only be compared to the old path if they share the same node
  - At one time, only the cost of the current path and new path will be calculated
  - Cost is influenced by both distance and movement factors
  
Nodes:
  - node elicitation will be influenced by:
  >   rotations: the amount of yaw rotations that are considered
  >   threshold: the quantity of paths that can exist at one time
  >   tolerance: maximum number of steps that unrestrict the threshold
```
