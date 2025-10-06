use rodio::{Sample, SampleRate, Source};

struct MidiChannelSampler {

}

impl Iterator for MidiChannelSampler {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl Source for MidiChannelSampler {
    fn current_span_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> rodio::ChannelCount {
        2
    }

    fn sample_rate(&self) -> rodio::SampleRate {
        44100
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}
