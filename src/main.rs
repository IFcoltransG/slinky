#![no_main]
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod music;

use std::process::abort;

use music::Music;
use wasm4::{
    draw::{BlitTransform, Framebuffer, Sprite, SpriteView},
    main,
    rt::{Resources, Runtime},
    sound::{Channel, Duration, Flags, Frames, LinearFrequency, Mode},
    sys::{
        BUTTON_1, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, DRAW_COLORS,
        GAMEPAD1,
    },
    trace,
};

struct SlinkyRuntime {
    frames: usize,
    resources: Resources,
    player: ([i32; 2], u8), /* pos of rightmost, length
                             * coins: Vec<[i32; 2], 10>, */
}
wasm4::include_sprites! {
    //blk-aqu4 on lospec
    const PALETTE: _ = common_palette!(
        0x9ff4e5,
        0x005f8c,
        0x00b9be,
        0x002b59,
    );
    const WAVE_ATLAS: _ = include_sprite!("./art/Wave-good.png");
    const BLOB: _ = include_sprite!("./art/Blob.png");
    const WAVE_BLOB: _ = include_sprite!("./art/Major-blob.png");
    const SLINKY_ATLAS: _ = include_sprite!("./art/Slinky.png");
}

const SLINKY_FRAMES: usize = 11;

fn parse_gamepad(gamepad: u8) -> (bool, bool, [i32; 2]) {
    let (x_dir, y_speed) = match (gamepad & BUTTON_RIGHT != 0, gamepad & BUTTON_LEFT != 0) {
        (true, false) => (1, 1),
        (false, true) => (-1, 1),
        (true, true) => (0, 1),
        (false, false) => (0, 2),
    };
    let (y_dir, x_speed) = match (gamepad & BUTTON_DOWN != 0, gamepad & BUTTON_UP != 0) {
        (true, false) => (1, 1),
        (false, true) => (-1, 1),
        (true, true) => (0, 1),
        (false, false) => (0, 2),
    };
    (
        gamepad & BUTTON_1 != 0,
        gamepad & BUTTON_2 != 0,
        [x_speed * x_dir, y_speed * y_dir],
    )
}

fn tile_horizontal(sprite_atlas: &Sprite, tile_size: u32, index: u32) -> Option<SpriteView> {
    sprite_atlas.view([index * tile_size, 0], [tile_size, tile_size])
}

impl Runtime for SlinkyRuntime {
    fn start(resources: Resources) -> Self {
        SlinkyRuntime {
            frames: 0,
            resources,
            player: ([160, 200], 2),
            // coins: Vec::new(),
        }
    }

