
pub mod OVERDOSE1 {
  use std::fs::File;
  use std::io::prelude::*;
  use std::path::Path;
  use std::io::{BufReader, BufWriter};
  use std::collections::btree_map;
  use std::collections::HashMap;
  use std::ops::Shr;
  use std::fmt::Debug;
  pub fn read( name:&str, header:bool ) -> Vec<HashMap<String, String>> {
    let path = Path::new(name);
    let mut file = BufReader::new(File::open(&path).unwrap());
    let v:Vec<String> = file.lines().map(|x| { x.unwrap() }).collect();

    let keys = &v[0].clone();
    let keys = keys.split(",").map( |x| { /*println!("{}", x);*/ x.to_string() } ).collect::<Vec<String>>() ;
   
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
}  

use std::collections::HashMap;
extern crate itertools;
use itertools::Itertools;
use std::ops::Shr;
mod OVERDOSE;
use OVERDOSE::List::List;
use OVERDOSE::List::newList;
use OVERDOSE::Enumerate::Enumerate;


fn main() {
  // The statements here will be executed when the compiled binary is called
  // Print text to the console
  let contents = OVERDOSE1::read("./resource/iris.csv", true);
  contents.iter().map( |hmap| { 
    hmap.iter().map( |x| {
      let (key,val) = x;
      println!("{} {}", key, val);
      (0,0)
    }).collect::<HashMap<_,_>>();
  } ).collect::<Vec<_>>();
  //contents.iter().map(|x| {println!("{}",x);0} ).collect::<Vec<_>>();
  (0..100).map( |x| { println!("{}",x);(x%5,x)}) ;
  List{ vec: (0..100).collect::<Vec<i32>>() }
    .map( &|x| { x*x } )
    .map( &|x| { 
      println!("{}",x);
      0
    } )  
    ;
  let ret = List{ vec: (0..100).collect::<Vec<i32>>() }.all( &|x| { x%1000 == x }); 
  println!("{}", ret);
  let ret = List{ vec: (0..100).collect::<Vec<i32>>() }.all( &|x| { x%10 == x }); 
  println!("{}", ret);
  List{ vec: (0..100).collect::<Vec<i32>>() }.echo();

  let reduce = List{ vec: (0..100).collect::<Vec<i32>>() }.reduce(0, &|y:i32, x:i32| { y + x });
  println!("reduce result {}", reduce);
  let reduce = List{ vec: (1..10).collect::<Vec<i64>>() }.reduce(1, &|y:i64, x:i64| { y * x });
  println!("reduce result {}", reduce);

  Enumerate( (1..10).collect::<Vec<i64>>().iter() );

  //List{ vec:(0..10) };
  newList(1,100).map( &|x| {  println!("{}",x); } );

  newList(1,100) 
    .map( &|x| { x } )
    .groupBy( &|x| { 
      let key = x%5;
      println!("KEY1 {}", key);
      key
    })
    .map( &|it| { 
      let (key, val) = it;
      println!("KEY : {} ",key);
      val.echo();
    });
}

