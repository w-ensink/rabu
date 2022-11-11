use std::cmp::min;
use std::ops::Range;

use crate::units::{Channels, Samples};

/// Multi-channel buffer for any type of audio. It has some utility
/// functions that make common audio related tasks simpler.
#[derive(Clone, Debug)]
pub struct Buffer<T> {
    /// The actual stored data, channels are stored one after the other (not interleaved)
    data: Vec<T>,
    num_channels: Channels,
    num_samples: Samples,
}

impl<T> Buffer<T>
where
    T: Copy + Default + PartialEq,
{
    /// Tells you whether the buffer is filled with the default value of the contained type.
    /// This is useful to check if the complete buffer is silent for example.
    pub fn is_default_filled(&self) -> bool {
        self.data.iter().all(|s| *s == T::default())
    }
}

impl<T> Buffer<T>
where
    T: Copy + Default,
{
    /// Allocates a new buffer with the given number of channels and samples.
    pub fn allocate(num_channels: Channels, num_samples: Samples) -> Self {
        let total_num_samples = num_samples.as_usize() * num_channels.as_usize();
        let mut data = Vec::with_capacity(total_num_samples);

        data.resize(total_num_samples, T::default());

        Self {
            data,
            num_channels,
            num_samples,
        }
    }

    /// Creates a new buffer with the given size, copying all data from self.
    pub fn clone_resized(&self, num_channels: Channels, num_samples: Samples) -> Self {
        let mut target = Self::allocate(num_channels, num_samples);

        for channel in 0..min(self.num_channels(), num_channels).as_usize() {
            for sample in 0..min(self.num_samples(), num_samples).as_usize() {
                target.chan_mut(channel)[sample] = self.chan(channel)[sample];
            }
        }

        target
    }

    /// Returns a reference to the internal buffer. Channels are stored one after the other,
    /// so **not** interleaved!
    pub fn data(&self) -> &[T] {
        &self.data
    }

    /// Returns a mutable reference to the internal buffer. Channels are stored one after the other,
    /// so **not** interleaved!
    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Fills the buffer with the default value of the given type `T`. This can be useful to
    /// make the buffer silent for example.
    pub fn fill_default(&mut self) {
        self.data.fill(T::default());
    }

    /// Gives you the channel numbers as a range. This can be useful when you want to iterate over
    /// the channel indices.
    pub fn channel_indices(&self) -> Range<usize> {
        0..self.num_channels.as_usize()
    }

    /// Gives you the sample indices as a range. This can be useful when you want to use the
    /// sample index in the loop for some reason.
    pub fn sample_indices(&self) -> Range<usize> {
        0..self.num_samples.as_usize()
    }

    /// Returns the number of channels in the buffer.
    pub fn num_channels(&self) -> Channels {
        self.num_channels
    }

    /// Returns the number of samples that each channel contains
    /// (**not the total number of samples in the buffer**).
    pub fn num_samples(&self) -> Samples {
        self.num_samples
    }

    /// Returns a reference to the given channel (indexing starts at 0).
    pub fn chan(&self, index: usize) -> &[T] {
        if index >= self.num_channels.as_usize() {
            panic!();
        }

        let start = index * self.num_samples.as_usize();
        let end = start + self.num_samples.as_usize();
        &self.data[start..end]
    }

    /// Returns a mutable reference to the given channel (indexing starts at 0).
    pub fn chan_mut(&mut self, index: usize) -> &mut [T] {
        if index >= self.num_channels().as_usize() {
            panic!();
        }

        let start = index * self.num_samples.as_usize();
        let end = start + self.num_samples.as_usize();
        &mut self.data[start..end]
    }

    /// Returns an iterator to iterate over the channels in the buffer.
    pub fn iter_chans(&self) -> ChannelIterator<T> {
        ChannelIterator {
            buffer: self,
            current_channel: 0,
        }
    }

    /// Returns a mutable iterator to iterate over the channels in the buffer.
    pub fn iter_chans_mut(&mut self) -> MutChannelIterator<T> {
        MutChannelIterator {
            buffer: self,
            current_channel: 0,
        }
    }

    /// Copies the content of self into the given target buffer.
    /// This will panic if the buffers are not of the same size.
    pub fn copy_into(&self, dest: &mut Self) {
        assert_eq!(self.num_channels(), dest.num_channels());
        assert_eq!(self.num_samples(), dest.num_samples());

        for channel in self.channel_indices() {
            for sample in self.sample_indices() {
                dest.chan_mut(channel)[sample] = self.chan(channel)[sample];
            }
        }
    }

    /// Applies the given map function to all samples in the buffer.
    /// This can be useful for multiplying all samples by some value, for example.
    pub fn map_samples(&mut self, mut func: impl FnMut(T) -> T) {
        self.data
            .iter_mut()
            .for_each(|sample| *sample = func(*sample));
    }

    /// Iterate over all samples in the buffer, but make it behave like an interleaved buffer.
    pub fn iter_interleaved(&self) -> InterleavedIterator<T> {
        InterleavedIterator {
            buffer: self,
            index: 0,
        }
    }
}

