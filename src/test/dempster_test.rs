#[cfg(test)]
mod dempster_tests {
    use crate::com::dempster::create_dempster;
    use crate::com::dempster_sub_focal_to_mul::FocalToMul;
    use crate::com::input::InputData;
    use crate::core::evidence::Evidence;
    use crate::utils::tools::{fp, s, show_mat};
    use crate::{mat, set, set_list};
    use std::collections::HashSet;

    fn get_data() -> (HashSet<String>, Vec<HashSet<String>>, Vec<Vec<f64>>) {
        let dcm: HashSet<String> = set! {s("a"), s("b"), s("c"), s("d")};
        let sl: Vec<HashSet<String>> = set_list![{s("a")}, {s("b")}, {s("c")}, {s("a"), s("b")}, {s("a"), s("b"), s("c"),s("d")}];
        let m: Vec<Vec<f64>> = mat![
            [0.1, 0.2, 0.3, 0.4, 0.0],
            [0.01, 0.11, 0.21, 0.31, 0.36],
            [0.001, 0.011, 0.021, 0.031, 0.936],
            [0.0001, 0.0011, 0.0021, 0.0031, 0.9936]
        ];
        (dcm, sl, m)
    }

    fn init_evidence() -> Evidence {
        let (d, ss, m) = get_data();
        let ipt: InputData = InputData {
            discernment: d.clone(),
            fact_set_list: ss.to_owned(),
            evidence_matrix: m,
        };
        let evd = ipt.build_evidence().unwrap();
        evd
    }

    #[test]
    fn import_evidence_matrix_test() {
        let (_, _, mms) = get_data();
        let mut dmst = create_dempster();
        let evd = init_evidence();
        dmst.import_evidence_matrix(&evd);
        assert_eq!(dmst.evd_mat, mms);
    }
    #[test]
    fn set_bi_mat_test() {
        let mut dmst = create_dempster();
        let evd = init_evidence();
        dmst.import_evidence_matrix(&evd);
        dmst.set_bi_mat();
        show_mat(&dmst.evd_mat);
        println!("\n");
        show_mat(&dmst.bi_mat);
        let bim = mat![
            [1, 1, 1, 1, 0],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1]
        ];
        assert_eq!(&dmst.bi_mat, &bim);
    }
    #[test]
    fn set_fact_combine_sheet_test() {
        let (_, ss, m1) = get_data();
        let mut dmst = create_dempster();
        let evd = init_evidence();
        dmst.set_fact_combine_inter_empty_sheet(&evd);
        let rst = dmst.fact_combine_inter_empty_sheet;
        show_mat(&m1);
        fp(ss);
        show_mat(&rst);
    }
    #[test]
    fn print_focal_to_mul_test() {
        let fm = FocalToMul {
            fitted_fact_list: Vec::new(),
            single_detector_list: Vec::new(),
            choice_detector_list: Vec::new(),
            single_focal_list: Vec::new(),
            choice_focal_sheet: Vec::new(),
            mul_plus_sheet_unit: Vec::new(),
        };
        println!("{:#?}", fm);
    }

    #[test]
    fn set_focal_to_mul_unit_empty_list_test() {
        let (_, _, _) = get_data();
        let mut dmst = create_dempster();
        let evd = init_evidence();
        dmst.import_evidence_matrix(&evd);
        dmst.set_bi_mat();
        dmst.set_fact_combine_inter_empty_sheet(&evd);
        dmst.set_focal_to_mul_unit_empty_list(&evd);
        let rst = dmst.focal_to_mul_unit_empty_list;
        fp(&rst)
    }

    #[test]
    fn set_mul_plus_empty_sheet_test() {
        let (_, _, _) = get_data();
        let mut dmst = create_dempster();
        let evd = init_evidence();
        dmst.import_evidence_matrix(&evd);
        dmst.set_bi_mat();
        dmst.set_fact_combine_inter_empty_sheet(&evd);
        dmst.set_focal_to_mul_unit_empty_list(&evd);
        dmst.set_mul_plus_empty_sheet();
        let rst = dmst.mul_plus_empty_sheet;
        show_mat(&rst);
    }
}
