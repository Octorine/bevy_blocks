use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Block {
    pub name: char,
    pub sprite_number: usize,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Level {
    pub level_width: usize,
    pub level_height: usize,
    pub ball_velocity: f32,
    pub block_margin: usize,
    pub block_width: usize,
    pub block_height: usize,
    pub blocks: Vec<Block>,
    pub rows: Vec<String>,
}

pub fn add_bricks(commands: &mut Commands, level: &Level, atlas: Handle<TextureAtlas>) {
    let mut block_map: std::collections::HashMap<char, usize> = std::collections::HashMap::new();
    for block in &level.blocks {
        block_map.insert(block.name, block.sprite_number);
    }
    let left = -(level.level_width as i32 - level.block_width as i32) / 2;
    let top = (crate::SCREEN_HEIGHT as i32 - level.block_height as i32) / 2;
    let mut current_row = 0.0;
    for row in &level.rows {
        let mut current_col = 0.0;
        for c in row.chars() {
            if let Some(num) = block_map.get(&c) {
                if c != ' ' {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            texture_atlas: atlas.clone(),
                            sprite: TextureAtlasSprite {
                                index: *num as u32,
                                ..Default::default()
                            },
                            transform: Transform {
                                translation: Vec3::new(
                                    left as f32
                                        + current_col
                                            * (level.block_margin + level.block_width) as f32,
                                    top as f32
                                        - current_row
                                            * (level.block_margin + level.block_height) as f32,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(crate::gameplay::Collider::Scorable {
                            size: Vec2::new(level.block_width as f32, level.block_height as f32),
                        });
                }
            }
            current_col += 1.0;
        }
        current_row += 1.0;
    }
}
