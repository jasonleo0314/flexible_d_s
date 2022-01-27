use std::collections::{HashMap, HashSet};
use std::time::Instant;

use crate::algo::complete_conflict::run_flexible_complete_conflict_algo;
use crate::algo::dempster_classic::dempster_merge;
use crate::algo::fast_algo::{
    col_weight, complete_conflict_col, independent_event_evidence_merge,
    independent_event_non_zero_evidence_merge, merge_all_single_event_evidence,
    similarity_all_rows,
};
use crate::com::input::InputData;
use crate::obj::evidence_matrix::EvidenceMatrix;
use crate::utils::tools::show_mat;
use crate::{mat, set, set_list};

//验证快速融合算法与经典D-S算法结论一致性
#[test]
pub fn complete_conflict_algo_and_classic_test() {
    let dcm: HashSet<usize> = set! {1,2,3};
    let sl: Vec<HashSet<usize>> = set_list![{ 1 }, { 2 }, { 3 }];
    let m: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3],
        [0.1, 0.7, 0.2],
        [0.55, 0.1, 0.35],
        [0.7, 0.1, 0.2],
    ];
    let a = Instant::now();
    let rst = independent_event_non_zero_evidence_merge(&m);
    println!(
        "independent_event_non_zero_evidence_merge:{:?}\n{:?}",
        rst,
        a.elapsed()
    );
    assert_eq!(rst, vec![0.825, 0.025, 0.15]);
    let idt = InputData {
        discernment: dcm,
        fact_set_list: sl,
        evidence_matrix: m,
    };
    let evd = idt.build_evidence(true).unwrap();
    let b = Instant::now();
    let rst_classic = dempster_merge(&evd);
    println!("rst_classic:{:?}\n{:?}", rst_classic, b.elapsed());
    assert_eq!(rst_classic, vec![0.82499999999999, 0.025, 0.15]);
}

