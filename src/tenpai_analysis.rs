//!
/// # テンパイ形解析を行う
/// 
///
///
use std::fmt;
use crate::pai::Pai;
use crate::furo::{Furo, FuroType};
use crate::shanten_analysis::calc_all;
use crate::mentsu::{Mentsu, MentsuType, VisibilityType};

pub enum WaitingType {
    Tanki,
    Kanchan,
    Penchan,
    Ryanmen,
    Shanpon,
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
    pub fn add_furos(&mut self, furo_mentsus: Vec<Mentsu>) {
        self.mentsus.extend(furo_mentsus);
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

fn calc_combination(taken: Pai, tehais: &Vec<Pai>, furos: &Vec<Furo>) -> Vec<Combination> {
    let mut combinations = Vec::new();
    let mut num_tehais = [0; 34];
    for t in tehais {
        num_tehais[t.id] += 1
    }
    num_tehais[taken.id] += 1;
    
    // それぞれの和了系のシャンテン数を取得する
    let shanten = calc_all(&num_tehais, furos.iter().count() as i8);
    let noraml_shanten = shanten[0];
    let kokushi_shanten = shanten[1];
    let chitoi_shanten = shanten[2];
    const HORA_SHANTEN:i8 = -1;

    if noraml_shanten == HORA_SHANTEN {
        let normal_hora_patterns = cut(
            num_tehais,
            furos,
        );
        let normal_combinations: Vec<Combination> = normal_hora_patterns.into_iter().map(|x| Combination::Normal(x)).collect();
        combinations.extend(normal_combinations);
    }
    if kokushi_shanten == HORA_SHANTEN {
        combinations.push(Combination::Kokushimuso);
    }
    if chitoi_shanten == HORA_SHANTEN {
        combinations.push(Combination::Chitoitsu);
    }
    combinations
}


fn cut(
    mut num_tehais: [usize; 34],
    mut furos: &Vec<Furo>,
) -> Vec<FixedHoraPattern>
{
    let mut result_hora_patterns = vec![];
    
    for i in 0..34 {
        if num_tehais[i] >= 2 {
            num_tehais[i] -= 2;
            let mut current_hora_pattern = FixedHoraPattern::new(
                Mentsu::new(MentsuType::Head, VisibilityType::An, i),
                vec!(),
            );
            // println!("head found:{:?}", i);
            result_hora_patterns.extend(cut_mentsu(
                num_tehais,
                current_hora_pattern,
                vec![],
                0,
            ));
            num_tehais[i] += 2;
        }
    };


    // append furos for all pattern.
    let mut furo_mentsu = vec![];
    for furo in furos {
        match furo.furo_type {
            FuroType::ANKAN => {
                furo_mentsu.push(
                    Mentsu::new(MentsuType::Kantsu, VisibilityType::An, furo.min_id)
                );
            }
            FuroType::CHI => {
                furo_mentsu.push(
                    Mentsu::new(MentsuType::Syuntsu, VisibilityType::Min, furo.min_id)
                );
            }
            FuroType::DAIMINKAN | FuroType::KAKAN => {
                furo_mentsu.push(
                    Mentsu::new(MentsuType::Kantsu, VisibilityType::Min, furo.min_id)
                );
            }
            FuroType::PON => {
                furo_mentsu.push(
                    Mentsu::new(MentsuType::Kotsu, VisibilityType::Min, furo.min_id)
                );
            }
        }
    }

    for pattern in result_hora_patterns.iter_mut() {
        pattern.add_furos(furo_mentsu.clone());
    }

    result_hora_patterns
}

fn cut_mentsu(
    mut num_tehais: [usize; 34],
    mut current_hora_pattern: FixedHoraPattern,
    mut result_hora_patterns: Vec<FixedHoraPattern>,
    start_id: usize,
) -> Vec<FixedHoraPattern> {

    // if complete, append result and return
    // this path is no mentsu
    // check rest_pai + free_pai can make mentsu.
    let rest_pai_num: usize = num_tehais.iter().sum();

    if rest_pai_num == 0 {
        // println!("rest_pai_num:{:?}", rest_pai_num);
        current_hora_pattern.mentsus.sort();
        result_hora_patterns.push(current_hora_pattern);
        // println!("result_hora_patterns:{:?}", result_hora_patterns);
        return result_hora_patterns;
    }

    // cut syuntsu
    for i in start_id..27 {
        if num_tehais[i] >= 1 && num_tehais[i + 1] >= 1 && num_tehais[i + 2] >= 1 {
            num_tehais[i] -= 1;
            num_tehais[i + 1] -= 1;
            num_tehais[i + 2] -= 1;
            let new_mentsu = Mentsu::new(MentsuType::Syuntsu, VisibilityType::An, i);
            current_hora_pattern.mentsus.push(new_mentsu);
            result_hora_patterns = cut_mentsu(
                num_tehais,
                current_hora_pattern.clone(),
                result_hora_patterns.clone(),
                i,
            );
            current_hora_pattern.mentsus.pop();
            num_tehais[i] += 1;
            num_tehais[i + 1] += 1;
            num_tehais[i + 2] += 1;
        }
    }

    // add kotsu
    for i in start_id..34 {
        if num_tehais[i] >= 3 {
            num_tehais[i] -= 3;
            let new_mentsu = Mentsu::new(MentsuType::Kotsu, VisibilityType::An, i);
            current_hora_pattern.mentsus.push(new_mentsu);
            result_hora_patterns = cut_mentsu(
                num_tehais,
                current_hora_pattern.clone(),
                result_hora_patterns.clone(),
                i,
            );
            current_hora_pattern.mentsus.pop();
            num_tehais[i] += 3;
        }
    }
    result_hora_patterns
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;


    #[test]
    fn test_combination() {
        let tehais = Pai::new_by_str_vec(vec!["2m","3m","4m","5m","6m","7m","8m","9m","1p","1p"]);
        let taken = Pai::new_str("1m");
        let furos = vec![
            Furo::new(FuroType::CHI, Some(Pai::new_str("1s")), Pai::new_by_str_vec(vec!["2s", "3s"])),
        ];
        
        let combinations = calc_combination(
            taken,
            &tehais,
            &furos,
        );
        for combination in combinations {
            println!("{:?}", combination);
        }
        
    }

    #[test]
    fn test_complex_combination() {
        let tehais = Pai::new_by_str_vec(vec!["2m","2m","2m","2m","3m","3m","3m","3m","4m","4m","4m","4m","5m"]);
        let mut taken = Pai::new_str("1m");
        let furos = vec![
        ];
        
        let _combinations = calc_combination(
            taken,
            &tehais,
            &furos,
        );
        let start = Instant::now();
        
        let mut taken = Pai::new_str("5m");
        
        let loop_num = 1000;
        for _ in 0..loop_num {
            let combinations = calc_combination(
                taken,
                &tehais,
                &furos,
            );    
        }
        
        let end = start.elapsed();

        println!(
            "passed {} micro sec in {} loops",
            end.subsec_nanos() / 1000,
            loop_num
        );
    }

}
