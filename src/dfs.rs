//! 
/// # ある手牌から深さ優先探索を行い和了点数が到達可能かを判定する
/// 
use crate::shanten_analysis::calc;
use crate::furo::Furo;
use std::fmt;

#[derive(Clone)]
pub struct DfsCandidate {
    tehai_nums:[usize;34],
    furos:Vec<Furo>,
    target_shanten:i8,
    target_depth:i8,
    current_depth:i8,
}

impl DfsCandidate {
    pub fn new(
        tehai_nums:[usize;34], 
        furos:Vec<Furo>, 
        target_shanten:i8, 
        target_depth:i8,
        current_depth:i8,
    ) -> Self {
        Self {
            tehai_nums: tehai_nums.clone(),
            furos: furos.clone(),
            target_shanten,
            target_depth,
            current_depth,
        }
    }  
}

impl fmt::Debug for DfsCandidate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m:String = "".to_string();
        for i in 0..9 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            m = m + n_str;            
        }
        let mut p:String = "".to_string();
        for i in 9..18 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            p = p + n_str;            
        }

        let mut s:String = "".to_string();
        for i in 18..27 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            s = s + n_str;            
        }

        let mut z:String = "".to_string();
        for i in 27..34 {
            let n_str: &str = &self.tehai_nums[i].to_string();
            z = z + n_str;            
        }
        write!(f, "tehais:{},{},{},{}\nfuros:{:?}\ntarget depth:{}\ncurrent depth:{}", 
            m, p, s, z, 
            self.furos, 
            self.target_depth,
            self.current_depth,
        )
    }
}

fn calc_dfs_14(tehai: &[usize; 34], furos:Vec<Furo>, depth:i8) -> i8 {
    let mut nodes = Vec::new();
    
    let shanten = calc(&tehai, furos.len() as i8);
            
    // let mut horas = Vec::new();
    let candidate = DfsCandidate::new(
        tehai.clone(),
        furos.clone(),
        -1,
        depth,
        0
    );

    let mut node_count = 0;
    nodes.push(candidate);
    loop {
        let element = nodes.pop();
        if let None = element {
            break;
        } else if let Some(e) = element {
            
            let shanten = calc(&e.tehai_nums, e.furos.len() as i8);
            // println!("shanten:{:?}", shanten);
            
            if e.current_depth >= e.target_depth {
                // 深さが指定深さに到達したらそれ以降は展開しない
                continue
            }else if shanten == -1 {
                // 和了したらそれ以降は展開しない
                // points, yakus = calc_horas();
                continue
            } else {
                // 手牌を変更する
                for i in 0..34 { // 減少
                    if e.tehai_nums[i] == 0 {
                        continue
                    }

                    for j in 0..34 { // 増加
                        if e.tehai_nums[j] == 4 {
                            continue
                        }
                        
                        let mut new_tehai_nums = e.tehai_nums.clone();
                        new_tehai_nums[i] -= 1;
                        new_tehai_nums[j] += 1;
                        
                        let new_shanten = calc(&new_tehai_nums, e.furos.len() as i8);
                        if new_shanten >= shanten {
                            continue
                        }
                        println!("{},{}",i,j);

                        let new_candidate = DfsCandidate::new(
                            new_tehai_nums,
                            furos.clone(),
                            e.target_shanten,
                            e.target_depth,
                            e.current_depth + 1
                        );
                        nodes.push(new_candidate);
                        node_count += 1;
                    }
                }

            }

        }
    }

    println!("node_count:{}", node_count);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_dfs_works() {
        let tehai:[usize; 34] = [
            1, 1, 1, 1, 1, 1, 1, 1, 3,
            0, 0, 1, 0, 1, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        let furos = vec!();
        let _ = calc_dfs_14(&tehai, furos, 3);
        assert_eq!(tehai[0], 1);
    }

}