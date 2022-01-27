//使用maturin develop 命令发布程序包

use std::collections::HashSet;

use pyo3::prelude::*;
use pyo3::{exceptions, wrap_pyfunction};

use crate::algo::complete_conflict::run_flexible_complete_conflict_algo;
use crate::algo::dempster_classic::dempster_merge;
use crate::algo::fast_algo::{
    independent_event_evidence_merge, independent_event_non_zero_evidence_merge,
};
use crate::com::input::InputData;

pub mod algo;
pub mod com;
pub mod obj;
#[cfg(test)]
pub mod test;
pub mod utils;

/*
   从python 读入冲突证据
   discernment是识别框架，
   fact_set_list是由识别框架的多个子集构成的列表，每个子集称为“识别事件”；
   evidence_matrix是证据矩阵，即传感器对“识别事件”进行探测所得到的事件发生概率；
   其中，每一行代表一个传感器的所有识别结论（一个概率值）且行内相加为1.0，
   每一列代表一个“识别事件”，每一个元素称为焦元数值[0,1]
    let dcm: HashSet<usize> = set! {1,2,3};
    let sl: Vec<HashSet<usize>> = set_list![{ 1 }, { 2 },{3},{1,3}, { 1,2,3}];
    let m: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3, 0.0, 0.0],
        [0.0, 0.8, 0.2, 0.0, 0.0],
        [0.55, 0.1, 0.35, 0.0, 0.0],
        [0.7, 0.1, 0.2, 0.0, 0.0],
        [0.0, 0.1, 0.0, 0.9, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0]
    ];
*/
#[pyfunction]
fn merge_evidence(
    discernment: HashSet<usize>,        //识别框架
    fact_set_list: Vec<HashSet<usize>>, //“识别事件”列表
    evidence_matrix: Vec<Vec<f64>>,     //证据矩阵
    algo_type: u32,                     //融合函数
) -> PyResult<Vec<f64>> {
    match algo_type {
        0 => Ok(independent_event_non_zero_evidence_merge(&evidence_matrix)),
        1 => Ok(independent_event_evidence_merge(&evidence_matrix)),
        2 => {
            let idt = InputData {
                discernment,
                fact_set_list,
                evidence_matrix,
            };
            let evd_wrapped = idt.build_evidence(true);
            return match evd_wrapped {
                None => Err(PyErr::new::<exceptions::PyImportError, _>(
                    "导入的冲突证据信息有误，无法开展分析",
                )),
                Some(evd) => {
                    //调用evd开展运算
                    //以下内容为占位符
                    let rst = run_flexible_complete_conflict_algo(&evd);
                    Ok(rst)
                }
            };
        }
        3 => {
            let idt = InputData {
                discernment,
                fact_set_list,
                evidence_matrix,
            };
            let evd_wrapped = idt.build_evidence(false);
            return match evd_wrapped {
                None => Err(PyErr::new::<exceptions::PyImportError, _>(
                    "导入的冲突证据信息有误，无法开展分析",
                )),
                Some(evd) => {
                    //调用evd开展运算
                    //以下内容为占位符
                    let rst = dempster_merge(&evd);
                    Ok(rst)
                }
            };
        }
        _ => Ok(independent_event_evidence_merge(&evidence_matrix)),
    }
}

#[pyfunction]
fn merge_evidence_without_check(
    discernment: HashSet<usize>,        //识别框架
    fact_set_list: Vec<HashSet<usize>>, //“识别事件”列表
    evidence_matrix: Vec<Vec<f64>>,     //证据矩阵
    algo_type: u32,                     //融合函数
) -> PyResult<Vec<f64>> {
    match algo_type {
        0 => Ok(independent_event_non_zero_evidence_merge(&evidence_matrix)),
        1 => Ok(independent_event_evidence_merge(&evidence_matrix)),
        2 => {
            let idt = InputData {
                discernment,
                fact_set_list,
                evidence_matrix,
            };
            let evd_wrapped = idt.build_evidence(false);
            return match evd_wrapped {
                None => Err(PyErr::new::<exceptions::PyImportError, _>(
                    "导入的冲突证据信息有误，无法开展分析",
                )),
                Some(evd) => {
                    //调用evd开展运算
                    //以下内容为占位符
                    let rst = run_flexible_complete_conflict_algo(&evd);
                    Ok(rst)
                }
            };
        }
        3 => {
            let idt = InputData {
                discernment,
                fact_set_list,
                evidence_matrix,
            };
            let evd_wrapped = idt.build_evidence(false);
            return match evd_wrapped {
                None => Err(PyErr::new::<exceptions::PyImportError, _>(
                    "导入的冲突证据信息有误，无法开展分析",
                )),
                Some(evd) => {
                    //调用evd开展运算
                    //以下内容为占位符
                    let rst = dempster_merge(&evd);
                    Ok(rst)
                }
            };
        }
        _ => Ok(independent_event_evidence_merge(&evidence_matrix)),
    }
}

#[pymodule]
fn evidence_conflict_merge(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(merge_evidence, m)?)?;
    m.add_function(wrap_pyfunction!(merge_evidence_without_check, m)?)?;
    Ok(())
}
