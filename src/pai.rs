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
    pub id: usize,
    pub number: usize,
    pub pai_type: PaiType,
}

impl Pai {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            number: (id % 9) + 1,
            pai_type: Self::get_pai_type(id),
        }
    }
    pub fn new_by_vec(ids: Vec<usize>) -> Vec<Self> {
        ids.into_iter().map(|x| Pai::new(x)).collect()
    }

    pub fn is_dragon(&self) -> bool {
        if let PaiType::JIHAI = &self.pai_type {
            match self.number {
                5 | 6 | 7 => return true,
                _ => return false,
            }
        }
        false
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
