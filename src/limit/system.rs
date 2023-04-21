use bevy::prelude::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::plugin::PixelSprite;

use super::plugin::SpriteCount;

pub fn sprite_count_limiter(
    sprite_count: Res<SpriteCount>,
    mut query: Query<&mut Visibility, With<PixelSprite>>,
) {
    let mut count = 0;
    if sprite_count.random {
        let mut collected_visibilities = query.iter_mut().collect::<Vec<_>>();
        collected_visibilities.shuffle(&mut thread_rng());
        for visibility in collected_visibilities.iter_mut() {
            count += 1;
            if count > sprite_count.count {
                **visibility = Visibility::Hidden;
            } else {
                **visibility = Visibility::Inherited;
            }
        }
    } else {
        for mut visibility in query.iter_mut() {
            count += 1;
            if count > sprite_count.count {
                *visibility = Visibility::Hidden;
            } else {
                *visibility = Visibility::Inherited;
            }
        }
    }
}
