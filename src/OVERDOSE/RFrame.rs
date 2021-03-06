
extern crate serde_json;
extern crate serde;
use serde_json::{Value, Error};
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
use std::marker::Copy;
// num packages dependencies
extern crate num;
use self::num::FromPrimitive;
use self::num::Num;
use std::hash::{Hash, Hasher};
//use std::num::Zero;
//use std::num::Num;
use Concurrent::Concurrent;                
use RowOrientedCSV;
#[derive(Debug, Serialize, Deserialize)]
pub struct RFrame<T:Clone>{
  pub header:Option<HashMap<String,i32>>,
  pub cursol:i32,
  pub vec: Vec<T>,
}
#[derive(Debug)]
pub struct INNER<TI:Clone>{
  pub cursol:i32,
  pub inner: Vec<TI>,
}

// operator >> のオーバーロード
impl<T: Clone> Shr<usize> for RFrame<T> {
  type Output = Self;
  fn shr(self, rhs: usize) -> RFrame<T> {
    RFrame::withVec( Vec::new() )
  }
}

// Cloneの実装
impl<T: Clone> Clone for RFrame<T> {
  fn clone(&self) -> RFrame<T> { 
    RFrame::withVec( self.vec.clone() )
  }
}

// iteratorによる初期化
impl<T:Clone> RFrame<T> {
  pub fn new<A:Clone,IT: Iterator<Item=A>>(it:IT) -> RFrame<A> {
    let mut tmp:Vec<A> = Vec::new();
    for i in it {
      tmp.push(i);
    }
    RFrame::withVec(tmp)
  }
}

// pushの実装
impl<T:Clone> RFrame<T> {
  pub fn push(mut self, t:T) -> () {
    self.vec.push(t.clone());
  }
}

