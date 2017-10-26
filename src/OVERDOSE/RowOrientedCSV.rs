use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::collections::btree_map;
use std::collections::HashMap;
use std::ops::Shr;
use std::fmt::Debug;

use Concurrent::Concurrent;
pub struct RowOrientedCSV {
}
impl RowOrientedCSV {
  pub fn echo() {
    println!("test");
  }
}
impl RowOrientedCSV {
  fn stateMachine(line:String) -> Vec<String> {
    let chars:Vec<char> = line.chars().collect::<Vec<char>>();
    // ,であって"が続かない系列を発見したらそれはsplitコードである
    let mut sv:Vec<String> = Vec::new();
    let mut cv:Vec<char> = Vec::new();
    let mut qo:bool = false;
    let mut start = 0;
    for cursol in (1..chars.len()-1) {
      if chars[cursol] == ',' && chars[cursol+1] == '"' {
        let stri = cv.iter().collect();
        sv.push(stri);
        // clear cv
        cv = Vec::new();
        qo = true;
        continue
      }
      if (qo == true && chars[cursol-1] == '"' || qo == false ) && chars[cursol] == ',' {
        let stri = cv.iter().collect();
        sv.push(stri);
        // clear cv
        cv = Vec::new();
        qo = false;
        continue;
      }

      let buf = chars[cursol];
      cv.push(buf);
    }
    // 残りのスタックを挿入
    cv.push( *chars.last().unwrap() );

    sv.push( cv.iter().collect() );

    // sample
    let mut i = 0;
    for v in sv.clone() {
      //print!("{}:{} ", i, v);
      i += 1;
    }
    //println!();
    //println!("sample {:?}", sv.len());
    sv
  }
  pub fn open(filename:String) -> Vec<HashMap<String, String>>  {
    let path = Path::new(&filename);
    let mut file = BufReader::new(File::open(&path).unwrap());
    let v:Vec<String> = file.lines().map(|x| { x.unwrap() }).collect();

    let keys = &v[0].clone();
    let keys = RowOrientedCSV::stateMachine(keys.clone());
 
    let mut ret:Vec<HashMap<String,String>> = Vec::new();
    for val in &v[1..v.len()-1] {
      let val = &val.clone();
      let vals = RowOrientedCSV::stateMachine(val.clone());
      let map = keys.clone().iter().zip(vals.clone()).map(|x| { let (key,val) = x; (key.clone(),val.clone()) }).collect::<HashMap<String,String>>();

      //println!("hashmap {:?}", map);
      ret.push(map);
    };
    ret
  }
  pub fn concurrentOpen(filename:String) -> Vec<HashMap<String, String>>  {
    let path = Path::new(&filename);
    let mut file = BufReader::new(File::open(&path).unwrap());
    let v:Vec<String> = file.lines().map(|x| { x.unwrap() }).collect();
    
    let keys = &v[0].clone();
    let keys = RowOrientedCSV::stateMachine(keys.clone());
 
    let mut ret:Vec<HashMap<String,String>> = Vec::new();
    let mut vals:Vec<String> = Vec::new();
    for val in &v[1..v.len()-1] {
      let val = &val.clone();
      vals.push(val.clone());
    };
    ret = Concurrent::chunkedMap(vals, move |val|{
      let keysc = &keys.clone();
      let vals = RowOrientedCSV::stateMachine(val.clone());
      let map = keys.clone().iter().zip(vals.clone()).map(|x| { let (key,val) = x; (key.clone(),val.clone()) }).collect::<HashMap<String,String>>();
      map
    } );
    
    ret
  }
}
