use crate::furo::{Furo, FuroType};
use crate::pai::Pai;
use crate::yaku::{Yaku, YakuName};

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

#[derive(Clone, Debug, PartialEq)]
pub struct Mentsu {
    pub mentsu_type: MentsuType,
    pub visibility: VisibilityType,
    pub id: usize,
    pub pais: Vec<Pai>,
}
impl Mentsu {
    pub fn new(mentsu_type: MentsuType, visibility: VisibilityType, id:usize) -> Self {
        let mut pais = vec![];
        match mentsu_type {
            MentsuType::Head => pais.extend(Pai::new_by_vec(vec![id,id])),
            MentsuType::Kantsu => pais.extend(Pai::new_by_vec(vec![id,id,id,id])),
            MentsuType::Kotsu => pais.extend(Pai::new_by_vec(vec![id,id,id])),
            MentsuType::Syuntsu => pais.extend(Pai::new_by_vec(vec![id,id+1,id+2])),
        }
        Self {
            mentsu_type,
            visibility,
            id,
            pais,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HoraPattern {
    pub head: Option<Mentsu>,
    pub mentsus: Vec<Mentsu>,
}
impl HoraPattern {
    pub fn new() -> Self {
        Self {
            head: None,
            mentsus: vec![],
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct FixedHoraPattern {
    pub head: Mentsu,
    pub mentsus: Vec<Mentsu>,
}
impl FixedHoraPattern {
    pub fn new(head: Mentsu, mentsus: Vec<Mentsu>) -> Self {
        Self { head, mentsus }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum HoraType {
    Ron,
    Tsumo,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Combination {
    Chitoitsu,
    Kokushimuso,
    Normal(FixedHoraPattern),
}

#[derive(Clone, Debug, PartialEq)]
pub struct HoraYakuInformation {
    oya: bool,
    hora_type: HoraType,
    first_turn: bool,
    num_doras: usize,
    num_akadoras: usize,
    num_uradoras: usize,
    reach: bool,
    double_reach: bool,
    ippatsu: bool,
    rinshan: bool,
    chankan: bool,
    haitei: bool,
    bakaze: Pai,
    jikaze: Pai,
}

#[derive(Clone, Debug)]
pub struct HoraCandidate {
    taken: Pai,
    furos: Vec<Furo>,
    all_pais: Vec<Pai>,
    yaku_info: HoraYakuInformation,
    combination: Combination,
    taken_index: i8,
    yakus: Vec<Yaku>,
}

impl HoraCandidate {
    pub fn new(
        taken: Pai,
        furos: Vec<Furo>,
        all_pais: Vec<Pai>,
        yaku_info: HoraYakuInformation,
        combination: Combination,
        taken_index: i8,
    ) -> Self {
        let mut initialized = Self {
            taken,
            furos,
            all_pais,
            yaku_info,
            combination,
            taken_index,
            yakus: vec![],
        };
        initialized.calc_yakus();
        initialized
    }

    fn calc_yakus(&mut self) {
        let menzen = self
            .furos
            .iter()
            .filter(|e| e.furo_type != FuroType::ANKAN)
            .count()
            == 0;

        if self.yaku_info.first_turn
            && self.yaku_info.hora_type == HoraType::Tsumo
            && self.yaku_info.oya
        {
            self.add_yaku(YakuName::Tenho, 100, 0, menzen);
        }
        if self.yaku_info.first_turn
            && self.yaku_info.hora_type == HoraType::Tsumo
            && self.yaku_info.oya == false
        {
            self.add_yaku(YakuName::Chiho, 100, 0, menzen);
        }
        if let Combination::Kokushimuso = self.combination {
            self.add_yaku(YakuName::Kokushimuso, 100, 0, menzen);
        }
    }

    fn add_yaku(&mut self, yaku_name: YakuName, menzen_fan: i8, kui_fan: i8, menzen: bool) {
        if menzen {
            self.yakus.push(Yaku::new(yaku_name, menzen_fan));
        } else {
            self.yakus.push(Yaku::new(yaku_name, kui_fan));
        }
    }
    fn delete_yaku(&mut self, yaku_name: YakuName) {
        self.yakus.retain(|x| x.yaku_name != yaku_name);
    }

    fn num_anko(&self) -> usize {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus
                .iter()
                .filter(|e| {
                    (e.mentsu_type == MentsuType::Kotsu && e.visibility == VisibilityType::An)
                        || (e.mentsu_type == MentsuType::Kantsu
                            && e.visibility == VisibilityType::An)
                })
                .count()
        } else {
            0
        }
    }

    fn num_kantsu(&self) -> usize {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus
                .iter()
                .filter(|e| e.mentsu_type == MentsuType::Kantsu)
                .count()
        } else {
            0
        }
    }

    fn ryuiso(&self) -> bool {
        self.all_pais.iter().filter(|e| e.is_green()).count() == 14
    }

    fn num_sangenpais(&self) -> usize {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus.iter().filter(|e| e.pais[0].is_dragon()).count()
        } else {
            0
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

        let mut all_pais: Vec<Pai> = vec![];

        let num_doras = 0;
        let num_uradoras = 0;
        let num_akadoras = 0;

        let mut candidates: Vec<HoraCandidate> = vec![];

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
    fn test_num_sangenpais() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 0);
        let mentsus = vec![
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 33),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 3),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 4),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 5),
        ];
        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        let taken = Pai::new(1);
        let candidate = get_hora_candidate(taken, vec![], vec![], combination);

        assert_eq!(candidate.num_sangenpais(), 1);
        println!("num_sangenpais():{}", candidate.num_sangenpais());
    }

    fn test_hora_candidate() {
        let taken = Pai::new(1);
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 0);
        let mentsus = vec![
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 33),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 3),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 4),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 5),
        ];

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();
        
        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        
        
        
        let candidate = get_hora_candidate(taken, all_pais, furos, combination);

        assert!(candidate.yakus.contains(&Yaku::new(YakuName::Suanko, 100)));
    }




    fn get_hora_candidate(taken: Pai, all_pais:Vec<Pai>, furos:Vec<Furo>, combination: Combination) -> HoraCandidate {
        let hora_yaku_information = get_hora_yaku_information();
        let taken_index = 0;
        
        
        let candidate = HoraCandidate::new(
            taken,
            furos,
            all_pais,
            hora_yaku_information,
            combination,
            taken_index,
        );
        candidate
    }

    fn get_tehais() -> Vec<Pai> {
        let tehai_nums: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 9, 10, 10, 10];
        let tehais: Vec<Pai> = tehai_nums.iter().map(|x| Pai::new(*x)).collect();
        tehais
    }

    fn get_hora_yaku_information() -> HoraYakuInformation {
        let hora_type = HoraType::Tsumo;
        let oya = true;
        let bakaze = Pai::new(27);
        let jikaze = Pai::new(27);
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
