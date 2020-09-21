use crate::pai::Pai;

pub struct Hora {
    tehai: Vec<Pai>,
}

impl Hora {
    pub fn new() -> Self {
        Self {
            tehai: Vec::new(),
        }
    }
}



#[cfg(test)]
mod test {
    #[test]
    fn test_hora() {
        assert_eq!(1,1);
    }
}