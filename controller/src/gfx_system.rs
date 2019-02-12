use sdl2;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::thread;

use self::ButtonType::*;
use crate::gamestate::*;

const WINDOW_WIDTH: u32 = 218;
const WINDOW_HEIGHT: u32 = 130;

const SH_SIZE: i32 = 2; // button shadow size

const COLOR_BG: pixels::Color = pixels::Color {
    r: 0x1E,
    g: 0x21,
    b: 0x24,
    a: 0xFF,
}; // background color
const COLOR_BLACK: pixels::Color = pixels::Color {
    r: 0x00,
    g: 0x00,
    b: 0x00,
    a: 0xFF,
};
const COLOR_WHITE: pixels::Color = pixels::Color {
    r: 0xFF,
    g: 0xFF,
    b: 0xFF,
    a: 0xFF,
};
const COLOR_RED: pixels::Color = pixels::Color {
    r: 0xFF,
    g: 0x00,
    b: 0x00,
    a: 0x7f,
};
const COLOR_GREEN: pixels::Color = pixels::Color {
    r: 0x00,
    g: 0xFF,
    b: 0x00,
    a: 0xFF,
};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub enum Alignment {
    Left,
    Center,
    Right,
}

enum ButtonType {
    WarpToggle,
    SystemToggle,
    SaveBackup
}

struct Button<'a> {
    rect: Rect,
    color: pixels::Color,
    text_color: sdl2::pixels::Color,
    text: &'a str,
    pressed: bool,
    button_type: ButtonType,
    state: bool,
}

impl<'a> Button<'a> {
    pub fn x(&self) -> i32 {
        self.rect.x()
    }
    pub fn y(&self) -> i32 {
        self.rect.y()
    }
    pub fn x2(&self) -> i32 {
        self.rect.right()
    }
    pub fn y2(&self) -> i32 {
        self.rect.bottom()
    }
}

pub struct GfxSystem {
    warp_enable: Arc<Mutex<bool>>,
    system_enable: Arc<Mutex<bool>>,
}

impl GfxSystem {
    pub fn new(warp_enable: Arc<Mutex<bool>>, system_enable: Arc<Mutex<bool>>,) -> GfxSystem {
        GfxSystem {
            warp_enable,
            system_enable
        }
    }

    fn render_button(&self, font: &Font, canvas: &mut Canvas<Window>, button: &Button, pressed: bool) {

        let offset_rect = rect!(button.rect.x() + SH_SIZE, 
	                            button.rect.y() + SH_SIZE, 
	                            button.rect.width(), 
	                            button.rect.height());

        canvas.set_draw_color(COLOR_BG);
        canvas.fill_rect(button.rect).ok();

        if pressed {
            canvas.set_draw_color(button.color);
            canvas.fill_rect(offset_rect).ok();

            self.render_text(font,
                             canvas,
                             button.text,
                             button.text_color,
                             offset_rect,
                             Alignment::Center);
        } else {
            canvas.set_draw_color(COLOR_BLACK);
            canvas.fill_rect(offset_rect).ok(); // shadow

            canvas.set_draw_color(button.color);
            canvas.fill_rect(button.rect).ok();

            self.render_text(font,
                             canvas,
                             button.text,
                             button.text_color,
                             button.rect,
                             Alignment::Center);
        }
    }

    pub fn render_text(&self,
                       font: &Font,
                       canvas: &mut Canvas<Window>,
                       message: &str,
                       color: sdl2::pixels::Color,
                       rect: Rect,
                       alignment: Alignment)
                       -> u32 {

        let mut x = rect.x;
        let mut y = rect.y + 3;
        let w = rect.w;
        let h = rect.h;

        let surface = font.render(message).blended(color).unwrap();

        let size = surface.size();
        let mut s_width = size.0 as i32;
        let mut s_height = size.1 as i32;

        if w > 8 || h > 8 {
            if w - 8 < s_width {
                s_width = w - 8;
            }

            if h < s_height - 4 {
                s_height = h;
            }  
        }

        y = y + (h / 2) - (s_height / 2); // center vertically

        match alignment {
            Alignment::Left => x += 4,
            Alignment::Center => {
                x += (w / 2) - (s_width / 2);
            }
            Alignment::Right => {
                x += w - s_width;
            }
        }

        let texture_creator = canvas.texture_creator();
        let text = texture_creator.create_texture_from_surface(surface).unwrap();

        //println!("{}, {}, {}, {}, {}, {}", x, y, w, h, s_width, s_height);

        canvas.copy(&text, None, Some(rect!(x,y,s_width,s_height)))
            .unwrap();

        s_width as u32

    }

