use crate::obj::cell::Cell;
use crate::obj::event::Event;
use crate::utils::tools::base_n_permutation;
use std::collections::HashSet;

//d-s证据冲突识别矩阵
#[derive(Debug, Clone)]
pub struct EvidenceMatrix {
    discernment: HashSet<usize>,
    event_vector: Vec<Event>,
    detect_matrix: Vec<Vec<Cell>>,
    row_cnt: usize,
    col_cnt: usize,
    col_base_row_bits_permutation: Vec<Vec<usize>>,
    n_base_cell_matrix: Vec<Vec<Cell>>,
}

impl EvidenceMatrix {
    pub fn n_base_cell_matrix(&self) -> &Vec<Vec<Cell>> {
        &self.n_base_cell_matrix
    }
}

impl EvidenceMatrix {
    pub fn discernment(&self) -> &HashSet<usize> {
        &self.discernment
    }
    pub fn event_vector(&self) -> &Vec<Event> {
        &self.event_vector
    }
    pub fn detect_matrix(&self) -> &Vec<Vec<Cell>> {
        &self.detect_matrix
    }
    pub fn row_cnt(&self) -> usize {
        self.row_cnt
    }
    pub fn col_cnt(&self) -> usize {
        self.col_cnt
    }
    pub fn col_base_row_bits_permutation(&self) -> &Vec<Vec<usize>> {
        &self.col_base_row_bits_permutation
    }
}

/// # 函数名：*构造n进制全排列下cell组合的cell矩阵*
/// ## 用途：
/// 用来辅助选出D-S证据理论公式中的各种集合规则列表，
/// 以便开展连乘连加运算
/// ## 输入：
/// 无
/// ## 输出：
/// 所有的组合cell的二维列表
/// ## 算法：
/// 将组合问题映射为n进制全排列后筛选问题
/// 矩阵中的每一行代表一个探测器，对应n进制算法中的数码位置，
/// 矩阵中的每一列代表一个事件，对应n进制算法中的数码；
/// 1. 计算col进制数码填满row个位置的全排列，得到（row,col）组合二维列表，
/// 其中每一行是一个组合，共有col^row列，代表所有可能的组合
/// 2. 将该（row,col）二维列表，映射为cell，并取出每个cell
/// # 用例：略
/// ```
/// ```
/// - 时间：22-1-7
/// - 路径：src\obj\evidence_matrix.rs
/// - 作者：刘绍偈 liushaojie_vip@163.com
fn set_n_base_cell_matrix(
    col_base_row_bits_permutation: &Vec<Vec<usize>>,
    detect_matrix: &Vec<Vec<Cell>>,
) -> Vec<Vec<Cell>> {
    col_base_row_bits_permutation
        .iter()
        .map(|combined_idx| {
            combined_idx
                .iter()
                .enumerate()
                .map(|(row, col)| {
                    let cur_cell: &Cell = detect_matrix.get(row).unwrap().get(*col).unwrap();
                    cur_cell.clone()
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>()
}

impl EvidenceMatrix {
    /// # 函数名：构造函数
    /// ## 用途：构造证据矩阵
    /// ## 输入：形如：
    /// let dcm: HashSet<usize> = set! {0, 1, 2, 3};
    ///         let sl: Vec<HashSet<usize>> = set_list![{0}, {1}, {2}, {0, 1}, {1, 2, 3, 0}];
    ///         let m: Vec<Vec<f64>> = mat![
    ///             [0.1, 0.0, 0.3, 0.6, 0.0],
    ///             [0.2, 0.4, 0.4, 0.0, 0.0],
    ///             [0.7, 0.1, 0.2, 0.0, 0.0],
    ///             [0.0, 0.0, 0.0, 0.0, 1.0]
    ///         ];
    /// ## 输出：证据矩阵实例
    /// ## 算法：略
    /// # 用例：略
    /// ```
    /// ```
    /// - 时间：22-1-7
    /// - 路径：src\obj\evidence_matrix.rs
    /// - 作者：刘绍偈 liushaojie_vip@163.com
    pub fn new(
        discernment: HashSet<usize>,
        event_set_list: Vec<HashSet<usize>>,
        matrix: Vec<Vec<f64>>,
    ) -> EvidenceMatrix {
        let ev = event_set_list
            .iter()
            .enumerate()
            .map(|(idx, hs)| Event::new_with_child(idx, hs.clone()))
            .collect::<Vec<Event>>();
        let dm = matrix
            .iter()
            .enumerate()
            .map(|(idx_row, detector_vec)| {
                detector_vec
                    .iter()
                    .enumerate()
                    .map(|(idx_col, val)| {
                        Cell::new_with_val(idx_row, idx_col, &ev.get(idx_col).unwrap(), *val)
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();
        let perm = &base_n_permutation(ev.len(), dm.len());
        EvidenceMatrix {
            discernment,
            col_cnt: ev.len(),
            row_cnt: dm.len(),
            n_base_cell_matrix: set_n_base_cell_matrix(&perm, &dm),
            col_base_row_bits_permutation: perm.clone(),
            event_vector: ev,
            detect_matrix: dm,
        }
    }
}
