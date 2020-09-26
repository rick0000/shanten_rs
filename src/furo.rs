use crate::pai::Pai;
use std::cmp;

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
    pub taken: Option<Pai>,
    pub consumed: Vec<Pai>,
    pub min_id: usize,
    pub pais: Vec<Pai>,
}

impl Furo {
    pub fn new(furo_type: FuroType, taken: Option<Pai>, consumed: Vec<Pai>) -> Self {
        let mut min_id = consumed[0].id;
        if let Some(x) = taken {
            min_id = std::cmp::min(min_id, x.id);
        }
        for c in &consumed {
            min_id = std::cmp::min(min_id, c.id);
        }

        let mut pais = vec![];
        if let Some(x) = taken {
            pais.push(x);
        }
        pais.extend(consumed.iter());
        pais.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Self {
            furo_type,
            taken,
            consumed,
            min_id,
            pais,
        }
    }
}
