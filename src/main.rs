

use std::collections::HashMap;
extern crate itertools;
use itertools::Itertools;
use std::ops::Shr;
mod OVERDOSE;
use OVERDOSE::List::List;
use OVERDOSE::List::newList;
use OVERDOSE::File::Read;
use OVERDOSE::Enumerate::Enumerate;


fn main() {
  // The statements here will be executed when the compiled binary is called
  // Print text to the console
  let contents = Read("./resource/iris.csv", true);
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
      println!("KEY GROUPBY : {}", key);
      key
    })
    .map( &|it| { 
      let (key, val) = it;
      println!("KEY : {} ",key);
      val.echo();
    });
}

