/// Enumeration representing custom implemented server-side modes.
pub enum CustomMode {
    VANILLA,
    RELAX,
    AUTOPILOT,
}

impl From<u8> for CustomMode {
    fn from(i: u8) -> Self {
        match i {
            1 => Self::RELAX,
            2 => Self::AUTOPILOT,
            _ => Self::VANILLA,
        }
    }
}

/// Enumeration representing in-game modes.
pub enum Mode {
    STANDARD,
    TAIKO,
    CATCH,
    MANIA,
}

impl From<u8> for Mode {
    fn from(i: u8) -> Self {
        match i {
            1 => Self::TAIKO,
            2 => Self::CATCH,
            3 => Self::MANIA,
            _ => Self::STANDARD,
        }
    }
}