//快速证据融合算法1
#[test]
pub fn independent_event_non_zero_evidence_merge_test1() {
    let m: Vec<Vec<f64>> = mat![
        [
            0.06510659, 0.06573851, 0.12665746, 0.05021125, 0.07782061, 0.10464535, 0.0660793,
            0.06741212, 0.0840469, 0.09755791, 0.10243446, 0.03683799, 0.05545155
        ],
        [
            0.13501804, 0.12185775, 0.1168829, 0.09882403, 0.03588922, 0.07736293, 0.09352667,
            0.0262496, 0.00558852, 0.10013514, 0.00898109, 0.0865463, 0.09313781
        ],
        [
            0.02934601, 0.08821653, 0.19303869, 0.05867143, 0.03068344, 0.02407283, 0.11110469,
            0.12582071, 0.06858059, 0.12294115, 0.04603515, 0.03952192, 0.06196686
        ],
        [
            0.12235519, 0.13091149, 0.05545477, 0.14482497, 0.07009327, 0.12112511, 0.02243756,
            0.02003462, 0.02035168, 0.03666706, 0.08515579, 0.05082101, 0.11976748
        ],
        [
            0.13768582, 0.11776836, 0.03672264, 0.00220011, 0.03579828, 0.00213265, 0.0267763,
            0.14137695, 0.17298062, 0.03204264, 0.14888029, 0.14527486, 0.00036048
        ],
        [
            0.12467423, 0.08727341, 0.00746833, 0.144021, 0.07283735, 0.10331052, 0.07228151,
            0.07047047, 0.1, 0.10535049, 0.0, 0.02501269, 0.0873
        ],
        [
            0.07899021, 0.09288628, 0.01287085, 0.04177601, 0.1188582, 0.0, 0.1218533, 0.09945631,
            0.07983702, 0.01481047, 0.1155513, 0.13, 0.09311
        ],
        [
            0.05710916, 0.10048078, 0.03694499, 0.15144632, 0.09042658, 0.08097990, 0.04592522,
            0.03702343, 0.07381645, 0.14620439, 0.02515228, 0.07567938, 0.07881108
        ],
        [
            0.06510659, 0.06573851, 0.12665746, 0.05021125, 0.07782061, 0.10464535, 0.0660793,
            0.06741212, 0.0840469, 0.09755791, 0.10243446, 0.03683799, 0.05545155
        ],
        [
            0.13501804, 0.12185775, 0.1168829, 0.09882403, 0.03588922, 0.07736293, 0.09352667,
            0.0262496, 0.00558852, 0.10013514, 0.00898109, 0.0865463, 0.09313781
        ],
        [
            0.02934601, 0.08821653, 0.19303869, 0.05867143, 0.03068344, 0.02407283, 0.11110469,
            0.12582071, 0.06858059, 0.12294115, 0.04603515, 0.03952192, 0.06196686
        ],
        [
            0.12235519, 0.13091149, 0.05545477, 0.14482497, 0.07009327, 0.12112511, 0.02243756,
            0.02003462, 0.02035168, 0.03666706, 0.08515579, 0.05082101, 0.11976748
        ],
        [
            0.13768582, 0.11776836, 0.03672264, 0.00220011, 0.03579828, 0.00213265, 0.0267763,
            0.14137695, 0.17298062, 0.03204264, 0.14888029, 0.14527486, 0.00036048
        ],
        [
            0.12467423, 0.08727341, 0.00746833, 0.144021, 0.07283735, 0.10331052, 0.07228151,
            0.07047047, 0.1, 0.10535049, 0.0, 0.02501269, 0.0873
        ],
        [
            0.07899021, 0.09288628, 0.01287085, 0.04177601, 0.1188582, 0.0, 0.1218533, 0.09945631,
            0.07983702, 0.01481047, 0.1155513, 0.13, 0.09311
        ],
        [
            0.05710916, 0.10048078, 0.03694499, 0.15144632, 0.09042658, 0.08097990, 0.04592522,
            0.03702343, 0.07381645, 0.14620439, 0.02515228, 0.07567938, 0.07881108
        ]
    ];
    let a = Instant::now();
    let rst = independent_event_non_zero_evidence_merge(&m);
    println!("{:?}\n{:?}", rst, a.elapsed());
    assert_eq!(
        rst,
        vec![
            0.07029692773025,
            0.92674832332011,
            5.02620771e-6,
            8.407158997e-5,
            0.00033345034817,
            0.0,
            0.00032765781131,
            0.00031509927436,
            5.255570426e-5,
            0.00121927861232,
            0.0,
            0.00061668745803,
            9.219435e-7
        ]
    )
}

//结合律是否适用
#[test]
fn combine_fit() {
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
    let idt = InputData {
        discernment: dcm.clone(),
        fact_set_list: sl.clone(),
        evidence_matrix: m.clone(),
    };
    let evd_wrapped = &idt.build_evidence(true).unwrap();
    let rst0 = run_flexible_complete_conflict_algo(evd_wrapped);
    let mc = m.clone();
    let m1 = mc[0].clone();
    let m_rest = m[1..].to_vec();
    let rst1 = m_rest.iter().fold(m1, |a, c| {
        let mut rows = Vec::new();
        rows.push(a);
        rows.push(c.clone());
        let idt2 = InputData {
            discernment: dcm.clone(),
            fact_set_list: sl.clone(),
            evidence_matrix: rows,
        };
        let evd_wrapped2 = &idt2.build_evidence(true).unwrap();
        let rst = run_flexible_complete_conflict_algo(evd_wrapped2);
        rst
    });
    assert_ne!(rst0, rst1);
}

//如果所有事件event均只有一个元素为独立事件，且事件之间互斥,可直接应用该函数
#[test]
pub fn independent_event_non_zero_evidence_merge_test() {
    let m: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3, 0.0, 0.0],
        [0.0, 0.8, 0.2, 0.0, 0.0],
        [0.55, 0.1, 0.35, 0.0, 0.0],
        [0.7, 0.1, 0.2, 0.0, 0.0],
        [0.0, 0.1, 0.0, 0.9, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0]
    ];
    let a = Instant::now();
    let rst = independent_event_non_zero_evidence_merge(&m);
    println!("{:?}\n{:?}", rst, a.elapsed());
    assert_eq!(rst, vec![0.0, 0.0, 0.0, 0.0, 0.0])
}

