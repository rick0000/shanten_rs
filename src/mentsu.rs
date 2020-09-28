use crate::pai::Pai;
use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MentsuType {
    Syuntsu,
    Kotsu,
    Kantsu,
    Head,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisibilityType {
    An,
    Min,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mentsu {
    pub mentsu_type: MentsuType,
    pub visibility: VisibilityType,
    pub id: usize,
    pub pais: Vec<Pai>,
    pub is_furo: bool
}

impl Mentsu {
    pub fn new(
            mentsu_type: MentsuType, 
            visibility: VisibilityType, 
            id: usize,
        ) -> Self {
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
            is_furo:false,
        }
    }
    pub fn new_furo(
        mentsu_type: MentsuType, 
        visibility: VisibilityType, 
        id: usize,
    ) -> Self {
    let mut initialized = Self::new(
        mentsu_type,
        visibility,
        id,
    );
    initialized.is_furo = true;
    initialized
}

    pub fn change_to_visible(&mut self){
        self.visibility = VisibilityType::Min;
    }
    pub fn change_to_furo_mentsu(&mut self){
        self.is_furo = true;
    }
}

impl fmt::Debug for Mentsu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ {:?}, {:?}, visibility:{:?} }}",
            self.mentsu_type, self.pais, self.visibility
        )
    }
}
