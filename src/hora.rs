use crate::pai::Pai;
use crate::furo::Furo;


pub enum WaitingType {
    Tanki,
    Kanchan,
    Penchan,
    Ryanmen,
    Shanpon,
}

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mentsu {
    pub mentsu_type:MentsuType,
    pub visibility:VisibilityType,
    pub id:usize
}
impl Mentsu {
    pub fn new(mentsu_type:MentsuType, visibility:VisibilityType ,id:usize) -> Self {
        Self {
            mentsu_type,
            visibility,
            id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HoraPattern {
    pub head:Option<Mentsu>,
    pub mentsus:Vec<Mentsu>,
}
impl HoraPattern {
    pub fn new() -> Self {
        Self {
            head:None,
            mentsus:vec!(),
        }
    }
}


pub struct Yaku {
    name:String,
    fan:i8,
}
impl Yaku {
    pub fn new(name:String, fan:i8) -> Self {
        Self {
            name,
            fan,
        }
    }
}

pub enum Combination {
    Chitoitsu([usize; 34]),
    Kokushimuso([usize; 34]),
    Normal(Vec<HoraPattern>),
}


pub struct HoraCandidate {
    hora:Box<Hora>,
    yakus:Vec<Yaku>,
    combination:Combination,
    taken_index:i8,
    all_pais:[usize; 34],
    

}
impl HoraCandidate {
    pub fn new(
        hora:Box<Hora>,
        yakus:Vec<Yaku>,
        combination:Combination,
        taken_index:i8,
        all_pais:

    ){

    }
}




pub enum HoraType {
    Ron,
    Tsumo,
}

pub struct Hora {
    tehais: [usize; 34],
    furos: Vec<Furo>,
    possible_taken: [usize; 34],
    hora_type: HoraType,
    oya: usize,
    bakaze: usize,
    jikaze: usize,
    doras: Vec<usize>,
    uradoras: Vec<usize>,
    reach: bool,
    double_reach: bool,
    ippatsu: bool,
    rinshan: bool,
    haitei: bool,
    first_turn: bool,
    chankan: bool,
    
    free_pais: [usize; 34],
    all_pais: [usize; 34],
    num_doras: usize,
    num_uradoras: usize,
    num_akadoras: usize,

    candidates: Vec<HoraCandidate>,
    best_candidate: HoraCandidate,
}

impl Hora {
    pub fn new(
        tehais: [usize; 34],
        furos: Vec<Furo>,
        possible_taken: [usize; 34],
        hora_type: HoraType,
        oya: usize,
        bakaze: usize,
        jikaze: usize,
        doras: Vec<usize>,
        uradoras: Vec<usize>,
        reach: bool,
        double_reach: bool,
        ippatsu: bool,
        rinshan: bool,
        haitei: bool,
        first_turn: bool,
        chankan: bool,        
    ) -> Self {


        Self {
            
        }
    }
}



#[cfg(test)]
mod test {
    #[test]
    fn test_hora() {
        assert_eq!(1,1);
    }
}