use merlin::Transcript;
use curve25519_dalek::scalar::Scalar;

use byteorder::LittleEndian;

pub trait TranscriptProtocol {
    fn dom_sep(&mut self);
    fn append_epoch(&mut self, epoch: Epoch);
}

impl TranscriptProtocol for Transcript {
    fn dom_sep(&mut self) {
        self.append_message(b"dom-sep", b"Danake v0.1");
    }

    fn append_epoch(&mut self, epoch: Epoch) {
        let mut bytes = [0u8; 16];
        LittleEndian::write_u64(&mut bytes[00..08], epoch.params.0);
        LittleEndian::write_u64(&mut bytes[08..16], epoch.index);
        self.append_message(b"epoch", &bytes[..]);
    }
}