#[test]
pub fn complete_conflict_col_test() {
    let m: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3],
        [0.0, 0.8, 0.2],
        [0.55, 0.1, 0.35],
        [0.7, 0.1, 0.2],
    ];
    let a = Instant::now();
    let rst = complete_conflict_col(&m);
    println!("{:?}\n{:?}", rst, a.elapsed());
    assert_eq!(rst, vec![0])
}

#[test]
pub fn similarity_all_rows_test() {
    let m: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3],
        [0.0, 0.8, 0.2],
        [0.55, 0.1, 0.35],
        [0.7, 0.1, 0.2],
    ];
    let a = Instant::now();
    let (sim, sum, cred) = &similarity_all_rows(&m);
    let rst_mat = mat![
        [1.0, 0.34425614756979994, 0.9500000000000001, 0.9],
        [
            0.34425614756979994,
            1.0,
            0.36164273325981466,
            0.30000000000000004
        ],
        [
            0.9500000000000001,
            0.36164273325981466,
            1.0,
            0.8500000000000001
        ],
        [0.9, 0.30000000000000004, 0.8500000000000001, 1.0]
    ];
    show_mat(sim);
    println!("{:?}\n{:?}", cred, a.elapsed());
    println!("{:?}\n{:?}", sum, a.elapsed());
    assert_eq!(sim, &rst_mat);
    assert_eq!(
        cred.clone(),
        vec![
            0.2799082330657574,
            0.1757741350419765,
            0.27705036483227374,
            0.2672672670599923
        ]
    );
    assert_eq!(sum.clone(), 11.41179776165923);
}

#[test]
pub fn col_weight_test() {
    let evm: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3],
        [0.0, 0.8, 0.2],
        [0.55, 0.1, 0.35],
        [0.7, 0.1, 0.2],
    ];
    let a = Instant::now();
    let conflict_cols = &complete_conflict_col(&evm);
    let (_, _, cred) = &similarity_all_rows(&evm);
    let weight_list = &col_weight(&evm, cred, conflict_cols);
    println!("{:?}\n{:?}", weight_list, a.elapsed());
    let mut rst_map = HashMap::with_capacity(1);
    rst_map.insert(0, 0.5074097274391997);
    assert_eq!(weight_list, &rst_map)
}

#[test]
pub fn merge_all_single_event_evidence_test() {
    let evm: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3],
        [0.0, 0.8, 0.2],
        [0.55, 0.1, 0.35],
        [0.7, 0.1, 0.2],
    ];
    let a = Instant::now();
    let conflict_cols = &complete_conflict_col(&evm);
    let (_, _, cred) = &similarity_all_rows(&evm);
    let weight_list = &col_weight(&evm, cred, conflict_cols);
    let classic = &independent_event_non_zero_evidence_merge(&evm);
    let rst = merge_all_single_event_evidence(classic, weight_list);
    println!("{:?}\n{:?}", rst, a.elapsed());
    assert_eq!(
        rst,
        vec![0.5074097274392, 0.07881444360973, 0.41377582895107]
    )
}

#[test]
pub fn independent_event_evidence_merge_test() {
    let dcm: HashSet<usize> = set! {1,2,3};
    let sl: Vec<HashSet<usize>> = set_list![{ 1 }, { 2 }, { 3 }];
    let evm: Vec<Vec<f64>> = mat![
        [0.6, 0.1, 0.3],
        [0.0, 0.8, 0.2],
        [0.55, 0.0, 0.45],
        [0.7, 0.1, 0.2],
    ];
    let a = Instant::now();
    let rst = independent_event_evidence_merge(&evm);
    println!("简易预测：{:?}\n{:?}", rst, a.elapsed());
    assert_eq!(
        rst,
        vec![0.50792692120098, 0.19668640771755, 0.29538667108147]
    );
    test_template(
        dcm,
        sl,
        evm,
        vec![0.507926921200982, 0.196686407717553, 0.29538667108148],
        run_flexible_complete_conflict_algo,
    );
}

