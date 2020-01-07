use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct EpochParameters(u64);

// XXX should this have a phantom type parameter instead of just bundling the params?
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Epoch {
    pub(crate) index: i64,
    pub(crate) params: EpochParameters,
}

pub enum EpochState {
    Active,
    Primary,
    Rollover,
    Invalid,
}

impl From<std::time::Duration> for EpochParameters {
    fn from(duration: std::time::Duration) -> Self {
        Self(duration.as_secs())
    }
}

impl EpochParameters {
    pub fn epoch_at(&self, time: DateTime<Utc>) -> Epoch {
        Epoch {
            index: time.timestamp() / (self.0 as i64),
            params: self.clone(), 
        }
    }
}

impl Epoch {
    pub fn state_at(&self, time: DateTime<Utc>) -> EpochState {
        let current = self.params.epoch_at(time);
        match current.index - self.index {
            -1 => EpochState::Active,
            0 => EpochState::Primary,
            1 => EpochState::Active,
            2 => EpochState::Rollover,
            _ => EpochState::Invalid,
        }
    }
}