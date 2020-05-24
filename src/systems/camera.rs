use amethyst::{
    core::Transform,
    ecs::{
        prelude::{Entity},
        Read, System, WriteStorage,
    },
    prelude::*,
    renderer::{Camera},
    shrev::{EventChannel, ReaderId},
    window::ScreenDimensions,
    winit::{Event, WindowEvent},
};

const CONTENT_WIDTH: f32 = 600.;
const CONTENT_HEIGHT: f32 = 400.;

pub struct CameraSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, CameraSystem> for CameraSystemDesc {
    fn build(self, world: &mut World) -> CameraSystem {
        let reader = world.fetch_mut::<EventChannel<Event>>().register_reader();
        CameraSystem::new(CONTENT_WIDTH, CONTENT_HEIGHT, reader)
    }
}

pub struct CameraSystem {
    min_width: f32,
    min_height: f32,
    camera: Option<Entity>,
    reader: ReaderId<Event>,
}

impl CameraSystem {
    pub fn new(min_width: f32, min_height: f32, reader: ReaderId<Event>) -> Self {
        CameraSystem{min_width, min_height, camera: None, reader}
    }

    // Given content and screen dimensions, return camera dimensions such that content is
    // maximized while maintaining aspect ratio.
    fn calc_camera_xy(&self, screen_x: f32, screen_y: f32) -> (f32, f32) {
        let screen_aspect = screen_x / screen_y;
        let content_aspect = self.min_width / self.min_height;
        match content_aspect > screen_aspect {
            true => /* Letterboxing. */ (self.min_width, self.min_width * screen_y / screen_x),
            false => /* Pillarboxing. */ (self.min_height * screen_x / screen_y, self.min_height),
        }
    }

    fn update_camera(&mut self, camera_storage: &mut WriteStorage<Camera>, x: f32, y: f32) {
        match self.camera {
            Some(c) => {
                let nc = Camera::standard_2d(x, y);
                camera_storage.get_mut(c).unwrap().set_projection(nc.projection().clone());
            },
            None => { println!("_K_ SELF.CAMERA EMPTY"); }
        }
    }
}

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        Read<'s, EventChannel<Event>>,
        WriteStorage<'s, Camera>,
    );

    fn run(&mut self, (input, mut cameras): Self::SystemData) {
        for event in input.read(&mut self.reader) {
            match *event {
                Event::WindowEvent { ref event, .. } => match *event {
                    WindowEvent::Resized(logical_size) => {
                        let (camera_x, camera_y) = self.calc_camera_xy(
                            logical_size.width as f32, logical_size.height as f32);
                        self.update_camera(&mut cameras, camera_x, camera_y);
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        let (camera_x, camera_y) = {
            let screen = world.fetch::<ScreenDimensions>();
            self.calc_camera_xy(screen.width(), screen.height())
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(self.min_width / 2.0, self.min_height / 2.0, 1.0);

        self.camera = Some(world.create_entity()
            .with(Camera::standard_2d(camera_x, camera_y))
            .with(transform)
            .build());
    }
}

pub fn get_camera_pixels() -> (f32, f32) {
    (CONTENT_WIDTH, CONTENT_HEIGHT)
}
