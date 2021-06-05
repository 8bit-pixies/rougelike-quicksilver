# rougelike-quicksilver

Changed my mind and trying to learn this via `bevy` with the promise of `wasm` [support](https://github.com/bevyengine/bevy/tree/latest/examples#wasm) - not sure if it will "work", but we'll figure it out...

```
cargo run --features bevy/dynamic
```

# Getting started

**Hello World**

Getting a window open is as simple as:

```rust
use bevy::prelude::*;


fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Rougelike!".to_string(),
            width: 800.,
            height: 600.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}
```

**Hello World with Text**

Create folders

```sh
mkdir -p assets/fonts
```

And add the fonts you want to use. In order to add text, we need to add the `UiCameraComponent` - there are ways to add it to a `2d` scene; we might consider that later.

```rust
use bevy::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // Text with one section
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Bevy Rougelike",
                TextStyle {
                    font: asset_server.load("fonts/mononoki-Regular.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                Default::default(),
            ),
            ..Default::default()
        });
    commands
        .spawn_bundle(TextBundle {
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
                    font_size:30.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..Default::default()
        });
}
```



**Adding 2d scene**

We can also add some text as a 2d scene - we will replace this with a grid later.

```rust
use bevy::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    // 2d camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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
                font_size: 100.0,
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
                font_size: 60.0,
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
```

**Creating a Simple Sprite**

The next step is rather than have text on the screen, we'll set up some grid and movement to start the game. We'll start by setting up with boxes (coloured in), and then transforming it to sprites/font for ease of programming (we want to focus on building out the game, not playing around with sprites and colours). 

The addition would be like this:

```rust
fn spawn_character(mut commands: Commands, mut materials: Res<Materials>) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.warrior_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
    })
    .insert(Character);
}
```

We'll create a grid; in order to enforce something to be square we use: `let target_size = window.width().min(window.height());`.

**Moving Things Around**

