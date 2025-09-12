use symphonia::{
    core::{
        audio::SampleBuffer,
        codecs::{self, CODEC_TYPE_NULL, DecoderOptions},
        errors::Error,
        formats::FormatReader,
        io::MediaSourceStream,
        probe::Hint,
    },
    default::get_probe,
};

pub struct Decoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn codecs::Decoder>,
    track_id: u32,
    sample_buf: Vec<f32>,
}

impl Decoder {
    pub fn new(path: &str) -> Self {
        let src = std::fs::File::open(path).expect("failed to open media");

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let probed = get_probe()
            .format(&Hint::new(), mss, &Default::default(), &Default::default())
            .expect("Unsupported format");

        // Get the instantiated format reader.
        let format = probed.format;

        // Find the first audio track with a known (decodeable) codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("no supported audio tracks");

        // Use the default options for the decoder.
        let dec_opts: DecoderOptions = Default::default();

        // Create a decoder for the track.
        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("unsupported codec");

        // Store the track identifier, it will be used to filter packets.
        let track_id = track.id;

        Self {
            decoder,
            format,
            track_id,
            sample_buf: Vec::new(),
        }
    }
    pub fn decode(&mut self) {
        // The decode loop.
        loop {
            // Get the next packet from the media format.
            let packet = match self.format.next_packet() {
                Ok(packet) => packet,
                Err(Error::ResetRequired) => {
                    // The track list has been changed. Re-examine it and create a new set of decoders,
                    // then restart the decode loop. This is an advanced feature and it is not
                    // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                    // for chained OGG physical streams.
                    unimplemented!();
                }
                Err(_) => {
                    // A unrecoverable error occured, halt decoding.
                    break;
                }
            };

            // Consume any new metadata that has been read since the last packet.
            while !self.format.metadata().is_latest() {
                // Pop the old head of the metadata queue.
                self.format.metadata().pop();

                // Consume the new metadata at the head of the metadata queue.
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != self.track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    let mut sample_buf =
                        SampleBuffer::<f32>::new(decoded.capacity() as u64, *decoded.spec());

                    // Copy the contents of the decoded audio buffer into the sample buffer whilst performing
                    // any required conversions.
                    sample_buf.copy_interleaved_ref(decoded);
                    sample_buf.samples().iter().for_each(|sample| {
                        self.sample_buf.push(*sample);
                    });
                }
                Err(Error::IoError(_)) => {
                    // The packet failed to decode due to an IO error, skip the packet.
                    continue;
                }
                Err(Error::DecodeError(_)) => {
                    // The packet failed to decode due to invalid data, skip the packet.
                    continue;
                }
                Err(err) => {
                    // An unrecoverable error occured, halt decoding.
                    panic!("{}", err);
                }
            }
        }
    }
    pub fn get_samples(&self) -> &[f32] {
        &self.sample_buf
    }
}
