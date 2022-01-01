use bevy::prelude::*;
use bevy::sprite::Rect;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
struct SpriteSpecs {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}
#[derive(Deserialize, Debug, Clone)]
struct SpriteSheetSpecs {
    texture_width: usize,
    texture_height: usize,
    sprites: Vec<SpriteSpecs>,
}

pub fn build_sprite_sheet(
    asset_server: &mut Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let specs: SpriteSheetSpecs =
        ron::de::from_reader(std::fs::File::open("assets/texture/blocks-spritesheet.ron").unwrap())
            .unwrap();

    let atlas_image = asset_server.load("texture/blocks-spritesheet.png");
    let mut texture_atlas = TextureAtlas::new_empty(
        atlas_image,
        Vec2::new(specs.texture_width as f32, specs.texture_height as f32),
    );
    for sprite in specs.sprites.iter() {
        texture_atlas.add_texture(Rect {
            min: Vec2::new(sprite.x as f32, sprite.y as f32),
            max: Vec2::new(
                (sprite.x + sprite.width) as f32,
                (sprite.y + sprite.height) as f32,
            ),
        });
    }

    atlases.add(texture_atlas)
}
