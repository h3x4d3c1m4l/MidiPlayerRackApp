use crate::midi_synth::synth;

pub fn load_sound_font(sound_font_data: Vec<u8>) {
    synth::load_sound_font(sound_font_data);
}

pub fn load_midi_data(midi_data: Vec<u8>) {
    synth::load_midi_file(midi_data);
}

pub fn start_playback() {
    synth::start_playback();
}
