//! 
/// # ある手牌から深さ優先探索を行い和了点数が到達可能かを判定する
/// 
use crate::shanten_analysis::calc;
use crate::furo::Furo;
use std::fmt;

#[derive(Clone)]
pub struct Candidate {
    tehai_nums:[usize;34],
    furos:Vec<Furo>,
    target_shanten:i8,
    target_depth:i8,
    current_depth:i8,
}

impl Candidate {
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

impl fmt::Debug for Candidate {
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
    let mut horas = Vec::new();
    let candidate = Candidate::new(
        tehai.clone(),
        furos,
        -1,
        depth,
        0
    );
    nodes.push(candidate);
    loop {
        let element = nodes.pop();
        if let None = element {
            break;
        } else if let Some(e) = element {
            println!("{:?}", e);
            let shanten = calc(&e.tehai_nums, e.furos.len() as i8);
            println!("{:?}", shanten);

            if shanten == -1 {
                // points, yakus = calc_horas();
            }
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_dfs_works() {
        let tehai:[usize; 34] = [
            1, 1, 1, 1, 1, 1, 1, 1, 3,
            3, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        let furos = vec!();
        let _ = calc_dfs_14(&tehai, furos, 3);
        assert_eq!(tehai[0], 1);
    }

}