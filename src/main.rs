use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::{Transform, TransformBundle},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Camera, ImageFormat, RenderingBundle, SpriteRender, SpriteSheet, SpriteSheetFormat,
        Texture,
    },
    ui::{Anchor, RenderUi, Stretch, TtfFormat, UiBundle, UiText, UiTransform},
    utils::application_root_dir,
};

mod systems;

#[derive(Default)]
struct HelloKittenState {}

impl SimpleState for HelloKittenState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf", TtfFormat, (), &world.read_resource());

        let label_transform = UiTransform::new(
            "label0".to_string(), Anchor::Middle, Anchor::Middle,
            0., 0., 0., 0., 0.)
            .with_stretch(Stretch::XY {x_margin: 10., y_margin: 10., keep_aspect_ratio: false});

        let test_label = world.create_entity()
            .with(label_transform)
            .with(UiText::new(font, "hello kitten".to_string(), [1., 1., 1., 1.], 50.))
            .build();

        world.insert(test_label);
    }
}

#[derive(Default)]
struct KittenSpriteState {}

impl SimpleState for KittenSpriteState{
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load("texture/kitten.png", ImageFormat::default(), (), &texture_storage)
        };

        let sprite_sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
            loader.load(
                "texture/kitten.ron", SpriteSheetFormat(texture_handle), (), &sprite_sheet_store)
        };

        let mut local_transform = Transform::default();
        let (cam_width, cam_height) = systems::get_camera_pixels();
        local_transform.set_translation_xyz(cam_width / 2., cam_height / 2., 0.);

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        };

        world.create_entity()
            .with(sprite_render)
            .with(local_transform)
            .build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let input_bindings_path = config_dir.join("input_bindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new()
            .with_bindings_from_file(input_bindings_path)?
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_system_desc(systems::CameraSystemDesc, "camera_system", &[]);

    let mut game = Application::new(assets_dir, KittenSpriteState::default(), game_data)?;
    game.run();

    Ok(())
}
