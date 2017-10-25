

use std::collections::HashMap;
use std::collections::HashSet;
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
  
  // test readl time map
  assert_eq!( newList(1,10).map( &|x| { println!("{}",x); x} ).vec, [1,2,3,4,5,6,7,8,9]);

  // test reduce 1
  let reduce = List{ vec: (0..100).collect::<Vec<i32>>() }.reduce(0, &|y:i32, x:i32| { y + x });
  assert_eq!( reduce, 4950);
  println!("reduce result {}", reduce);

  // test reduce 2
  let reduce = List{ vec: (1..10).collect::<Vec<i64>>() }.reduce(1, &|y:i64, x:i64| { y * x });
  assert_eq!( reduce, 362880 );
  println!("reduce result {}", reduce);

  // groupbyとsortbyのテスト
  let groupby = newList(1,100) 
    .map( &|x| { x } )
    .groupBy( &|x| { 
      let key = x%3;
      key
    })
    .map( &|it| { 
      let (key, val) = it;
      let len = val.vec.len();
      println!("KEY : {} VAL-LEN : {}",key, len);
      (key, len)
    }).sortBy( &|x|{ x.0 } );
  assert_eq!(groupby.vec, [(0, 33), (1, 33), (2, 33)].iter().cloned().collect::<Vec<(i32,usize)>>());
  // これはただ出せばいいだけ
  newList(10,30).show();

  // sum関数のテスト
  assert_eq!(newList(1,100).sum(), 4950);

  // min関数のテスト
  let min = newList(10, 100).min();
  println!("MIN : {}", min.unwrap());
  assert_eq!(min, Some(10));
  
  // max関数のテスト
  let max = newList(10, 100).max();
  println!("MAX : {}", max.unwrap());
  assert_eq!(max, Some(99));

  let repeat = newList(1,3).repeat(2);
  repeat.echo();

  let product = newList(1,3).product(3);
  for p in product.vec {
    p.echo();
  }
 
  let accumulate = newList(1,10).accumulate();
  assert_eq!(accumulate.vec, [1,3,6,10,15,21,28,36,45]);
  println!("Accumulated {:?}", accumulate); 

  // toVecのテスト
  let to_vec = newList(1,4).map( &|x|{ x*2} ).toVec();
  println!("to vec {:?}", to_vec);
  assert_eq!(to_vec, [2,4,6]);
  
  // toSetのテスト
  let to_set = List{vec: vec![1,2,3,4,5,4,3]}.toSet();
  println!("to set {:?}", to_set);
  let hashset:HashSet<i32> = vec![1,2,3,4,5].into_iter().collect();
  assert_eq!(to_set, hashset);
}

