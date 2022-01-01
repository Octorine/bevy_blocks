mod gameplay;
mod level;
mod sprite_sheet;
use bevy::prelude::*;

pub struct Ball {
    pub velocity_x: f32,
    pub velocity_y: f32,
}
const SCREEN_WIDTH: f32 = 1280.;
const SCREEN_HEIGHT: f32 = 720.;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Break the Blocks!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_system(gameplay::ball_movement_system.system())
        .add_system(gameplay::paddle_movement_system.system())
        .add_system(gameplay::ball_collision_system.system())
        .add_system(gameplay::ball_boundary_system.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}
pub fn load_all_levels() -> std::vec::Vec<level::Level> {
    let levels_dir = std::fs::read_dir("assets/levels").unwrap();
    levels_dir
        .filter_map(Result::ok)
        .map(|de| ron::de::from_reader(std::fs::File::open(de.path()).unwrap()))
        .filter_map(Result::ok)
        .collect()
}
fn setup(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let level_files = load_all_levels();
    let level = level_files[0].clone();
    let atlas = sprite_sheet::build_sprite_sheet(&mut asset_server, atlases);
    level::add_bricks(&mut commands, &level, atlas.clone());
    gameplay::setup_ball_and_paddle(&mut commands, atlas);
}
