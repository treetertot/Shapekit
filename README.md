# Shapekit

Note: 0.3.0+ don't work, just stick to 0.2.0 for now;

Shapekit is a collision engine for games. It is designed to be used with lame, but that isn't necessary.

Basics of Shapekit:
All the shapes are in a world. Shapes and their collisions are accessed through a ShapeHandle.
When a ShapeHandle goes out of scope, the corresponding shape is also removed. (drop is cool like that).
To generate the collisions, call run on the worldhandle.
Use Vector types to move shapes and f32s to rotate them.
Have fun!

It also uses serde to load/write worlds to files.