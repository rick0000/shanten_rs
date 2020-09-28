use crate::furo::{Furo, FuroType};
use crate::mentsu::{Mentsu, MentsuType, VisibilityType};
use crate::pai::{Pai, PaiType};
use crate::tenpai_analysis::{Combination, FixedHoraPattern, HoraType, calc_combination};
use crate::yaku::{Yaku, YakuName};
use crate::hora_candidate::{HoraYakuInformation, HoraCandidate, TakenPosition};
use crate::point_datam::PointDatam;
use std::fmt;

// #[derive(Debug)]
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

impl fmt::Debug for Hora {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "tehai:{:?}\ntaken:{:?}\nfuro:{:?}\npoints:{:?}\nyakus:{:?}",
            self.tehais,
            self.taken,
            self.furos,
            self.best_candidate.points,
            self.best_candidate.yakus,
        )
    }
}

impl Hora {
    pub fn new(
        tehais: Vec<Pai>,
        furos: Vec<Furo>,
        taken: Pai,
        all_pais: Vec<Pai>,
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
        
        let combinations:Vec<Combination> = calc_combination(
            taken,
            &tehais,
            &furos,
        );
        if combinations.len() == 0 {
            panic!("not hora!");
        }

        let mut candidates: Vec<HoraCandidate> = vec![];
        for combination in combinations {
            candidates.extend(Self::get_candidate(
                &combination,
                taken,
                furos.clone(),
                all_pais.clone(),
                hora_yaku_information.clone(),        
            ));
        }
        
        
        let best_candidate = candidates
            .clone()
            .into_iter()
            .max_by_key(|x| x.get_priority())
            .unwrap();

        Self {
            tehais,
            furos,
            taken,
            hora_yaku_information,
            free_pais,
            all_pais,
            candidates,
            best_candidate,
        }
    }
    

    fn get_candidate(
        combination:&Combination,
        taken:Pai,
        furos:Vec<Furo>,
        all_pais:Vec<Pai>,
        hora_yaku_information:HoraYakuInformation,
    ) -> Vec<HoraCandidate> {

        let mut candidates:Vec<HoraCandidate> = vec![];
        match combination {
            // check taken position

            // kokushi and chitoitsu => None
            Combination::Kokushimuso 
            | Combination::Chitoitsu => {
                let candidate = HoraCandidate::new(
                    taken,
                    furos.clone(),
                    all_pais.clone(),
                    hora_yaku_information.clone(),
                    combination.clone(),
                    None,
                );
                candidates.push(candidate);
            },
            Combination::Normal(c) => {
                // head check
                if c.head.pais[0].is_same_symbol(taken) {
                    let candidate = HoraCandidate::new(
                        taken,
                        furos.clone(),
                        all_pais.clone(),
                        hora_yaku_information.clone(),
                        combination.clone(),
                        Some(TakenPosition::Head),
                    );
                    candidates.push(candidate);
                }

                // mentsu check
                for (index, m) in c.mentsus.iter().enumerate() {
                    if m.is_furo {
                        continue;
                    }
                    let syuntsu_eq = m.mentsu_type == MentsuType::Syuntsu
                    && m.id <= taken.id && taken.id <= m.id + 2;
                    let kotsu_eq = taken.id == m.id;
                    if syuntsu_eq || kotsu_eq {
                        let candidate = HoraCandidate::new(
                            taken,
                            furos.clone(),
                            all_pais.clone(),
                            hora_yaku_information.clone(),
                            combination.clone(),
                            Some(TakenPosition::Mentsu(index)),
                        );    
                        candidates.push(candidate);
                    }
                }
            },
        }
        candidates         
    }

    fn count_doras(all_pais: &Vec<Pai>, doras: Vec<Pai>) -> usize {
        let mut num = 0;
        // println!("num doras {:?}, {:?}", all_pais, doras);
        let mut num_doras = [0;34];
        for d in &doras{
            num_doras[d.id] += 1;
        }

        for p in all_pais {
            num += num_doras[p.id];
        }
        num
    }

    pub fn get_pointdatam(&self) -> PointDatam {
        self.best_candidate.get_pointdatam()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hora(){
        
        let tehais: Vec<Pai> = Pai::new_by_vec(vec![0,1,2,3,4,5,6,7,8,20,20,20,21,]);
        let furos: Vec<Furo> = vec![];
        let taken: Pai = Pai::new(22);
        let oya: bool = false;
        let hora_type: HoraType = HoraType::Ron;
        let first_turn: bool = false;
        let doras: Vec<Pai> = vec![];
        let uradoras: Vec<Pai> = vec![];
        let reach: bool = false;
        let double_reach: bool = false;
        let ippatsu: bool = false;
        let rinshan: bool = false;
        let chankan: bool = false;
        let haitei: bool = false;
        let bakaze: Pai = Pai::new_str("E");
        let jikaze: Pai = Pai::new_str("S");
        
        let mut all_pais = tehais.clone();
        all_pais.push(taken);

        let hora = Hora::new(
            tehais,
            furos,
            taken,
            all_pais,
            oya,
            hora_type,
            first_turn,
            doras,
            uradoras,
            reach,
            double_reach,
            ippatsu,
            rinshan,
            chankan,
            haitei,
            bakaze,
            jikaze,
        );
        println!("{:?}",hora);
    }
}
