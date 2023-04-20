### Bevy Vinox Pixel
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_vinox_pixel)](https://crates.io/crates/bevy_vinox_pixel)
[![docs.rs](https://docs.rs/bevy_vinox_pixel/badge.svg)](https://docs.rs/bevy_vinox_pixel)

A crate for pixel art games in bevy.

The goal of this crate is to provide tools commonly needed in pixel art games in an easy to use crate.
Here is a list of features(Indicated by being crossed out) and planned features:
* ~Cameras~ _I need to cleanup the texture version a little bit though_
* Runtime Pixelated Sprites(such as 3d objects or procedural generated assets)
* Pixel ui system for games that need it.
* Tilemaps via bevy_ecs_tilemap?
* Pixelated cursor support.
* Limited palettes that can be automatically assigned to any colors by finding the closest match or map from one palette to another.
* A simple aabb(and possibly sat) physics engine as a lot of games don't need complicated physics.
* A more abstracted layer system so you don't have to manually choose z depths.
* Optional limitations? Such as an option to limit sprite count to emulate more limited systems.
* Optional abstracted positions. Ie a px position which will always correspond to the pixel grid. And another type subpxposition. (This idea is straight from seldom_pixel)

Please open issues for more feature suggestions if you have any!

Thanks to [bevy_pixel_camera](https://github.com/drakmaniso/bevy_pixel_camera) for both the approach shown there and for how to setup a custom camera.
also thanks to [seldom_pixel](https://github.com/Seldom-SE/seldom_pixel) for some of the ideas 
