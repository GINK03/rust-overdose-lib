
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::collections::btree_map;
use std::collections::HashMap;
use std::ops::Shr;
use std::fmt::Debug;
pub fn Read( name:&str, header:bool ) -> Vec<HashMap<String, String>> {
  let path = Path::new(name);
  let mut file = BufReader::new(File::open(&path).unwrap());
  let v:Vec<String> = file.lines().map(|x| { x.unwrap() }).collect();

  let keys = &v[0].clone();
  let keys = keys.split(",").map( |x| { 
                /*println!("{}", x);*/ 
                x.to_string() 
              } ).collect::<Vec<String>>();
 
  let mut ret:Vec<HashMap<String,String>> = Vec::new();
  for val in &v[1..v.len()-1] {
    let val = &val.clone();
    let vals = val.split(",").map( |x| x.to_string() ).collect::<Vec<String>>();
    let mut iter = keys.iter().clone().zip(vals); 
    let one = iter.map( |x| {
      let (key,val) = x;
      (key.clone(),val.clone())
    } ).collect::<HashMap<_,_>>();

    ret.push(one);
  }
  return ret;
}