    fn update(&mut self) {
        self.resources.framebuffer.replace_palette(PALETTE);
        // background
        // unsafe { *DRAW_COLORS = 0x2222 }
        // self.resources.framebuffer.rect([0, 0], [160, 160]);
        // waves
        // for i in 0..20 {
        //     self.resources.framebuffer.blit(
        //         &(match tile_horizontal(&WAVE_ATLAS, 8, (self.frames / 2 % 8) as _) {
        //             Some(view) => view,
        //             _ => {
        //                 trace("sprite out of bounds");
        //                 continue;
        //             }
        //         }),
        //         [i * 8, 152],
        //         <_>::default(),
        //     )
        // }
        let (button1, button2, mut direction) = if self.frames % 5 == 0 {
            parse_gamepad(unsafe { *GAMEPAD1 })
        } else {
            (false, false, [0, 0])
        };
        let ([player_x, player_y], player_length) = self.player;
        let [player_x_square, player_y_square] = [
            player_x as usize / SLINKY_FRAMES / 2,
            player_y as usize / SLINKY_FRAMES / 2,
        ];
        let player_x_down = (player_x as usize / SLINKY_FRAMES) % 2 == 1;
        let carry =
            ((player_x - 10).rem_euclid(22) != 0).then(|| if player_x_down { -1 } else { 1 });
        if direction[0] == 0 && self.frames % 5 == 0 {
            if let Some(offset) = carry {
                direction[0] = offset
            }
        }
        // draw head
        if player_x_down {
            // extra head piece
            self.resources.framebuffer.blit(
                &(match tile_horizontal(&SLINKY_ATLAS, 16, (SLINKY_FRAMES - 1) as _) {
                    Some(view) => view,
                    _ => {
                        trace("Slinky out of bounds");
                        abort()
                    }
                }),
                [
                    ((player_x_square) * 16) as i32 % 160,
                    ((player_y_square) * 16) as i32 % 160,
                ],
                BlitTransform::ROTATE | BlitTransform::FLIP_Y | BlitTransform::FLIP_X,
            );
        }
        self.resources.framebuffer.blit(
            &(match tile_horizontal(&SLINKY_ATLAS, 16, (player_x as usize % SLINKY_FRAMES) as _) {
                Some(view) => view,
                _ => {
                    trace("Slinky out of bounds");
                    abort()
                }
            }),
            [
                ((player_x_square) * 16) as i32 % 160,
                ((player_y_square) * 16) as i32 % 160,
            ],
            if player_x_down {
                <_>::default()
            } else {
                BlitTransform::ROTATE | BlitTransform::FLIP_Y | BlitTransform::FLIP_X
            },
        );
        // draw body
        for i in 1..player_length {
            let transforms = if true {
                [
                    BlitTransform::ROTATE | BlitTransform::FLIP_Y | BlitTransform::FLIP_X,
                    <_>::default(),
                ]
            } else {
                [
                    <_>::default(),
                    BlitTransform::ROTATE | BlitTransform::FLIP_Y | BlitTransform::FLIP_X,
                ]
            };
            for transform in transforms {
                self.resources.framebuffer.blit(
                    &(match tile_horizontal(&SLINKY_ATLAS, 16, (SLINKY_FRAMES - 1) as _) {
                        Some(view) => view,
                        _ => {
                            trace("Slinky out of bounds!");
                            continue;
                        }
                    }),
                    [
                        (((player_x_square - i as usize) * 16) as i32) % 160,
                        (((player_y_square) * 16) as i32) % 160,
                    ],
                    transform,
                );
            }
        }
        self.resources
            .framebuffer
            .replace_palette([PALETTE[2], PALETTE[2], PALETTE[2], PALETTE[3]]);
        // draw main tail
        self.resources.framebuffer.blit(
            &(match tile_horizontal(
                &SLINKY_ATLAS,
                16,
                ((SLINKY_FRAMES as isize - (player_x % SLINKY_FRAMES as i32) as isize - 1) as usize
                    % SLINKY_FRAMES) as _,
            ) {
                Some(view) => view,
                _ => {
                    trace("Slinky out of bounds!!");
                    abort()
                }
            }),
            [
                (((player_x_square - player_length as usize) * 16) as i32) % 160,
                (((player_y_square) * 16) as i32) % 160,
            ],
            if player_x_down {
                // away from rest of slinky
                BlitTransform::FLIP_X | BlitTransform::ROTATE
            } else {
                // paired with previous section
                BlitTransform::FLIP_X
            },
        );
        if !player_x_down {
            // extra tail piece
            self.resources.framebuffer.blit(
                &(match tile_horizontal(&SLINKY_ATLAS, 16, (SLINKY_FRAMES - 1) as _) {
                    Some(view) => view,
                    _ => {
                        trace("Slinky out of bounds!!");
                        abort()
                    }
                }),
                [
                    (((player_x_square - player_length as usize) * 16) as i32) % 160,
                    (((player_y_square) * 16) as i32) % 160,
                ],
                BlitTransform::FLIP_X | BlitTransform::ROTATE,
            );
        }
        self.resources.framebuffer.replace_palette(PALETTE);
        // music
        let middle_c = 69;
        // vi - IV - I - V
        // let notes = [69, 71, 65, 70, 67, 63];
        let scale = [0, 2, 4, 7, 9, 11];
        let chords = [(6, false), (4, true), (1, true), (5, true)];
        let octave = 0;
        let (current_chord, major) = chords[self.frames / 60 % chords.len()];
        let frequency = scale[current_chord - 1] + (middle_c - octave * 12) as u8;
        if self.frames % 60 == 0 {
            if major {
                self.play_major_chord(30, frequency, Duration(10))
            } else {
                self.play_minor_chord(30, frequency, Duration(10))
            }
        }
        if self.frames % 60 == 30 {
            self.play_harmonic(60, middle_c - 4 * 12, Duration(10))
        }
        // movement
        self.player.0[0] = self.player.0[0] + direction[0];
        // timing
        self.frames += 1;
        // self.resources
        //     .framebuffer
        //     .text(if player_x_down { "down" } else { "up" }, [0, 0]);
        // self.resources
        //     .framebuffer
        //     .text(&format!("{}", player_x), [0, 8]);
        // self.resources
        //     .framebuffer
        //     .text(&format!("{}", (player_x - 10).rem_euclid(22)), [0, 16]);
    }
}

impl Music for SlinkyRuntime {
    fn play_tones(&self, volume: u32, tones: [Option<(u16, Mode)>; 4], length: Duration) {
        for (tone, instrument) in tones.iter().zip([
            Channel::Noise,
            Channel::Triangle,
            Channel::Pulse1,
            Channel::Pulse2,
        ]) {
            if let Some((frequency, mode)) = *tone {
                self.resources.sound.tone(
                    LinearFrequency::constant(frequency.into()),
                    if instrument == Channel::Noise {
                        length.with_release(Frames::from(length.inner() as u8))
                    } else {
                        length
                    },
                    volume,
                    Flags::new(instrument, mode),
                );
            }
        }
    }
}

main! { SlinkyRuntime }
