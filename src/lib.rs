
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyList;

mod shanten_analysis;
mod mentsu_tartsu_num;
mod resource;
mod tenpai_analysis;
mod dfs;
mod pai;
mod hora;
mod furo;
mod yaku;

#[macro_use]
extern crate lazy_static;

pub use crate::shanten_analysis::calc;
pub use crate::shanten_analysis::calc_all;






#[pyfunction]
fn get_shanten(tehai: &PyList, furo_num: i8) -> i8 {
    let v: Vec<usize> = tehai.as_ref().extract().unwrap();
    if v.len() != 34 {
        panic!("tehai length must be 34");
    }
    let mut tehai_array = [0;34];
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
    let mut tehai_array = [0;34];
    for i in 0..v.len() {
        tehai_array[i] = v[i];
    }
    calc_all(&tehai_array, furo_num)
}

#[pymodule]
fn shanten(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_shanten))?;
    m.add_wrapped(wrap_pyfunction!(get_shanten_all))?;

    Ok(())
}