


#[derive(Clone, Debug, PartialEq)]
pub struct Yaku {
    name:YakuName,
    fan:i8,
}
impl Yaku {
    pub fn new(name:YakuName, fan:i8) -> Self {
        Self {
            name,
            fan,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum YakuName {
    Tenho,
    Chiho,
    Kokushimuso,
    Daisangen,
    Suanko,
    Tsuiso,
    Ryuiso,
    Chinroto,
    Daisushi,
    Shosushi,
    Sukantsu,
    Churenpoton,
    Dora,
    Uradora,
    Akadora,
    Reach,
    Ippatsu,
    MenzenchinTsumoho,
    Tanyaochu,
    Pinfu,
    Ipeko,
    Sangenpai,
    Bakaze,
    Jikaze,
    Rinshankaiho,
    Chankan,
    Haiteiraoyue,
    Hoteiraoyui,
    
    Sanshokudojun,
    Ikkitsukan,
    Honchantaiyao,
    Chitoitsu,
    Toitoiho,
    Sananko,
    Honroto,
    Sanshokudoko,
    Sankantsu,
    Shosangen,
    DoubleReach,

    Honiso,
    Junchantaiyao,
    Ryanpeko,
    
    Chiniso,
}

impl YakuName {
    fn name(&self) -> &str {
        match *self {
            YakuName::Tenho => "tenho",
            YakuName::Chiho => "chiho",
            YakuName::Kokushimuso => "kokushimuso",
            YakuName::Daisangen => "daisangen",
            YakuName::Suanko => "suanko",
            YakuName::Tsuiso => "tsuiso",
            YakuName::Ryuiso => "ryuiso",
            YakuName::Chinroto => "chinroto",
            YakuName::Daisushi => "daisushi",
            YakuName::Shosushi => "shosushi",
            YakuName::Sukantsu => "sukantsu",
            YakuName::Churenpoton => "churenpoton",
            YakuName::Dora => "dora",
            YakuName::Uradora => "uradora",
            YakuName::Akadora => "akadora",
            YakuName::Reach => "reach",
            YakuName::Ippatsu => "ippatsu",
            
            YakuName::MenzenchinTsumoho => "menzenchin_tsumoho",
            YakuName::Tanyaochu => "tanyaochu",
            YakuName::Pinfu => "pinfu",
            YakuName::Ipeko => "ipeko",
            YakuName::Sangenpai => "sangenpai",
            YakuName::Bakaze => "bakaze",
            YakuName::Jikaze => "jikaze",
            YakuName::Rinshankaiho => "rinshankaiho",
            YakuName::Chankan => "chankan",
            YakuName::Haiteiraoyue => "haiteiraoyue",
            YakuName::Hoteiraoyui => "hoteiraoyui",
            
            YakuName::Sanshokudojun => "sanshokudojun",
            YakuName::Ikkitsukan => "ikkitsukan",
            YakuName::Honchantaiyao => "honchantaiyao",
            YakuName::Chitoitsu => "chitoitsu",
            YakuName::Toitoiho => "toitoiho",
            YakuName::Sananko => "sananko",
            YakuName::Honroto => "honroto",
            YakuName::Sanshokudoko => "sanshokudoko",
            YakuName::Sankantsu => "sankantsu",
            YakuName::Shosangen => "shosangen",
            YakuName::DoubleReach => "double_reach",
            
            YakuName::Honiso => "honiso",
            YakuName::Junchantaiyao => "junchantaiyao",
            YakuName::Ryanpeko => "ryanpeko",
            
            YakuName::Chiniso => "chiniso",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_yakuname () {
        println!("{}", YakuName::Tenho.name());
        
    }
}