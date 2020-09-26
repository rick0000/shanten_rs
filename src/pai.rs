use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaiType {
    MANZU,
    PINZU,
    SOUZU,
    JIHAI,
    UNKNOWN,
}

#[derive(Clone, Copy)]
pub struct Pai {
    pub id: usize,
    pub number: usize,
    pub pai_type: PaiType,
    is_red: bool,
}
impl fmt::Debug for Pai {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?}", self.get_str())
    }    
} 


impl PartialOrd for Pai {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl PartialEq for Pai {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Pai {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            number: (id % 9) + 1,
            pai_type: Self::get_pai_type(id),
            is_red: false,
        }
    }
    pub fn new_str(pai_str: &str) -> Self {
        let id = Self::str_to_id(pai_str);
        Self {
            id,
            number: (id % 9) + 1,
            pai_type: Self::get_pai_type(id),
            is_red: pai_str.ends_with("r"),
        }
    }

    pub fn new_by_vec(ids: Vec<usize>) -> Vec<Self> {
        ids.into_iter().map(|x| Pai::new(x)).collect()
    }

    pub fn new_by_str_vec(pai_strs: Vec<&str>) -> Vec<Self> {
        pai_strs.into_iter().map(|x| Pai::new_str(x)).collect()
    }

    pub fn get_str(&self) -> String {
        let suffix = if self.is_red {"r"} else {""};
        match self.pai_type {
            PaiType::MANZU => format!("{}m{}", self.number,suffix),
            PaiType::PINZU => format!("{}p{}", self.number,suffix),
            PaiType::SOUZU => format!("{}s{}", self.number,suffix),
            _ => Self::id_to_str(self.id).to_string(),
        }
    }

    pub fn is_red(&self) -> bool {
        self.is_red
    }
    
    pub fn is_sangenpai(&self) -> bool {
        if let PaiType::JIHAI = &self.pai_type {
            match self.number {
                5 | 6 | 7 => return true,
                _ => return false,
            }
        }
        false
    }
    pub fn is_wind(&self) -> bool {
        if let PaiType::JIHAI = &self.pai_type {
            match self.number {
                1 | 2 | 3 | 4 => return true,
                _ => return false,
            }
        }
        false
    }
    pub fn is_jihai(&self) -> bool {
        self.pai_type == PaiType::JIHAI
    }
    pub fn is_number(&self) -> bool {
        self.pai_type != PaiType::JIHAI
    }
    pub fn is_yaochu(&self) -> bool {
        self.number == 1 || self.number == 9 || self.pai_type == PaiType::JIHAI
    }
    pub fn is_same_symbol(&self, other: Self) -> bool {
        self.number == other.number && self.pai_type == self.pai_type
    }
    pub fn is_green(&self) -> bool {
        if let PaiType::SOUZU = &self.pai_type {
            match self.number {
                2 | 3 | 4 | 6 | 8 => return true,
                _ => return false,
            }
        } else if let PaiType::JIHAI = &self.pai_type {
            match self.number {
                6 => return true,
                _ => return false,
            }
        }
        false
    }

    fn get_pai_type(id: usize) -> PaiType {
        match id / 9 {
            0 => PaiType::MANZU,
            1 => PaiType::PINZU,
            2 => PaiType::SOUZU,
            3 => PaiType::JIHAI,
            _ => PaiType::UNKNOWN,
        }
    }



    fn id_to_str(id:usize) -> &'static str {
        match id {
            0 => "1m",
            1 => "2m",
            2 => "3m",
            3 => "4m",
            4 => "5m",
            5 => "6m",
            6 => "7m",
            7 => "8m",
            8 => "9m",

            9 =>  "1p",
            10 => "2p",
            11 => "3p",
            12 => "4p",
            13 => "5p",
            14 => "6p",
            15 => "7p",
            16 => "8p",
            17 => "9p",

            18 => "1s",
            19 => "2s",
            20 => "3s",
            21 => "4s",
            22 => "5s",
            23 => "6s",
            24 => "7s",
            25 => "8s",
            26 => "9s",

            27 => "E",
            28 => "S",
            29 => "W",
            30 => "N",
            31 => "P",
            32 => "F",
            33 => "C",

            _ => panic!("not intended pai id {}", id)
        }
    }

    fn str_to_id(pai_str:&str) -> usize {
        match pai_str {
            "1m" => 0,
            "2m" => 1,
            "3m" => 2,
            "4m" => 3,
            "5m" => 4,
            "6m" => 5,
            "7m" => 6,
            "8m" => 7,
            "9m" => 8,

            "1p" => 9,
            "2p" => 10,
            "3p" => 11,
            "4p" => 12,
            "5p" => 13,
            "6p" => 14,
            "7p" => 15,
            "8p" => 16,
            "9p" => 17,
            
            "1s" => 18,
            "2s" => 19,
            "3s" => 20,
            "4s" => 21,
            "5s" => 22,
            "6s" => 23,
            "7s" => 24,
            "8s" => 25,
            "9s" => 26,

            "E" => 27,
            "S" => 28,
            "W" => 29,
            "N" => 30,
            "P" => 31,
            "F" => 32,
            "C" => 33,

            "5mr" => 4,
            "5pr" => 13,
            "5sr" => 22,
            
            "?" => 100,
            _ => panic!("not intended pai str {}", pai_str),
        }
    }


}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pai() {
        let p: Pai = Pai::new(8);
        println!("{:?}", p);
        assert_eq!(p.pai_type, PaiType::MANZU);
    }
}
