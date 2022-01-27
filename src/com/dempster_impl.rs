use crate::com::dempster::Dempster;
use crate::com::dempster_sub_focal_to_mul::FocalToMul;
use crate::core::evidence::Evidence;
use crate::utils::matrix_calc::{cut_mat, sum_horizontal_unchecked};
use crate::utils::tools::approx_eq;
use std::collections::HashSet;

impl Dempster {
    //依据无交集组合，为每一行构建一个焦元排列组合单元计算过程缓存结构体
    pub fn set_focal_to_mul_unit_empty_list(&mut self, evd: &Evidence) {
        let mt = self.bi_mat.clone();
        let mat = self.evd_mat.clone();
        self.focal_to_mul_unit_empty_list = self
            .fact_combine_inter_empty_sheet //交集为空的事件序号组合清单
            .iter()
            .map(|i| {
                //按照事件索引标号，切片二值化的焦元矩阵，切出事件列
                let sub_mt = cut_mat(&mt, &[], i).unwrap();
                //水平方向计算二值化的事件列的和
                let sum_col = sum_horizontal_unchecked(&sub_mt);
                //返回事件的列序号和二值列和
                (i, sum_col)
            })
            //剔除列二值加和中包含0的记录，因为如果某一行（探测器）方向上，特定事件对应的焦元全部为0，
            //则该行不用参加连乘运算
            .filter(|(_, s)| !s.contains(&0u8))
            .map(|(x, sum_col)| {
                //找出列二值加和为1的行（探测器）序号
                //这些里找出的每一行（在指定事件（列）下）都有且只有一个焦元不为0
                let row_one = sum_col
                    .iter()
                    .enumerate()
                    .filter(|(_, &x)| x == 1u8)
                    .map(|(idx, _)| idx)
                    .collect::<Vec<usize>>();
                //找出列二值加和大于1的行（探测器）序号
                //这些里找出的每一行（在指定事件（列）下）都有多个焦元不为0
                let row_big = sum_col
                    .iter()
                    .enumerate()
                    .filter(|(_, &x)| x > 1u8)
                    .map(|(idx, _)| idx)
                    .collect::<Vec<usize>>();

                //通过横纵坐标找出列二值加和为1的焦元
                let single_focal = row_one
                    .iter()
                    .map(|&row| {
                        //行列为索引，选出某一行中多列候选值
                        let candidate = cut_mat(&mat, &[row], &x)
                            .unwrap()
                            .get(0)
                            .unwrap()
                            .to_owned();
                        //从候选值中，找出不为0的值
                        let focal_one = candidate
                            .iter()
                            .filter(|&s1| !approx_eq(s1.clone(), 0.0, 6))
                            .map(|&s2| s2)
                            .collect::<Vec<f64>>()[0];
                        (focal_one, x.clone())
                    })
                    .collect::<Vec<(f64, Vec<usize>)>>();
                //通过横纵坐标找出多个列二值加和大于1的焦元矩阵
                let multi_matrix = row_big
                    .iter()
                    .map(|&v| {
                        let value = cut_mat(&mat, &[v], &x).unwrap().get(0).unwrap().clone();
                        let rst = value.iter().map(|&vl| vl).collect::<Vec<f64>>();
                        rst
                    })
                    .collect::<Vec<Vec<f64>>>();
                let mut tmp_hs = HashSet::new();
                single_focal.iter().for_each(|(_, kk)| {
                    kk.iter().for_each(|kk0| {
                        tmp_hs.insert(kk0.clone());
                    })
                });

                let mut m = FocalToMul {
                    fitted_fact_list: x.clone(),
                    single_cols: tmp_hs,
                    single_detector_list: row_one,
                    choice_detector_list: row_big,
                    single_focal_list: single_focal
                        .iter()
                        .map(|(dt, _)| dt.clone())
                        .collect::<Vec<f64>>(),
                    choice_focal_sheet: multi_matrix,
                    mul_plus_sheet_unit: vec![],
                };
                m.mul_plus_sheet_unit = m.set_mul_plus_sheet_unit(evd.clone());
                m
            })
            .collect::<Vec<FocalToMul>>();
    }
}