#[test]
pub fn independent_event_evidence_merge_test2() {
    let dcm: HashSet<usize> = set! {0, 1, 2};
    let sl: Vec<HashSet<usize>> = set_list![{ 0 }, { 1 }, { 2 }];
    let m: Vec<Vec<f64>> = mat![[0.999, 0.001, 0.0], [0.0, 0.001, 0.999]];
    let a = Instant::now();
    let rst = independent_event_evidence_merge(&m);
    println!("简易预测：{:?}\n{:?}", rst, a.elapsed());
    assert_eq!(rst, vec![0.4995, 0.001, 0.4995]);
    test_template(
        dcm,
        sl,
        m,
        vec![0.4995, 0.00099999999997, 0.4995],
        run_flexible_complete_conflict_algo,
    );
}

#[test]
fn bench_test2() {
    let m: Vec<Vec<f64>> = mat![
        [
            0.06510659, 0.06573851, 0.12665746, 0.05021125, 0.07782061, 0.10464535, 0.0660793,
            0.06741212, 0.0840469, 0.09755791, 0.10243446, 0.03683799, 0.05545155
        ],
        [
            0.13501804, 0.12185775, 0.1168829, 0.09882403, 0.03588922, 0.07736293, 0.09352667,
            0.0262496, 0.00558852, 0.10013514, 0.00898109, 0.0865463, 0.09313781
        ],
        [
            0.02934601, 0.08821653, 0.19303869, 0.05867143, 0.03068344, 0.02407283, 0.11110469,
            0.12582071, 0.06858059, 0.12294115, 0.04603515, 0.03952192, 0.06196686
        ],
        [
            0.12235519, 0.13091149, 0.05545477, 0.14482497, 0.07009327, 0.12112511, 0.02243756,
            0.02003462, 0.02035168, 0.03666706, 0.08515579, 0.05082101, 0.11976748
        ],
        [
            0.13768582, 0.11776836, 0.03672264, 0.00220011, 0.03579828, 0.00213265, 0.0267763,
            0.14137695, 0.17298062, 0.03204264, 0.14888029, 0.14527486, 0.00036048
        ],
        [
            0.12467423, 0.08727341, 0.00746833, 0.144021, 0.07283735, 0.10331052, 0.07228151,
            0.07047047, 0.1, 0.10535049, 0.0, 0.02501269, 0.0873
        ],
        [
            0.07899021, 0.09288628, 0.01287085, 0.04177601, 0.1188582, 0.0, 0.1218533, 0.09945631,
            0.07983702, 0.01481047, 0.1155513, 0.13, 0.09311
        ],
        [
            0.05710916, 0.10048078, 0.03694499, 0.15144632, 0.09042658, 0.08097990, 0.04592522,
            0.03702343, 0.07381645, 0.14620439, 0.02515228, 0.07567938, 0.07881108
        ]
    ];
    let a = Instant::now();
    let rst = independent_event_evidence_merge(&m);
    println!("简易预测：{:?}\n{:?}", rst, a.elapsed());
    assert_eq!(
        rst,
        vec![
            0.1692910148363,
            0.61467654349434,
            0.00143148219988,
            0.00585450749625,
            0.01165953750013,
            0.06493240301621,
            0.01155782192628,
            0.01133416256703,
            0.00462887726942,
            0.02229549987422,
            0.06586889875408,
            0.01585617002002,
            0.00061308104583
        ]
    );
}

fn test_template<F: Fn(&EvidenceMatrix) -> Vec<f64>>(
    discernment: HashSet<usize>,
    fact_set_list: Vec<HashSet<usize>>,
    evidence_matrix: Vec<Vec<f64>>,
    assert_vec: Vec<f64>,
    algo: F,
) {
    let idt = InputData {
        discernment,
        fact_set_list,
        evidence_matrix,
    };
    let evd_wrapped = idt.build_evidence(true);
    match evd_wrapped {
        None => {
            println!("输入不合法");
        }
        Some(evd) => {
            //调用evd开展运算
            //以下内容为占位符
            let b = Instant::now();
            let rst = algo(&evd);
            println!("复杂预测：{:?}", b.elapsed());
            println!("{:?}", rst);
            assert_eq!(assert_vec, rst);
        }
    };
}
