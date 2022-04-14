use bevy::prelude::*;

pub struct Collision;

impl Plugin for Collision {
    fn build(&self, app: &mut App) {
        app.add_system(Self::render_debug);
    }
}

#[derive(Component)]
pub enum ColliderShape {
    Rectangular { half_width: f32, half_height: f32 },
}

impl Default for ColliderShape {
    fn default() -> Self {
        Self::Rectangular {
            half_width: 0.5,
            half_height: 0.5,
        }
    }
}

#[derive(Component)]
pub enum ColliderDebugRender {
    Show,
    Hide,
}

#[derive(Component, Default)]
pub struct ColliderDebugRenderColor(pub Color);

impl Default for ColliderDebugRender {
    fn default() -> Self {
        Self::Hide
    }
}

#[derive(Bundle, Default)]
pub struct ColliderBundle {
    pub shape: ColliderShape,
    pub debug_render: ColliderDebugRender,
    pub debug_color: ColliderDebugRenderColor,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Component)]
pub struct ColliderDebugRenderSprite;

impl Collision {
    fn render_debug(
        mut commands: Commands,
        colliders: Query<
            (
                Entity,
                &ColliderDebugRender,
                &ColliderShape,
                &ColliderDebugRenderColor,
                Option<&Children>,
            ),
            Changed<ColliderDebugRender>,
        >,
        sprites: Query<(), With<ColliderDebugRenderSprite>>,
    ) {
        for (entity, render, shape, ColliderDebugRenderColor(color), children) in colliders.iter() {
            match render {
                ColliderDebugRender::Show => {
                    commands.entity(entity).with_children(|parent| {
                        match *shape {
                            ColliderShape::Rectangular {
                                half_width,
                                half_height,
                            } => parent
                                .spawn_bundle(SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(half_width, half_height)),
                                        color: *color,
                                        ..Sprite::default()
                                    },
                                    ..SpriteBundle::default()
                                })
                                .insert(ColliderDebugRenderSprite),
                        };
                    });
                }
                ColliderDebugRender::Hide => {
                    if let Some(children) = children {
                        for child in children.iter() {
                            if sprites.get(*child).is_ok() {
                                commands.entity(*child).despawn_recursive();
                            }
                        }
                    }
                }
            }
        }
    }
}
