//! # Dempster 算法主流水线

use crate::com::dempster_sub_focal_to_mul::FocalToMul;
use crate::core::evidence::Evidence;
use crate::utils::tools::{
    approx_eq, combination_set_list, hash_hashset, intersect_set_list, set_set_list_to_hashmap,
};

/// # Dempster 算法缓存结构体
/// ## 用途
/// 多探测器多识别事件的融合场景下，开展经典Dempster证据冲突融合计算，
/// 本结构体用于缓存每个计算步骤的中间过程数据，*目的*是使得算法宏观设计过程
/// 和微观实现过程分离，并且通过缓存上一步的结果，提高结果复用率从而提高运行效率
/// ## 属性
/// evd_mat 证据矩阵
/// bi_mat 二值化证据矩阵
/// fact_combine_inter_empty_sheet 识别事件列号组合（相交为空）（根据公式要求的集合子交并补关系确定）
/// focal_to_mul_unit_empty_list 拼合连加连乘清单（相交为空）原始计算结构
/// mul_plus_empty_sheet 连乘连加清单（相交为空）
pub struct Dempster {
    pub evd_mat: Vec<Vec<f64>>,                          //证据矩阵
    pub bi_mat: Vec<Vec<u8>>,                            //二值化证据矩阵
    pub fact_combine_inter_empty_sheet: Vec<Vec<usize>>, //识别事件列号组合（相交为空）（根据公式要求的集合子交并补关系确定）
    pub focal_to_mul_unit_empty_list: Vec<FocalToMul>,   //拼合连加连乘清单（相交为空）原始计算结构
    pub mul_plus_empty_sheet: Vec<Vec<f64>>,             //连乘连加清单（相交为空）
}

/// # 函数名：*Dempster结构体空构造函数*
/// ## 用途：略
/// ## 输入：无
/// ## 输出：Dempster结构体
/// ## 算法：略
/// # 用例：略
/// ```
/// ```
/// - 时间：22-1-6
/// - 路径：src\com\dempster.rs
/// - 作者：刘绍偈 liushaojie_vip@163.com
pub fn create_dempster() -> Dempster {
    Dempster {
        evd_mat: Vec::<Vec<f64>>::new(),
        bi_mat: Vec::<Vec<u8>>::new(),
        fact_combine_inter_empty_sheet: Vec::<Vec<usize>>::new(),
        focal_to_mul_unit_empty_list: Vec::<FocalToMul>::new(),
        mul_plus_empty_sheet: Vec::<Vec<f64>>::new(),
    }
}

impl Dempster {
    pub fn run(&mut self, evd: &Evidence) -> f64 {
        self.import_evidence_matrix(evd);
        self.set_bi_mat();
        self.set_fact_combine_inter_empty_sheet(evd);
        self.set_focal_to_mul_unit_empty_list(evd);
        self.set_mul_plus_empty_sheet();
        self.mul_plus_empty_sheet
            .iter()
            .map(|v| v[0] * v[1])
            .fold(0.0, |a, c| a + c)
    }

