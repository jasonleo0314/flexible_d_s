#[cfg(test)]
pub mod integrate_dempster {
    use crate::com::dempster::create_dempster;
    use crate::com::input::InputData;
    use crate::utils::tools::{fp, s, show_mat};
    use crate::{mat, set, set_list};
    use std::collections::HashSet;

    #[test]
    fn integrate_test_1() {
        let dis0 = set! {s("b"),s("s"),s("h")};
        let fact0 = set_list![{s("b")},{s("s")},{s("h")},{s("b"),s("s")},{s("b"),s("s"),s("h")}];
        let mat0 = mat![[0.4, 0.3, 0.1, 0.1, 0.1], [0.2, 0.2, 0.05, 0.5, 0.05]];
        common_progress(dis0, fact0, mat0);
    }

    #[test]
    fn integrate_test_2() {
        //设置数据
        let dis0 = set! {s("p"),s("l"),s("m")};
        let fact0 = set_list![{ s("p") }, { s("l") }, { s("m") }];
        let mat0 = mat![[0.99, 0.01, 0.0], [0.0, 0.01, 0.99]];
        common_progress(dis0, fact0, mat0);
    }

    #[test]
    fn integrate_test_3() {
        //设置数据
        let dis0 = set! {s("a"),s("b"),s("c")};
        let fact0 =
            set_list![{ s("a") }, { s("b") }, { s("c") },{s("a"),s("c")},{s("a"),s("b"),s("c")}];
        let mat0 = mat![
            [0.6, 0.1, 0.3, 0.0, 0.0],
            [0.0, 0.8, 0.2, 0.0, 0.0],
            [0.55, 0.1, 0.35, 0.0, 0.0],
            [0.7, 0.1, 0.2, 0.0, 0.0],
            [0.0, 0.1, 0.0, 0.9, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0]
        ];
        common_progress(dis0, fact0, mat0);
    }

    fn common_progress(dis0: HashSet<String>, fact0: Vec<HashSet<String>>, mat0: Vec<Vec<f64>>) {
        //构建证据
        let evd = InputData::new(dis0, fact0, mat0).build_evidence().unwrap();
        //构建Dempter
        let mut dmst = create_dempster();
        //开展运算
        let rst = dmst.run(&evd);
        //打印结果
        fp(rst);
        //打印连乘连加元素集合
        show_mat(&dmst.mul_plus_empty_sheet);
    }
}