// insertHeader
impl<T:Clone> RFrame<T> {
  pub fn insertHeader<IN:Clone+Display>(mut self, headers:Vec<IN> ) -> RFrame<T> {
    let mut index:i32 = 0;
    let mut map:HashMap<String,i32> = HashMap::new();
    for head in headers.clone() {
      map.insert( head.to_string(), index); 
      index+=1;
    }
    self.header = Some( map );
    self
  }
}
// showの実装
impl<T:Clone + Display> RFrame<T> {
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
impl<T: Clone+Send+Debug+'static> RFrame<T> { 
  // cmap
  pub fn cmap<OUTPUT: Clone+Send+Debug+'static,F:Send+Sized>(self, functor: F) 
    -> RFrame<OUTPUT> where F: Send + 'static + Fn(T) -> OUTPUT + Sync {
    let vec = Concurrent::chunkedMap(self.vec, move |x| { functor(x.clone()) } );
    RFrame::withVec(vec)
  }
}
impl<T: Clone> RFrame<T> {
  // map(安全なマップ)
  pub fn map<OUTPUT: Clone>(self, functor: &Fn(T) -> OUTPUT) -> RFrame<OUTPUT> {
    let vec = self.vec.iter().map( |x| { functor(x.clone()) } ).collect::<Vec<OUTPUT>>();
    RFrame::withVec(vec)
  }
  // reduce
  pub fn reduce<OUTPUT: Clone>(self, init:OUTPUT, functor: &Fn(OUTPUT, T) -> OUTPUT) -> OUTPUT {
    let mut y = init.clone();
    for v in self.vec {
      y = functor(y, v);
    };
    y
  }
  // filter
  pub fn filter(self, functor: &Fn(T) -> bool) -> RFrame<T> {
    let mut ret:Vec<T> = Vec::new();
    for v in self.vec {
      if functor(v.clone()) == true {
        ret.push(v); 
      }
    }
    RFrame::withVec(ret)
  }
  // concat
  pub fn concat(mut self, rframe: RFrame<T> ) -> RFrame<T> {
    for v in rframe.vec {
      self.vec.push(v); 
    }
    RFrame::withVec(self.vec)
  }
  // sortBy (安全なソート)
  pub fn sortBy<FUNCRET: Clone+Ord>(self, functor: &Fn(T) -> FUNCRET) -> RFrame<T> {
    let mut cloned = self.vec.clone();
    cloned.sort_by_key( |key|{
      let funcret:FUNCRET = functor(key.clone());
      funcret
    }); 
    RFrame::withVec(cloned)
  }
  // zip 
  pub fn zip<RI:Clone>(self, rframe:RFrame<RI>) -> RFrame<(T,RI)> {
    let mut size:i32 = self.vec.len() as i32;
    if size < rframe.vec.len() as i32 { size = rframe.vec.len() as i32 };
    let mut ret:Vec<(T,RI)> = Vec::new();
    for i in (0..self.vec.len()) {
      ret.push( (self.vec[i].clone(), rframe.vec[i].clone()) );
    }
    RFrame::withVec(ret)
  }
  // groupBy
  pub fn groupBy<OUTPUT: Clone + Eq + Hash + PartialEq>(self, functor: &Fn(T) -> OUTPUT) -> RFrame<(OUTPUT,RFrame<T>)> {
    let mut map:HashMap<OUTPUT,RFrame<T>> = HashMap::new();
    for v in self.vec.clone() {
      let key:OUTPUT = functor(v.clone());
      let val = v.clone();
      let mut result = match map.entry(key) {
        Vacant(entry) => entry.insert( RFrame::withVec(Vec::new()) ),
        Occupied(entry) => entry.into_mut(),
      };
      result.vec.push(val);
    }
    let mut ret:RFrame<(OUTPUT, RFrame<T>)> = RFrame::withVec(Vec::new());
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
  // any
  pub fn any(self, functor: &Fn(T) -> bool) -> bool {
    let mut counter = 0;
    for v in self.vec { 
      if functor(v) == true { counter+=1 }
    }
    counter > 0
  }
  // repeat
  pub fn repeat(self, repeatNum: i32) -> RFrame<T> {
    let mut ret:Vec<T> = Vec::new();
    for x in 0..repeatNum {
      for v in self.vec.clone() {
        ret.push(v);
      }
    }
    RFrame::withVec(ret)
  }
}

// echo
impl<T: Clone+Debug> RFrame<T> {
  pub fn echo(self) -> () {
    print!("[");
    for v in self.vec {
      print!("{:?},", v);
    }
    println!("]");
  }
}

// product 
impl<T: Clone+Copy+Debug> RFrame<T> {
  pub fn product(self, depth: i32) -> RFrame<RFrame<T>> {
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
    let mut rr: RFrame<RFrame<T>> = RFrame::withVec(Vec::new());
    for r in rs {
      rr.vec.push( RFrame::withVec(r) );
    } 
    rr
  }
}
// toVecの実装
impl<T: Clone+Num+Copy+Debug> RFrame<T> {
  pub fn toVec(self) -> Vec<T> {
    self.vec.clone()
  }
}
// toSetの実装
impl<T: Clone+Eq+Hash+Num+Copy+Debug> RFrame<T> {
  pub fn toSet(self) -> HashSet<T> {
    let mut set:HashSet<T> = HashSet::new();
    for v in self.vec { 
      set.insert(v);
    }
    set
  }
}
// toJsonの実装y
impl<T: Clone+Eq+Debug+serde::ser::Serialize> RFrame<T> {
  pub fn toJson(self) -> Vec<String> {
    let mut ret:Vec<String> = Vec::new();
    for entry in self.vec {
      if let Some(json) = serde_json::to_string(&entry).ok() {
        ret.push(json);
      }
    }
    ret
  }
}
// uniqの実装
impl<T: Clone+Eq+Hash+Num+Copy+Debug> RFrame<T> {
  pub fn toUniq(self) -> RFrame<T> {
    let mut set:HashSet<T> = HashSet::new();
    for v in self.vec { 
      set.insert(v);
    }
    let mut vec:Vec<T> = Vec::new();
    for s in set {
      vec.push(s);
    }
    RFrame::withVec(vec)
  }
}
// accumulate
impl<T: Clone+Num+Copy+Debug> RFrame<T> {
  pub fn accumulate(self) -> RFrame<T> {
    let mut acc:T = T::zero();
    let mut ret:Vec<T> = Vec::new();
    for v in self.vec {
      acc = acc + v;
      ret.push(acc);
    }
    RFrame::withVec(ret)
  }
}
// sum
impl<T: Clone+Num+Copy+Debug> RFrame<T> {
  pub fn sum(self) -> T {
    let mut ret:T = T::zero();
    for v in self.vec {
      ret = ret + v;
    }
    ret
  }
}
// min
impl<T: Clone+Num+PartialOrd+Copy+Debug> RFrame<T> {
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
impl<T: Clone+Num+PartialOrd+Copy+Debug> RFrame<T> {
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
// histAuto(自動のキーの粒度で、histを作成)

// csvで初期化することができる
impl RFrame<i32> {
  pub fn withCSV(name:&str) -> RFrame<HashMap<String,String>> {
    let name = name.to_string();
    let mut tmp:Vec<HashMap<String,String>> = RowOrientedCSV::concurrentOpen(name);
    RFrame::withVec( tmp )
  }
}
// rangeで初期化することができる
impl RFrame<i32> {
  pub fn withRange(start:i32, end:i32) -> RFrame<i32> {
    let mut tmp:Vec<i32> = (start..end).collect::<Vec<i32>>();
    RFrame::withVec( tmp )
  }
}
// vecで初期化することができる( vecはコピーでなくて譲渡 )
impl<T: Clone> RFrame<T> {
  pub fn withVec( vs:Vec<T> ) -> RFrame<T> {
    RFrame { header:None, cursol:0, vec:vs }
  }
  pub fn withVecIndexed( vi:Vec<String>, vs:Vec<T> ) -> RFrame<T> {
    let mut i = 0;
    let mut map:HashMap<String, i32> = HashMap::new();
    for v in vi {
      map.insert(v,i); 
      i += 1;
    }
    RFrame { header:Some(map), cursol:0, vec:vs }
  }
}
// headerでindexing
impl<T: Clone> RFrame<Vec<T>> {
  pub fn index<AG:Display>(self, key:AG) -> RFrame<Vec<T>> {
    let key:String = key.to_string();
    let mut vec:Vec<Vec<T>> = Vec::new();
    match self.header {
      Some(head) => { 
        let index:usize = match head.get(&key) {
           Some(entry) => *entry as usize, 
           None => 0,
        };
        for v in self.vec { 
          vec.push( [v[index].clone()].to_vec() );
        };
      },
      None => {},
    };
    RFrame::withVec(vec)
  }
}
// headerでindexing(複数要素)
impl<T: Clone> RFrame<Vec<T>> {
  pub fn indexes<AC:Display>(self, keys:Vec<AC>) -> RFrame<Vec<T>> {
    let keysStr:Vec<String> = keys.iter().map( |x| x.to_string()).collect::<Vec<String>>();
    let mut vec:Vec<Vec<T>> = Vec::new();
    match self.header {
      Some(head) => { 
        for v in self.vec { 
          let mut temp:Vec<T> = Vec::new();
          for k in keysStr.clone() {
            let index:usize = match head.get(&k) {
              Some(entry) => *entry as usize, 
              None => 0,
            };
            temp.push( v[index].clone() );
          }
          vec.push( temp );
        };
      }
      None => {
        // use phantom keys
        let head = keysStr.iter().map (|x| { 
          (x, x.parse::<usize>().unwrap())} 
        ).collect::<HashMap<&String,usize>>();

        for v in self.vec { 
          let mut temp:Vec<T> = Vec::new();
          for k in keysStr.clone() {
            let index:usize = match head.get(&k) {
              Some(entry) => *entry as usize, 
              None => 0,
            };
            temp.push( v[index].clone() );
          }
          vec.push( temp );
        };
      },
    };
    RFrame::withVec(vec)
  }
}

// Blackのデータフレームを作る
impl<T: Clone> RFrame<T> {
  pub fn withBlank( ) -> RFrame<T> {
    let v:Vec<T> = Vec::new();
    RFrame::withVec(v)
  }
}
