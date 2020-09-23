#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PaiType {
    MANZU,
    PINZU,
    SOUZU,
    JIHAI,
    UNKNOWN,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pai {
    id: i8,
    number: i8,
    pai_type: PaiType,
}

impl Pai {
    pub fn new_by_id(id:i8) -> Self {
        Self {
            id,
            number: id % 9,
            pai_type: Self::get_pai_type(id),
        } 
    }
    
    fn get_pai_type(id:i8) -> PaiType {
        match id / 9 {
            0 => PaiType::MANZU,
            1 => PaiType::PINZU,
            2 => PaiType::SOUZU,
            3 => PaiType::JIHAI,
            _ => PaiType::UNKNOWN, 
        }
    }
}


#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn test_pai(){
        let p:Pai = Pai::new_by_id(8);
        println!("{:?}", p);
        assert_eq!(p.pai_type, PaiType::MANZU);
    }
}

