use crate::utils::tools::base_n_permutation;
use crate::utils::verify::is_legal_matrix;
use std::collections::HashSet;
use std::ops::{Add, AddAssign};

//检查矩阵合法性后，行与行相加
pub fn sum_vertical_checked<T>(mat: &Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Add + AddAssign + Clone,
{
    let check_rst = is_legal_matrix(&mat);
    if check_rst.0 {
        let rst_vec = sum_vertical_unchecked(&mat);
        Some(rst_vec)
    } else {
        println!("{}", check_rst.1);
        None
    }
}

//检查矩阵合法性后，行与行相加
pub fn sum_horizontal_checked<T>(mat: &Vec<Vec<T>>) -> Option<Vec<T>>
where
    T: Add + AddAssign + Clone,
{
    let check_rst = is_legal_matrix(&mat);
    if check_rst.0 {
        let rst_vec = sum_horizontal_unchecked(&mat);
        Some(rst_vec)
    } else {
        println!("{}", check_rst.1);
        None
    }
}

//不检查矩阵合法性，直接行与行相加
//行空或者列空矩阵返回空矩阵
//不规则矩阵中第一行的列数为返回的列数，如果后续行的列数小于第一行的列数，则抛出越界错误，程序停止
pub fn sum_vertical_unchecked<T>(mat: &Vec<Vec<T>>) -> Vec<T>
where
    T: Add + AddAssign + Clone,
{
    let (row_mat, col_mat) = get_shape_unchecked(&mat);
    let mut rst_vec: Vec<T> = Vec::with_capacity(col_mat);
    for row in 0..row_mat {
        if row != 0 {
            for col in 0..col_mat {
                rst_vec[col] += mat[row][col].clone();
            }
        } else {
            rst_vec = mat[0].clone();
        }
    }
    rst_vec
}

//获取矩阵的形状
//输入矩阵，返回(行数，列数)
pub fn get_shape_unchecked<T>(mat: &Vec<Vec<T>>) -> (usize, usize) {
    let row_mat = mat.len();
    let col_mat = mat[0].len();
    (row_mat, col_mat)
}

//矩阵列相加
//不检查矩阵合法性，直接列与列相加
//行空或者列空矩阵返回空矩阵
//不规则矩阵中第一行的列数为返回的列数，如果后续行的列数小于第一行的列数，则抛出越界错误，程序停止
pub fn sum_horizontal_unchecked<T>(mat: &Vec<Vec<T>>) -> Vec<T>
where
    T: Add + AddAssign + Clone,
{
    let (row_mat, col_mat) = get_shape_unchecked(&mat);
    let mut rst_vec: Vec<T> = Vec::with_capacity(row_mat);
    for col in 0..col_mat {
        for row in 0..row_mat {
            if col != 0 {
                rst_vec[row] += mat[row][col].clone();
            } else {
                rst_vec.push(mat[row][col].clone());
            }
        }
    }
    rst_vec
}

//切片矩阵
//如果输入的行或者列切片指令数组为空，则表示全保留
//如果输入的行或者列切片指令超出了数组界限，则返回None
//成功则返回Some（new_mat)
pub fn cut_mat<T: Copy>(
    mat: &Vec<Vec<T>>,
    row_slice: &[usize],
    col_slice: &[usize],
) -> Option<Vec<Vec<T>>> {
    let row_max = row_slice
        .iter()
        .fold(0usize, |a, c| if a < *c { *c } else { a });
    let col_max = col_slice
        .iter()
        .fold(0usize, |a, c| if a < *c { *c } else { a });
    let mat_row_len = mat.len();
    let mat_col_len = mat[0].len();
    let row_slice_len = row_slice.len();
    let col_slice_len = col_slice.len();
    if row_max >= mat_row_len || col_max >= mat_col_len {
        return None;
    }
    let cut_row;
    let cut_col;
    if row_slice_len == 0 {
        cut_row = (0..mat_row_len).into_iter().collect::<Vec<usize>>();
    } else {
        cut_row = Vec::from(row_slice);
    }
    if col_slice_len == 0 {
        cut_col = (0..mat_col_len).into_iter().collect::<Vec<usize>>();
    } else {
        cut_col = Vec::from(col_slice);
    }
    let new_mat = mat
        .iter()
        .enumerate()
        .filter(|(idx, _)| cut_row.contains(idx))
        .map(|(_, v)| {
            v.iter()
                .enumerate()
                .filter(|(jdx, _)| cut_col.contains(jdx))
                .map(|(_, v0)| *v0)
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>();
    Some(new_mat)
}

pub fn permutation_cols<T: Clone>(org_list: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let base0 = org_list[0].len();
    let bits0 = org_list.len();
    //按照base进制的bits0位数码全排列
    let k = base_n_permutation(base0, bits0);
    //从全排列充删除不符合要求的排列
    //将每一行列表放入set中去重，然后计算长度是否与base 一致
    let mut set_to_fill = HashSet::<usize>::with_capacity(base0);
    let fill_check_mat = k
        .iter()
        .filter(|&vv| {
            set_to_fill.clear();
            vv.iter().for_each(|&x1| {
                set_to_fill.insert(x1);
            });
            set_to_fill.len() == base0
        })
        .map(|k1| k1.clone())
        .collect::<Vec<Vec<usize>>>();
    let rst: Vec<Vec<T>> = fill_check_mat
        .iter()
        .enumerate()
        .map(|(_, v)| {
            v.iter()
                .enumerate()
                .map(|(row, &col)| org_list[row][col].clone())
                .collect()
        })
        .collect();
    rst
}
