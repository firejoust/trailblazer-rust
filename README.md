```
Trailblazer - Pathfinding engine for Minecraft (NodeJS Edition)

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
  - when a new path is found, it will be appended to a node on the current path

Cost:
  - yaw and impeded movement influence the "quality" of a path (0 to 1)
  - cost is calculated by scaling distance by the quality factor
  - the cost of the new and old path are compared from the same node

Raycasting:
  - each ray will be bridged together by a sequence of vectors
  - the physics engine is used to produce these vectors (given a yaw and pitch angle)
  - the order of what raycast will be applied first is defined by the developer

Nodes:
  - node elicitation will be influenced by:
  >   rotations: the amount of yaw rotations that can be considered
  >   threshold: the maximum quantity of paths that can exist simultaneously
  >   tolerance: maximum number of steps that the threshold is unrestricted for
  >   distance: how many steps ahead new paths can be made
```
