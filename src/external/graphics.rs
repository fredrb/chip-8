use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE_FACTOR: u32 = 20;
const SCREEN_WIDTH: u32 = (64 as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (32 as u32) * SCALE_FACTOR;

pub struct Screen {
    canvas: Canvas<Window>,
}

impl Screen {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "Chip 8 Emulator",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Screen { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[bool; 32]; 64]) {
        for (x, col) in pixels.iter().enumerate() {
            for (y, &p) in col.iter().enumerate() {
                let x = (x as u32) * SCALE_FACTOR;
                let y = (y as u32) * SCALE_FACTOR;
                self.canvas.set_draw_color(color(p));
                let _ = self.canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
            }
        }
        self.canvas.present();
    }
}

fn color(value: bool) -> pixels::Color {
    match value {
        true => pixels::Color::RGB(255, 255, 255),
        false => pixels::Color::RGB(0, 0, 0)
    }
}