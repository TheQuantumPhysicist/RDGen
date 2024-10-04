use std::{io::Cursor, num::NonZeroUsize};

use blake2::{Blake2b, Digest};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Error while reading data stream: `{0}`")]
    DataStreamError(String),
}

#[must_use]
pub struct InfiniteDataWriter {
    seed: [u8; 64],
}

impl InfiniteDataWriter {
    pub fn new(seed: impl AsRef<[u8]>) -> Self {
        Self::new_from_stream(Cursor::new(seed.as_ref())).expect("Cannot fail")
    }

    pub fn new_from_stream(mut source: impl std::io::Read) -> Result<Self, Error> {
        let mut hasher = Blake2b::new();

        let mut buffer = [0; 4096];

        loop {
            let bytes_read = source
                .read(&mut buffer)
                .map_err(|e| Error::DataStreamError(e.to_string()))?;

            if bytes_read == 0 {
                break;
            }

            hasher.update(&buffer[..bytes_read]);
        }

        let seed = hasher.finalize().into();
        Ok(Self { seed })
    }

    /// Pull a batch of data, and generate new data in seed
    pub fn pull(&mut self) -> [u8; 64] {
        let mut hasher = Blake2b::new();
        hasher.update(self.seed.as_ref());
        let mut seed = hasher.finalize().into();
        std::mem::swap(&mut seed, &mut self.seed);
        seed
    }

    pub const fn batch_size(&self) -> NonZeroUsize {
        match NonZeroUsize::new(self.seed.len()) {
            Some(r) => r,
            None => panic!("Size must be larger than zero"),
        }
    }
}

impl Iterator for InfiniteDataWriter {
    type Item = [u8; 64];

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.pull())
    }
}

#[must_use]
pub struct FiniteDataWriter {
    writer: InfiniteDataWriter,
    desired_length: Option<usize>,
    pulled_length: usize,
}

impl FiniteDataWriter {
    pub fn new(seed: impl AsRef<[u8]>, desired_length: Option<usize>) -> Self {
        Self {
            writer: InfiniteDataWriter::new(seed),
            desired_length,
            pulled_length: 0,
        }
    }

    pub fn new_from_stream(
        source: impl std::io::Read,
        desired_length: Option<usize>,
    ) -> Result<Self, Error> {
        Ok(Self {
            writer: InfiniteDataWriter::new_from_stream(source)?,
            desired_length,
            pulled_length: 0,
        })
    }

    /// Pull a batch of data, and generate new data in seed
    pub fn pull(&mut self) -> Vec<u8> {
        let data = self.writer.pull();

        let desired_length = match self.desired_length {
            Some(l) => l,
            None => return data.to_vec(),
        };

        let max_length_to_push = desired_length - self.pulled_length;

        if max_length_to_push > self.writer.batch_size().get() {
            self.pulled_length += self.writer.batch_size().get();
            data.to_vec()
        } else {
            self.pulled_length += max_length_to_push;
            data.split_at(max_length_to_push).0.to_vec()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut writer = FiniteDataWriter::new("abc", Some(256));
        assert_eq!(hex::encode(writer.pull()), "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923");
        assert_eq!(hex::encode(writer.pull()), "66cb547665e462bbdd51d9b6ce1221116e9cfc6711c78d8798158349d12fa8ca513efb14bd84edf4e7cd3551355f14c1cf54dd203669b95675e52d72d3ec00d9");
        assert_eq!(hex::encode(writer.pull()), "2ddda015a6b31d39fa9e6d54bb55bab1999a224d23b094fb1f77c41a1ea597c485e10bc721dd5531f1cddc52fdafa09c03ac4fbaaac9271241bd1da64dbd390c");
        assert_eq!(hex::encode(writer.pull()), "50f4b533357084ec5a41ff26dfd36e069a1bf23ed6fd17ee341cf082d409854480332831399565d3f6fa0bed4cab0fad7c81c62b66c2b328ab880f139a094e1c");
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
    }

    #[test]
    fn non_multiple_len() {
        let mut writer = FiniteDataWriter::new("abc", Some(100));
        assert_eq!(hex::encode(writer.pull()), "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923");
        assert_eq!(
            hex::encode(writer.pull()),
            "66cb547665e462bbdd51d9b6ce1221116e9cfc6711c78d8798158349d12fa8ca513efb14"
        );
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
    }

    #[test]
    fn empty() {
        let mut writer = FiniteDataWriter::new("abc", Some(0));
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
        assert_eq!(hex::encode(writer.pull()), "");
    }

    #[test]
    fn no_data() {
        let mut writer = FiniteDataWriter::new("abc", None);
        assert_eq!(hex::encode(writer.pull()), "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d17d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923");
        assert_eq!(hex::encode(writer.pull()), "66cb547665e462bbdd51d9b6ce1221116e9cfc6711c78d8798158349d12fa8ca513efb14bd84edf4e7cd3551355f14c1cf54dd203669b95675e52d72d3ec00d9");
        assert_eq!(hex::encode(writer.pull()), "2ddda015a6b31d39fa9e6d54bb55bab1999a224d23b094fb1f77c41a1ea597c485e10bc721dd5531f1cddc52fdafa09c03ac4fbaaac9271241bd1da64dbd390c");
        assert_eq!(hex::encode(writer.pull()), "50f4b533357084ec5a41ff26dfd36e069a1bf23ed6fd17ee341cf082d409854480332831399565d3f6fa0bed4cab0fad7c81c62b66c2b328ab880f139a094e1c");
        assert_eq!(hex::encode(writer.pull()), "500cb0c9c086a7d65309a6e1d792501f811812411dc22f557c687af44428b68ce19f15ffe1f469cad0fe1180182151ac86f7f406f97e35f943bb084f1f51462b");
        for _ in 0..100 {
            assert!(writer.pull().len() > 0);
        }
    }
}

impl Iterator for FiniteDataWriter {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let data = self.pull();
        if data.is_empty() {
            None
        } else {
            Some(data)
        }
    }
}
