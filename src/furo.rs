use crate::pai::Pai;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FuroType {
    CHI,
    PON,
    ANKAN,
    KAKAN,
    DAIMINKAN,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Furo {
    pub furo_type: FuroType,
    pub taken: Pai,
    pub consumed: Vec<Pai>,
    pub min_id: i8,
}

