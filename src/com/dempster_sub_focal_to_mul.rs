use crate::core::evidence::Evidence;
use crate::utils::matrix_calc::permutation_cols;
use crate::utils::tools::approx_eq;
use std::collections::HashSet;

use std::fmt::{Debug, Formatter};

//展开为连乘连加清单前的焦元排列组合单元计算过程缓存结构体
pub struct FocalToMul {
    pub fitted_fact_list: Vec<usize>,       //选中的识别事件列号组合
    pub single_cols: HashSet<usize>,        //独立元素集中列
    pub single_detector_list: Vec<usize>,   //无需排列组合的探测器序号列表
    pub choice_detector_list: Vec<usize>,   //需要排列组合的探测器（行号）列表
    pub single_focal_list: Vec<f64>,        //无需排列组合的焦元列表
    pub choice_focal_sheet: Vec<Vec<f64>>,  //需要排列组合的焦元清单（行为探测器，列为事件）
    pub mul_plus_sheet_unit: Vec<Vec<f64>>, //由以上独立元素和排列组合元素拼合而成的连乘连加清单单元
}

impl Debug for FocalToMul {
    //打印格式
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FocalToMul")
            .field(
                "\n\nfitted_fact_list-选中的识别事件列号组合\n",
                &self.fitted_fact_list,
            )
            .field("\n\nsingle_cols-独立元素集中列\n", &self.single_cols)
            .field(
                "\n\nsingle_detector_list-无需排列组合的探测器序号集合\n",
                &self.single_detector_list,
            )
            .field(
                "\n\nchoice_detector_list-需要排列组合的探测器（行号）列表\n",
                &self.choice_detector_list,
            )
            .field(
                "\n\nsingle_focal_list-无需排列组合的焦元列表\n",
                &self.single_focal_list,
            )
            .field(
                "\n\nchoice_focal_sheet-需要排列组合的焦元清单（行为探测器，列为事件）\n",
                &self.choice_focal_sheet,
            )
            .field(
                "\n\nmul_plus_sheet_unit-由以上独立元素和排列组合元素拼合而成的连乘连加清单单元\n",
                &self.mul_plus_sheet_unit,
            )
            .finish()
    }
}

impl FocalToMul {
    //根据现有实体数据计算待连乘连加单元
    pub fn set_mul_plus_sheet_unit(&mut self, evd: &Evidence) -> Vec<Vec<f64>> {
        let multi_focal = &self.choice_focal_sheet.clone();
        let single_focal = &self.single_focal_list.clone();
        return if multi_focal.len() == 0 {
            vec![single_focal
                .iter()
                .map(|&x| x)
                .collect::<Vec<f64>>()
                .clone()]
        } else if multi_focal.len() == 1 {
            let a = multi_focal.get(0).unwrap();
            let b = single_focal;
            let c = a.iter().chain(b).map(|&x1| x1).collect::<Vec<f64>>();
            vec![c]
        } else {
            let perm_multi_focal = permutation_cols(&multi_focal);
            let x1: Vec<Vec<f64>> = perm_multi_focal
                .iter()
                .map(|v| {
                    v.iter()
                        .chain(single_focal.iter())
                        //删除等于0的元素
                        .filter(|&fd| !approx_eq(fd.clone(), 0.0, 6))
                        .map(|x| x.clone())
                        .collect::<Vec<f64>>()
                })
                .collect::<Vec<Vec<f64>>>();
            //删除探测器数量不匹配的记录
            let x2 = x1
                .iter()
                .filter(|&ff| evd.evidence_matrix().len() == ff.len())
                .map(|ddd| ddd.clone())
                .collect::<Vec<Vec<f64>>>();
            x2
        };
    }
}
