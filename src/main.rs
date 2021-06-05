use bevy::prelude::*;
use std::cmp::min;

const ARENA_HEIGHT: u32 = 10;
const ARENA_WIDTH: u32 = 10;
const GLOBAL_PADDING: u32 = 100;
const SQUARE_SCALE: f32 = 0.6;

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

struct Character;
struct Materials {
    warrior_material: Handle<ColorMaterial>,
}

/// This example illustrates how to create UI text and update it in a system. It displays the
/// current FPS in the top left corner, as well as text that changes colour in the bottom right.
/// For text within a scene, please see the text2d example.
fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Bevy Rougelike!".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_character.system()))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation.system())
                .with_system(size_scaling.system()),
        )
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_character(mut commands: Commands, mut materials: Res<Materials>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.warrior_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Character)
        .insert(Position { x: 0, y: 0 })
        .insert(Size::square(SQUARE_SCALE));

    //
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.warrior_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Character)
        .insert(Position { x: 9, y: 9 })
        .insert(Size::square(SQUARE_SCALE));

    //
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.warrior_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Character)
        .insert(Position { x: 0, y: 2 })
        .insert(Size::square(SQUARE_SCALE));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.warrior_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .insert(Character)
        .insert(Position { x: 1, y: 0 })
        .insert(Size::square(SQUARE_SCALE));
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    let target_size = window.width().min(window.height());

    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * target_size as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * target_size as f32,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.get_primary().unwrap();
    let target_size = window.width().min(window.height()) - GLOBAL_PADDING as f32;
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, target_size as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, target_size as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // materials
    commands.insert_resource(Materials {
        warrior_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
    // Text with one section
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Bevy Rougelike",
            TextStyle {
                font: asset_server.load("fonts/mononoki-Regular.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            Default::default(),
        ),
        ..Default::default()
    });
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Mononoki font by Matthias Tellen, terms: SIL Open Font License 1.1",
            TextStyle {
                font: asset_server.load("fonts/mononoki-Regular.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    });

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            "This text is in the 2D scene.",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
            TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
            },
        ),
        ..Default::default()
    });
}
