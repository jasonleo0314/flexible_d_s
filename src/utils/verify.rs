use std::collections::HashSet;

use crate::com::input::InputData;
use crate::utils::matrix_calc::{get_shape_unchecked, sum_horizontal_unchecked};
use crate::utils::tools::{approx, approx_eq, calculate_hash};

//todo:增加一个验证条件：全集列必须在最后且只能有一个

pub fn is_legal_matrix<T>(mat: &Vec<Vec<T>>) -> (bool, &'static str) {
    if mat.len() == 0 {
        return (false, "矩阵行数为0");
    }
    let len_0 = mat[0].len();
    if len_0 == 0 {
        return (false, "矩阵列数位0");
    }
    let diff_len_row = mat.iter().map(|x| x.len()).filter(|&x| x != len_0).count();
    if diff_len_row > 0 {
        return (false, "矩阵列不整齐");
    }
    return (true, "矩阵合法");
}

//验证输入冲突证据的的合法性
pub fn strict_verify(idt: &InputData) -> (bool, String) {
    //验证矩阵形状是否合法
    if is_legal_matrix(&idt.evidence_matrix).0 == false {
        return (false, String::from("矩阵形状不合法"));
    }
    //验证行列数是否为空
    let (row, col) = get_shape_unchecked(&idt.evidence_matrix);
    if row < 2 {
        return (false, String::from("证据矩阵行数不得小于2"));
    }
    if col < 2 {
        return (false, String::from("证据矩阵列数不得小于2"));
    }
    //验证识别框架数量是否合法
    if idt.discernment.len() < 2 {
        return (false, String::from("识别框架不得少于两个元素"));
    }
    //“识别事件”数量是否合法
    if idt.fact_set_list.len() < 2 {
        return (false, String::from("“识别事件”不得少于两个元素"));
    }
    //所有传感器值∈[0,1]
    let illegal_focal = &idt
        .evidence_matrix
        .iter()
        .flat_map(|v| {
            v.iter().filter(|&&x| {
                let x = approx(x, 7);
                x > 1f64 || x < 0f64
            })
        })
        .collect::<Vec<&f64>>();
    if illegal_focal.len() > 0 {
        let alert_str = format!(
            "矩阵中元素不合法，所有证据值必须在[0,1]范围内，非法值为：{:?}",
            illegal_focal
        );
        return (false, alert_str);
    }

    //识别事件是否互斥
    let f1 = &idt.fact_set_list.clone();
    let f2 = f1
        .iter()
        .map(|h| {
            h.iter()
                .map(|r| calculate_hash(r.clone()))
                .fold(0u128, |a, c| a + (c as u128))
        })
        .collect::<HashSet<u128>>();
    if f2.len() != f1.len() {
        return (false, String::from("识别事件列表中存在重复项"));
    }

    //识别事件长度是否等于矩阵列数
    if col != idt.fact_set_list.len() {
        return (false, String::from("识别事件长度不等于矩阵列数"));
    }
    //识别事件是否完全由识别框架的元素构成
    let dis_set = idt.discernment.clone();
    let fact_lst = idt.fact_set_list.clone();
    for set_fi in fact_lst {
        if !set_fi.is_subset(&dis_set) {
            return (false, String::from("识别事件必须完全由识别框架的元素构成"));
        }
        if set_fi.is_empty() {
            return (
                false,
                String::from("识别事件不能为空集合，必须由识别框架中的元素构成"),
            );
        }
    }
    //任一MRow中所有Map的值之和为1
    let col_sum = &sum_horizontal_unchecked(&idt.evidence_matrix)
        .iter()
        .map(|&x| approx(x, 7))
        .collect::<Vec<f64>>();
    let illegal_cnt = col_sum.iter().filter(|&x| !approx_eq(*x, 1.0, 7)).count();
    if illegal_cnt > 0usize {
        return (
            false,
            format!(
                "任意一行的传感器接收的证据之和必须为1，各传感器的和为：{:?}",
                col_sum
            ),
        );
    }
    return (true, String::from("输入合法!"));
}
