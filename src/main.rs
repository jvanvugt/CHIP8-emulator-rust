use std::env;
use std::time::{Duration, Instant};
use sdl2;

mod audio;
mod chip8;

fn draw_screen(state: &chip8::CHIP8, mut screen_surface: &mut sdl2::surface::SurfaceRef) {
    let mut tex_surface = sdl2::surface::Surface::new(
        chip8::SCREEN_WIDTH as u32,
        chip8::SCREEN_HEIGHT as u32,
        sdl2::pixels::PixelFormatEnum::RGB24,
    )
    .unwrap();
    let pixels = tex_surface.without_lock_mut().unwrap();
    for y in 0..chip8::SCREEN_HEIGHT {
        for x in 0..chip8::SCREEN_WIDTH {
            let pixel_loc = 3 * (chip8::SCREEN_WIDTH * y + x);
            pixels[pixel_loc + 0] = state.screen[y][x] as u8 * 255;
            pixels[pixel_loc + 1] = state.screen[y][x] as u8 * 255;
            pixels[pixel_loc + 2] = state.screen[y][x] as u8 * 255;
        }
    }
    let dest_rect = screen_surface.rect();
    tex_surface
        .blit_scaled(tex_surface.rect(), &mut screen_surface, dest_rect)
        .unwrap();
}

fn keycode_to_idx(keycode: sdl2::keyboard::Keycode) -> Option<usize> {
    use sdl2::keyboard::Keycode;
    let key_num = keycode as i32;
    if key_num >= Keycode::Num0 as i32 && key_num <= Keycode::Num9 as i32 {
        Some((key_num - Keycode::Num0 as i32) as usize)
    } else if key_num >= Keycode::A as i32 && key_num <= Keycode::F as i32 {
        Some((key_num - Keycode::A as i32 + 10) as usize)
    } else {
        None
    }
}

fn process_events(state: &mut chip8::CHIP8, event_pump: &mut sdl2::EventPump) -> bool {
    use sdl2::event::Event;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => { return true; }
            Event::KeyDown { keycode: Some(keycode), .. } => {
                match keycode_to_idx(keycode) {
                    Some(idx) => { state.keys_down[idx] = true; },
                    None => {},
                }
            },
            Event::KeyUp { keycode: Some(keycode), .. } => {
                match keycode_to_idx(keycode) {
                    Some(idx) => { state.keys_down[idx] = false; },
                    None => {},
                }
            },
            _ => {}
        }
    }
    return false;
}

fn execute(state: &mut chip8::CHIP8) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let screen_scale = 10;
    let window = video_subsystem
        .window(
            "CHIP-8 Emulator",
            (chip8::SCREEN_WIDTH * screen_scale) as u32,
            (chip8::SCREEN_HEIGHT * screen_scale) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut last_update = Instant::now();

    let audio = audio::setup_audio(&sdl_context);

    let ns_per_s = 1_000_000_000;
    let fps = 60;
    let time_between_frames = Duration::new(0, ns_per_s / fps);
    'main_loop: loop {
        let cur_time = Instant::now();
        while cur_time - last_update > time_between_frames {
            last_update += time_between_frames;

            let should_quit = process_events(state, &mut event_pump);
            if should_quit {
                break 'main_loop;
            }

            let high_byte = state.memory[state.pc as usize] as u16;
            let low_byte = state.memory[(state.pc + 1) as usize] as u16;
            state.execute_op((high_byte << 8) | low_byte);

            state.sound_timer = state.sound_timer.saturating_sub(1);
            state.delay_timer = state.delay_timer.saturating_sub(1);

            let mut surface = window.surface(&mut event_pump).unwrap();
            draw_screen(state, &mut surface);
            surface.finish().unwrap();

            if state.sound_timer > 0 {
                audio.resume();
            } else {
                audio.pause();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please provide a rom as argument");
        return;
    }

    let rom_file = &args[1];
    let rom_contents = std::fs::read(rom_file).unwrap();
    let mut chip8 = chip8::CHIP8::new(&rom_contents);
    execute(&mut chip8);
}
