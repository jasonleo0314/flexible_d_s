use crate::algo::dempster_classic::{
    check_combination_on_set, get_empty_intersection, get_x_intersection, plus_mul_cell_matrix,
};
use crate::utils::tools::fp;

#[test]
fn check_combination_on_set_test() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

    let dcm: HashSet<usize> = set! {5, 55, 555, 5555};
    let sl: Vec<HashSet<usize>> = set_list![{5}, {55}, {555}, {5, 55}, {5, 55, 555, 5555}];
    let m: Vec<Vec<f64>> = mat![
        [0.1, 0.0, 0.3, 0.6, 0.0],
        [0.2, 0.4, 0.4, 0.0, 0.0],
        [0.7, 0.1, 0.2, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0]
    ];

    let evd = &EvidenceMatrix::new(dcm, sl, m);

    let rst = check_combination_on_set(evd, |x| x.is_empty());

    fp(rst.len())
}

#[test]
fn get_empty_intersection_test() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

    let dcm: HashSet<usize> = set! {5, 55, 555, 5555};
    let sl: Vec<HashSet<usize>> = set_list![{5}, {55}, {555}, {5, 55}, {5, 55, 555, 5555}];
    let m: Vec<Vec<f64>> = mat![
        [0.1, 0.0, 0.3, 0.6, 0.0],
        [0.2, 0.4, 0.4, 0.0, 0.0],
        [0.7, 0.1, 0.2, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0]
    ];

    let evd = &EvidenceMatrix::new(dcm, sl, m);

    let rst = get_empty_intersection(evd);

    fp(rst.len())
}

#[test]
fn get_x_intersection_test() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

    let dcm: HashSet<usize> = set! {5, 55, 555, 5555};
    let sl: Vec<HashSet<usize>> = set_list![{5}, {55}, {555}, {5, 55}, {5, 55, 555, 5555}];
    let m: Vec<Vec<f64>> = mat![
        [0.1, 0.0, 0.3, 0.6, 0.0],
        [0.2, 0.4, 0.4, 0.0, 0.0],
        [0.7, 0.1, 0.2, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0]
    ];

    let evd = &EvidenceMatrix::new(dcm, sl, m);

    let rst = get_x_intersection(evd, 0);

    fp(rst.len())
}

#[test]
fn plus_mul_cell_matrix_test() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

    let dcm: HashSet<usize> = set! {5, 55, 555, 5555};
    let sl: Vec<HashSet<usize>> = set_list![{5}, {55}, {555}, {5, 55}, {5, 55, 555, 5555}];
    let m: Vec<Vec<f64>> = mat![
        [0.1, 0.0, 0.3, 0.6, 0.0],
        [0.2, 0.4, 0.4, 0.0, 0.0],
        [0.7, 0.1, 0.2, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0]
    ];

    let evd = &EvidenceMatrix::new(dcm, sl, m);

    let rst = get_x_intersection(evd, 0);

    let mul_sum = plus_mul_cell_matrix(&rst);

    fp(mul_sum)
}

#[test]
fn dempster_merge_test1() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

    let dcm: HashSet<usize> = set! {1, 2,3};
    let sl: Vec<HashSet<usize>> = set_list![{1}, {2},{3},{1,2},{2,3},{1,3}];
    let m: Vec<Vec<f64>> = mat![
        [0.5, 0.0, 0.0, 0.3, 0.2, 0.0],
        [0.8, 0.0, 0.0, 0.0, 0.1, 0.1],
        [0.6, 0.2, 0.0, 0.0, 0.0, 0.2],
        [0.7, 0.1, 0.0, 0.0, 0.0, 0.2],
        [0.8, 0.1, 0.0, 0.0, 0.1, 0.0]
    ];

    let evd = &EvidenceMatrix::new(dcm, sl.clone(), m);
    let rst = dempster_merge(evd);
    println!("事件序列为：{:?}\n真的概率为：\n {:?}", sl, rst);
}

#[test]
fn dempster_merge_test2() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

    let dcm: HashSet<usize> = set! {0, 1, 2};
    let sl: Vec<HashSet<usize>> = set_list![{ 0 }, { 1 }, { 2 }];
    let m: Vec<Vec<f64>> = mat![[0.999, 0.001, 0.0], [0.0, 0.001, 0.999]];

    let evd = &EvidenceMatrix::new(dcm, sl.clone(), m);
    let rst = dempster_merge(evd);
    println!("事件序列为：{:?}\n真的概率为：\n {:?}", sl, rst);
}

#[test]
fn dempster_merge_test3() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

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
    let evd = &EvidenceMatrix::new(dcm, sl.clone(), m);
    let rst = dempster_merge(evd);
    println!("事件序列为：{:?}\n真的概率为：\n {:?}", sl, rst);
}

#[test]
fn dempster_merge_test4() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;
    let dcm: HashSet<usize> = set! {1,2,3};
    let sl: Vec<HashSet<usize>> = set_list![{ 1 }, { 2 },{ 3 },{ 1 , 2 } , { 1,2,3}];
    let m: Vec<Vec<f64>> = mat![[0.4, 0.3, 0.1, 0.1, 0.1], [0.2, 0.2, 0.05, 0.5, 0.05]];
    let evd = &EvidenceMatrix::new(dcm, sl.clone(), m);
    let rst = dempster_merge(evd);
    println!("事件序列为：{:?}\n真的概率为：\n {:?}", sl, rst);
}

#[test]
fn dempster_merge_test_peter_paul_marry() {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;
    let dcm: HashSet<usize> = set! {1,2,3};
    let sl: Vec<HashSet<usize>> = set_list![{ 1 }, { 2 },{ 3 },{ 1,2,3}];
    let m: Vec<Vec<f64>> = mat![[0.98, 0.01, 0.0, 0.01], [0.0, 0.01, 0.98, 0.01]];
    let evd = &EvidenceMatrix::new(dcm, sl.clone(), m);
    let rst = dempster_merge(evd);
    println!("事件序列为：{:?}\n真的概率为：\n {:?}", sl, rst);
}
