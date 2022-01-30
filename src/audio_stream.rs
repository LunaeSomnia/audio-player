use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    PauseStreamError, PlayStreamError, Stream,
};
use rb::{Producer, RbConsumer, SpscRb, RB};

const BUFFER_SIZE: usize = 4096;

pub struct AudioStream {
    ring_buffer: SpscRb<f32>,
    cpal_stream: Stream,
}

impl AudioStream {
    pub fn new() -> (Self, Producer<f32>) {
        let ring_buf: SpscRb<f32> = rb::SpscRb::new(BUFFER_SIZE);
        let (pr, cn) = (ring_buf.producer(), ring_buf.consumer());

        let dev = cpal::default_host().default_output_device().unwrap();
        let conf = dev.default_output_config().unwrap();
        let str = dev
            .build_output_stream(
                &conf.config(),
                move |d: &mut [f32], _| {
                    let written = cn.read(d).unwrap_or(0);
                    d[written..].iter_mut().for_each(|s| *s = 0.0);
                },
                |e| {
                    panic!("{}", e);
                },
            )
            .unwrap();

        (
            AudioStream {
                ring_buffer: ring_buf,
                cpal_stream: str,
            },
            pr,
        )
    }

    pub fn play(&self) -> Result<(), PlayStreamError> {
        self.cpal_stream.play()
    }

    pub fn stop(&self) -> Result<(), PauseStreamError> {
        self.ring_buffer.clear();
        self.cpal_stream.pause()
    }
}
