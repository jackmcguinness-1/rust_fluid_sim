use speedy2d::Window;
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper, WindowCreationOptions, WindowSize, WindowPosition};
use speedy2d::Graphics2D;
use speedy2d::dimen::Vector2;
use speedy2d::image::{ImageDataType, ImageSmoothingMode};

// fixed size window for the simulation
const WINDOW_WIDTH: usize = 1080;
const WINDOW_HEIGHT: usize = 720;

use crate::simulation as sim;

struct MyWindowHandler{
    needs_draw: bool,
    pixel_buffer: Box<[u8]>
}
impl WindowHandler for MyWindowHandler{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        if self.needs_draw {
            self.needs_draw = false;

            let image_data_type = ImageDataType::RGBA;
            let image_smoothing_mode = ImageSmoothingMode::Linear;
            static IMAGE_SIZE: Vector2<u32> = Vector2{
                x: WINDOW_WIDTH as u32,
                y: WINDOW_HEIGHT as u32
            };

            let image = Graphics2D::create_image_from_raw_pixels(
                graphics,
                image_data_type,
                image_smoothing_mode,
                IMAGE_SIZE,
                &self.pixel_buffer
            ).unwrap();

            graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));
            //graphics.draw_circle((100.0, 100.0), 75.0, Color::BLUE);
            let image_pos = Vector2{
                x: 0.0,
                y: 0.0
            };
            graphics.draw_image(image_pos, &image);
    
            // Request that we draw another frame once this one has finished
            helper.request_redraw();
        }
    }
}


pub fn start_window(){
    let physical_window_size: Vector2<u32> = Vector2{
        x: 1080,
        y: 720
    };
    let window_position: Option<WindowPosition> = Option::Some(WindowPosition::Center);
    let window_name = "Fluid Simulation in Rust";

    let window_size = WindowSize::PhysicalPixels(physical_window_size);

    let window_options = WindowCreationOptions::new_windowed(window_size, window_position)
        .with_resizable(false)
        .with_vsync(true);
        
    let window = Window::new_with_options(window_name, window_options).unwrap();

    let mut pixel_buffer: Box<[u8]>= (vec![0; WINDOW_HEIGHT*WINDOW_WIDTH*4]).into_boxed_slice();

    for i in 0..WINDOW_WIDTH {
        for j in 0..WINDOW_HEIGHT {
            let red: u8 = (255.0 * (i as f32)/(WINDOW_WIDTH as f32)).floor() as u8;
            let green: u8 = (255.0 * (j as f32)/(WINDOW_HEIGHT as f32)).floor() as u8;
            let blue: u8 = (i*j % 255) as u8;
            let alpha: u8 = 255;
            let start_idx: usize = (j * 4) * WINDOW_WIDTH + (i * 4);
            pixel_buffer[start_idx] = red;
            pixel_buffer[start_idx + 1] = green;
            pixel_buffer[start_idx + 2] = blue;
            pixel_buffer[start_idx + 3] = alpha;
        }
    }

    let window_handler = MyWindowHandler{ 
        needs_draw: true,
        pixel_buffer: pixel_buffer
    };

    window.run_loop(window_handler);
}