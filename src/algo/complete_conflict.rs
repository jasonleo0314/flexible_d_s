use crate::algo::dempster_classic::dempster_merge;
use crate::obj::cell::Cell;
use crate::obj::event::Event;
use crate::obj::evidence_matrix::EvidenceMatrix;
use crate::utils::tools::{approx, approx_eq};
use std::collections::HashSet;

pub fn run_flexible_complete_conflict_algo(input_evidence_matrix: &EvidenceMatrix) -> Vec<f64> {
    let mt = input_evidence_matrix;
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    // if ig_cell.clone().is_empty() {
    //     return get_classic_dempster_result(mt);
    // } else {
    let sim = &measure_detector_similarity(mt, ig_cell);
    let crd = &detector_credibility_vec(sim, ig_cell);
    let total = sum_all_detector_similarity(sim);
    let col_weight_vec = &col_weight_vec(mt.detect_matrix(), crd, total);
    let father_vec = &find_fathers_of_col(mt.event_vector());
    let dist = &get_compatible_redistribute_vec(mt.event_vector(), father_vec, col_weight_vec);
    let complete_conf = &pick_complete_conflict_event(mt);
    let neg_delta = get_neg_delta(complete_conf, dist);
    let classic = &get_classic_dempster_result(mt);
    let rst = final_result(complete_conf, dist, neg_delta, classic);
    rst
    // }
}

pub fn find_inter(event_vector: &Vec<Event>, col: usize) -> Vec<usize> {
    let event = event_vector;
    let cur_eve = event[col].clone();
    let rst = event
        .iter()
        .enumerate()
        .filter(|(_, val)| cur_eve.inter_event(val.clone().clone()).child().len() != 0)
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();
    rst
}

pub fn scan_rows_in_one_col(event_vec: &Vec<Event>, mat: &Vec<Vec<Cell>>, col: usize) -> bool {
    let matter_cols = find_inter(event_vec, col);
    let row_len = mat.len();
    let mut all_zero;
    for cur_row in 0..row_len {
        if mat[cur_row][col].val() != 0.0 {
            for scan_row in 0..row_len {
                if cur_row != scan_row {
                    all_zero = true;
                    for &scan_col in matter_cols.iter() {
                        if mat[scan_row][scan_col].val() != 0.0 {
                            all_zero = false;
                        }
                    }
                    if all_zero == true {
                        return true;
                    }
                }
            }
        }
    }

    return false;
}

pub fn pick_complete_conflict_event(evm: &EvidenceMatrix) -> HashSet<usize> {
    let mat = evm.detect_matrix();
    let col_num = evm.col_cnt();
    let mut complete_conflict_cols = HashSet::new();
    for cur_col in 0..col_num {
        if scan_rows_in_one_col(evm.event_vector(), mat, cur_col) {
            complete_conflict_cols.insert(cur_col);
        }
    }
    complete_conflict_cols
}

pub fn cart_event_intersect(evm: &EvidenceMatrix) -> Vec<(f64, usize, usize)> {
    let col_num = evm.col_cnt();
    (0..col_num)
        .into_iter()
        .map(|a| {
            (0..col_num)
                .into_iter()
                .map(|b| {
                    let a_eve = &evm.event_vector()[a];
                    let b_eve = &evm.event_vector()[b];
                    let inter_len = a_eve.inter_event(b_eve.clone()).child().len();
                    if inter_len == 0 {
                        (0f64, a, b)
                    } else {
                        let union_len = a_eve.union_event(b_eve).child().len();
                        let div = (inter_len as f64) / (union_len as f64);
                        (div, a, b)
                    }
                })
                .filter(|(v, _, _)| v != &0.0)
                .collect::<Vec<(f64, usize, usize)>>()
        })
        .flatten()
        .collect::<Vec<(f64, usize, usize)>>()
}

