//面向对象设计

//单元格
use crate::obj::event::Event;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::{Sub, SubAssign};
use std::rc::Rc;

const APPROX_BIT: i32 = 15;

#[derive(Debug, Clone)]
pub struct Cell {
    row: usize,
    col: usize,
    val: f64,
    event: Event,
}

impl Sub for Cell {
    type Output = f64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.val - rhs.val
    }
}

impl SubAssign for Cell {
    fn sub_assign(&mut self, rhs: Self) {
        let tmp = self.val - rhs.val;
        self.val = tmp;
    }
}

impl Cell {
    pub fn rc_ref(&self) -> Rc<&Cell> {
        Rc::new(self)
    }
    pub fn index(&self) -> (usize, usize) {
        (self.row, self.col)
    }
    pub fn is_zero(&self) -> bool {
        self.approx_eq(0f64)
    }

    pub fn to_int(&self) -> i64 {
        let times: f64 = 10f64.powi(APPROX_BIT);
        ((self.val * times * 10.0).trunc() / 10.0).round() as i64
    }

    pub fn approx_eq(&self, f64_num: f64) -> bool {
        let times: f64 = 10f64.powi(APPROX_BIT);
        let bi = ((f64_num * times * 10.0).trunc() / 10.0).round() as i64;
        self.to_int() == bi
    }

    pub fn approx(&self) -> f64 {
        let times: f64 = 10f64.powi(APPROX_BIT);
        let xi = ((self.val * times * 10.0).trunc() / 10.0).round() as i64;
        (xi as f64) / times
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_int().cmp(&other.to_int())
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.approx_eq(other.val())
    }
}
impl Eq for Cell {}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{}", self.val).hash(state)
    }
}

//Getter
impl Cell {
    pub fn row(&self) -> usize {
        self.row
    }
    pub fn col(&self) -> usize {
        self.col
    }
    pub fn val(&self) -> f64 {
        self.val
    }
    pub fn to_string(&self) -> String {
        format!("{}({},{})", self.val, self.row, self.col)
    }
    pub fn event(&self) -> &Event {
        self.event.borrow()
    }
}
// 构造
impl Cell {
    pub fn new_with_val(row: usize, col: usize, event: &Event, val: f64) -> Self {
        Cell {
            row,
            col,
            val,
            event: event.clone(),
        }
    }
    pub fn new(row: usize, col: usize, event: &Event) -> Self {
        Cell {
            row,
            col,
            val: 0f64,
            event: event.clone(),
        }
    }
}
