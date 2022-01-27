use crate::algo::complete_conflict::{
    cart_event_intersect, col_weight_vec, detector_credibility_upper, detector_credibility_vec,
    final_result, find_fathers_of_col, find_inter, find_set_theta_solo_row,
    get_classic_dempster_result, get_compatible_redistribute_vec, get_neg_delta, get_similarity,
    measure_detector_similarity, pick_complete_conflict_event, run_flexible_complete_conflict_algo,
    scan_rows_in_one_col, sum_all_detector_similarity,
};
use crate::obj::evidence_matrix::EvidenceMatrix;
use crate::utils::tools::{fp, show_mat};
use crate::*;
use std::collections::HashSet;

//初始化一个证据矩阵
fn init_evidence_matrix() -> EvidenceMatrix {
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
    let evd = EvidenceMatrix::new(dcm, sl.clone(), m);
    evd
}

//run_flexible_complete_conflict_algo
#[test]
fn run_flexible_complete_conflict_algo_test() {
    let mt = &init_evidence_matrix();
    let rst = run_flexible_complete_conflict_algo(&mt);
    fp(rst)
}

#[test]
fn find_inter_test() {
    let test_data_list = vec![
        (0, vec![0, 3, 4]),
        (1, vec![1, 4]),
        (2, vec![2, 3, 4]),
        (3, vec![0, 2, 3, 4]),
        (4, vec![0, 1, 2, 3, 4]),
    ];
    let mt = &init_evidence_matrix();
    test_data_list.iter().for_each(|(input, out)| {
        assert_eq!(find_inter(&mt.event_vector(), *input), *out);
    });
}

#[test]
fn scan_rows_in_one_col_test() {
    let test_data_vec = vec![(0, true), (1, false), (2, false), (3, false), (4, false)];
    let mt = &init_evidence_matrix();
    test_data_vec.iter().for_each(|(in0, out0)| {
        assert_eq!(
            scan_rows_in_one_col(&mt.event_vector(), &mt.detect_matrix(), *in0),
            *out0
        )
    })
}
#[test]
fn pick_complete_conflict_event_test() {
    let mt = &init_evidence_matrix();
    let hs = pick_complete_conflict_event(mt);
    assert_eq!(hs, set! {0});
}
#[test]
fn cart_event_intersect_test() {
    let mt = &init_evidence_matrix();
    let cart = cart_event_intersect(mt);
    fp(cart)
}
#[test]
fn find_set_theta_solo_row_test() {
    let mt = &init_evidence_matrix();
    let rst = find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    fp(rst)
}
#[test]
fn measure_detector_similarity_test() {
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = measure_detector_similarity(mt, ig_cell);
    show_mat(&sim);
}
#[test]
fn get_similarity_test() {
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = measure_detector_similarity(mt, ig_cell);
    let sim0 = get_similarity(&sim, 3, 4);
    assert_eq!(sim0, 0.48521849295065);
}
#[test]
fn sum_all_detector_similarity_test() {
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = &measure_detector_similarity(mt, ig_cell);
    let sum = sum_all_detector_similarity(sim);
    assert_eq!(sum, 15.988442382607948);
}
#[test]
fn detector_credibility_upper_test() {
    let test_data_list = vec![
        (0, 3.719914498544543),
        (1, 2.244321570243224),
        (2, 3.700665510395171),
        (3, 3.53521849295065),
        (4, 2.788322310474358),
        (5, 0.0),
    ];
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = &measure_detector_similarity(mt, ig_cell);
    test_data_list.iter().for_each(|(i, o)| {
        let crd = detector_credibility_upper(sim, ig_cell, *i);
        assert_eq!(crd, *o);
    })
}
#[test]
fn detector_credibility_vec_test() {
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = &measure_detector_similarity(mt, ig_cell);
    let crd = detector_credibility_vec(sim, ig_cell);
    assert_eq!(
        crd,
        vec![
            3.719914498544543,
            2.244321570243224,
            3.700665510395171,
            3.53521849295065,
            2.788322310474358,
            0.0
        ]
    );
}
#[test]
fn col_weight_vec_test() {
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = &measure_detector_similarity(mt, ig_cell);
    let crd = &detector_credibility_vec(sim, ig_cell);
    let total = sum_all_detector_similarity(sim);
    let k = col_weight_vec(mt.detect_matrix(), crd, total);
    println!("{:?}", k);
    //此处解落顺序和论文不一致，1和3顺序颠倒，经查，论文错误，教授抄串行了
    assert_eq!(
        k,
        vec![
            0.421677578939357,
            0.198260046949864,
            0.22310586644267,
            0.156956507668109,
            0.0
        ]
    );
}
#[test]
fn find_fathers_of_col_test() {
    let mt = &init_evidence_matrix();
    let a = find_fathers_of_col(mt.event_vector());
    show_mat(&a);
    let as_rst = mat![[0, 3, 4], [1, 4], [2, 3, 4], [3, 4], [4]];
    assert_eq!(a, as_rst);
}
#[test]
fn get_compatible_redistribute_vec_test() {
    //因为上一步骤的列权重论文中有错误，所以本步骤依赖的列权重论文中全部组合错误
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = &measure_detector_similarity(mt, ig_cell);
    let crd = &detector_credibility_vec(sim, ig_cell);
    let total = sum_all_detector_similarity(sim);
    let col_weight_vec = &col_weight_vec(mt.detect_matrix(), crd, total);
    let father_vec = &find_fathers_of_col(mt.event_vector());
    let rst = get_compatible_redistribute_vec(mt.event_vector(), father_vec, col_weight_vec);
    fp(rst)
}

#[test]
fn get_neg_delta_test() {
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
    let sim = &measure_detector_similarity(mt, ig_cell);
    let crd = &detector_credibility_vec(sim, ig_cell);
    let total = sum_all_detector_similarity(sim);
    let col_weight_vec = &col_weight_vec(mt.detect_matrix(), crd, total);
    let father_vec = &find_fathers_of_col(mt.event_vector());
    let dist = &get_compatible_redistribute_vec(mt.event_vector(), father_vec, col_weight_vec);
    let comlete_conf = &pick_complete_conflict_event(mt);
    let rst = get_neg_delta(comlete_conf, dist);
    fp(rst)
}

//final_result
#[test]
fn final_result_test() {
    let mt = &init_evidence_matrix();
    let ig_cell = &find_set_theta_solo_row(mt.detect_matrix(), mt.discernment());
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
    fp(rst)
}
