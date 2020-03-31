# Nuuro

Nuuro is specialized game development library.

When creating a game, it is good practice to make a layer,
specific to one's needs, that separates the
game logic from the resource management, rendering, audio, and other interfacing
that is needed for a game.

Users of this crate should create a build script in their project,
invoking functionality from the sibling crate "nuuro_build".
This will generate texture atlases and enums to reference assets.
See the "nuuro_build" crate for more details.

You can start with the [nuuro template](https://github.com/juandroid007/nuuro_template).
