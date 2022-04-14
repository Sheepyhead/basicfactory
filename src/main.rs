#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use collision::{ColliderBundle, ColliderDebugRender, ColliderDebugRenderColor, ColliderShape, Collision};

mod collision;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(Collision)
        .add_startup_system(setup_cameras)
        .add_startup_system(Player::setup)
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

pub struct Player;

impl Player {
    fn setup(mut commands: Commands) {
        commands.spawn_bundle(ColliderBundle {
            shape: ColliderShape::Rectangular {
                half_width: 25.0,
                half_height: 25.0,
            },
            debug_render: ColliderDebugRender::Show,
            debug_color: ColliderDebugRenderColor(Color::RED),
            ..ColliderBundle::default()
        });
    }
}
