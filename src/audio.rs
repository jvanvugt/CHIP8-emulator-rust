pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl sdl2::audio::AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub fn setup_audio(sdl_context: &sdl2::Sdl) -> sdl2::audio::AudioDevice<SquareWave> {
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = sdl2::audio::AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),
        samples: None,
    };

    audio_subsystem
    .open_playback(None, &desired_spec, |spec| SquareWave {
        phase_inc: 220.0 / spec.freq as f32,
        phase: 0.0,
        volume: 0.25,
    })
    .unwrap()
}