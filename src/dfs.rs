//!
use crate::furo::Furo;
use crate::mentsu::{Mentsu, MentsuType, VisibilityType};
use crate::pai::Pai;
/// # ある手牌から深さ優先探索を行い和了点数が到達可能かを判定する
///
use crate::shanten_analysis::calc;
use crate::tenpai_analysis::{FixedHoraPattern, HoraPattern};
use std::fmt;

#[derive(Clone)]
pub struct DfsCandidate {
    tehai_nums: [usize; 34],
    furos: Vec<Furo>,
    target_shanten: i8,
    current_shanten: i8,
    target_depth: i8,
    current_depth: i8,
}

impl DfsCandidate {
    pub fn new(
        tehai_nums: [usize; 34],
        furos: Vec<Furo>,
        target_shanten: i8,
        current_shanten: i8,
        target_depth: i8,
        current_depth: i8,
    ) -> Self {
        Self {
            tehai_nums: tehai_nums.clone(),
            furos: furos.clone(),
            target_shanten,
            current_shanten,
            target_depth,
            current_depth,
        }
    }
}

impl fmt::Debug for DfsCandidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m: String = "".to_string();
        for i in 0..9 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            m = m + n_str;
        }
        let mut p: String = "".to_string();
        for i in 9..18 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            p = p + n_str;
        }

        let mut s: String = "".to_string();
        for i in 18..27 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            s = s + n_str;
        }

        let mut z: String = "".to_string();
        for i in 27..34 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            z = z + n_str;
        }
        write!(
            f,
            "tehais:{},{},{},{}\nfuros:{:?}\ntarget depth:{}\ncurrent depth:{}\ncurrent shanten:{}",
            m, p, s, z, self.furos, self.target_depth, self.current_depth, self.current_shanten,
        )
    }
}

fn calc_dfs_14(tehai: &[usize; 34], furos: Vec<Furo>, depth: i8) -> i8 {
    let mut nodes = Vec::new();
    let mut horas = Vec::new();

    let shanten = calc(&tehai, furos.len() as i8);
    let hora_shanten = -1;

    let candidate = DfsCandidate::new(
        tehai.clone(),
        furos.clone(),
        hora_shanten,
        shanten,
        depth,
        0,
    );

    let mut node_count = 0;
    nodes.push(candidate);

    loop {
        let element = nodes.pop();
        if let None = element {
            break;
        } else if let Some(e) = element {
            let shanten = e.current_shanten;
            // println!("shanten:{:?}", shanten);

            if shanten == -1 {
                horas.push(e);
                // 和了したらそれ以降は展開しない
                // println!("{:?}", e);

                // points, yakus = calc_horas();
                continue;
            } else if e.current_depth >= e.target_depth {
                // 深さが指定深さに到達したらそれ以降は展開しない
                continue;
            } else if e.current_shanten - e.target_shanten > e.target_depth - e.current_depth {
                // 残り探索深さがtargetシャンテンに到達不可能になったら探索打ち切り
                continue;
            }
            // 手牌を変更する
            for i in 0..34 {
                // 減少
                if e.tehai_nums[i] == 0 {
                    continue;
                }

                for j in 0..34 {
                    // 増加
                    if e.tehai_nums[j] == 4 {
                        continue;
                    }
                    if i == j {
                        continue;
                    }

                    let mut new_tehai_nums = e.tehai_nums.clone();
                    new_tehai_nums[i] -= 1;
                    new_tehai_nums[j] += 1;

                    let new_shanten = calc(&new_tehai_nums, e.furos.len() as i8);
                    if new_shanten > shanten {
                        continue;
                    }
                    // println!("{},{}",i,j);

                    let new_candidate = DfsCandidate::new(
                        new_tehai_nums,
                        furos.clone(),
                        e.target_shanten,
                        new_shanten,
                        e.target_depth,
                        e.current_depth + 1,
                    );
                    nodes.push(new_candidate);
                    node_count += 1;
                }
            }
        }
    }

    println!("node_count:{}", node_count);
    println!("hora_count:{}", horas.len());
    // for h in horas.iter() {
    //     println!("{:?}",h);
    // }
    let i8mod = (node_count % 128) as i8;
    i8mod
}



