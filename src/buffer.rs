use crate::units::{Channels, Samples};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Buffer {
    data: Vec<f32>,
    num_channels: Channels,
    num_samples: Samples,
}

impl Buffer {
    pub fn allocate(num_channels: Channels, num_samples: Samples) -> Self {
        let total_num_samples = num_samples.as_usize() * num_channels.as_usize();
        let mut data = Vec::with_capacity(total_num_samples);

        data.resize(total_num_samples, 0.0);

        Self {
            data,
            num_channels,
            num_samples,
        }
    }

    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [f32] {
        &mut self.data
    }

    pub fn zero_out(&mut self) {
        self.data.fill(0.0);
    }

    pub fn channel_indices(&self) -> Range<usize> {
        0..self.num_channels.as_usize()
    }

    pub fn sample_indices(&self) -> Range<usize> {
        0..self.num_samples.as_usize()
    }

    pub fn is_silent(&self) -> bool {
        self.data.iter().all(|s| *s == 0.0)
    }

    pub fn num_channels(&self) -> Channels {
        self.num_channels
    }

    pub fn num_samples(&self) -> Samples {
        self.num_samples
    }

    pub fn chan(&self, index: usize) -> &[f32] {
        if index >= self.num_channels.as_usize() {
            panic!();
        }

        let start = index * self.num_samples.as_usize();
        let end = start + self.num_samples.as_usize();
        &self.data[start..end]
    }

    pub fn chan_mut(&mut self, index: usize) -> &mut [f32] {
        if index >= self.num_channels().as_usize() {
            panic!();
        }

        let start = index * self.num_samples.as_usize();
        let end = start + self.num_samples.as_usize();
        &mut self.data[start..end]
    }

    pub fn iter_chans(&self) -> ChannelIterator {
        ChannelIterator {
            buffer: self,
            current_channel: 0,
        }
    }

    pub fn iter_chans_mut(&mut self) -> MutChannelIterator {
        MutChannelIterator {
            buffer: self,
            current_channel: 0,
        }
    }

    pub fn copy_into(&self, dest: &mut Self) {
        assert_eq!(self.num_channels(), dest.num_channels());
        assert_eq!(self.num_samples(), dest.num_samples());

        for channel in self.channel_indices() {
            for sample in self.sample_indices() {
                dest.chan_mut(channel)[sample] = self.chan(channel)[sample];
            }
        }
    }

    pub fn map_samples(&mut self, mut func: impl FnMut(f32) -> f32) {
        self.data
            .iter_mut()
            .for_each(|sample| *sample = func(*sample));
    }

    pub fn iter_interleaved(&self) -> InterleavedIterator {
        InterleavedIterator {
            buffer: self,
            index: 0,
        }
    }
}

pub struct InterleavedIterator<'a> {
    buffer: &'a Buffer,
    index: usize,
}

impl<'a> Iterator for InterleavedIterator<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let num_channels = self.buffer.num_channels().as_usize();
        let num_samples = self.buffer.num_samples().as_usize();
        let total_num_samples = num_samples * num_channels;
        if self.index >= total_num_samples {
            None
        } else {
            let sample_index = self.index / num_channels;
            let channel_index = self.index - (sample_index * num_channels);
            self.index += 1;
            Some(self.buffer.chan(channel_index)[sample_index])
        }
    }
}

pub struct MutChannelIterator<'a> {
    buffer: &'a mut Buffer,
    current_channel: usize,
}

impl<'a> Iterator for MutChannelIterator<'a> {
    type Item = &'a mut [f32];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_channel >= self.buffer.num_channels().as_usize() {
            return None;
        }
        let channel = self.buffer.chan_mut(self.current_channel);
        let channel_len = channel.len();
        let channel_ptr = channel.as_mut_ptr();
        self.current_channel += 1;
        Some(unsafe { std::slice::from_raw_parts_mut(channel_ptr, channel_len) })
    }
}

pub struct ChannelIterator<'a> {
    buffer: &'a Buffer,
    current_channel: usize,
}

impl<'a> Iterator for ChannelIterator<'a> {
    type Item = &'a [f32];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_channel >= self.buffer.num_channels.as_usize() {
            return None;
        }
        let channel = self.buffer.chan(self.current_channel);
        self.current_channel += 1;
        Some(channel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleaved_iterator() {
        let mut buffer = Buffer::allocate(Channels(2), Samples(3));
        buffer.chan_mut(0)[0] = 1.0;
        buffer.chan_mut(0)[1] = 1.0;
        buffer.chan_mut(0)[2] = 1.0;

        let mut result = Vec::new();
        for sample in buffer.iter_interleaved() {
            result.push(sample);
        }

        assert_eq!(result, &[1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn correct_num_samples_and_channels() {
        let buffer = Buffer::allocate(Channels(2), Samples(10));
        assert_eq!(buffer.num_samples(), Samples(10));
        assert_eq!(buffer.num_channels(), Channels(2));
    }

    #[test]
    fn index_into_channels() {
        let buffer = Buffer::allocate(Channels(2), Samples(10));

        assert_eq!(buffer.chan(0).len(), buffer.num_samples().as_usize());
    }

    #[test]
    fn iterate_channels() {
        let buffer = Buffer::allocate(Channels(2), Samples(10));
        let mut num = 0;
        for _chan in buffer.iter_chans() {
            num += 1;
        }

        assert_eq!(Channels(num), buffer.num_channels());
    }

    #[test]
    fn map_samples() {
        let mut buffer = Buffer::allocate(Channels(2), Samples(3));
        buffer.map_samples(|_| 0.5);
        assert_eq!(buffer.chan(1)[2], 0.5);
    }
}