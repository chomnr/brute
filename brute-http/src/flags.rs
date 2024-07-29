use bitflags::bitflags;
use serde::Deserialize;

bitflags! {
    // flags for query actions.
    #[derive(Default, Debug, Clone, Deserialize)]
    pub struct Flags: u32 {
        const INSERT = 0b00000001;
        const UPDATE = 0b00000010;
        const DELETE = 0b00000100;
        const EXISTS = 0b00001000;
    }
}