    /// # 函数名：*导入证据矩阵*
    /// ## 用途：略
    /// ## 输入：证据矩阵结构体的引用
    /// ## 输出：空
    /// ## 算法：略
    /// # 用例：略
    /// ```
    /// ```
    /// - 时间：22-1-6
    /// - 路径：src\com\dempster.rs
    /// - 作者：刘绍偈 liushaojie_vip@163.com
    pub fn import_evidence_matrix(&mut self, evd: &Evidence) {
        self.evd_mat = evd.evidence_matrix().clone()
    }
    /// # 函数名：*二值化证据矩阵*
    /// ## 用途：
    /// 计算多列横向加和后，
    /// 1. 如果某一行结果为0，则说明该行没有证据数值
    /// 2. 如果某一行结果为1，则说明该行有且只有一个证据数值
    /// 3. 如果某一行结果大于0，泽说明改行有多个证据数值
    /// ## 输入：略
    /// ## 输出：略
    /// ## 算法：
    /// 利用函数approx_eq，在第6位的精确度上，比较矩阵元素值和0是否相等，相等返回0，不相等返回1
    /// # 用例：略
    /// ```
    /// ```
    /// - 时间：22-1-6
    /// - 路径：src\com\dempster.rs
    /// - 作者：刘绍偈 liushaojie_vip@163.com
    pub fn set_bi_mat(&mut self) {
        self.bi_mat = self
            .evd_mat
            .iter()
            .map(|v| {
                v.iter()
                    //利用函数approx_eq，在第6位的精确度上，比较矩阵元素值和0是否相等，相等返回0，不相等返回1
                    .map(|&item| if approx_eq(item, 0.0, 6) { 0u8 } else { 1u8 })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();
    }
    /// # 函数名：*设置“识别事件”合并表单*
    /// ## 用途：“识别事件” 列号组合（根据公式要求的集合子交并补关系确定）
    /// ## 输入：证据矩阵结构体的引用
    /// ## 输出：无
    /// ## 算法：
    /// 获取识别事件集合列表
    /// 构造事件集合列表的 hashmap ，令key=事件set的hash；value=事件set的序号
    /// 对事件集合列表开展组合，找出没有交集的组合，并返回事件集合的序号矩阵，
    /// 调用函数intersect_set_list求一行set的交集，保留交集为空的行
    /// # 用例：略
    /// ```
    /// ```
    /// - 时间：22-1-6
    /// - 路径：src\com\dempster.rs
    /// - 作者：刘绍偈 liushaojie_vip@163.com
    pub fn set_fact_combine_inter_empty_sheet(&mut self, evd: &Evidence) {
        //获取识别事件集合列表
        let lst_set = &evd.fact_set_list().clone();
        //构造事件集合列表的 hashmap
        //令key: 事件set 的 hash
        //value: 事件set 的 序号
        let hm = set_set_list_to_hashmap(lst_set);
        //对事件集合列表开展组合
        let cb_lst_set = combination_set_list(lst_set);
        //找出没有交集的组合，并返回事件集合的序号矩阵
        let inter_empty = cb_lst_set
            .iter()
            //调用函数intersect_set_list求一行set的交集，保留交集为空的行
            .filter(|&v| intersect_set_list(v).iter().count() == 0)
            .map(|s| {
                s.iter()
                    //调用函数hash_hashset，将hashset<&str>格式的元素变为哈希值u128，
                    // 然后利用hm事件集合列表的 hashmap 将hash值映射到“识别事件”的序号
                    .map(|hs| hm.get(&hash_hashset(hs)).unwrap().clone())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        self.fact_combine_inter_empty_sheet = inter_empty;
    }
    /// # 函数名：*设置连乘连加模块清单方法*
    /// ## 用途：合并所有的待连乘连加模块
    /// ## 输入：空
    /// ## 输出：空
    /// ## 算法：
    /// 获取连加连乘对象列表，摘取对象中的连加连乘列表，将所有列表折叠拼接成一个列表
    /// # 用例：略
    /// ```
    /// ```
    /// - 时间：22-1-6
    /// - 路径：src\com\dempster.rs
    /// - 作者：刘绍偈 liushaojie_vip@163.com
    pub fn set_mul_plus_empty_sheet(&mut self) {
        let x1 = self
            //获取连加连乘对象列表
            .focal_to_mul_unit_empty_list
            .iter()
            //摘取对象中的连加连乘列表
            .map(|m| m.mul_plus_sheet_unit.clone())
            //将所有列表折叠拼接成一个列表
            .fold(Vec::new(), |a, c| {
                a.iter().chain(&c).map(|x| x.clone()).collect()
            });
        // //去重
        // let mut hm = HashMap::new();
        // //对数字计算hash后累加作为列表的hash值，以hash为键，以列表为值，建立hashMap
        // //在向hashmap中逐项插入时，重复内容就会被删除
        // x1.iter().map(|vv| (vv, hash_vec(vv))).for_each(|(vv, uu)| {
        //     hm.insert(uu, vv);
        // });
        // //从Map中取出结果
        // let kk = hm.values().map(|&v5| v5.clone()).collect::<Vec<Vec<f64>>>();
        // self.mul_plus_empty_sheet = kk;
        self.mul_plus_empty_sheet = x1;
    }
}
