
use crate::mentsu_tartsu_num::MentsuTartsuNumCalclator;
use crate::mentsu_tartsu_num::MentsuTartsuNum;


const SHANTEN_MAX_NORMAL : i8 = 8;
const SHANTEN_MAX_KOKUSHI : i8 = 13;
const SHANTEN_MAX_CHITOITSU : i8 = 6;
const SYU_NUM_CHITOITSU : i8 = 7;
const KOKUSHI_PAIS : [usize;13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];


pub fn calc(tehai: &[usize; 34], furo_num: i8) -> i8 {
    if furo_num == 0 {
        calc_shanten_normal(tehai, 0).
        min(calc_shanten_kokushi(tehai)).
        min(calc_shanten_chitoitsu(tehai))    
    } else {
        calc_shanten_normal(tehai, furo_num)
    }
}

pub fn calc_all(tehai: &[usize; 34], furo_num: i8) -> [i8; 3] {
    if furo_num == 0 {
        [
            calc_shanten_normal(tehai, 0),
            calc_shanten_kokushi(tehai),
            calc_shanten_chitoitsu(tehai),
        ]    
    } else {
        [
            calc_shanten_normal(tehai, furo_num),
            SHANTEN_MAX_KOKUSHI,
            SHANTEN_MAX_CHITOITSU,
        ]
    }
}



fn calc_shanten_kokushi(tehai: &[usize; 34]) -> i8 {
    let mut type_num = 0;
    let mut head_num = 0;
    for i in 0..KOKUSHI_PAIS.len() {
        if tehai[KOKUSHI_PAIS[i]] >= 1 {
            type_num += 1;

            if tehai[KOKUSHI_PAIS[i]] >= 2 {
                head_num += 1;
            }
        }
    }

    if head_num > 0 {
        13 - type_num - 1
    }
    else {
        13 - type_num
    }
}

fn calc_shanten_chitoitsu(tehai: &[usize; 34]) -> i8 {
    let mut syu_num = 0;
    let mut toitsu = 0;

    for i in 0..tehai.len() {
        if tehai[i] >= 1{
            syu_num += 1;
            if tehai[i] >= 2{
                toitsu += 1;
            }
        }
    }
    if syu_num < SYU_NUM_CHITOITSU {
        SHANTEN_MAX_CHITOITSU - toitsu + (7-syu_num)
    } else {
        SHANTEN_MAX_CHITOITSU - toitsu
    }    
}

fn calc_shanten_normal(tehai: &[usize; 34], furo_num: i8) -> i8 {
    
    let mut tehai_copy = tehai.clone();
    let mut min_shanten = calc_shanten_normal_inner(&tehai_copy, furo_num);
    // println!("head no, minshanten {}", min_shanten);

    for i in 0..tehai_copy.len() {
        if tehai_copy[i] >= 2 {
            tehai_copy[i] -= 2;
            let temp_shanten = calc_shanten_normal_inner(&tehai_copy, furo_num) - 1;
            if min_shanten > temp_shanten{
                // println!("head {}, minshanten {}-> {}", i, min_shanten, temp_shanten);
                min_shanten = temp_shanten;
            }
            tehai_copy[i] += 2;
        }
    }
    min_shanten
}


