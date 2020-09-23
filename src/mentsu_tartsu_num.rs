
use crate::resource::constants;

extern crate failure;
use failure::Error;
use fnv::FnvHashMap;



const INDEX_CACHE:[usize;9] = [
    1,
    5,
    5*5,
    5*5*5,
    5*5*5*5,
    5*5*5*5*5,
    5*5*5*5*5*5,
    5*5*5*5*5*5*5,
    5*5*5*5*5*5*5*5,
];

#[derive(Debug, Clone, Copy)]
pub struct MentsuTartsuNum {
    pub mentsu_coef2:i8,
    pub tartsu_coef2:i8,
    pub mentsu_coef10:i8,
    pub tartsu_coef10:i8,
}
impl MentsuTartsuNum{
    pub fn new(mentsu_coef2: i8, tartsu_coef2: i8, mentsu_coef10: i8, tartsu_coef10: i8) -> Self {
        Self {
            mentsu_coef2 : mentsu_coef2,
            tartsu_coef2 : tartsu_coef2,
            mentsu_coef10 : mentsu_coef10,
            tartsu_coef10 : tartsu_coef10,
        }
    }
}
pub struct MentsuTartsuNumCalclator {
    mentsu_tartsu_num_hash : FnvHashMap<usize, MentsuTartsuNum>,
    // mtn_array : Vec<MentsuTartsuNum>
}
impl MentsuTartsuNumCalclator{
    pub fn new()-> Result<MentsuTartsuNumCalclator, Error> {
        // let mut mtn_array : Vec<MentsuTartsuNum> = Vec::<MentsuTartsuNum>::with_capacity(MTN_ARRAY_LENGTH);
        // for _ in 0..MTN_ARRAY_LENGTH{
        //     mtn_array.push(MentsuTartsuNum::new(0,0,0,0));
        // }

        // println!("{}", constants::SHANTEN_PARTS_DEFINITION);

        let mut mentsu_tartsu_num_hash:FnvHashMap<usize, MentsuTartsuNum> = FnvHashMap::with_capacity_and_hasher(
            500000, // 400000 lines as 80%
            Default::default()
        ); 
        for l in constants::SHANTEN_PARTS_DEFINITION.lines() {
            let v : Vec<&str> = l.split_terminator(' ').collect();
            
            let splited_v0s = v[0].chars().collect::<Vec<char>>();
            // println!("{:?}", splited_v0s);
            // let mut key = 0;
            // let mut count = 0;
            let mut keys :[usize;9] = [0;9];
            
            for (i, splited_v0) in splited_v0s.iter().enumerate() {
                keys[i] = splited_v0.to_digit(10).unwrap() as usize;
            }
            // println!("{}",key);
            // println!("{:?} {}", &keys, l);

            let mc2 :i8  = v[1].parse()?;
            let tc2 :i8  = v[2].parse()?;
            let mc10 :i8 = v[3].parse()?;
            let tc10 :i8 = v[4].parse()?;

            // println!("{:?} {} v1:{} v2:{} v3:{} v4:{}", &keys, l, v[1], v[2], v[3],v[4]);
            // println!("get_index{}",MentsuTartsuNumCalclator::get_index(&keys));

            mentsu_tartsu_num_hash.insert(
                MentsuTartsuNumCalclator::get_index(&keys),
                MentsuTartsuNum::new(mc2,tc2,mc10,tc10)
            );

            // mtn_array[MentsuTartsuNumCalclator::get_index(&keys)] = MentsuTartsuNum::new(mc2,tc2,mc10,tc10);
        }

        Ok(MentsuTartsuNumCalclator{
            mentsu_tartsu_num_hash : mentsu_tartsu_num_hash,
            // mtn_array : mtn_array,
        })
    }
    
    pub fn calc(&self, keys: &[usize;9])-> MentsuTartsuNum{
        self.mentsu_tartsu_num_hash[&MentsuTartsuNumCalclator::get_index(keys)]
    }

    


    pub fn calc_ji(&self, keys: &[usize;7]) -> [i8; 2] {
        let mut tartsu_num = 0;
        let mut mentsu_num = 0;
        for i in 0..keys.len() {
            if keys[i] == 2 {
                tartsu_num += 1;
            }
            else if keys[i] == 3 {
                mentsu_num += 1;
            }
        }

        [mentsu_num, tartsu_num]
    }

    pub fn get_index(num:&[usize;9]) -> usize{
        INDEX_CACHE[0] * num[0] +
        INDEX_CACHE[1] * num[1] +
        INDEX_CACHE[2] * num[2] +
        INDEX_CACHE[3] * num[3] +
        INDEX_CACHE[4] * num[4] +
        INDEX_CACHE[5] * num[5] +
        INDEX_CACHE[6] * num[6] +
        INDEX_CACHE[7] * num[7] +
        INDEX_CACHE[8] * num[8]
    
    }
    

    // pub fn len(&self) -> usize{
    //     self.mentsu_tartsu_num_hash.len()
    // }
}

