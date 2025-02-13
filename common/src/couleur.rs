use serde::{Deserialize, Serialize};

#[repr(u64)]
#[derive(PartialEq, Copy, Clone)]
pub enum CouleurBase {
    AMANDE = 1,
    DOREE = 2,
    EBENE = 3,
    EMERAUDE = 4,
    INDIGO = 5,
    IVOIRE = 6,
    ORCHIDEE = 7,
    POURPRE = 8,
    PRUNE = 9,
    ROUSSE = 10,
    TURQUOISE = 11,
}

impl CouleurBase {
    pub fn to_id(self) -> u64 {
        unsafe { ::std::mem::transmute(self) }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Couleur {
    pub id: u64,
    name: String,
}