fn calc_shanten_normal_inner(tehai: &[usize; 34], furo_num: i8) -> i8 {
    let mut min_shanten = SHANTEN_MAX_NORMAL;

    let man = [tehai[0], tehai[1], tehai[2], tehai[3], tehai[4], tehai[5], tehai[6], tehai[7], tehai[8]];
    let pin = [tehai[9], tehai[10], tehai[11], tehai[12], tehai[13], tehai[14], tehai[15], tehai[16], tehai[17]];
    let sou = [tehai[18], tehai[19], tehai[20], tehai[21], tehai[22], tehai[23], tehai[24], tehai[25], tehai[26]];
    let ji = [tehai[27], tehai[28], tehai[29], tehai[30], tehai[31], tehai[32], tehai[33]];

    let man_analysis:MentsuTartsuNum = MENTSU_TARTSU_NUM_CALCLATOR.calc(&man);
    let pin_analysis:MentsuTartsuNum = MENTSU_TARTSU_NUM_CALCLATOR.calc(&pin);
    let sou_analysis:MentsuTartsuNum = MENTSU_TARTSU_NUM_CALCLATOR.calc(&sou);
    let ji_analysis:[i8;2] = MENTSU_TARTSU_NUM_CALCLATOR.calc_ji(&ji);

    
    // println!("------");
    // println!("man{:?}", man);
    // println!("{:?}", man_analysis);
    
    // println!("pin{:?}", pin);
    // println!("{:?}", pin_analysis);
    // println!("sou{:?}", sou);
    // println!("{:?}", sou_analysis);
    // println!("ji{:?}", ji);
    // println!("{:?}", ji_analysis);

    for m in &[(man_analysis.mentsu_coef2, man_analysis.tartsu_coef2), (man_analysis.mentsu_coef10, man_analysis.tartsu_coef10)] {
        for p in &[(pin_analysis.mentsu_coef2, pin_analysis.tartsu_coef2), (pin_analysis.mentsu_coef10, pin_analysis.tartsu_coef10)]{
            for s in &[(sou_analysis.mentsu_coef2, sou_analysis.tartsu_coef2), (sou_analysis.mentsu_coef10, sou_analysis.tartsu_coef10)] {
                let mentsu_num = m.0 + p.0 + s.0 + ji_analysis[0];
                let mut tartsu_num = m.1 + p.1 + s.1 + ji_analysis[1];
                if (mentsu_num + tartsu_num + furo_num) > 4 {
                    tartsu_num = 4 - mentsu_num - furo_num;
                }
                // println!("m{:?} p{:?} s{:?} {} {} {} {}",m,p,s,SHANTEN_MAX_NORMAL, mentsu_num, furo_num, tartsu_num);
                let temp_shanten = SHANTEN_MAX_NORMAL - mentsu_num * 2 - furo_num * 2 - tartsu_num;
                if min_shanten > temp_shanten{
                    
                    min_shanten = temp_shanten;
                }
            }
        }
    }

    min_shanten
}


lazy_static!{
    pub static ref MENTSU_TARTSU_NUM_CALCLATOR: MentsuTartsuNumCalclator = {
        match MentsuTartsuNumCalclator::new() {
            Err(_) => panic!("fail to initialize!"),
            Ok(e) => e
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_works() {
        calc(&[
            4,4,4,2,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
            ], 1);
    }

    #[test]
    fn chitoitsu_calc_works() {
        let s = calc_all(&[
            2,0,2,0,2,0,2,0,0,
            0,0,0,2,0,2,0,2,0,
            0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,
            ], 0);

        assert_eq!(3, s[0]);
        assert_eq!(11, s[1]);
        assert_eq!(-1, s[2]);
    }

    #[test]
    fn chitoitsu_4_calc_works() {
        let s = calc_all(&[
            2,0,0,0,2,0,2,0,0,
            0,0,0,2,0,0,0,4,0,
            0,0,0,0,0,0,0,0,2,
            0,0,0,0,0,0,0,
            ], 0);

        assert_eq!(2, s[0]);
        assert_eq!(10, s[1]);
        assert_eq!(1, s[2]);
    }
    #[test]
    fn chitoitsu_many_type_calc_works() {
        let s = calc_all(&[
            2,0,0,0,2,0,2,0,0,
            0,0,0,2,0,0,0,3,0,
            0,0,0,0,0,1,0,0,2,
            0,0,0,0,0,0,0,
            ], 0);

        assert_eq!(2, s[0]);
        assert_eq!(10, s[1]);
        assert_eq!(0, s[2]);
    }
}
