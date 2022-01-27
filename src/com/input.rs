/*
   输入的数据临时存储结构体
   discernment是识别框架，
   fact_set_list是由识别框架的多个子集构成的列表，每个子集称为“识别事件”；
   evidence_matrix是证据矩阵，即传感器对“识别事件”进行探测所得到的事件发生概率；
   其中，每一行代表一个传感器的所有识别结论（一个概率值）且行内相加为1.0，
   每一列代表一个“识别事件”，每一个元素称为焦元数值[0,1]
   discernment = {'a', 'b', 'c', 'd'}
   fact_set_list = [{'a'}, {'b'}, {'c'}, {'a', 'b'}, {'a', 'b', 'c'}]
   evidence_matrix = [[0.1, 0.0, 0.3, 0.6, 0.0],
        [2.0, 0.4, 0.4, 0.0, 0.0],
        [7.0, 0.1, 0.2, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0, 1.0]]
*/
use crate::obj::evidence_matrix::EvidenceMatrix;
use crate::utils::verify::strict_verify;
use std::collections::HashSet;

pub struct InputData {
    pub discernment: HashSet<usize>,        //识别框架
    pub fact_set_list: Vec<HashSet<usize>>, //“识别事件”列表
    pub evidence_matrix: Vec<Vec<f64>>,     //证据矩阵
}

impl InputData {
    pub fn new(
        discernment: HashSet<usize>,
        fact_set_list: Vec<HashSet<usize>>,
        evidence_matrix: Vec<Vec<f64>>,
    ) -> Self {
        InputData {
            discernment,
            fact_set_list,
            evidence_matrix,
        }
    }
}

impl InputData {
    pub fn show(&self) {
        println!("输入的识别框架为：{:?}", self.discernment);
        println!("输入的“识别事件”为：{:?}", self.fact_set_list);
        println!("输入的证据矩阵为：");
        self.evidence_matrix
            .iter()
            .for_each(|v| println!("{:?}", v));
    }
    //生成证据
    pub fn build_evidence(&self, to_check: bool) -> Option<EvidenceMatrix> {
        if to_check {
            let (ver_rst, rst_info) = strict_verify(&self);
            if ver_rst == false {
                println!("输入验证结果：{}", rst_info);
            }
            if ver_rst == false {
                None
            } else {
                let evd = EvidenceMatrix::new(
                    self.discernment.clone(),
                    self.fact_set_list.clone(),
                    self.evidence_matrix.clone(),
                );
                Some(evd)
            }
        } else {
            let evd = EvidenceMatrix::new(
                self.discernment.clone(),
                self.fact_set_list.clone(),
                self.evidence_matrix.clone(),
            );
            Some(evd)
        }
    }
}
