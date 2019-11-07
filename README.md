# Shapekit

Shapekit is a collision engine for games. It is designed to be used with lame, but that isn't necessary.
Shapekit contains two ways for interacting with the shapes.
world is designed with lame in mind to maximize distribution of processing.
iterator_based is more efficient but doesn't lend itself to multithreading.

Basics of Shapekit when using the world module:
All the shapes are in a world. Shapes and their collisions are accessed through a ShapeHandle.
When a ShapeHandle goes out of scope, the corresponding shape is also removed. (drop is cool like that).
To reduce lock overhead, it checks all of a shape's collisions at once as a vector.

Basics of Shapekit iter_friendly:
Just put the shapes in whatever.
If you can iterate through them, Shapekit can check collisions.
It uses IntoIterator so it looks nicer to use.
This prefers iterators to collections.
