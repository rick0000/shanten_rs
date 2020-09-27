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
/// AAA
fn get_hora(tehai: &PyList, furos: &PyList, taken: usize) -> [i32;4] {
    // assert_eq!(tehai.len() + furos.len()*3 , 14);
        // println!("{:?}",tehai);
        let tehai_rs: Vec<&str> = tehai.as_ref().extract().unwrap();
        let tehai_pai = Pai::new_by_str_vec(tehai_rs);
        // println!("{:?}",tehai_pai);

        // println!("{:?}",furos);
        let mut furo_converted :Vec<Furo> = vec![];
        if furos.len() > 0 {
            let furos_rs:Vec<&PyDict> = furos.as_ref().extract().unwrap();

            for f in &furos_rs {
                println!("{:?}",f.get_item("type"));
                
                let pais:Vec<&str> = f.get_item("pais").unwrap().extract().unwrap();
                println!("{:?}",pais);
            }
            
        }
        

    for i in 0..1000000 {

        
    }

    // println!("{:?}", taken);
    [0;4]
}



#[pymodule]
fn shanten(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_shanten))?;
    m.add_wrapped(wrap_pyfunction!(get_shanten_all))?;
    m.add_wrapped(wrap_pyfunction!(get_hora))?;
    Ok(())
}
