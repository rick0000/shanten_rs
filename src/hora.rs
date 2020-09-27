use crate::furo::{Furo, FuroType};
use crate::mentsu::{Mentsu, MentsuType, VisibilityType};
use crate::pai::{Pai, PaiType};
use crate::tenpai_analysis::{Combination, FixedHoraPattern, HoraType, calc_combination};
use crate::yaku::{Yaku, YakuName};
use crate::hora_candidate::{HoraYakuInformation, HoraCandidate, TakenPosition};

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
        all_pais.extend(free_pais.clone());
        for furo in &furos {
            all_pais.extend(furo.pais.clone());
        }

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
        for c in &candidates {
            println!("{:?} @ HORA", c);
        }
        
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

}
