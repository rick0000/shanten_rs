//!
/// # テンパイ形解析を行う
/// 
///
///

use crate::pai::Pai;
use crate::furo::Furo;
use crate::shanten_analysis;
use crate::mentsu::Mentsu;

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


fn calc_combination(taken: Pai, tehais: Vec<Pai>, furos: Vec<Furo>) -> i8 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
}
