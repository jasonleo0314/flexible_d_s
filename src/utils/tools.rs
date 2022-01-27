// 通用小工具

use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

//精确到小数bit位，四舍五入后，是否相等
pub fn approx_eq(a: f64, b: f64, bit: u8) -> bool {
    let times = 10f64.powi(bit as i32);
    let ai = ((a * times * 10.0).trunc() / 10.0).round() as i64;
    let bi = ((b * times * 10.0).trunc() / 10.0).round() as i64;
    ai == bi
}

//四舍五入精确到第几位
pub fn approx(x: f64, bit: i32) -> f64 {
    let times = 10f64.powi(bit);
    let xi = ((x * times * 10.0).trunc() / 10.0).round() as i64;
    (xi as f64) / times
}

//计算hash
pub fn calculate_hash<T: Hash + Sized>(ipt: T) -> u64 {
    let mut s = DefaultHasher::new();
    ipt.hash(&mut s);
    s.finish()
}

//打印矩阵
pub fn show_mat<T: Debug>(mat: &Vec<Vec<T>>) {
    mat.iter().for_each(|v| println!("{:?}", v));
}

//n进制全排列
pub fn base_n_permutation(base: usize, bits: usize) -> Vec<Vec<usize>> {
    let max_len = base.pow(bits as u32) as u128;
    assert!(
        max_len <= usize::MAX as u128,
        "超过计算极限，请减少进制base或者位数bit"
    );
    let max_len = max_len as usize;
    assert!(bits >= 1, "位数必须大于0");
    let u_bits = bits - 1;
    let last_idx = u_bits;
    let vi = &mut vec![0usize; last_idx + 1];
    let rst = &mut Vec::<Vec<usize>>::with_capacity(max_len);
    let mut sec_last_idx = 1;
    rst.push(vi.clone());
    (0..max_len - 1).into_iter().for_each(|_| {
        if vi[last_idx] != base - 1 {
            vi[last_idx] += 1;
            rst.push(vi.clone());
        } else {
            vi[last_idx] = 0;
            sec_last_idx = last_idx - 1;
            while sec_last_idx != 0 && vi[sec_last_idx] == base - 1 {
                vi[sec_last_idx] = 0;
                sec_last_idx -= 1;
            }
            vi[sec_last_idx] += 1;
            rst.push(vi.clone());
        }
    });
    rst.clone()
}

//列出集合set列表的组合
//最少2个set才可以使用
//输出集合set为元素的二维不整齐（列数量从2到列表最大长度）列表
pub fn combination_set_list<T: Clone>(lst_set: &Vec<HashSet<T>>) -> Vec<Vec<HashSet<T>>> {
    let len_lst = lst_set.len();
    assert!(len_lst >= 2, "最少2个set才可以使用combination_set_list");
    let rst = &mut vec![];
    fn push_vec<T: Clone>(in0: &[&HashSet<T>], out0: &mut Vec<Vec<HashSet<T>>>) {
        let t: Vec<HashSet<T>> = in0.iter().map(|&k| k.to_owned()).collect();
        out0.push(t.to_owned())
    }
    for n in 2..len_lst {
        permutator::combination(lst_set, n, |c| push_vec(c, rst));
    }
    rst.clone()
}

//计算集合set列表中各个集合的全排列
pub fn permutation_set_list(lst_set: &Vec<HashSet<char>>) -> Vec<Vec<HashSet<char>>> {
    let len = lst_set.len();
    let rst: Vec<Vec<HashSet<char>>> = (2..len)
        .map(|i| {
            let mut input: Vec<&[HashSet<char>]> = vec![];
            for _ in 0..i {
                input.push((&lst_set).clone().as_slice());
            }
            permutator::CartesianProductIterator::new(input.as_slice())
                .map(|x| x.iter().map(|&a| a.to_owned()).collect())
                .collect::<Vec<Vec<HashSet<char>>>>()
        })
        .fold(vec![], |a, c| {
            a.iter().chain(&c).map(|n| n.to_owned()).collect()
        });
    rst
}

//根据可迭代对象列表简历哈西表
//key:集合列表的hash值
//value:输入的列表（最外层）的序号
pub fn set_set_list_to_hashmap<T: Hash>(lst_set: &[HashSet<T>]) -> HashMap<u128, usize> {
    let mut map_from_set_to_idx: HashMap<u128, usize> = HashMap::new();
    lst_set
        .iter()
        .enumerate()
        .map(|(idx, y)| {
            let k = hash_hashset(y);
            (idx, k)
        })
        .for_each(|(idx, v)| {
            let _ = &map_from_set_to_idx.insert(v, idx);
        });
    map_from_set_to_idx
}

//对HashSet求hash值
pub fn hash_hashset<T: Hash>(hs: &HashSet<T>) -> u128 {
    hs.iter()
        .map(|x| calculate_hash(&x))
        .fold(0u128, |a, c| a + (c as u128))
}

//对Vec求hash值
pub fn hash_vec(hs: &Vec<f64>) -> u128 {
    hs.iter()
        .map(|x0| {
            let x9 = format!("{:.2}", x0);
            calculate_hash(x9)
        })
        .fold(0u128, |a, c| a + (c as u128))
}

//求集合列表的所有set的交集
pub fn intersect_set_list<T: Clone + Eq + Hash>(lst_set: &[HashSet<T>]) -> HashSet<T> {
    let init_set = lst_set[0].clone();
    // let b = &lst_set[3];
    let rst = lst_set.iter().fold(init_set, |a, c| {
        let mut n = HashSet::new();
        a.intersection(c).map(|d| d.clone()).for_each(|x| {
            n.insert(x);
        });
        n
    });
    rst
}

//字符串切片转String类型
pub fn s(s: &str) -> String {
    String::from(s)
}

//自动格式化打印复杂类型
pub fn fp<T: Debug>(sth: T) {
    println!("{:?}", sth);
}

//自动高级格式化打印复杂类型
pub fn fpa<T: Debug>(sth: T) {
    println!("{:#?}", sth);
}
