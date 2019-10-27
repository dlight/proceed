use coffee::graphics::{Frame, Image, Window};
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Result, Timer};

mod map;

use map::DisplayedMap;

pub fn main() -> Result<()> {
    use coffee::graphics::WindowSettings;

    <MapScreen as Game>::run(WindowSettings {
        title: String::from("ImageScreen - Coffee"),
        size: (660, 400),
        resizable: true,
        fullscreen: false,
        maximized: true,
    })
}

struct InGpu {
    dirty: bool,
    cpu_buffer: DisplayedMap,
    gpu_handler: Image,
}

struct MapScreen {
    quit: bool,
    ticks: u32,
    map: InGpu,
}

impl Game for MapScreen {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Self> {
        let cpu_buffer = DisplayedMap::new(36, 20);

        println!("{}", cpu_buffer.title);

        Task::using_gpu(|gpu| {
            let gpu_handler = Image::from_image(gpu, &cpu_buffer.image).unwrap();

            Ok(Self {
                ticks: 0,
                quit: false,
                map: InGpu {
                    dirty: true,
                    cpu_buffer,
                    gpu_handler,
                },
            })
        })
    }

    fn is_finished(&self) -> bool {
        self.quit
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, _window: &mut Window) {
        use coffee::input::keyboard::KeyCode;

        let keyboard = input.keyboard();

        if keyboard.was_key_released(KeyCode::Q) {
            self.quit = true;
        }
    }

    fn update(&mut self, _window: &Window) {
        self.ticks += 1;

        self.map.dirty = self.map.cpu_buffer.update(self.ticks);
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        use coffee::graphics::{Color, Point, Quad, Rectangle};

        frame.clear(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        });

        if self.map.dirty {
            self.map.gpu_handler =
                Image::from_image(frame.gpu(), &self.map.cpu_buffer.image).unwrap();
            self.map.dirty = false;
        }

        self.map.gpu_handler.draw(
            Quad {
                source: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                },
                position: Point::new(0.0, 0.0),
                size: (frame.width(), frame.height()),
            },
            &mut frame.as_target(),
        );
    }
}
