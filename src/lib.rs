use pyo3::prelude::*;
use pyo3::types::{PyList, PyDict};
use pyo3::wrap_pyfunction;

mod dfs;
mod furo;
mod hora;
mod mentsu;
mod mentsu_tartsu_num;
mod pai;
mod point_datam;
mod resource;
mod shanten_analysis;
mod tenpai_analysis;
mod yaku;
mod hora_candidate;


#[macro_use]
extern crate lazy_static;

use crate::shanten_analysis::calc;
use crate::shanten_analysis::calc_all;
use crate::pai::Pai;
use crate::furo::{Furo,FuroType};
use crate::hora::{Hora};
use crate::tenpai_analysis::HoraType;


#[pyfunction]
fn get_shanten(tehai: &PyList, furo_num: i8) -> i8 {
    let v: Vec<usize> = tehai.as_ref().extract().unwrap();
    if v.len() != 34 {
        panic!("tehai length must be 34");
    }
    let mut tehai_array = [0; 34];
    for i in 0..v.len() {
        tehai_array[i] = v[i];
    }
    calc(&tehai_array, furo_num)
}

#[pyfunction]
fn get_shanten_all(tehai: &PyList, furo_num: i8) -> [i8; 3] {
    let v: Vec<usize> = tehai.as_ref().extract().unwrap();
    if v.len() != 34 {
        panic!("tehai length must be 34");
    }
    let mut tehai_array = [0; 34];
    for i in 0..v.len() {
        tehai_array[i] = v[i];
    }
    calc_all(&tehai_array, furo_num)
}

#[pyfunction]
/// calclate hora points for input.
/// returns [fu, fan, points, oya_payment, ko_payment] 
fn get_hora(
        py: Python,
        tehai: &PyList, 
        furos: &PyList, 
        taken: &str,

        oya: bool,
        hora_type: &str,
        first_turn: bool,
        doras: Vec<&str>,
        uradoras: Vec<&str>,
        reach: bool,
        double_reach: bool,
        ippatsu: bool,
        rinshan: bool,
        chankan: bool,
        haitei: bool,
        bakaze: &str,
        jikaze: &str,
        show:bool
    ) -> ([u32;5], Vec<(String,usize)>) {
    // ) -> [u32;5] {    
    // assert_eq!(tehai.len() + furos.len()*3 , 13);
    // println!("{:?}",tehai);
    let tehai_rs: Vec<&str> = tehai.as_ref().extract().unwrap();
    let converted_tehais = Pai::new_by_str_vec(tehai_rs);
    let (converted_furos, furo_all_pais) = convert_furo(furos);
    let converted_taken = Pai::new_str(taken);
    
    
    let mut all_pais = converted_tehais.clone();
    all_pais.extend(furo_all_pais);
    all_pais.push(converted_taken);
    

    let mut parsed_hora_type:HoraType = HoraType::Ron;
    match hora_type {
        "ron" => {parsed_hora_type = HoraType::Ron},
        "tsumo" => {parsed_hora_type = HoraType::Tsumo},
        _ => {},
    }

    let parsed_doras = Pai::new_by_str_vec(doras);
    let parsed_uradoras = Pai::new_by_str_vec(uradoras);
    let parased_bakaze = Pai::new_str(bakaze);
    let parased_jikaze = Pai::new_str(jikaze);

    
    let hora = Hora::new(
        converted_tehais,
        converted_furos,
        converted_taken,
        all_pais,
        oya,
        parsed_hora_type,
        first_turn,
        parsed_doras,
        parsed_uradoras,
        reach,
        double_reach,
        ippatsu,
        rinshan,
        chankan,
        haitei,
        parased_bakaze,
        parased_jikaze,
    );

    if show {
        println!("{:?}", hora);
        
    }
    
    let pointdatam = hora.get_pointdatam();
    let result = [
        pointdatam.fu,
        pointdatam.fan,
        pointdatam.points,
        pointdatam.oya_payment,
        pointdatam.ko_payment,
    ];
    let mut yaku_fan_tuple_vec:Vec<(String,usize)> = vec![];
    let yakus = hora.get_yaku_fans();
    for yaku in &yakus {
        let name = yaku.yaku_name.name().to_string();
        yaku_fan_tuple_vec.push((name, yaku.fan));
    }
    

    (result, yaku_fan_tuple_vec)
}


fn convert_furo(furos: &PyList) -> (Vec<Furo>, Vec<Pai>) {
    // println!("convert_furo:{:?}",furos);
    let mut furo_converted :Vec<Furo> = vec![];
    let mut all_pais :Vec<Pai> = vec![];
    
    if furos.len() > 0 {
        let furos_rs_result:PyResult<Vec<&PyDict>> = furos.as_ref().extract();
        if let Ok(furos_rs) = furos_rs_result {
            for f in &furos_rs {
                if let Some(type_value) = f.get_item("type") {
                    let furo_type_parsed_result:PyResult<&str> = FromPyObject::extract(type_value);
                    if let Ok(furo_type_parsed) = furo_type_parsed_result {
    
                        let mut furo_taken:Option<Pai> = None;
                        if let Some(taken_value) = f.get_item("taken") {
                            let taken_value_parsed:PyResult<&str> = FromPyObject::extract(taken_value);
                            if let Ok(pai_str) = taken_value_parsed {
                                furo_taken = Some(Pai::new_str(pai_str));
                                all_pais.push(Pai::new_str(pai_str));
                            }
                        }
                        let consumed_pais_str:Vec<&str> = f.get_item("consumed").unwrap().extract().unwrap();
                        let consumed_pais:Vec<Pai> = Pai::new_by_str_vec(consumed_pais_str.clone()); 
                        all_pais.extend(Pai::new_by_str_vec(consumed_pais_str));
                        let furo:Furo = Furo::new(
                            convert_furo_type(furo_type_parsed),
                            furo_taken,
                            consumed_pais,
                        );
                        furo_converted.push(furo);
    
                    }
                }
            }
        }
    }
    (furo_converted, all_pais)
}

fn convert_furo_type(furo_type: &str) -> FuroType {
    match furo_type {
        "chi" => FuroType::CHI,
        "pon" => FuroType::PON,
        "ankan" => FuroType::ANKAN,
        "kakan" => FuroType::KAKAN,
        "daiminkan" => FuroType::DAIMINKAN,
        _ => panic!("invalid type {}", furo_type),
    }
}


#[pymodule]
fn shanten(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_shanten))?;
    m.add_wrapped(wrap_pyfunction!(get_shanten_all))?;
    m.add_wrapped(wrap_pyfunction!(get_hora))?;
    Ok(())
}

