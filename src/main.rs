mod gameplay;
mod level;
mod sprite_sheet;
mod state;
use bevy::prelude::*;
pub struct Ball {
    pub velocity_x: f32,
    pub velocity_y: f32,
}
const SCREEN_WIDTH: f32 = 1280.;
const SCREEN_HEIGHT: f32 = 720.;

fn main() {
    App::build()
        .add_state(state::GameState::Level)
        .insert_resource(WindowDescriptor {
            title: "Break the Blocks!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system_set(gameplay::enter_system_set())
        .add_system_set(gameplay::update_system_set())
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
    commands.insert_resource(level_files.clone());
    let level = level_files[0].clone();
    let atlas = sprite_sheet::build_sprite_sheet(&mut asset_server, atlases);
}
