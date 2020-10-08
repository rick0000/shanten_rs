use crate::furo::{Furo, FuroType};
use crate::mentsu::{Mentsu, MentsuType, VisibilityType};
use crate::pai::{Pai, PaiType};
use crate::tenpai_analysis::{Combination, FixedHoraPattern, HoraType};
use crate::yaku::{Yaku, YakuName};
use crate::point_datam::PointDatam;

#[derive(Clone, Debug)]
pub enum TakenPosition {
    Head,
    Mentsu(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Machi {
    Tanki,
    Kanchan,
    Penchan,
    Ryanmen,
    Shanpon,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HoraYakuInformation {
    pub oya: bool,
    pub hora_type: HoraType,
    pub first_turn: bool,
    pub num_doras: usize,
    pub num_akadoras: usize,
    pub num_uradoras: usize,
    pub reach: bool,
    pub double_reach: bool,
    pub ippatsu: bool,
    pub rinshan: bool,
    pub chankan: bool,
    pub haitei: bool,
    pub bakaze: Pai,
    pub jikaze: Pai,
}

#[derive(Clone, Debug)]
pub struct HoraCandidate {
    pub taken: Pai,
    pub furos: Vec<Furo>,
    pub all_pais: Vec<Pai>,
    pub yaku_info: HoraYakuInformation,
    pub combination: Combination,
    pub taken_position: Option<TakenPosition>,
    pub yakus: Vec<Yaku>,
    pub janto: Option<Pai>,
    pub machi: Option<Machi>,
    pub points: PointDatam,
}

const YAKUMAN_FAN: usize = 100;
impl HoraCandidate {
    pub fn new(
        taken: Pai,
        furos: Vec<Furo>,
        all_pais: Vec<Pai>,
        yaku_info: HoraYakuInformation,
        combination: Combination,
        taken_position: Option<TakenPosition>,
    ) -> Self {

        // fix visivility
        
        let mut visivility_fixed_combination = combination.clone();
        if yaku_info.hora_type == HoraType::Ron {
            visivility_fixed_combination = Self::get_machi_fixed_combination(combination.clone(), taken_position.clone());
        }
            
        
        let machi = Self::get_machi(taken, taken_position.clone(), combination.clone());
        let mut initialized = Self {
            taken,
            furos,
            all_pais,
            yaku_info,
            combination: visivility_fixed_combination,
            taken_position,
            yakus: vec![],
            janto: None,
            machi,
            points: PointDatam::new(0,0,false,HoraType::Ron), // dummy
        };
        
        initialized.calc_yakus();
        let fu = initialized.get_fu();
        let fan = initialized.yakus.iter().fold(0, |sum, y| sum + y.fan) as u32;
        let oya = yaku_info.oya;
        let hora_type = yaku_info.hora_type;

        initialized.points = PointDatam::new(
            fu, 
            fan, 
            oya, 
            hora_type, 
        );
        
        initialized
    }

    pub fn get_priority(&self) -> u32 {
        self.points.points * 1000 + self.points.fan * 1000 + self.points.fu
    }

    pub fn get_points(&self) -> u32 {
        self.points.points
    }

    pub fn get_pointdatam(&self) -> PointDatam {
        self.points.clone()
    }

    
    fn get_machi_fixed_combination(
        combination: Combination,
        taken_position: Option<TakenPosition>,
    ) -> Combination {
        if let (Combination::Normal(c), Some(pos)) = (combination.clone(), taken_position) {
            let mut pattern:FixedHoraPattern = c.clone();
            match pos {
                TakenPosition::Mentsu(mentsu_index) => {
                    pattern.mentsus[mentsu_index].change_to_visible();
                    return Combination::Normal(pattern);
                }
                TakenPosition::Head => {
                    pattern.head.change_to_visible();
                    return Combination::Normal(pattern);
                }
            }
        }

        combination.clone()
    }

    fn get_machi(
        taken:Pai,
        taken_position: Option<TakenPosition>,
        combination: Combination,
    ) -> Option<Machi> {
        if let Some(pos) = taken_position.clone() {
            match pos {
                TakenPosition::Head => {
                    return Some(Machi::Tanki)
                },
                TakenPosition::Mentsu(mentsu_index) => {
                    if let Combination::Normal(c) = combination {
                        let target_mentsu_pais = &c.mentsus[mentsu_index].pais;
                        // println!("{:?}",target_mentsu_pais);
                        match &c.mentsus[mentsu_index].mentsu_type {
                            MentsuType::Syuntsu => {
                                if target_mentsu_pais[1].is_same_symbol(taken) {
                                    return Some(Machi::Kanchan);
                                }
                                if target_mentsu_pais[0].is_same_symbol(taken) 
                                    && target_mentsu_pais[0].number == 7 {
                                        return Some(Machi::Penchan);
                                    }
                                if target_mentsu_pais[2].is_same_symbol(taken) 
                                    && target_mentsu_pais[2].number == 3 {
                                        return Some(Machi::Penchan);
                                    }
                                return Some(Machi::Ryanmen);
                            },
                            _ => {
                                return Some(Machi::Shanpon);
                            }
                        }
                    }
                },
            }
        }
        None
    }
    


    fn get_fu(&self) -> u32 {
        match &self.combination {
            Combination::Chitoitsu => {
                return 25;
            },
            Combination::Kokushimuso => {
                return 20;
            },
            Combination::Normal(c) => {
                if self.is_menzen() && self.pinfu() {
                    if self.yaku_info.hora_type == HoraType::Ron {
                        return 30;
                    } else if self.yaku_info.hora_type == HoraType::Tsumo {
                        return 20;
                    }
                }
                
                let mut fu = 20;
                if self.is_menzen() 
                    && self.yaku_info.hora_type == HoraType::Ron {
                    fu += 10;
                }
                if self.yaku_info.hora_type == HoraType::Tsumo{
                    fu += 2;
                }
                
                for mentsu in &c.mentsus {
                    let mut mentsu_fu = 0;
                    if mentsu.mentsu_type == MentsuType::Kotsu {
                        mentsu_fu += 2;
                    } else if mentsu.mentsu_type == MentsuType::Kantsu {
                        mentsu_fu += 8;
                    }
                    if mentsu.pais[0].is_yaochu() {
                        mentsu_fu *= 2;
                    }
                    if mentsu.visibility == VisibilityType::An {
                        mentsu_fu *= 2;
                    }
                    fu += mentsu_fu;
                }
                
                fu += self.fanpai_fan(c.head.pais[0]) * 2;
                if let Some(m) = &self.machi {
                    match m {
                        Machi::Kanchan | Machi::Penchan | Machi::Tanki => {
                            fu += 2;
                        }
                        _ => {},
                    }    
                }
                if fu == 20 {
                    // furoed pinfu is treated as 30 fu.
                    fu = 30;
                }
                (f32::ceil(fu as f32/10.0) * 10.0) as u32
            },
            
        }
    }

    fn is_menzen(&self) -> bool {
        self
        .furos
        .iter()
        .filter(|e| e.furo_type != FuroType::ANKAN)
        .count()
        == 0
    }

    fn calc_yakus(&mut self) {
        let menzen = self.is_menzen();
        

        if self.yaku_info.first_turn
            && self.yaku_info.hora_type == HoraType::Tsumo
            && self.yaku_info.oya
        {
            self.add_yaku(YakuName::Tenho, YAKUMAN_FAN, 0, menzen);
        }
        if self.yaku_info.first_turn
            && self.yaku_info.hora_type == HoraType::Tsumo
            && self.yaku_info.oya == false
        {
            self.add_yaku(YakuName::Chiho, YAKUMAN_FAN, 0, menzen);
        }
        if let Combination::Kokushimuso = self.combination {
            self.add_yaku(YakuName::Kokushimuso, YAKUMAN_FAN, 0, menzen);
        }
        if self.num_sangenpais() == 3 {
            self.add_yaku(YakuName::Daisangen, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }
        if self.num_anko() == 4 {
            self.add_yaku(YakuName::Suanko, YAKUMAN_FAN, 0, menzen);
        }
        if self.all_pais.iter().all(|x| x.is_jihai()) {
            self.add_yaku(YakuName::Tsuiso, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }
        if self.ryuiso() {
            self.add_yaku(YakuName::Ryuiso, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }
        if self.chinroto() {
            self.add_yaku(YakuName::Chinroto, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }
        if self.daisushi() {
            self.add_yaku(YakuName::Daisushi, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }
        if self.shosushi() {
            self.add_yaku(YakuName::Shosushi, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }
        if self.num_kantsu() == 4 {
            self.add_yaku(YakuName::Sukantsu, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }
        if self.churenpoton() {
            self.add_yaku(YakuName::Churenpoton, YAKUMAN_FAN, YAKUMAN_FAN, menzen);
        }

        if self.yakus.len() > 0 {
            return;
        }

        self.add_yaku(
            YakuName::Dora,
            self.yaku_info.num_doras,
            self.yaku_info.num_doras,
            menzen,
        );
        self.add_yaku(
            YakuName::Uradora,
            self.yaku_info.num_uradoras,
            self.yaku_info.num_uradoras,
            menzen,
        );
        self.add_yaku(
            YakuName::Akadora,
            self.yaku_info.num_akadoras,
            self.yaku_info.num_akadoras,
            menzen,
        );

        // 1fan
        if self.yaku_info.reach {
            self.add_yaku(YakuName::Reach, 1, 0, menzen);
        }
        if self.yaku_info.ippatsu {
            self.add_yaku(YakuName::Ippatsu, 1, 0, menzen);
        }
        if menzen && self.yaku_info.hora_type == HoraType::Tsumo {
            self.add_yaku(YakuName::MenzenchinTsumoho, 1, 0, menzen);
        }
        if self.tanyaochu() {
            self.add_yaku(YakuName::Tanyaochu, 1, 1, menzen);
        }
        if self.pinfu() {
            self.add_yaku(YakuName::Pinfu, 1, 0, menzen);
        }
        if self.ipeko() {
            self.add_yaku(YakuName::Ipeko, 1, 0, menzen);
        }
        let num_sangenpai = self.num_sangenpais();
        self.add_yaku(YakuName::Sangenpai, num_sangenpai, num_sangenpai, menzen);
        if self.bakaze() {
            self.add_yaku(YakuName::Bakaze, 1, 1, menzen);
        }
        if self.jikaze() {
            self.add_yaku(YakuName::Jikaze, 1, 1, menzen);
        }
        if self.yaku_info.chankan {
            self.add_yaku(YakuName::Chankan, 1, 1, menzen);
        }
        if self.yaku_info.haitei && self.yaku_info.hora_type == HoraType::Tsumo {
            self.add_yaku(YakuName::Haiteiraoyue, 1, 1, menzen);
        }
        if self.yaku_info.haitei && self.yaku_info.hora_type == HoraType::Ron {
            self.add_yaku(YakuName::Hoteiraoyui, 1, 1, menzen);
        }
        if self.yaku_info.rinshan {
            self.add_yaku(YakuName::Rinshankaiho, 1, 1, menzen);
            self.delete_yaku(YakuName::Haiteiraoyue);
        }

        if self.sanshokudojun() {
            self.add_yaku(YakuName::Sanshokudojun, 2, 1, menzen);
        }
        if self.ikkitsukan() {
            self.add_yaku(YakuName::Ikkitsukan, 2, 1, menzen);
        }
        if self.honchantaiyao() {
            self.add_yaku(YakuName::Honchantaiyao, 2, 1, menzen);
        }
        if let Combination::Chitoitsu = self.combination {
            self.add_yaku(YakuName::Chitoitsu, 2, 0, menzen);
        }
        if self.toitoiho() {
            self.add_yaku(YakuName::Toitoiho, 2, 2, menzen);
        }
        if self.num_anko() == 3 {
            self.add_yaku(YakuName::Sananko, 2, 2, menzen);
        }
        if self.honroto() {
            self.add_yaku(YakuName::Honroto, 2, 2, menzen);
            self.delete_yaku(YakuName::Honchantaiyao);
        }
        if self.sanshokudoko() {
            self.add_yaku(YakuName::Sanshokudoko, 2, 2, menzen);
        }
        if self.num_kantsu() == 3 {
            self.add_yaku(YakuName::Sankantsu, 2, 2, menzen);
        }
        if self.shosangen() {
            self.add_yaku(YakuName::Shosangen, 2, 2, menzen);
        }
        if self.yaku_info.double_reach {
            self.add_yaku(YakuName::DoubleReach, 2, 0, menzen);
            self.delete_yaku(YakuName::Reach);
        }

        if self.honiso() {
            self.add_yaku(YakuName::Honiso, 3, 2, menzen);
        }
        if self.junchantaiyao() {
            self.add_yaku(YakuName::Junchantaiyao, 3, 2, menzen);
            self.delete_yaku(YakuName::Honchantaiyao);
        }
        if self.ryanpeko() {
            self.add_yaku(YakuName::Ryanpeko, 3, 0, menzen);
            self.delete_yaku(YakuName::Ipeko);
        }

        if self.chiniso() {
            self.add_yaku(YakuName::Chiniso, 6, 5, menzen);
            self.delete_yaku(YakuName::Honiso);
        }

    }

    fn add_yaku(&mut self, yaku_name: YakuName, menzen_fan: usize, kui_fan: usize, menzen: bool) {
        if menzen {
            if menzen_fan > 0 {
                self.yakus.push(Yaku::new(yaku_name, menzen_fan));
            }
        } else {
            if kui_fan > 0 {
                self.yakus.push(Yaku::new(yaku_name, kui_fan));
            }
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
                    || (e.mentsu_type == MentsuType::Kantsu && e.visibility == VisibilityType::An)
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
        self.all_pais.iter().all(|e| e.is_green())
    }

    fn chinroto(&self) -> bool {
        self.all_pais.iter().all(|e| (e.number == 1 || e.number == 9) && e.pai_type != PaiType::JIHAI)
    }

    pub fn num_sangenpais(&self) -> usize {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus
                .iter()
                .filter(|e| e.pais[0].is_sangenpai())
                .count()
        } else {
            0
        }
    }

    fn daisushi(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus.iter().filter(|x| x.pais[0].is_wind()).count() == 4
        } else {
            false
        }
    }
    fn shosushi(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus.iter().filter(|x| x.pais[0].is_wind()).count() == 3
                && c.head.pais[0].is_wind()
        } else {
            false
        }
    }
    fn churenpoton(&self) -> bool {
        if !self.is_menzen() {
            return false;
        }
        let target_type = self.all_pais[0].pai_type;
        if !self.all_pais.iter().all(|x| x.pai_type == target_type) {
            return false;
        }
        
        let mut all_num: Vec<usize> = self.all_pais.iter().map(|x| x.number).collect();
        all_num.sort();
        // println!("ap:{:?}", all_num);
        for compare in vec![
            vec![1,1,1,1,2,3,4,5,6,7,8,9,9,9],
            vec![1,1,1,2,2,3,4,5,6,7,8,9,9,9],
            vec![1,1,1,2,3,3,4,5,6,7,8,9,9,9],
            vec![1,1,1,2,3,4,4,5,6,7,8,9,9,9],
            vec![1,1,1,2,3,4,5,5,6,7,8,9,9,9],
            vec![1,1,1,2,3,4,5,6,6,7,8,9,9,9],
            vec![1,1,1,2,3,4,5,6,7,7,8,9,9,9],
            vec![1,1,1,2,3,4,5,6,7,8,8,9,9,9],
            vec![1,1,1,2,3,4,5,6,7,8,9,9,9,9],
        ] {
            if all_num.iter().eq(compare.iter()) {
                return true;
            }
        }
        false
    }
    fn pinfu(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            if let Some(m) = &self.machi {
                match m {
                    Machi::Ryanmen => {
                            return c.mentsus
                            .iter()
                            .filter(|x| x.mentsu_type == MentsuType::Syuntsu)
                            .count()
                            == 4
                            && self.fanpai_fan(c.head.pais[0]) == 0
                    },
                    _ => {}
                }
            }
        }
        return false
    }

    fn tanyaochu(&self) -> bool {
        self.all_pais.iter().all(|x| x.is_yaochu() == false)
    }

    fn ipeko(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            
            for i in 0..c.mentsus.len() {
                for j in 0..c.mentsus.len() {
                    if i == j {
                        continue;
                    }
                    if c.mentsus[i].mentsu_type != MentsuType::Syuntsu {
                        continue
                    }
                    if c.mentsus[j].mentsu_type != MentsuType::Syuntsu {
                        continue
                    }
                    // println!("{:?},{:?}", c.mentsus[i].pais[0], c.mentsus[j].pais[0]);
                    if c.mentsus[i].pais[0].is_same_symbol(c.mentsus[j].pais[0]) {
                        return true;
                    }
                }
            }
            false
        } else {
            false
        }
    }

    fn bakaze(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            
            for mentsu in &c.mentsus {
                if mentsu.pais[0].is_same_symbol(self.yaku_info.bakaze) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }
    fn jikaze(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus.iter().any(|x| {
                (x.mentsu_type == MentsuType::Kantsu || x.mentsu_type == MentsuType::Kotsu)
                    && x.pais[0].is_same_symbol(self.yaku_info.jikaze)
            })
        } else {
            false
        }
    }

    fn sanshokudojun(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            let shuntsus: Vec<&Mentsu> = c
                .mentsus
                .iter()
                .filter(|x| x.mentsu_type == MentsuType::Syuntsu)
                .collect();

            if shuntsus.len() < 3 {
                return false;
            }
            for i in 0..shuntsus.len() {
                let target_shuntsu = shuntsus[i];
                let pai_number = target_shuntsu.pais[0].number;
                if shuntsus
                    .iter()
                    .any(|x| x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::MANZU)
                    && shuntsus.iter().any(|x| {
                        x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::PINZU
                    })
                    && shuntsus.iter().any(|x| {
                        x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::SOUZU
                    })
                {
                    return true;
                }
            }
        }
        false
    }
    fn sanshokudoko(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            let kotsus: Vec<&Mentsu> = c
                .mentsus
                .iter()
                .filter(|x| {
                    x.mentsu_type == MentsuType::Kotsu || x.mentsu_type == MentsuType::Kantsu
                })
                .collect();

            if kotsus.len() < 3 {
                return false;
            }
            for i in 0..kotsus.len() {
                let target_shuntsu = kotsus[i];
                let pai_number = target_shuntsu.pais[0].number;
                if kotsus
                    .iter()
                    .any(|x| x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::MANZU)
                    && kotsus.iter().any(|x| {
                        x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::PINZU
                    })
                    && kotsus.iter().any(|x| {
                        x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::SOUZU
                    })
                {
                    return true;
                }
            }
        }
        false
    }

    fn ikkitsukan(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            let shuntsus: Vec<&Mentsu> = c
                .mentsus
                .iter()
                .filter(|x| x.mentsu_type == MentsuType::Syuntsu)
                .collect();

            if shuntsus.len() < 3 {
                return false;
            }
            for mps in vec![PaiType::MANZU, PaiType::PINZU, PaiType::SOUZU] {
                if shuntsus
                    .iter()
                    .any(|x| x.pais[0].number == 1 && x.pais[0].pai_type == mps)
                    && shuntsus
                        .iter()
                        .any(|x| x.pais[0].number == 4 && x.pais[0].pai_type == mps)
                    && shuntsus
                        .iter()
                        .any(|x| x.pais[0].number == 7 && x.pais[0].pai_type == mps)
                {
                    return true;
                }
            }
        }
        false
    }

    fn honchantaiyao(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus
                .iter()
                .all(|x| x.pais.iter().any(|p| p.is_yaochu()))
                && c.head.pais.iter().any(|p| p.is_yaochu())
        }else{
            false
        }
        
    }

    fn toitoiho(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus
                .iter()
                .all(|x| x.mentsu_type == MentsuType::Kotsu || x.mentsu_type == MentsuType::Kantsu)
        } else {
            false
        }
    }

    fn honroto(&self) -> bool {
        self.all_pais.iter().all(|x| x.is_yaochu())
    }

    fn shosangen(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus
                .iter()
                .filter(|x| x.pais[0].is_sangenpai())
                .count()
                == 2
                && c.head.pais[0].is_sangenpai()
        } else {
            false
        }
    }