pub struct InterleavedIterator<'a, T>
where
    T: Copy + Default,
{
    buffer: &'a Buffer<T>,
    index: usize,
}

impl<'a, T> Iterator for InterleavedIterator<'a, T>
where
    T: Copy + Default,
{
    type Item = T;

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

pub struct MutChannelIterator<'a, T>
where
    T: Copy,
{
    buffer: &'a mut Buffer<T>,
    current_channel: usize,
}

impl<'a, T> Iterator for MutChannelIterator<'a, T>
where
    T: Copy + Default,
{
    type Item = &'a mut [T];

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

pub struct ChannelIterator<'a, T>
where
    T: Copy + Default,
{
    buffer: &'a Buffer<T>,
    current_channel: usize,
}

impl<'a, T> Iterator for ChannelIterator<'a, T>
where
    T: Copy + Default,
{
    type Item = &'a [T];

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
        let buffer = Buffer::<f32>::allocate(Channels(2), Samples(10));
        assert_eq!(buffer.num_samples(), Samples(10));
        assert_eq!(buffer.num_channels(), Channels(2));
    }

    #[test]
    fn index_into_channels() {
        let buffer = Buffer::<f32>::allocate(Channels(2), Samples(10));

        assert_eq!(buffer.chan(0).len(), buffer.num_samples().as_usize());
    }

    #[test]
    fn iterate_channels() {
        let buffer = Buffer::<f32>::allocate(Channels(2), Samples(10));
        let mut num = 0;
        for _chan in buffer.iter_chans() {
            num += 1;
        }

        assert_eq!(Channels(num), buffer.num_channels());
    }

    #[test]
    fn map_samples() {
        let mut buffer = Buffer::<f32>::allocate(Channels(2), Samples(3));
        buffer.map_samples(|_| 0.5);
        assert_eq!(buffer.chan(1)[2], 0.5);
    }

    #[test]
    fn clone_with_new_bigger_size() {
        let mut buffer = Buffer::<f32>::allocate(Channels(2), Samples(3));
        for chan in buffer.channel_indices() {
            for samp in buffer.sample_indices() {
                buffer.chan_mut(chan)[samp] = samp as f32;
            }
        }

        let resized = buffer.clone_resized(Channels(3), Samples(4));

        assert_eq!(resized.chan(0)[1], 1.0);
        assert_eq!(resized.chan(0)[3], 0.0);

        assert_eq!(resized.chan(1)[1], 1.0);
        assert_eq!(resized.chan(1)[3], 0.0);

        assert_eq!(resized.chan(2)[1], 0.0);
    }

    #[test]
    fn clone_with_new_smaller_size() {
        let mut buffer = Buffer::<f32>::allocate(Channels(2), Samples(3));
        for chan in buffer.channel_indices() {
            for samp in buffer.sample_indices() {
                buffer.chan_mut(chan)[samp] = samp as f32;
            }
        }

        let resized = buffer.clone_resized(Channels(1), Samples(2));

        assert_eq!(resized.chan(0)[1], 1.0);
        assert_eq!(resized.chan(0)[0], 0.0);
    }
}
