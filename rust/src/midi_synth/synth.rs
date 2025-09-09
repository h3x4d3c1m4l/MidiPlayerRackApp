use itertools::Itertools;
use rustysynth::MidiFile;
use rustysynth::MidiFileSequencer;
use rustysynth::SoundFont;
use rustysynth::Synthesizer;
use rustysynth::SynthesizerSettings;
use std::sync::RwLock;
use std::io::Cursor;
use std::sync::Arc;
use tinyaudio::prelude::*;

// ///////// //
// Soundfont //
// ///////// //

static SOUND_FONT: RwLock<Option<Arc<SoundFont>>> = RwLock::new(None);

pub fn load_sound_font(data: Vec<u8>) {
    let mut cursor = Cursor::new(data);
    let mut sf_guard = SOUND_FONT.write().unwrap();
    *sf_guard = Some(Arc::new(SoundFont::new(&mut cursor).unwrap()));
}

// ///////// //
// MIDI file //
// ///////// //

static MIDI_FILE: RwLock<Option<Arc<MidiFile>>> = RwLock::new(None);

pub fn load_midi_file(data: Vec<u8>) {
    let mut cursor = Cursor::new(data);
    let mut mf_guard = MIDI_FILE.write().unwrap();
    *mf_guard = Some(Arc::new(MidiFile::new(&mut cursor).unwrap()));
}

pub fn start_playback() {
    // Setup the audio output.
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    // Buffer for the audio output.
    let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
    let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let synthesizer = Synthesizer::new(&SOUND_FONT.read().unwrap().clone().unwrap(), &settings).unwrap();
    let mut sequencer = MidiFileSequencer::new(synthesizer);

    // Play the MIDI file.
    sequencer.play(&MIDI_FILE.read().unwrap().clone().unwrap(), false);

    // Start the audio output.
    let _device = run_output_device(params, {
        move |data| {
            sequencer.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();

    // Wait for 10 seconds.
    std::thread::sleep(std::time::Duration::from_secs(200));
}
