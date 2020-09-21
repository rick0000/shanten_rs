use crate::pai::Pai;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FuroType {
    CHI,
    PON,
    ANKAN,
    KAKAN,
    DAIMINKAN,
}

#[derive(Clone, Debug)]
pub struct Furo {
    furo_type: FuroType,
    taken: Pai,
    consumed0: Pai,
    consumed1: Pai,
    consumed2: Pai,
    consumed3: Pai,
    min_id: i8,
}