fn cut_mentsu(
    mut tehai: [usize; 34],
    free_pai_num: i8,
    head_num: i8,
    mentsu_num: i8,
    mut current_hora_pattern: HoraPattern,
    mut result_hora_patterns: Vec<HoraPattern>,
    start_id: usize,
) -> Vec<HoraPattern> {
    assert!(free_pai_num < 3);

    // if complete, append result and return
    // this path is no mentsu
    // check rest_pai + free_pai can make mentsu.
    let rest_pai_num: usize = tehai.iter().sum();

    if rest_pai_num == 0 {
        println!("rest_pai_num:{:?}", rest_pai_num);
        result_hora_patterns.push(current_hora_pattern);
        println!("result_hora_patterns:{:?}", result_hora_patterns);
        return result_hora_patterns;
    }

    // cut head
    if let None = current_hora_pattern.head {
        for i in start_id..34 {
            if tehai[i] >= 2 {
                tehai[i] -= 2;
                current_hora_pattern.head =
                    Some(Mentsu::new(MentsuType::Head, VisibilityType::An, i));
                println!("head found:{:?}", i);
                result_hora_patterns = cut_mentsu(
                    tehai,
                    free_pai_num,
                    head_num,
                    mentsu_num,
                    current_hora_pattern.clone(),
                    result_hora_patterns.clone(),
                    0,
                );
                tehai[i] += 2;
            }
        }
    }

    // cut syuntsu
    for i in start_id..27 {
        if tehai[i] >= 1 && tehai[i + 1] >= 1 && tehai[i + 2] >= 1 {
            tehai[i] -= 1;
            tehai[i + 1] -= 1;
            tehai[i + 2] -= 1;
            let new_mentsu = Mentsu::new(MentsuType::Syuntsu, VisibilityType::An, i);
            current_hora_pattern.mentsus.push(new_mentsu);
            result_hora_patterns = cut_mentsu(
                tehai,
                free_pai_num,
                head_num,
                mentsu_num,
                current_hora_pattern.clone(),
                result_hora_patterns.clone(),
                i,
            );
            current_hora_pattern.mentsus.pop();
            tehai[i] += 1;
            tehai[i + 1] += 1;
            tehai[i + 2] += 1;
        }
    }

    // add kotsu
    for i in start_id..34 {
        if tehai[i] >= 3 {
            tehai[i] -= 3;
            let new_mentsu = Mentsu::new(MentsuType::Kotsu, VisibilityType::An, i);
            current_hora_pattern.mentsus.push(new_mentsu);
            result_hora_patterns = cut_mentsu(
                tehai,
                free_pai_num,
                head_num,
                mentsu_num,
                current_hora_pattern.clone(),
                result_hora_patterns.clone(),
                i,
            );
            current_hora_pattern.mentsus.pop();
            tehai[i] += 3;
        }
    }

    // if need head and mentsu can be builded, append hora pattern
    if free_pai_num > 0 {
        // let mentsu = search_mentsu(tehai, free_pai_num, );
    }

    result_hora_patterns
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn get_tehai() -> [usize; 34] {
        let tehai: [usize; 34] = [
            0, 1, 0, 1, 1, 2, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0,
        ];
        tehai
    }

    #[test]
    fn calc_dfs_works() {
        let tehai: [usize; 34] = get_tehai();
        let shanten = calc(&tehai, 0);
        println!("initial shanten:{}", shanten);
        let start = Instant::now();
        

        for _ in 0..1 {
            let furos = vec![];
            let a = calc_dfs_14(&tehai, furos, 3);
            assert!(a != 1);
        }
        let end = start.elapsed();
        println!(
            "passed {}.{:03}sec",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    }

    #[test]
    fn test_cut_mentsu() {
        let tehai = get_tehai();
        let current_hora_pattern = HoraPattern::new();
        let result_hora_patterns = Vec::new();
        let result = cut_mentsu(
            tehai,
            0,
            0,
            0,
            current_hora_pattern,
            result_hora_patterns,
            0,
        );
        println!("test_cut_mentsu result:{:?}", result);
    }
}
