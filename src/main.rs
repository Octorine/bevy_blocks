mod gameplay;
mod level;
mod pause_menu;
mod main_menu;
mod sprite_sheet;
mod state;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

const SCREEN_WIDTH: f32 = 1100.;
const SCREEN_HEIGHT: f32 = 720.;
const BACKGROUND_COLOR: Color = Color::rgb(0.58, 0.31, 0.15);

fn main() {
    App::new()
        .add_state(state::GameState::MainMenu)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(WindowDescriptor {
            title: "Break the Blocks!".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            present_mode: bevy::window::PresentMode::Mailbox,
            ..Default::default()
        })
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system_set(gameplay::enter_system_set())
        .add_system_set(gameplay::update_system_set())
        .add_system_set(gameplay::paused_update_system_set())
        .add_system_set(gameplay::exit_system_set())
        .add_system_set(pause_menu::enter_system_set())
        .add_system_set(pause_menu::update_system_set())
        .add_system_set(pause_menu::exit_system_set())
        .add_system_set(main_menu::enter_system_set())
        .add_system_set(main_menu::update_system_set())
        .add_system_set(main_menu::exit_system_set())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
    commands.spawn_bundle(UiCameraBundle::default());
    let level_files = load_all_levels();
    commands.insert_resource(level_files.clone());
    let _level = level_files[0].clone();
    let _atlas = sprite_sheet::build_sprite_sheet(&mut asset_server, atlases);
}