    pub fn run(&mut self) {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let ttf_context = sdl2::ttf::init().unwrap();

        let font = ttf_context.load_font(Path::new("Resources//PokemonDPPt.ttf"), 32).unwrap();

        let mut button_vec = vec![
                                Button {
                                    rect: rect!(158,10,50,30),
                                    color: pixels::Color::RGB(0x60, 0x60, 0x60),
                                    text_color: COLOR_GREEN,
                                    text: "ON",
                                    pressed: false,
                                    button_type: WarpToggle,
                                    state: true
                                },
                                Button {
                                    rect: rect!(158,50,50,30),
                                    color: pixels::Color::RGB(0x60, 0x60, 0x60),
                                    text_color: COLOR_RED,
                                    text: "OFF",
                                    pressed: false,
                                    button_type: SystemToggle,
                                    state: false
                                },
                                Button {
                                    rect: rect!(30,90,160,30),
                                    color: pixels::Color::RGB(0x60, 0x60, 0x60),
                                    text_color: COLOR_WHITE,
                                    text: "Backup State",
                                    pressed: false,
                                    button_type: SaveBackup,
                                    state: false
                                }];

        let window = video_subsystem
            .window("TppBurningRed",
                    WINDOW_WIDTH,
                    WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .target_texture()
            .build()
            .unwrap();

        let mut events = sdl_context.event_pump().unwrap();

        canvas.set_draw_color(COLOR_BG);
        canvas.clear();

        self.render_text(&font,
                        &mut canvas,
                         "Warps: ",
                         COLOR_WHITE,
                         rect!(75,10,100,30),
                         Alignment::Center);
        
        self.render_text(&font,
                        &mut canvas,
                         "Force Enable: ",
                         COLOR_WHITE,
                         rect!(8,50,160,30),
                         Alignment::Center);

        for i in &button_vec {
            self.render_button(&font, &mut canvas, i, false);
        }

        let mut old_time = Instant::now();

        let mut update_display = true;

        let frame_time = Duration::from_millis(1000) / 60;

        'main: loop {
            for e in events.poll_iter() {
                match e {
                    Event::Quit { .. } => break 'main,
                    Event::MouseButtonDown { x, y, .. } => {
                        for b in &mut button_vec {
                            if x > b.x() && x < b.x2() && y > b.y() && y < b.y2() {
                                match &b.button_type {
                                    WarpToggle => {
                                        if let Ok(mut enable) = self.warp_enable.lock() {
                                            if b.state {
                                                println!("Warps Disabled");
                                                *enable = false;
                                                b.state = false;
                                                b.text = "OFF";
                                                b.text_color = COLOR_RED;
                                            } else {
                                                println!("Warps Enabled");
                                                *enable = true;
                                                b.state = true;
                                                b.text = "ON";
                                                b.text_color = COLOR_GREEN;
                                            }
                                        }
                                    },
                                    SystemToggle => {
                                        if let Ok(mut enable) = self.system_enable.lock() {
                                            if b.state {
                                                println!("Enable Bypass Disabled");
                                                *enable = false;
                                                b.state = false;
                                                b.text = "OFF";
                                                b.text_color = COLOR_RED;
                                            } else {
                                                println!("Enable Bypass Enabled");
                                                *enable = true;
                                                b.state = true;
                                                b.text = "ON";
                                                b.text_color = COLOR_GREEN;
                                            }
                                        }
                                    },
                                    SaveBackup => {
                                        make_backup(true);
                                    }
                                }

                                update_display = true;
                                self.render_button(&font, &mut canvas, b, true);

                                b.pressed = true;
                            }
                        }
                    },

                    Event::MouseButtonUp { .. } => {
                        for button in &mut button_vec {
                            if button.pressed {
                                self.render_button(&font, &mut canvas, button, false);

                                button.pressed = false;
                                update_display = true;
                            }
                        }
                    }

                    _ => {}
                }
            }

            if update_display {
                canvas.present();
                update_display = false;
            }

            let new_time = Instant::now();

            if old_time + frame_time > new_time {
                thread::sleep(frame_time - (new_time - old_time));
            }
            old_time = new_time;
        }
    }
}
