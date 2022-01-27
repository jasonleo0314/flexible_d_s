//经典D-S证据冲突算法

use std::collections::HashSet;

use crate::obj::cell::Cell;
use crate::obj::evidence_matrix::EvidenceMatrix;
use crate::utils::tools::approx;

pub fn get_event_matrix(cell_matrix: &Vec<Vec<Cell>>) -> Vec<Vec<HashSet<usize>>> {
    cell_matrix
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| c.event().child().clone())
                .collect::<Vec<HashSet<usize>>>()
        })
        .collect::<Vec<Vec<HashSet<usize>>>>()
}

pub fn dempster_merge(evm: &EvidenceMatrix) -> Vec<f64> {
    let k_matrix = &get_empty_intersection(evm);
    let k = plus_mul_cell_matrix(k_matrix);
    let neg_lower_k = 1.0 / (1.0 - k);
    let rst_vec = (0..evm.col_cnt())
        .into_iter()
        .map(|idx| plus_mul_cell_matrix(&get_x_intersection(evm, idx)))
        .map(|upper| approx(upper * neg_lower_k, 14))
        .collect::<Vec<f64>>();
    rst_vec
}

pub fn plus_mul_cell_matrix(cell_matrix: &Vec<Vec<Cell>>) -> f64 {
    cell_matrix
        .iter()
        .map(|cbv| cbv.iter().map(|c| c.val()).fold(1.0, |a, c| a * c))
        .fold(0f64, |a, c| a + c)
}

pub fn get_x_intersection(evm: &EvidenceMatrix, event_idx: usize) -> Vec<Vec<Cell>> {
    let target_set = &evm.event_vector().get(event_idx).unwrap().child().clone();
    let rst = check_combination_on_set(evm, |x| target_set == x);
    // rst.iter()
    //     .map(|v| v.iter().map(|c| c.val()).collect::<Vec<f64>>())
    //     .for_each(|k| println!("{:?}", k));
    rst
}

pub fn get_empty_intersection(evm: &EvidenceMatrix) -> Vec<Vec<Cell>> {
    check_combination_on_set(evm, |x| x.is_empty())
}

//在组合起来的一行一行的集合上检查条件
pub fn check_combination_on_set<F: Fn(&HashSet<usize>) -> bool>(
    evm: &EvidenceMatrix,
    condition: F,
) -> Vec<Vec<Cell>> {
    let event_matrix = evm.n_base_cell_matrix();
    let rst = event_matrix
        .iter()
        .filter(|&cb_line| {
            let fold_init = cb_line[0].event().clone();
            let event_to_check = &cb_line
                .iter()
                .map(|cell| cell.event().clone())
                .fold(fold_init, |a, c| a.inter_event(c));
            condition(event_to_check.child())
        })
        .map(|i| i.clone())
        .collect::<Vec<Vec<Cell>>>();
    rst
}
