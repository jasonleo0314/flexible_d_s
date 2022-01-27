//测试冲突证据模块
#[cfg(test)]
mod evidence_test {
    use crate::core::evidence::Evidence;
    use crate::utils::tools::s;
    use crate::{mat, set, set_list};
    use std::collections::HashSet;

    #[test]
    fn never_new_an_evidence_from_me_test() {
        let dcm: HashSet<String> = set! {s("a"), s("b"), s("c"), s("d")};
        let sl: Vec<HashSet<String>> = set_list![{s("a")}, {s("b")}, {s("c")}, {s("a"), s("b")}, {s("a"), s("b"), s("c"),s("d")}];
        let m: Vec<Vec<f64>> = mat![
            [0.1, 0.0, 0.3, 0.6, 0.0],
            [2.0, 0.4, 0.4, 0.0, 0.0],
            [7.0, 0.1, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0]
        ];
        let evd = Evidence::never_new_an_evidence_from_me(dcm.clone(), sl.clone(), m.clone());
        println!("{:?}", evd);
    }

    #[test]
    fn discernment_test() {
        let dcm: HashSet<String> = set! {s("a"), s("b"), s("c"), s("d")};
        let sl: Vec<HashSet<String>> = set_list![{s("a")}, {s("b")}, {s("c")}, {s("a"), s("b")}, {s("a"), s("b"), s("c"),s("d")}];
        let m: Vec<Vec<f64>> = mat![
            [0.1, 0.0, 0.3, 0.6, 0.0],
            [2.0, 0.4, 0.4, 0.0, 0.0],
            [7.0, 0.1, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0]
        ];
        let evd = Evidence::never_new_an_evidence_from_me(dcm.clone(), sl.clone(), m.clone());

        println!("{:?}", evd.discernment());
        assert_eq!(evd.discernment(), &dcm);
    }
    #[test]
    fn fact_set_list_test() {
        let dcm: HashSet<String> = set! {s("a"), s("b"), s("c"), s("d")};
        let sl: Vec<HashSet<String>> = set_list![{s("a")}, {s("b")}, {s("c")}, {s("a"), s("b")}, {s("a"), s("b"), s("c"),s("d")}];
        let m: Vec<Vec<f64>> = mat![
            [0.1, 0.0, 0.3, 0.6, 0.0],
            [2.0, 0.4, 0.4, 0.0, 0.0],
            [7.0, 0.1, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0]
        ];
        let evd = Evidence::never_new_an_evidence_from_me(dcm.clone(), sl.clone(), m.clone());
        println!("{:?}", evd.fact_set_list());
        assert_eq!(evd.fact_set_list(), &sl);
    }
    #[test]
    fn evidence_matrix_test() {
        let dcm: HashSet<String> = set! {s("a"), s("b"), s("c"), s("d")};
        let sl: Vec<HashSet<String>> = set_list![{s("a")}, {s("b")}, {s("c")}, {s("a"), s("b")}, {s("a"), s("b"), s("c"),s("d")}];
        let m: Vec<Vec<f64>> = mat![
            [0.1, 0.0, 0.3, 0.6, 0.0],
            [2.0, 0.4, 0.4, 0.0, 0.0],
            [7.0, 0.1, 0.2, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0]
        ];
        let evd = Evidence::never_new_an_evidence_from_me(dcm.clone(), sl.clone(), m.clone());
        println!("{:?}", evd.evidence_matrix());
        assert_eq!(evd.evidence_matrix(), &m);
    }

    #[test]
    fn fuse_test() {}
}
