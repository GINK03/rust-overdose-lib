
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::collections::btree_map;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::ops::Shr;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add};

// num packages dependencies
extern crate num;
use num::FromPrimitive;
//use std::num::Zero;
//use std::num::Num;

use std::hash::{Hash, Hasher};
#[derive(Debug)]
pub struct List<T:Clone>{
  pub vec: Vec<T>,
}
// operator >> のオーバーロード
impl<T: Clone> Shr<usize> for List<T> {
  type Output = Self;
  fn shr(self, rhs: usize) -> List<T> {
    List { vec: Vec::new() }
  }
}
// Cloneの実装
impl<T: Clone> Clone for List<T> {
  fn clone(&self) -> List<T> { 
    List{ vec:self.vec.clone() }
  }
}
impl<T:Clone> List<T> {
  // iteratorによる初期化
  pub fn new<A:Clone,IT: Iterator<Item=A>>(it:IT) -> List<A> {
    let mut tmp:Vec<A> = Vec::new();
    for i in it {
      tmp.push(i);
    }
    List { vec: tmp }
  }
}
impl<T:Clone> List<T> {
  pub fn push(mut self, t:T) -> () {
    self.vec.push(t.clone());
  }
}
impl<T:Clone + Display> List<T> {
  pub fn show(self) -> () {
    let mut vstring:Vec<String> = Vec::new();
    for v in self.vec.clone() {
      vstring.push(v.to_string());
      vstring.push(", ".to_string());
    }
    let last = vstring.len() - 1;
    vstring.remove(last);
    print!("[");
    for s in vstring { 
      print!("{}", s);
    }
    println!("]");
  }
}
impl<T: Clone> List<T> {
  // map
  pub fn map<OUTPUT: Clone>(self, functor: &Fn(T) -> OUTPUT) -> List<OUTPUT> {
    let vec = self.vec.iter().map( |x| { functor(x.clone()) } ).collect::<Vec<OUTPUT>>();
    List { vec:vec }
  }
  // reduce
  pub fn reduce<OUTPUT: Clone>(self, init:OUTPUT, functor: &Fn(OUTPUT, T) -> OUTPUT) -> OUTPUT {
    let mut y = init.clone();
    for v in self.vec {
      y = functor(y, v);
    };
    y
  }
  // groupBy
  pub fn groupBy<OUTPUT: Clone + Eq + Hash + PartialEq>(self, functor: &Fn(T) -> OUTPUT) -> List<(OUTPUT,List<T>)> {
    let mut map:HashMap<OUTPUT,List<T>> = HashMap::new();
    for v in self.vec.clone() {
      let key:OUTPUT = functor(v.clone());
      let val = v.clone();
      let mut result = match map.entry(key) {
        Vacant(entry) => entry.insert( List{ vec:Vec::new()} ),
        Occupied(entry) => entry.into_mut(),
      };
      result.vec.push(val);
    }
    let mut ret:List<(OUTPUT, List<T>)> = List{ vec:Vec::new()};
    for (key, vals) in map {
      ret.vec.push( (key,vals) );
    }
    ret
  }
  // all
  pub fn all(self, functor: &Fn(T) -> bool) -> bool {
    let mut counter = 0;
    let total_size = self.vec.len();
    for v in self.vec { 
      if functor(v) == true { counter+=1 }
    }
    counter == total_size
  }
}

// echo
impl<T: Clone+Debug> List<T> {
  pub fn echo(self) -> () {
    print!("[");
    for v in self.vec {
      print!("{:?},", v);
    }
    println!("]");
  }
}

// sum
impl<T: Clone+Int+Debug> List<T> {
  pub fn sum(self) -> T {
    let mut ret:T = T::zero();
    for v in self.vec {
      ret + v;
    }
    ret
  }
}
pub fn newList(start:i32, end:i32) -> List<i32> {
  let mut tmp:Vec<i32> = (start..end).collect::<Vec<i32>>();
  List { vec: tmp }
}
pub fn newBlankList<T:Clone>() -> List<T> {
  let tmp:Vec<T> = Vec::new();
  List { vec: tmp }
}
