```
Trailblazer - Pathfinding module for Minecraft (NPM/Neon)

Roadmap:
- create a runtime JSON to Rust compatibility layer
- parse chunk data directly from prismarine-chunk
- detect hitbox collision and block line intersection
- simulate player physics and block collision
- implement non-specific pathfinding algorithm

Pathfinding:
- linear & parabolic raycasting will elicit nodes
- the point of vertical collision determines the next node
- a path formed by consecutive rays will accumulate a cost
- when conditions are met, a destination node and path are chosen

Movement:
- the start position of a path will be determined by the "header" node
- if the header node is too many steps away from the current node, it will destroy its path
- new paths will be initialised a few steps ahead of the current node
```
