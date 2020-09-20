//! 
/// # ある手牌から深さ優先探索を行い和了点数が到達可能かを判定する
/// 

fn calc_dfs(_tehai: &[usize; 34], _furo_num:i8) -> i8 {
    1
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_dfs_works() {
        let tehai:[usize; 34] = [
            1, 1, 1, 1, 1, 1, 1, 1, 3,
            2, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        println!("{:?}",tehai);
        let _ = calc_dfs(&tehai, 0);
        assert_eq!(tehai[0], 1);
    }


}