    fn honiso(&self) -> bool {
        for mps in vec![PaiType::MANZU, PaiType::PINZU, PaiType::SOUZU] {
            if self.all_pais.iter().all(|x| x.pai_type == mps || x.pai_type == PaiType::JIHAI) {
                return true;
            }
        }
        false
    }
    
    fn junchantaiyao(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            if c.head.pais[0].pai_type == PaiType::JIHAI {
                return false;
            }
            if !c.head.pais[0].is_yaochu() {
                return false;
            }

            for mentsu in &c.mentsus {
                if mentsu.pais
                    .iter()
                    .any(|x| (x.number == 1 || x.number == 9) 
                        && !x.is_jihai())
                    == false {
                    return false;
                }
            }
            return true;
    
        } else {
            false
        }
        

    }

    fn ryanpeko(&self) -> bool {
        
        if let Combination::Normal(c) = &self.combination {
            if c.mentsus
                .iter()
                .all(|x| x.mentsu_type == MentsuType::Syuntsu) 
                == false {
                return false
            }
            
            for i in 0..c.mentsus.len() {
                let mut ok = false;
                for j in 0..c.mentsus.len() {
                    if i == j {
                        continue
                    }
                    let m1 = &c.mentsus[i];
                    let m2 = &c.mentsus[j];

                    if m1.pais[0].is_same_symbol(m2.pais[0]) {
                        ok = true;
                    }
                }
                if ok == false {
                    return false;
                }
            }
            return true;
        }
        false
    }

    fn chiniso(&self) -> bool {
        if self.all_pais[0].is_jihai(){
            return false;
        }
        let target_type = self.all_pais[0].pai_type;
        self.all_pais.iter().all(|x| x.pai_type == target_type)
    }


    fn fanpai_fan(&self, pai: Pai) -> usize {
        if pai.is_sangenpai() {
            return 1;
        }
        let mut fan = 0;
        if pai.is_same_symbol(self.yaku_info.jikaze) {
            fan += 1;
        }
        if pai.is_same_symbol(self.yaku_info.bakaze) {
            fan += 1;
        }
        return fan;
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
        let taken = Pai::new(0);
        let taken_position = Some(TakenPosition::Head);

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination,taken_position);

        assert_eq!(candidate.num_sangenpais(), 1);
        println!("num_sangenpais():{}", candidate.num_sangenpais());
    }

    #[test]
    fn test_hora_candidate() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 0);
        let mentsus = vec![
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 33),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 3),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 4),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 5),
        ];
        let taken = Pai::new(0);
        let taken_position = Some(TakenPosition::Head);

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination, taken_position);

        assert!(candidate.yakus.contains(&Yaku::new(YakuName::Suanko, 100)));
    }

    #[test]
    fn test_sukantsu() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 0);
        let mentsus = vec![
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 33),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 3),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 4),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::Min, 5),
        ];
        let taken = Pai::new(0);
        let taken_position = Some(TakenPosition::Head);

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination, taken_position);

        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Sukantsu, 100)));
    }

    #[test]
    fn test_yakumans_tsuiso() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 27);
        let mentsus = vec![
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 28),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 29),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 30),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::Min, 33),
        ];
        let taken = Pai::new(27);
        let taken_position = Some(TakenPosition::Head);

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        
        let candidate = get_hora_candidate(taken, all_pais, furos, combination, taken_position);

        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Sukantsu, 100)));
        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Shosushi, 100)));
        assert!(candidate.yakus.contains(&Yaku::new(YakuName::Tsuiso, 100)));
    }
    #[test]
    fn test_daisangen() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 27);
        let mentsus = vec![
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 32),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::Min, 31),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 33),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::Min, 1),
        ];
        let taken = Pai::new(27);
        let taken_position = Some(TakenPosition::Head);

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination, taken_position);

        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Sukantsu, 100)));
        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Daisangen, 100)));
        assert!(candidate.yakus.contains(&Yaku::new(YakuName::Tsuiso, 100)) == false);
    }

    #[test]
    fn test_sanshokudojun() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 27);
        let mentsus = vec![
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::An, 1),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::Min, 10),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::An, 19),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::Min, 1),
        ];
        let taken = Pai::new(1);
        let taken_position = Some(TakenPosition::Mentsu(0));

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = vec![
            Furo::new(
                FuroType::CHI,
                Some(Pai::new(10)),
                Pai::new_by_vec(vec![11, 12]),
            ),
            Furo::new(
                FuroType::CHI,
                Some(Pai::new(1)),
                Pai::new_by_vec(vec![2, 3]),
            ),
        ];

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination, taken_position);

        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Sanshokudojun, 1)));
    }

    #[test]
    fn test_sanshokudojun_not() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 27);
        let mentsus = vec![
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::An, 1),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::Min, 10),
            Mentsu::new(MentsuType::Kotsu, VisibilityType::An, 19),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::Min, 1),
        ];
        let taken = Pai::new(19);
        let taken_position = Some(TakenPosition::Mentsu(2));

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = vec![
            Furo::new(
                FuroType::CHI,
                Some(Pai::new(10)),
                Pai::new_by_vec(vec![11, 12]),
            ),
            Furo::new(
                FuroType::CHI,
                Some(Pai::new(1)),
                Pai::new_by_vec(vec![2, 3]),
            ),
        ];

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination, taken_position);

        assert!(
            candidate
                .yakus
                .contains(&Yaku::new(YakuName::Sanshokudojun, 1))
                == false
        );
    }
    #[test]
    fn test_ikkitsukan() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 27);
        let mentsus = vec![
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::An, 0),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::Min, 3),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::An, 6),
            Mentsu::new(MentsuType::Syuntsu, VisibilityType::Min, 1),
        ];
        let taken = Pai::new(0);
        let taken_position = Some(TakenPosition::Mentsu(0));

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = vec![
            Furo::new(
                FuroType::CHI,
                Some(Pai::new(8)),
                Pai::new_by_vec(vec![6, 7]),
            ),
            Furo::new(
                FuroType::CHI,
                Some(Pai::new(0)),
                Pai::new_by_vec(vec![1, 2]),
            ),
        ];

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination, taken_position);

        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Ikkitsukan, 1)));
    }

    
    fn get_hora_candidate(
        taken: Pai,
        all_pais: Vec<Pai>,
        furos: Vec<Furo>,
        combination: Combination,
        taken_position:Option<TakenPosition>
    ) -> HoraCandidate {
        let hora_yaku_information = get_hora_yaku_information();
        

        let candidate = HoraCandidate::new(
            taken,
            furos,
            all_pais,
            hora_yaku_information,
            combination,
            taken_position,
        );
        candidate
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