pub fn find_set_theta_solo_row(edm: &Vec<Vec<Cell>>, discernment: &HashSet<usize>) -> Vec<Cell> {
    edm.iter()
        .map(|v0| {
            v0.iter()
                .filter(|&v1| approx_eq(v1.val(), 1.0, 15))
                .filter(|&c0| c0.event().child() == discernment)
                .map(|c1| c1.clone())
                .collect::<Vec<Cell>>()
        })
        .flatten()
        .map(|v| v.clone())
        .collect::<Vec<Cell>>()
        .clone()
}

pub fn measure_detector_similarity(evm: &EvidenceMatrix, ignore_cell: &Vec<Cell>) -> Vec<Vec<f64>> {
    let inter = cart_event_intersect(evm);
    let mat = evm.detect_matrix();
    let ignore_row = if ignore_cell.len() > 0 {
        ignore_cell[0].row()
    } else {
        usize::MAX
    };
    let row_num = evm.row_cnt();
    let tmp = (0..row_num)
        .into_iter()
        .map(|row1| {
            (0..row_num)
                .into_iter()
                .map(|row2| {
                    let main = inter
                        .iter()
                        .map(|(volt, col1, col2)| {
                            let f11 = mat[row1][*col1].clone();
                            let f12 = mat[row1][*col2].clone();
                            let f21 = mat[row2][*col1].clone();
                            let f22 = mat[row2][*col2].clone();
                            let rst = (f11 - f21) * (f12 - f22) * volt;
                            rst
                        })
                        .fold(0.0, |a, c| a + c);
                    let tmp0 = 1f64 - ((0.5 * main).sqrt());
                    if row1 == ignore_row || row2 == ignore_row {
                        0.0
                    } else {
                        approx(tmp0, 15)
                    }
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>();
    tmp
}

pub fn get_similarity(detector_similarity: &Vec<Vec<f64>>, row_a: usize, row_b: usize) -> f64 {
    if detector_similarity.len() == 0 {
        panic!("尚未运行完全冲突模型算法")
    } else {
        detector_similarity[row_a][row_b]
    }
}

pub fn sum_all_detector_similarity(detector_similarity: &Vec<Vec<f64>>) -> f64 {
    let lens = detector_similarity.len();
    let mut rst = 0.0;
    for i in 0..lens {
        for j in 0..lens {
            rst += detector_similarity[i][j];
        }
    }
    rst
}

pub fn detector_credibility_upper(
    detector_similarity: &Vec<Vec<f64>>,
    ignore_cell: &Vec<Cell>,
    row_num: usize,
) -> f64 {
    let ig_row_num = if ignore_cell.len() > 0 {
        ignore_cell[0].row()
    } else {
        usize::MAX
    };
    return if row_num == ig_row_num {
        0.0
    } else {
        let row_len = detector_similarity.len();
        let upper = (0..row_len)
            .into_iter()
            .map(|row_tmp| get_similarity(detector_similarity, row_tmp, row_num))
            .fold(0.0, |a, c| a + c);
        approx(upper, 15)
    };
}

pub fn detector_credibility_vec(
    detector_similarity: &Vec<Vec<f64>>,
    ignore_cell: &Vec<Cell>,
) -> Vec<f64> {
    let d_len = detector_similarity.len();
    (0..d_len)
        .into_iter()
        .map(|row| detector_credibility_upper(detector_similarity, ignore_cell, row))
        .collect::<Vec<f64>>()
}
//获取的列权重值列表顺序和论文有出入，但每个数值一致
//现已查明，论文上出错，数值抄串行了
pub fn col_weight_vec(
    detector_matrix: &Vec<Vec<Cell>>,
    detector_cred_vec: &Vec<f64>,
    total_detector_similarity: f64,
) -> Vec<f64> {
    let row_len = detector_matrix.len();
    let col_len = detector_matrix[0].len();
    (0..col_len)
        .into_iter()
        .map(|target_col| {
            let upper = (0..row_len)
                .into_iter()
                .map(|row| {
                    let tmp = &detector_matrix[row][target_col];
                    let ita = detector_cred_vec[row];
                    let rst0 = tmp.val() * ita;
                    // println!(
                    //     "当前第{}列，单元格为：{:?}，运算：【{}】 × 【{}】 = 【{}】 ,",
                    //     target_col,
                    //     tmp.clone(),
                    //     tmp.val(),
                    //     ita,
                    //     rst0
                    // );
                    rst0
                })
                .fold(0.0, |a, c| a + c);
            let rst;
            if total_detector_similarity == 0.0 {
                rst = 0.0
            } else {
                rst = approx(upper / total_detector_similarity, 15);
            }

            // println!(
            //     "当前第{}列乘积加和：{}，结论为：【{}】 ÷ 【{}】 = 【{}】",
            //     target_col, upper, upper, total_detector_similarity, rst
            // );
            rst
        })
        .collect::<Vec<f64>>()
}

pub fn find_fathers_of_col(event_set: &Vec<Event>) -> Vec<Vec<usize>> {
    let col_len = event_set.len();
    (0..col_len)
        .into_iter()
        .map(|target_col| {
            let target_set = event_set[target_col].clone();
            (0..col_len)
                .into_iter()
                .filter(|&cur_col| target_set.belong(&event_set[cur_col]))
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}
//因为col_weight_vec步骤在论文中抄错行，所以这一步组合使用col_weight_vec步骤结果时，论文全部出错
pub fn get_compatible_redistribute_vec(
    event_vec: &Vec<Event>,
    father_vec: &Vec<Vec<usize>>,
    col_weight_vec: &Vec<f64>,
) -> Vec<f64> {
    (0..event_vec.len())
        .into_iter()
        .map(|target_col| {
            let b = event_vec[target_col].child().len() as f64;
            let fathers = father_vec[target_col].clone();
            let rst1 = fathers
                .iter()
                .map(|&father_col| {
                    let a = col_weight_vec[father_col];
                    let c = event_vec[father_col].child().len() as f64;
                    let rst0 = a * b / c;
                    // println!("当前列：{}，当前事件：{:?}，当前父列：{}，父列事件：{:?}，父列权重【{}】 × 当前事件集合势【{}】 ÷ 父事件集合势【{}】 = 【{}】",
                    // target_col,event_vec[target_col].child(),father_col,event_vec[father_col].child(),a,b,c,rst0);
                    rst0
                })
                .fold(0.0, |a, c| a + c);
            // println!("当前列：{}，当前事件：{:?}，合计：【{}】",target_col,event_vec[target_col].child(),rst1);
            rst1
        })
        .collect::<Vec<f64>>()
}

pub fn get_neg_delta(
    complete_conflict_set: &HashSet<usize>,
    compatible_redistribute_vec: &Vec<f64>,
) -> f64 {
    let delta = complete_conflict_set
        .iter()
        .map(|col| compatible_redistribute_vec[*col])
        .fold(0.0, |a, c| a + c);
    approx(1.0 - delta, 15)
}

pub fn get_classic_dempster_result(evidence_matrix: &EvidenceMatrix) -> Vec<f64> {
    dempster_merge(evidence_matrix)
}

pub fn final_result(
    complete_conflict_set: &HashSet<usize>,
    compatible_redistribute_vec: &Vec<f64>,
    neg_delta: f64,
    classic_dempster_result: &Vec<f64>,
) -> Vec<f64> {
    let neg_mul_delta_classic = &mut classic_dempster_result
        .iter()
        .map(|f1| approx(f1 * neg_delta, 14))
        .collect::<Vec<f64>>();
    let _ = complete_conflict_set
        .iter()
        .map(|col| {
            let tmp = compatible_redistribute_vec[*col];
            neg_mul_delta_classic[*col] = tmp;
        })
        .collect::<Vec<()>>();
    neg_mul_delta_classic.clone()
}
