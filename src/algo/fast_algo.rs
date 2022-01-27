use std::collections::HashMap;

use crate::utils::tools::{approx, approx_eq};

//如果所有事件event均只有一个元素为独立事件，且事件之间互斥,所有元素均不为零是可直接应用该函数
//本算法可达到70微秒左右的效率
pub fn independent_event_non_zero_evidence_merge(evm: &Vec<Vec<f64>>) -> Vec<f64> {
    let col_len = evm[0].len();
    let row_len = evm.len();
    let col_mul_vec = &mut Vec::<f64>::with_capacity(col_len);
    let mut tmp_row_mul: f64;
    let mut tmp_col_plus: f64 = 0f64;
    for col in 0..col_len {
        tmp_row_mul = 1.0;
        for row in 0..row_len {
            tmp_row_mul *= evm[row][col];
        }
        col_mul_vec.push(tmp_row_mul);
        tmp_col_plus += tmp_row_mul;
    }
    if tmp_col_plus == 0.0 {
        let mut rst = Vec::<f64>::with_capacity(col_len);
        for _ in 0..col_len {
            rst.push(0.0);
        }
        rst
    } else {
        col_mul_vec
            .iter()
            .map(|f0| approx(f0 / tmp_col_plus, 14))
            .collect::<Vec<f64>>()
    }
}
//如果所有事件event均只有一个元素为独立事件，且事件之间互斥,可直接应用该函数
pub fn independent_event_evidence_merge(evm: &Vec<Vec<f64>>) -> Vec<f64> {
    let conflict_cols = &complete_conflict_col(evm);
    let (_, _, cred) = &similarity_all_rows(evm);
    let weight_list = &col_weight(evm, cred, conflict_cols);
    let classic = &independent_event_non_zero_evidence_merge(evm);
    let rst = merge_all_single_event_evidence(classic, weight_list);
    rst
}

//找到完全冲突的列（事件之间互斥）
pub fn complete_conflict_col(evm: &Vec<Vec<f64>>) -> Vec<usize> {
    let row_len = evm.len();
    let col_len = evm[0].len();
    let mut complete_conflict_col_vec = vec![];
    for col in 0..col_len {
        let mut is_complete_conflict = false;
        for row in 0..row_len {
            if approx_eq(evm[row][col], 0.0, 15) {
                is_complete_conflict = true;
            }
        }
        if is_complete_conflict == true {
            complete_conflict_col_vec.push(col);
        }
    }
    complete_conflict_col_vec
}

//计算两行之间的相似度(事件之间互斥）
pub fn similarity_between_rows(
    evm: &Vec<Vec<f64>>,
    row_a: usize,
    row_b: usize,
    col_len: usize,
) -> f64 {
    let mut mul_tmp = 0.0;
    for col in 0..col_len {
        mul_tmp += (evm[row_a][col] - evm[row_b][col]).powi(2);
    }
    1.0 - (mul_tmp / 2.0).sqrt()
}
//计算各行的相似度,以及全部相似度的和,全部行的可信度
pub fn similarity_all_rows(evm: &Vec<Vec<f64>>) -> (Vec<Vec<f64>>, f64, Vec<f64>) {
    let row_len = evm.len();
    let col_len = evm[0].len();
    let mut total_sim = 0.0;
    let mut similarity_matrix = Vec::<Vec<f64>>::with_capacity(row_len);
    let mut row_cred_vec = Vec::<f64>::with_capacity(row_len);
    let tmp_vec = &mut Vec::<f64>::with_capacity(row_len);
    for a_row in 0..row_len {
        let mut tmp_cred = 0.0;
        tmp_vec.clear();
        for b_row in 0..row_len {
            let tmp_sim = similarity_between_rows(evm, a_row, b_row, col_len);
            tmp_vec.push(tmp_sim);
            tmp_cred += tmp_sim;
            total_sim += tmp_sim;
        }
        row_cred_vec.push(tmp_cred);
        similarity_matrix.push(tmp_vec.clone())
    }
    let total = total_sim;
    let credibility_degree = row_cred_vec.iter().map(|f| f / total).collect::<Vec<f64>>();
    (similarity_matrix, total, credibility_degree)
}

//计算所有完全冲突列的权重
pub fn col_weight(
    evm: &Vec<Vec<f64>>,
    credibility_degree: &Vec<f64>,
    complete_conflict_col: &Vec<usize>,
) -> HashMap<usize, f64> {
    let row_len = evm.len();
    let mut rst = HashMap::<usize, f64>::with_capacity(complete_conflict_col.len());
    if !complete_conflict_col.is_empty() {
        for &conf_col in complete_conflict_col {
            let mut weight_tmp = 0.0;
            for row in 0..row_len {
                weight_tmp += evm[row][conf_col] * credibility_degree[row];
            }
            rst.insert(conf_col, weight_tmp);
        }
    }
    rst
}

//融合证据
pub fn merge_all_single_event_evidence(
    classic_vec: &Vec<f64>,
    col_weight: &HashMap<usize, f64>,
) -> Vec<f64> {
    let col_len = classic_vec.len();
    let neg_sum_of_weight = 1.0 - col_weight.values().into_iter().fold(0.0, |a, c| a + c);
    let mut rst = Vec::<f64>::with_capacity(col_len);
    for col in 0..col_len {
        if col_weight.contains_key(&col) {
            rst.push(approx(col_weight[&col], 14));
        } else {
            rst.push(approx(neg_sum_of_weight * classic_vec[col], 14))
        }
    }
    rst
}
