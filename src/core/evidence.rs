//冲突证据核心类文件

use std::collections::{HashMap, HashSet};
#[derive(Debug)]
pub struct Evidence {
    discernment: HashSet<String>,          //识别框架
    fact_set_list: Vec<HashSet<String>>,   //“识别事件”列表
    evidence_matrix: Vec<Vec<f64>>,        //证据矩阵
    _completely_conflict_set: Vec<String>, //完全冲突集
    _delta_m: f64,                         //完全冲突集的适配重分布值
    _sim_matrix: Vec<Vec<f64>>,            //传感器之间的相似度交叉矩阵
    _total_sim: f64,                       //总传感器相似度
    _d_s_result: HashMap<String, f64>,     //最终融合结论
}

impl Evidence {
    /// # 函数名：*构造函数(该函数不得直接调用，必须由Input生成)*
    /// ## 用途：构造冲突证据分析过程缓存
    /// ## 输入：
    /// 识别框架 discernment
    /// 识别事件 fact_set_list
    /// 证据矩阵 evidence_matrix
    /// ## 输出：冲突证据算法缓存结构体
    /// ## 算法：略
    /// # 用例：*不应当直接调用*
    /// ```
    /// ```
    /// - 时间：22-1-6
    /// - 路径：src\core\evidence.rs
    /// - 作者：刘绍偈 liushaojie_vip@163.com
    pub fn never_new_an_evidence_from_me(
        discernment: HashSet<String>,
        fact_set_list: Vec<HashSet<String>>,
        evidence_matrix: Vec<Vec<f64>>,
    ) -> Self {
        Evidence {
            discernment,
            fact_set_list,
            evidence_matrix,
            _completely_conflict_set: Vec::<String>::new(),
            _delta_m: 0.0,
            _sim_matrix: Vec::new(),
            _total_sim: 0.0,
            _d_s_result: HashMap::<String, f64>::new(),
        }
    }
}

//开始融合冲突证据
pub fn fuse() {
    //1. 找出完全冲突集
    //2. 开展传统D-S融合计算
    //3. 如果完全冲突集不为空，则计算各个传感器MRow之间的相似度
    //4. 如果有Map就是识别框架且值为1，令该Map的η值为0，否则η为当前传感器对其他传感器的相似度之和除以总相似度
    //5. 计算完全冲突集中每个元素X及其超集A的ΩmA,存入结果作为X的融合结果
    //6. 将已存入的完全冲突X的融合结果相加得到δm
    //7. 将传统D-S融合计算结果中非完全冲突集元素与（1-δm）相乘得到融合结果
    todo!()
}

impl Evidence {
    //todo:待完善冲突证据算法的其他缓存变量后，补齐get方法
    /// # 函数名：*冲突证据结构体的 get方法*
    /// ## 用途：略
    /// ## 输入：略
    /// ## 输出：略
    /// ## 算法：略
    /// # 用例：略
    /// ```
    /// ```
    /// - 时间：22-1-6
    /// - 路径：src\core\evidence.rs
    /// - 作者：刘绍偈 liushaojie_vip@163.com
    pub fn discernment(&self) -> &HashSet<String> {
        &self.discernment
    }
    pub fn fact_set_list(&self) -> &Vec<HashSet<String>> {
        &self.fact_set_list
    }
    pub fn evidence_matrix(&self) -> &Vec<Vec<f64>> {
        &self.evidence_matrix
    }
}
