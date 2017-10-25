

use std::collections::HashMap;
use std::collections::HashSet;
extern crate itertools;
use itertools::Itertools;
use std::ops::Shr;
mod OVERDOSE;
use OVERDOSE::RFrame::RFrame;
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
  RFrame::withRange(0,100)
    .map( &|x| { x*x } )
    .map( &|x| { 
      println!("{}",x);
      0
    } )  
    ;
  let ret = RFrame::withRange(1,1000).all( &|x| { x%1000 == x }); 
  println!("{}", ret);
  let ret = RFrame::withRange(1,1000).all( &|x| { x%10 == x }); 
  println!("{}", ret);
  RFrame::withRange(1,16).echo();
  
  // test readl time map
  assert_eq!( RFrame::withRange(1,10).vec, [1,2,3,4,5,6,7,8,9]);

  // test reduce 1
  let reduce = RFrame::withRange(0,100).reduce(0, &|y:i32, x:i32| { y + x });
  assert_eq!( reduce, 4950);
  println!("reduce result {}", reduce);

  // test reduce 2)
  let reduce = RFrame::withRange(1,10).reduce(1, &|y:i32, x:i32| { y * x });
  assert_eq!( reduce, 362880 );
  println!("reduce result {}", reduce);

  // groupbyとsortbyのテスト
  let groupby = RFrame::withRange(1,100) 
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
  RFrame::withRange(10,30).show();

  // sum関数のテスト
  assert_eq!(RFrame::withRange(1,100).sum(), 4950);

  // min関数のテスト
  let min = RFrame::withRange(10, 100).min();
  println!("MIN : {}", min.unwrap());
  assert_eq!(min, Some(10));
  
  // max関数のテスト
  let max = RFrame::withRange(10, 100).max();
  println!("MAX : {}", max.unwrap());
  assert_eq!(max, Some(99));

  let repeat = RFrame::withRange(1,3).repeat(2);
  repeat.echo();

  let product = RFrame::withRange(1,3).product(3);
  for p in product.vec {
    p.echo();
  }
 
  let accumulate = RFrame::withRange(1,10).accumulate();
  assert_eq!(accumulate.vec, [1,3,6,10,15,21,28,36,45]);
  println!("Accumulated {:?}", accumulate); 

  // toVecのテスト
  let to_vec = RFrame::withRange(1,4).map( &|x|{ x*2} ).toVec();
  println!("to vec {:?}", to_vec);
  assert_eq!(to_vec, [2,4,6]);
  
  // toSetのテスト
  let to_set = RFrame::withVec(vec![1,2,3,4,5,4,3]).toSet();
  println!("to set {:?}", to_set);
  let hashset:HashSet<i32> = vec![1,2,3,4,5].into_iter().collect();
  assert_eq!(to_set, hashset);

  let to_uniq = RFrame::withVec(vec![1,2,2,2,3,3,3]).toUniq();
  assert_eq!(to_uniq.vec.clone().into_iter().collect::<HashSet<i32>>(), vec![3,2,1].into_iter().collect::<HashSet<i32>>());
  println!("to uniq {:?}", to_uniq.vec);
  
  // withVecで初期化可能
  let rv = RFrame::withVec( [3,4,5,6,7].to_vec() );
  println!("Vector initializer list {:?}", rv.vec );
  assert_eq!( rv.toVec(), [3,4,5,6,7] ); 

  // filterのテスト
  let filter = RFrame::withRange(1,10).filter( &|x|{ 
    x > 5
  });
  println!("{:?}", filter.vec);
  assert_eq!(filter.vec, [6,7,8,9]);

  // headerのテスト
  let x2 = RFrame::withVecIndexed(vec!["A","B","C"].iter().map(|x| x.to_string()).collect::<Vec<String>>(), vec![vec![1,2,3], vec![4,5,6]] );
  println!("{:?}", x2.header);

  let ix2 = x2.index("A");
  println!("{:?}", ix2.vec);
}

