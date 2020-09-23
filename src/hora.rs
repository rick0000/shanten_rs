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



#[derive(Clone, Debug, PartialEq)]
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


#[derive(Clone, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct FixedHoraPattern {
    pub head:Mentsu,
    pub mentsus:[Mentsu;4],
}
impl FixedHoraPattern {
    pub fn new(head:Mentsu, mentsus:[Mentsu;4]) -> Self {
        Self {
            head,
            mentsus,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HoraType {
    Ron,
    Tsumo,
}


#[derive(Clone, Debug, PartialEq)]
pub enum Combination {
    Chitoitsu(),
    Kokushimuso(),
    Normal(FixedHoraPattern),
}

#[derive(Clone, Debug, PartialEq)]
pub struct HoraYakuInformation {
    oya:bool,
    hora_type:HoraType,
    first_turn:bool,
    num_doras:usize,
    num_akadoras:usize,
    num_uradoras:usize,
    reach:bool,
    double_reach:bool,
    ippatsu:bool,
    rinshan:bool,
    chankan:bool,
    haitei:bool,
    bakaze:Pai,
    jikaze:Pai,
}

#[derive(Clone, Debug)]
pub struct HoraCandidate {
    taken:Pai,
    yaku_info:HoraYakuInformation,
    combination:Combination,
    taken_index:i8,
    yakus:Vec<Yaku>,
}

impl HoraCandidate {
    pub fn new(
        taken:Pai,
        yaku_info:HoraYakuInformation,
        combination:Combination,
        taken_index:i8,
    ) -> Self {
        
        let mut initialized = Self {
            taken,
            yaku_info,
            combination,
            taken_index,
            yakus:vec!(),
        };
        initialized.calc_yakus();
        initialized
    }

    pub fn calc_yakus(&self) {
        if self.yaku_info.first_turn && 
            self.yaku_info.hora_type == HoraType::Tsumo && 
            self.yaku_info.oya {
                
            }
    }

    
}

pub struct Hora {
    tehais: Vec<Pai>,
    furos: Vec<Furo>,
    taken: Pai,
    hora_yaku_information: HoraYakuInformation,
    
    free_pais: Vec<Pai>,
    all_pais: Vec<Pai>,
    num_doras: usize,
    num_uradoras: usize,
    num_akadoras: usize,

    candidates: Vec<HoraCandidate>,
    best_candidate: HoraCandidate,
}

impl Hora {
    pub fn new(
        tehais: Vec<Pai>,
        furos: Vec<Furo>,
        taken: Pai,
        hora_yaku_information: HoraYakuInformation,
    ) -> Self {

        let mut free_pais = tehais.clone();
        free_pais.push(taken);

        let mut all_pais: Vec<Pai> = vec!();
        
        let num_doras = 0;
        let num_uradoras = 0;
        let num_akadoras = 0;
    
        let mut candidates: Vec<HoraCandidate> = vec!();
        
        let mut best_candidate: HoraCandidate = candidates.pop().unwrap();
    

        Self {
            tehais,
            furos,
            taken,
            hora_yaku_information,

            free_pais: free_pais,
            all_pais: all_pais,
            num_doras: num_doras,
            num_uradoras: num_uradoras,
            num_akadoras: num_akadoras,

            candidates: candidates,
            best_candidate: best_candidate,
        }

    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hora_candidate() {
        let taken = Pai::new_by_id(1);
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 1);
        let mentsus = [
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 2),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 3),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 4),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 5),
        ];
        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        
        let candidate = get_hora_candidate(taken, combination);
        
        // assert!(candidate.yakus.contains(&Yaku::new("suanko".to_string(), 100)));
    }

    fn get_hora_candidate(taken:Pai, combination:Combination) -> HoraCandidate {
        let hora_yaku_information = get_hora_yaku_information();
        let taken_index = 0;
        let candidate = HoraCandidate::new(
            taken,
            hora_yaku_information,
            combination,
            taken_index,
        );
        candidate
    }


    fn get_tehais() -> Vec<Pai>{
        let tehai_nums:Vec<i8> = vec![0,1,2,3,4,5,6,7,8,9,9,10,10,10];
        let tehais:Vec<Pai> = tehai_nums.iter().map(|x| Pai::new_by_id(*x)).collect();
        tehais
    }

    fn get_hora_yaku_information() -> HoraYakuInformation {
        let hora_type = HoraType::Tsumo;
        let oya = true;
        let bakaze = Pai::new_by_id(27);
        let jikaze = Pai::new_by_id(27);
        let num_doras = 0;
        let num_akadoras = 0;
        let num_uradoras = 0;
        let reach = true;
        let double_reach = false;
        let ippatsu = false;
        let rinshan = false;
        let haitei = false;
        let first_turn = false;
        let chankan = false;

        HoraYakuInformation {
            hora_type,
            oya,
            bakaze,
            jikaze,
            num_doras,
            num_akadoras,
            num_uradoras,
            reach,
            double_reach,
            ippatsu,
            rinshan,
            haitei,
            first_turn,
            chankan,
        }
    }


}