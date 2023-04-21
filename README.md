### Bevy Vinox Pixel
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_vinox_pixel)](https://crates.io/crates/bevy_vinox_pixel)
[![docs.rs](https://docs.rs/bevy_vinox_pixel/badge.svg)](https://docs.rs/bevy_vinox_pixel)

A crate for pixel art games in bevy.

:warning: **IF YOU WANT TO USE THE TEXTURED CAMERA**: Be warned this feature is considering being dropped in favor of expanding the scaled camera to support usecases of the texture version!
:warning: **MAKE SURE TO BE CAREFUL WHEN USING LIMITED SPRITES SPRITES WILL RANDOMLY FLASH IF RANDOM MODE IS TURNED ON OR AT ALL AS ADDING ENTITIES OR REMOVING MAY CAUSE THE NON RANDOM VERSION TO ALSO FLICKER WHEN CHANGED**: I'm not responsible for anything that may happen if you ignore this warning so be warned!

## Goals
The goal of this crate is to provide tools commonly needed in pixel art games in an easy to use crate.
Here is a list of features(Indicated by being crossed out) and planned features:
* ~Cameras~ _I need to cleanup the texture version a little bit though_
* ~Pixelated cursor support.~(Technically partially done since it only works for the scaled camera. However since the texture camera may be dropped we are crossing it off)
* Limited palettes that can be automatically assigned to any colors by finding the closest match or map from one palette to another.
* ~Layers A more abstracted layer system so you don't have to manually choose z depths.~ _may change if bevy introduces a better system_
* ~Optional limitations? Such as an option to limit sprite count to emulate more limited systems.~
* Optional abstracted positions. Ie a px position which will always correspond to the pixel grid. And another type subpxposition. (This idea is straight from seldom_pixel)

## No longer planned or on a backburner
* Runtime Pixelated Sprites(such as 3d objects or procedural generated assets) // BACKBURNER
* Pixel ui system for games that need it. // BACKBURNER
* Tilemaps via bevy_ecs_tilemap? // NO LONGER PLANNED
* A simple aabb(and possibly sat) physics engine as a lot of games don't need complicated physics. // NO LONGER PLANNED

Collisions most likely won't come for anything more than aabbs but even those may not come. After some thinking in 2d rapier should be good enough in most cases.
In the cases where it isn't then at that point your physics is probably very opionated anywyas and you'll probably do your own system.


Please open issues for more feature suggestions if you have any!

## Thanks
Thanks to [bevy_pixel_camera](https://github.com/drakmaniso/bevy_pixel_camera) for both the approach shown there and for how to setup a custom camera.
also thanks to [seldom_pixel](https://github.com/Seldom-SE/seldom_pixel) for some of the ideas 

## Version Support
| bevy | bevy_vinox_pixel |
|------|------------------|
| 0.10 | 0.0.1            |


