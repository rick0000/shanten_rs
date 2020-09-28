use pyo3::prelude::*;
use pyo3::types::{PyList, PyDict, PyString};
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
fn get_hora(tehai: &PyList, furos: &PyList, taken: &str) -> [i32;5] {    
    assert_eq!(tehai.len() + furos.len()*3 , 13);
    // println!("{:?}",tehai);
    let tehai_rs: Vec<&str> = tehai.as_ref().extract().unwrap();
    let converted_tehais = Pai::new_by_str_vec(tehai_rs);
    let converted_furos:Vec<Furo> = convert_furo(furos);
    let converted_taken = Pai::new_str(taken);

    let oya: bool = false;
    let hora_type: HoraType = HoraType::Ron;
    let first_turn: bool = false;
    let doras: Vec<Pai> = vec![];
    let uradoras: Vec<Pai> = vec![];
    let reach: bool = false;
    let double_reach: bool = false;
    let ippatsu: bool = false;
    let rinshan: bool = false;
    let chankan: bool = false;
    let haitei: bool = false;
    let bakaze: Pai = Pai::new_str("E");
    let jikaze: Pai = Pai::new_str("S");
    
    let hora = Hora::new(
        converted_tehais,
        converted_furos,
        converted_taken,
        oya,
        hora_type,
        first_turn,
        doras,
        uradoras,
        reach,
        double_reach,
        ippatsu,
        rinshan,
        chankan,
        haitei,
        bakaze,
        jikaze,
    );

    println!("{:?}",hora);
    let pointdatam = hora.get_pointdatam();
    [
        pointdatam.fu as i32,
        pointdatam.fan as i32,
        pointdatam.points,
        pointdatam.oya_payment,
        pointdatam.ko_payment,
    ]
}

fn convert_furo(furos: &PyList) -> Vec<Furo> {
    // println!("{:?}",furos);
    let mut furo_converted :Vec<Furo> = vec![];

    if furos.len() > 0 {
        let furos_rs:Vec<&PyDict> = furos.as_ref().extract().unwrap();
        for f in &furos_rs {
            let furo_type_parsed_result:PyResult<&str> = FromPyObject::extract(f.get_item("type").unwrap());
            let furo_type_parsed:&str = furo_type_parsed_result.ok().unwrap();

            let mut furo_taken:Option<Pai> = None;
            if let Some(taken_value) = f.get_item("taken") {
                let taken_value_parsed:PyResult<&str> = FromPyObject::extract(taken_value);
                if let Ok(pai_str) = taken_value_parsed {
                    furo_taken = Some(Pai::new_str(pai_str));
                }
            }
            let consumed_pais_str:Vec<&str> = f.get_item("consumed").unwrap().extract().unwrap();
            let consumed_pais:Vec<Pai> = Pai::new_by_str_vec(consumed_pais_str); 
            let furo:Furo = Furo::new(
                convert_furo_type(furo_type_parsed),
                furo_taken,
                consumed_pais,
            );
            furo_converted.push(furo);
        }
    }
    furo_converted
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
