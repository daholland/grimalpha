extern crate sdl2;
extern crate gl;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

use std::thread;
use std::sync::mpsc::channel;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0...0.5 => self.volume,
                _ => -self.volume
                   
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

struct AudioSystem;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();

    let video_sys = sdl_ctx.video().unwrap();
    let audio_sys = sdl_ctx.audio().unwrap();

    let (audio_tx, audio_rx) = channel();

    thread::spawn(move || {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let audio_dev = audio_sys.open_playback(None, &desired_spec, |spec| {
            println!("{:?}", spec);

            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25,
            }
        }).unwrap();

        loop {
            match audio_rx.recv() {
                Ok(1u8) => {audio_dev.resume();},
                Ok(0u8) => {audio_dev.pause();},
                _ => {},
            }
        }
    });
    

    

    gl::load_with(|name| video_sys.gl_get_proc_address(name) as *const _);

    let window = video_sys.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();
    let mut event_pump = sdl_ctx.event_pump().unwrap();

    'running: loop {
        for ev in event_pump.poll_iter() {
            match ev {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::P), ..} => {
                    //audio_dev.resume();
                    audio_tx.send(1u8);
                },
                Event::KeyDown { keycode: Some(Keycode::X), ..} => {
                    //audio_dev.pause();
                    audio_tx.send(0u8);
                },
                _ => {println!("{:?}", ev)},
            }
        }
    }
}
