# 证据冲突理论算法实践

## 一、背景

公安工作中笔录是重要的言辞证据形式。对控告人、举报人、证人、被举报人、犯罪嫌疑人和其他案件关系人制作的笔录中，往往因立场、认知水平、观察角度、法律认识等因素的差异，造成同一事实的描述存在差异和冲突。如何从这些差异和冲突中推断实际情况，或者获取各种可能情况的概率，从而指引侦查方向，提示取证角度，是一个重要课题。

## 二、算法选型

在抽取笔录数据后，针对识别出的差异和冲突事件进行冲突证据解决，可以采用现有证据冲突理论。经调研，经典D-S理论在证据间冲突程度较高时结论违背直觉，因此不能直接使用。后续的主要从两个方向开展了大量有益的探索，在“结论必须不能违背直觉”这一点的解决方案上可谓“百花齐放”，但往往都会忽略一个重要问题——这些方案只解决了“必要性”，没有解决“充分性”。换言之，证据冲突融合理论算法方案中的所有步骤必须被证明是解决冲突证据的“充分必要条件”，这样的算法才有扎实的数理依据，才是科学的。经比较，最终将算法方案锁定在了论文 *A flexible rule for evidential combination in Dempster–Shafer theory of evidence* 上。其中的算法是基于作者提出的几条公设之上的，基础扎实、可证伪，因此作为软件实现的数学依据。该论文的主要思路是基于经典D-S理论，为解决违反直觉问题引入了“完全冲突证据”概念，在计算证据源可靠性的基础上，对“完全冲突证据”进行赋值，并调整其他证据源的结论数值，使得全部结论概率和为1.

## 三、算法实现情况

本代码首先完全依照论文进行编码，采用rust语言实现了全部计算过程 run_flexible_complete_conflict_algo 。其次，针对完全互斥的事件，代码采用了简化的算法 independent_event_evidence_merge，以提高效率。最后，针对完全互斥事件，并且待融合的证据矩阵中不存在0概率的情况，采用专门简化算法 independent_event_non_zero_evidence_merge。三个接口的结论与论文中的计算结论进行交叉比较，完全一致。之后封装三个接口为python 调用函数 merge_evidence 。

```rust
//从python 读入冲突证据
//discernment是识别框架，
//fact_set_list是由识别框架的多个子集构成的列表，每个子集称为“识别事件”；
//evidence_matrix是证据矩阵，即传感器对“识别事件”进行探测所得到的事件发生概率；
//其中，每一行代表一个传感器的所有识别结论（一个概率值）且行内相加为1.0，
//每一列代表一个“识别事件”，每一个元素称为焦元数值[0,1]
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

 //python 调用方法
 merge_evidence(
    discernment: HashSet<usize>,        //识别框架
    fact_set_list: Vec<HashSet<usize>>, //“识别事件”列表
    evidence_matrix: Vec<Vec<f64>>,     //证据矩阵
    algo_type: u32,                     //融合函数
) -> PyResult<Vec<f64>>

//python 包路径
import evidence_conflict_merge.evidence_conflict_merge.*
```

## 四、算法问题

本算法中，independent_event_evidence_merge 和 independent_event_non_zero_evidence_merge 两个接口效率还是可以的，正常需求下微秒级响应，随证据源数量线性增长。然而，虽然run_flexible_complete_conflict_algo 接口的计算过程完全依据论文写成，但明显算法时间和空间复杂度为均为 O(灾难级)，有待优化。目前，该接口的运算时间大致如下：设待融合的证据源数量为 n, 则运算事件为 10^n 毫秒。初步判断，一是代码中存在不合理的迭代需要改正，二是根据论文进行事件集合运算时，算法复杂度过高。以上两个问题有待精力充裕时再行调整。若是有走过路过的好心人提些个建议，当然是万分感谢了。倘若有大神肯伸出援手，更是感激不尽。

不知道自己做的算不算公益，公安工作的智能化无论如何也算是为我们这个社会做贡献了吧。
