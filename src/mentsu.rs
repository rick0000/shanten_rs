use crate::pai::Pai;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MentsuType {
    Syuntsu,
    Kotsu,
    Kantsu,
    Head,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VisibilityType {
    An,
    Min,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Mentsu {
    pub mentsu_type: MentsuType,
    pub visibility: VisibilityType,
    pub id: usize,
    pub pais: Vec<Pai>,
}

impl Mentsu {
    pub fn new(mentsu_type: MentsuType, visibility: VisibilityType, id: usize) -> Self {
        let mut pais = vec![];
        match mentsu_type {
            MentsuType::Head => pais.extend(Pai::new_by_vec(vec![id, id])),
            MentsuType::Kantsu => pais.extend(Pai::new_by_vec(vec![id, id, id, id])),
            MentsuType::Kotsu => pais.extend(Pai::new_by_vec(vec![id, id, id])),
            MentsuType::Syuntsu => pais.extend(Pai::new_by_vec(vec![id, id + 1, id + 2])),
        }
        Self {
            mentsu_type,
            visibility,
            id,
            pais,
        }
    }
}
