#[cfg(test)]
#[macro_use]
pub mod evidence_matrix_test {
    use crate::obj::evidence_matrix::EvidenceMatrix;
    use crate::*;
    use std::collections::HashSet;

    //构造函数
    #[test]
    fn new_test() {
        let dcm: HashSet<usize> = set! {0, 1, 2, 3};
        let sl: Vec<HashSet<usize>> = set_list![{0}, {1}, {2}, {0, 1}, {1, 2, 3, 0}];
        let m: Vec<Vec<f64>> = mat![
            [0.1, 0.0, 0.3, 0.6, 0.0],
            [0.2, 0.4, 0.4, 0.0, 0.0],
            [0.7, 0.1, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0]
        ];
        let _ = EvidenceMatrix::new(dcm, sl, m);
    }

    //构造函数
    #[test]
    fn set_n_base_trans_matrix_to_event_test() {
        let dcm: HashSet<usize> = set! {5, 55, 555, 5555};
        let sl: Vec<HashSet<usize>> = set_list![{5}, {55}, {555}, {5, 55}, {5, 55, 555, 5555}];
        let m: Vec<Vec<f64>> = mat![
            [0.1, 0.0, 0.3, 0.6, 0.0],
            [0.2, 0.4, 0.4, 0.0, 0.0],
            [0.7, 0.1, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0]
        ];
        let evd = EvidenceMatrix::new(dcm, sl, m);
        let _ = evd.n_base_cell_matrix();
    }
}
