//测试 input 时相关的验证和构造函数
#[cfg(test)]
mod input_tests {
    use std::collections::HashSet;

    use crate::{mat, set, set_list};
    //导入类型和别名
    use crate::com::input::InputData;
    use crate::utils::matrix_calc::{
        sum_horizontal_checked, sum_horizontal_unchecked, sum_vertical_checked,
        sum_vertical_unchecked,
    };
    use crate::utils::tools::{approx, approx_eq};
    use crate::utils::verify::{is_legal_matrix, strict_verify};

    #[test]
    fn input_test() {
        use crate::mat;
        use crate::set;
        use crate::set_list;
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
            discernment: dcm,
            fact_set_list: sl,
            evidence_matrix: m,
        };
        idt.show();
    }

    //矩阵合法性校验测试
    #[test]
    pub fn is_legal_matrix_test() {
        use crate::mat;
        //矩阵行数为0
        let m1: Vec<Vec<i32>> = Vec::new();
        let r1 = is_legal_matrix(&m1);
        println!("{:?}", r1);
        assert_eq!(r1, (false, "矩阵行数为0"));
        //矩阵列不整齐
        let m2: Vec<Vec<i32>> = mat![[1], [1, 2]];
        let r2 = is_legal_matrix(&m2);
        println!("{:?}", r2);
        assert_eq!(r2, (false, "矩阵列不整齐"));
        //矩阵列不整齐
        let m3: Vec<Vec<i32>> = mat![[1], []];
        let r3 = is_legal_matrix(&m3);
        println!("{:?}", r3);
        assert_eq!(r3, (false, "矩阵列不整齐"));
        //矩阵列数位0
        let m4: Vec<Vec<i32>> = mat![[], [2]];
        let r4 = is_legal_matrix(&m4);
        println!("{:?}", r4);
        assert_eq!(r4, (false, "矩阵列数位0"));
        //矩阵合法
        let m5 = mat![[1.0, 9.0], [0.2, 0.5]];
        let r5 = is_legal_matrix(&m5);
        println!("{:?}", r5);
        assert_eq!(r5, (true, "矩阵合法"));
        //矩阵合法
        let m6 = mat![[1.0, 9.0]];
        let r6 = is_legal_matrix(&m6);
        println!("{:?}", r6);
        assert_eq!(r6, (true, "矩阵合法"));
        //矩阵合法
        let m7 = mat![[1.0], [9.0]];
        let r7 = is_legal_matrix(&m7);
        println!("{:?}", r7);
        assert_eq!(r7, (true, "矩阵合法"));
    }

    //矩阵加法测试
    #[test]
    fn sum_vertical_checked_test() {
        use crate::mat;
        //整数
        let m1 = mat![[1, 2], [3, 4]];
        let r1 = sum_vertical_checked(&m1);
        println!("{:?}", r1);
        assert_eq!(r1.unwrap(), vec![4, 6]);
        //浮点数
        let m2 = mat![[1.1e-30, 2.1e-30], [3.1e-30, 4.1e-30]];
        let r2 = sum_vertical_checked(&m2).unwrap();
        println!("{:?}", r2);
        assert_eq!(r2, vec![4.1999999999999996e-30, 6.2000000000000005e-30]);
        //一行矩阵
        let m3 = mat![[1.1, 2.1]];
        let r3 = sum_vertical_checked(&m3).unwrap();
        println!("{:?}", r3);
        assert_eq!(r3, vec![1.1, 2.1]);
        //一列矩阵
        let m4 = mat![[1.1], [2.1]];
        let r4 = sum_vertical_checked(&m4).unwrap();
        println!("{:?}", r4);
        assert_eq!(r4, vec![3.2]);
        //不规则矩阵
        let m5 = mat![[1.1, 9.0], [2.1]];
        let r5 = sum_vertical_checked(&m5);
        println!("{:?}", r5);
        assert_eq!(r5, None);
        //空矩阵
        let m6: Vec<Vec<f32>> = mat![[]];
        let r6 = sum_vertical_checked(&m6);
        println!("{:?}", r6);
        assert_eq!(r6, None);
        //行空矩阵
        let m7: Vec<Vec<f32>> = mat![[], []];
        let r7 = sum_vertical_checked(&m7);
        println!("{:?}", r7);
        assert_eq!(r7, None);
    }

    //矩阵加法测试
    #[test]
    fn sum_vertical_unchecked_test() {
        use crate::mat;
        //整数
        let m1 = mat![[1, 2], [3, 4]];
        let r1 = sum_vertical_unchecked(&m1);
        println!("{:?}", r1);
        assert_eq!(r1, vec![4, 6]);
        //浮点数
        let m2 = mat![[1.1e-30, 2.1e-30], [3.1e-30, 4.1e-30]];
        let r2 = sum_vertical_unchecked(&m2);
        println!("{:?}", r2);
        assert_eq!(r2, vec![4.1999999999999996e-30, 6.2000000000000005e-30]);
        //一行矩阵
        let m3 = mat![[1.1, 2.1]];
        let r3 = sum_vertical_unchecked(&m3);
        println!("{:?}", r3);
        assert_eq!(r3, vec![1.1, 2.1]);
        //一列矩阵
        let m4 = mat![[1.1], [2.1]];
        let r4 = sum_vertical_unchecked(&m4);
        println!("{:?}", r4);
        assert_eq!(r4, vec![3.2]);

        //不规则矩阵
        // let m5 = mat![[1.1, 9.0], [2.1]];
        // sum_vertical_unchecked(&m5);
        //index out of bounds

        //空矩阵
        let m6: Vec<Vec<f32>> = mat![[]];
        let r6 = sum_vertical_unchecked(&m6);
        println!("{:?}", r6);
        assert_eq!(r6, vec![]);

        //行空矩阵
        let m7: Vec<Vec<f32>> = mat![[], []];
        let r7 = sum_vertical_unchecked(&m7);
        println!("{:?}", r7);
        assert_eq!(r7, vec![]);

        //不规则矩阵
        let m8 = mat![[1.1], [2.1, 9.0]];
        let r8 = sum_vertical_unchecked(&m8);
        println!("{:?}", r8);
        assert_eq!(r8, vec![3.2]);

        //不规则矩阵
        let m9 = mat![[1.1], [2.1, 9.0], [9.0]];
        let r9 = sum_vertical_unchecked(&m9);
        println!("{:?}", r9);
        assert_eq!(r9, vec![12.2]);

        //不规则矩阵
        // let m10 = mat![[1.1, 8.0], [2.1, 9.0], [9.0]];
        // let r10 = sum_vertical_unchecked(&m10);
        // index out of bounds
    }

    #[test]
    fn sum_horizontal_unchecked_test() {
        use crate::mat;
        //整数
        let m1 = mat![[1, 2], [3, 4], [5, 6]];
        let r1 = sum_horizontal_unchecked(&m1);
        println!("{:?}", r1);
        assert_eq!(r1, vec![3, 7, 11]);
        //浮点数
        let m2 = mat![[1.1e-30, 2.1e-30], [3.1e-30, 4.1e-30]];
        let r2 = sum_horizontal_unchecked(&m2);
        println!("{:?}", r2);
        assert_eq!(r2, vec![3.2e-30, 7.2e-30]);
        //一行矩阵
        let m3 = mat![[1.1, 2.1]];
        let r3 = sum_horizontal_unchecked(&m3);
        println!("{:?}", r3);
        assert_eq!(r3, vec![3.2]);
        //一列矩阵
        let m4 = mat![[1.1], [2.1]];
        let r4 = sum_horizontal_unchecked(&m4);
        println!("{:?}", r4);
        assert_eq!(r4, vec![1.1, 2.1]);

        //不规则矩阵
        // let m5 = mat![[1.1, 9.0], [2.1]];
        // let r5 = sum_horizontal_unchecked(&m5);
        // println!("{:?}", r5);
        //index out of bounds

        //空矩阵
        let m6: Vec<Vec<f32>> = mat![[]];
        let r6 = sum_horizontal_unchecked(&m6);
        println!("{:?}", r6);
        assert_eq!(r6, vec![]);

        //行空矩阵
        let m7: Vec<Vec<f32>> = mat![[], []];
        let r7 = sum_horizontal_unchecked(&m7);
        println!("{:?}", r7);
        assert_eq!(r7, vec![]);

        //不规则矩阵
        let m8 = mat![[1.1], [2.1, 9.0]];
        let r8 = sum_horizontal_unchecked(&m8);
        println!("{:?}", r8);
        assert_eq!(r8, vec![1.1, 2.1]);

        //不规则矩阵
        let m9 = mat![[1.1], [2.1, 9.0], [9.0]];
        let r9 = sum_horizontal_unchecked(&m9);
        println!("{:?}", r9);
        assert_eq!(r9, vec![1.1, 2.1, 9.0]);

        //不规则矩阵
        // let m10 = mat![[1.1, 8.0], [2.1, 9.0], [9.0]];
        // let r10 = sum_horizontal_unchecked(&m10);
        // index out of bounds
    }

    #[test]
    fn sum_horizontal_checked_test() {
        use crate::mat;
        //整数
        let m1 = mat![[1, 2], [3, 4]];
        let r1 = sum_horizontal_checked(&m1);
        println!("{:?}", r1);
        assert_eq!(r1.unwrap(), vec![3, 7]);
        //浮点数
        let m2 = mat![[1.1e-30, 2.1e-30], [3.1e-30, 4.1e-30]];
        let r2 = sum_horizontal_checked(&m2).unwrap();
        println!("{:?}", r2);
        assert_eq!(r2, vec![3.2e-30, 7.2e-30]);
        //一行矩阵
        let m3 = mat![[1.1, 2.1]];
        let r3 = sum_horizontal_checked(&m3).unwrap();
        println!("{:?}", r3);
        assert_eq!(r3, vec![3.2]);
        //一列矩阵
        let m4 = mat![[1.1], [2.1]];
        let r4 = sum_horizontal_checked(&m4).unwrap();
        println!("{:?}", r4);
        assert_eq!(r4, vec![1.1, 2.1]);
        //不规则矩阵
        let m5 = mat![[1.1, 9.0], [2.1]];
        let r5 = sum_horizontal_checked(&m5);
        println!("{:?}", r5);
        assert_eq!(r5, None);
        //空矩阵
        let m6: Vec<Vec<f32>> = mat![[]];
        let r6 = sum_horizontal_checked(&m6);
        println!("{:?}", r6);
        assert_eq!(r6, None);
        //行空矩阵
        let m7: Vec<Vec<f32>> = mat![[], []];
        let r7 = sum_horizontal_checked(&m7);
        println!("{:?}", r7);
        assert_eq!(r7, None);
    }
    #[test]
    fn approx_eq_test() {
        let a = 0.9999995;
        let b = 1.0000004;
        let rst = approx_eq(a, b, 15);
        println!("{:?}", rst);
    }

    #[test]
    fn approx_test() {
        let c = 2.14365;
        let rst = approx(c, 15);
        println!("{:?}", rst);
    }

    #[test]
    fn strict_verify_test() {
        use crate::mat;
        use crate::set;
        use crate::set_list;
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
        println!("{:?}", sl.clone());
        let idt = InputData {
            discernment: dcm,
            fact_set_list: sl,
            evidence_matrix: m,
        };

        let r1 = strict_verify(&idt);
        println!("{:?}", r1);
    }

    #[test]
    fn build_evidence_test() {
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
        let d: InputData = InputData {
            discernment: dcm.clone(),
            fact_set_list: sl.clone(),
            evidence_matrix: m.clone(),
        };
        let evd = d.build_evidence(true).unwrap();
        assert_eq!(evd.detect_matrix().len(), 6);
    }
}
