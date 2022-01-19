use wasm4::sound::{Duration, Mode};

fn shift_semitones(frequency: f32, semitones: f32) -> u16 {
    (frequency * 2_f32.powf(semitones / 12.)).round() as u16
}

pub fn midi_to_frequency(note: u8) -> u16 {
    shift_semitones(440., note as f32 - 69.)
}
pub trait Music {
    fn play_tones(&self, volume: u32, tones: [Option<(u16, Mode)>; 4], length: Duration);

    fn play_harmonic(&self, volume: u32, midi_note: u8, length: Duration) {
        let frequency = midi_to_frequency(midi_note);
        self.play_tones(
            volume,
            [
                Some((frequency, Mode::N1D8)),
                Some((frequency * 2, Mode::N1D2)),
                Some((frequency * 4, Mode::N1D4)),
                Some((frequency * 8, Mode::N1D8)),
            ],
            length,
        );
    }

    fn play_major_chord(&self, volume: u32, midi_note: u8, length: Duration) {
        self.play_tones(
            volume,
            [
                Some((midi_to_frequency(midi_note), Mode::N1D8)),
                Some((midi_to_frequency(midi_note), Mode::N1D2)),
                Some((midi_to_frequency(midi_note + 4), Mode::N1D4)),
                Some((midi_to_frequency(midi_note + 7), Mode::N1D8)),
            ],
            length,
        );
    }

    fn play_minor_chord(&self, volume: u32, midi_note: u8, length: Duration) {
        let a: [u8; 10] = [10; 10];
        let b: &[u8] = &a;
        self.play_tones(
            volume,
            [
                Some((midi_to_frequency(midi_note), Mode::N1D8)),
                Some((midi_to_frequency(midi_note), Mode::N1D2)),
                Some((midi_to_frequency(midi_note + 3), Mode::N1D4)),
                Some((midi_to_frequency(midi_note + 7), Mode::N1D8)),
            ],
            length,
        );
    }
}
