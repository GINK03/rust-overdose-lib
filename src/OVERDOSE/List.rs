
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
use std::cmp::{Eq, Ord, PartialOrd};
use std::collections::HashSet;
// num packages dependencies
extern crate num;
use self::num::FromPrimitive;
use self::num::Num;
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

// iteratorによる初期化
impl<T:Clone> List<T> {
  pub fn new<A:Clone,IT: Iterator<Item=A>>(it:IT) -> List<A> {
    let mut tmp:Vec<A> = Vec::new();
    for i in it {
      tmp.push(i);
    }
    List { vec: tmp }
  }
}

// pushの実装
impl<T:Clone> List<T> {
  pub fn push(mut self, t:T) -> () {
    self.vec.push(t.clone());
  }
}

// showの実装
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
  // map(安全なマップ)
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
  // sortBy (安全なソート)
  pub fn sortBy<FUNCRET: Clone+Ord>(self, functor: &Fn(T) -> FUNCRET) -> List<T> {
    let mut cloned = self.vec.clone();
    cloned.sort_by_key( |key|{
      let funcret:FUNCRET = functor(key.clone());
      funcret
    }); 
    List{ vec:cloned }
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
  // repeat
  pub fn repeat(self, repeatNum: i32) -> List<T> {
    let mut ret:Vec<T> = Vec::new();
    for x in 0..repeatNum {
      for v in self.vec.clone() {
        ret.push(v);
      }
    }
    List{ vec:ret }
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

// product 
impl<T: Clone+Copy+Debug> List<T> {
  pub fn product(self, depth: i32) -> List<List<T>> {
    let mut rs:Vec<Vec<T>> = Vec::new(); 
    // initialize
    for v in self.vec.clone() {  rs.push( [v].to_vec() ); }

    for r in 0..depth-1 { 
      let mut tmp:Vec<Vec<T>> = Vec::new();
      for v in self.vec.clone() {
        for mut r in rs.clone() {
          r.push(v);
          tmp.push(r);
        }
      }
      rs = tmp;
    }
    let mut rr: List<List<T>> = List{ vec:Vec::new() };
    for r in rs {
      rr.vec.push( List{vec:r} );
    } 
    rr
  }
}
// toVecの実装
impl<T: Clone+Num+Copy+Debug> List<T> {
  pub fn toVec(self) -> Vec<T> {
    self.vec
  }
}
// toSetの実装
impl<T: Clone+Eq+Hash+Num+Copy+Debug> List<T> {
  pub fn toSet(self) -> HashSet<T> {
    let mut set:HashSet<T> = HashSet::new();
    for v in self.vec { 
      set.insert(v);
    }
    set
  }
}
// uniqの実装
impl<T: Clone+Eq+Hash+Num+Copy+Debug> List<T> {
  pub fn toUniq(self) -> List<T> {
    let mut set:HashSet<T> = HashSet::new();
    for v in self.vec { 
      set.insert(v);
    }
    let mut vec:Vec<T> = Vec::new();
    for s in set {
      vec.push(s);
    }
    List{vec:vec}
  }
}
// accumulate
impl<T: Clone+Num+Copy+Debug> List<T> {
  pub fn accumulate(self) -> List<T> {
    let mut acc:T = T::zero();
    let mut ret:Vec<T> = Vec::new();
    for v in self.vec {
      acc = acc + v;
      ret.push(acc);
    }
    List{vec:ret}
  }
}
// sum
impl<T: Clone+Num+Copy+Debug> List<T> {
  pub fn sum(self) -> T {
    let mut ret:T = T::zero();
    for v in self.vec {
      ret = ret + v;
    }
    ret
  }
}
// min
impl<T: Clone+Num+PartialOrd+Copy+Debug> List<T> {
  pub fn min(self) -> Option<T> {
    let result = match self.vec.clone().first() {
      Some(head) => {
        let mut ret = *head; 
        for v in self.vec {
          if( ret > v ) { ret = v; }
        }
        Some(ret)
      },
      None => None
    };
    result
  }
}
// max
impl<T: Clone+Num+PartialOrd+Copy+Debug> List<T> {
  pub fn max(self) -> Option<T> {
    let result = match self.vec.clone().first() {
      Some(head) => {
        let mut ret = *head; 
        for v in self.vec {
          if( ret < v ) { ret = v; }
        }
        Some(ret)
      },
      None => None
    };
    result
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
