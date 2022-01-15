#![no_main]
#[cfg(feature = "buddy-alloc")]
mod alloc;

use wasm4::{
    draw::Sprite,
    main,
    rt::{Resources, Runtime},
    sound::{Channel, Duration, Flags, Frequency, LinearFrequency, Mode},
};

struct SlinkyRuntime {
    count: i32,
    resources: Resources,
    player: (usize, usize),
}
wasm4::include_sprites! {
    //blk-aqu4 on lospec
    const PALETTE: _ = common_palette!(
        0x002b59,
        0x005f8c,
        0x00b9be,
        0x9ff4e5
    );
    const FRAME_1: _ = include_sprite!("./art/Wave-good1.png");
    const FRAME_2: _ = include_sprite!("./art/Wave-good2.png");
    const FRAME_3: _ = include_sprite!("./art/Wave-good3.png");
    const FRAME_4: _ = include_sprite!("./art/Wave-good4.png");
    const FRAME_5: _ = include_sprite!("./art/Wave-good5.png");
    const FRAME_6: _ = include_sprite!("./art/Wave-good6.png");
    const FRAME_7: _ = include_sprite!("./art/Wave-good7.png");
    const FRAME_8: _ = include_sprite!("./art/Wave-good8.png");
}

const FRAMES: [Sprite<[u8; 16]>; 8] = [
    FRAME_1, FRAME_2, FRAME_3, FRAME_4, FRAME_5, FRAME_6, FRAME_7, FRAME_8,
];

fn midi_to_frequency(note: u8) -> u16 {
    (440. * 2f32.powf((note as f32 - 69.) / 12.)).round() as u16
}

impl SlinkyRuntime {
    fn play_big_note(&self, volume: u32, midi_note: u8, length: Duration) {
        let frequency = midi_to_frequency(midi_note);
        self.resources.sound.tone(
            LinearFrequency::constant((frequency / 2).into()),
            length,
            volume,
            Flags::new(Channel::Noise, Mode::N1D8),
        );
        self.resources.sound.tone(
            LinearFrequency::constant((frequency).into()),
            length,
            volume,
            Flags::new(Channel::Pulse1, Mode::N1D2),
        );
        self.resources.sound.tone(
            LinearFrequency::constant((frequency * 2).into()),
            length,
            volume,
            Flags::new(Channel::Pulse2, Mode::N1D4),
        );
        self.resources.sound.tone(
            LinearFrequency::constant((frequency * 4).into()),
            length,
            volume,
            Flags::new(Channel::Triangle, Mode::N1D8),
        );
    }
}

impl Runtime for SlinkyRuntime {
    fn start(resources: Resources) -> Self {
        SlinkyRuntime {
            count: 0,
            resources,
            player: (0, 0),
        }
    }

    fn update(&mut self) {
        self.resources.framebuffer.replace_palette(PALETTE);
        for i in 0..20 {
            self.resources.framebuffer.blit(
                &FRAMES[(self.count % 8) as usize],
                [i * 8, 152],
                <_>::default(),
            )
        }
        if self.count % 60 == 0 {
            self.play_big_note(60, 70, Duration(10))
        }
        self.count += 1;
        self.resources.framebuffer.hline([159,159 ], 160)
    }
}

main! { SlinkyRuntime }
