use crate::furo::{Furo, FuroType};
use crate::pai::{Pai, PaiType};
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

const YAKUMAN_FAN: usize = 100;
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
        if self.yaku_info.rinshan {
            self.add_yaku(YakuName::Rinshankaiho, 1, 1, menzen);
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
        }

    }

    fn add_yaku(&mut self, yaku_name: YakuName, menzen_fan: usize, kui_fan: usize, menzen: bool) {
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
        self.all_pais.iter().all(|e| e.is_green())
    }

    fn chinroto(&self) -> bool {
        self.all_pais.iter().all(|e| e.number == 1 || e.number == 9)
    }

    fn num_sangenpais(&self) -> usize {
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
        let target_type = self.all_pais[0].pai_type;
        if !self.all_pais.iter().all(|x| x.pai_type == target_type) {
            return false;
        }

        let all_num: Vec<usize> = self.all_pais.iter().map(|x| x.number).collect();
        for compare in vec![
            vec![4, 1, 1, 1, 1, 1, 1, 1, 3],
            vec![3, 2, 1, 1, 1, 1, 1, 1, 3],
            vec![3, 1, 2, 1, 1, 1, 1, 1, 3],
            vec![3, 1, 1, 2, 1, 1, 1, 1, 3],
            vec![3, 1, 1, 1, 2, 1, 1, 1, 3],
            vec![3, 1, 1, 1, 1, 2, 1, 1, 3],
            vec![3, 1, 1, 1, 1, 1, 2, 1, 3],
            vec![3, 1, 1, 1, 1, 1, 1, 2, 3],
            vec![3, 1, 1, 1, 1, 1, 1, 1, 4],
        ] {
            if all_num.iter().eq(compare.iter()) {
                return true;
            }
        }
        false
    }
    fn pinfu(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus
                .iter()
                .filter(|x| x.mentsu_type == MentsuType::Syuntsu)
                .count()
                == 4
                && c.head.pais[0].is_wind()
        } else {
            false
        }
    }

    fn tanyaochu(&self) -> bool {
        self.all_pais.iter().all(|x| x.is_yaochu() == false)
    }

    fn ipeko(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            for i in &c.mentsus {
                for j in &c.mentsus {
                    if i == j {
                        continue;
                    }
                    if i.pais[0].is_same_symbol(j.pais[0]) {
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
            c.mentsus.iter().any(|x| {
                (x.mentsu_type == MentsuType::Kantsu || x.mentsu_type == MentsuType::Kotsu)
                    && x.pais[0].is_same_symbol(self.yaku_info.bakaze)
            })
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
                if shuntsus.iter().any(|x| {
                        x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::MANZU
                    })
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
                .filter(|x| x.mentsu_type == MentsuType::Kotsu || x.mentsu_type == MentsuType::Kantsu)
                .collect();

            if kotsus.len() < 3 {
                return false;
            }
            for i in 0..kotsus.len() {
                let target_shuntsu = kotsus[i];
                let pai_number = target_shuntsu.pais[0].number;
                if kotsus.iter().any(|x| {
                        x.pais[0].number == pai_number && x.pais[0].pai_type == PaiType::MANZU
                    })
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
            c.mentsus.iter().all(|x| x.pais.iter().any(|p| p.is_yaochu()))
            && c.head.pais.iter().any(|p| p.is_yaochu())
        }else{
            false
        }
    }
    
    fn toitoiho(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus.iter().all(
                |x| x.mentsu_type == MentsuType::Kotsu 
                || x.mentsu_type == MentsuType::Kantsu)
        }else{
            false
        }
    }

    fn honroto(&self) -> bool {
        self.all_pais.iter().all(|x| x.is_yaochu())
    }

    fn shosangen(&self) -> bool {
        if let Combination::Normal(c) = &self.combination {
            c.mentsus.iter().filter(|x| x.pais[0].is_sangenpai()).count() == 2
                && c.head.pais[0].is_sangenpai()
        } else {
            false
        }
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

pub struct Hora {
    tehais: Vec<Pai>,
    furos: Vec<Furo>,
    taken: Pai,
    hora_yaku_information: HoraYakuInformation,

    free_pais: Vec<Pai>,
    all_pais: Vec<Pai>,


    candidates: Vec<HoraCandidate>,
    best_candidate: HoraCandidate,
}

impl Hora {
    pub fn new(
        tehais: Vec<Pai>,
        furos: Vec<Furo>,
        taken: Pai,

        oya: bool,
        hora_type: HoraType,
        first_turn: bool,
        doras: Vec<Pai>,
        uradoras: Vec<Pai>,
        reach: bool,
        double_reach: bool,
        ippatsu: bool,
        rinshan: bool,
        chankan: bool,
        haitei: bool,
        bakaze: Pai,
        jikaze: Pai,
    ) -> Self {

        let mut free_pais = tehais.clone();
        free_pais.push(taken);

        let mut all_pais: Vec<Pai> = vec![];

        let num_doras = Self::count_doras(&all_pais, doras);
        let num_uradoras = Self::count_doras(&all_pais, uradoras);
        let num_akadoras = all_pais.iter().filter(|x| x.is_red()).count();

        let hora_yaku_information = HoraYakuInformation {
            oya,
            hora_type,
            first_turn,
            num_doras,
            num_uradoras,
            num_akadoras,
            reach,
            double_reach,
            ippatsu,
            rinshan,
            chankan,
            haitei,
            bakaze,
            jikaze,    
        };


        
        let mut candidates: Vec<HoraCandidate> = vec![];
        let mut best_candidate: HoraCandidate = candidates.pop().unwrap();


        Self {
            tehais,
            furos,
            taken,
            hora_yaku_information,

            free_pais: free_pais,
            all_pais: all_pais,

            candidates: candidates,
            best_candidate: best_candidate,
        }

    }

    fn count_doras(all_pais:&Vec<Pai>, doras:Vec<Pai>) -> usize {
        let mut num = 0;
        for p in all_pais {
            if doras.contains(p) {
                num += 1;
            }
        }
        num
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

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        let taken = Pai::new(1);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination);

        assert_eq!(candidate.num_sangenpais(), 1);
        println!("num_sangenpais():{}", candidate.num_sangenpais());
    }

    #[test]
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

    #[test]
    fn test_sukantsu() {
        let head = Mentsu::new(MentsuType::Head, VisibilityType::An, 0);
        let mentsus = vec![
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 33),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 3),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::An, 4),
            Mentsu::new(MentsuType::Kantsu, VisibilityType::Min, 5),
        ];

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        let taken = Pai::new(1);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination);

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

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        let taken = Pai::new(27);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination);

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

        let mut all_pais = Vec::new();
        for mentsu in &mentsus {
            all_pais.extend(&mentsu.pais)
        }
        let furos = Vec::new();

        let pattern = FixedHoraPattern::new(head, mentsus);
        let combination = Combination::Normal(pattern);
        let taken = Pai::new(27);

        let candidate = get_hora_candidate(taken, all_pais, furos, combination);

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
        let taken = Pai::new(27);

        let mut candidate = get_hora_candidate(taken, all_pais, furos, combination);

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
        let taken = Pai::new(27);

        let mut candidate = get_hora_candidate(taken, all_pais, furos, combination);

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
        let taken = Pai::new(27);

        let mut candidate = get_hora_candidate(taken, all_pais, furos, combination);

        assert!(candidate
            .yakus
            .contains(&Yaku::new(YakuName::Ikkitsukan, 1)));
    }

    fn get_hora_candidate(
        taken: Pai,
        all_pais: Vec<Pai>,
        furos: Vec<Furo>,
        combination: Combination,
    ) -> HoraCandidate {
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
