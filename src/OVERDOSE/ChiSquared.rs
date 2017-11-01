
use RFrame;
extern crate num;
use self::num::Num;
use self::num::cast;
use self::num::ToPrimitive;
use self::num::NumCast;

// Please Refere this URL
// if you need enbeded to paramters, please hardcode here
// https://www.medcalc.org/manual/chi-square-table.php
pub struct ChiSquared{
}

impl ChiSquared{
  pub fn compareFreqs<T:Clone+Num+NumCast>(left:RFrame<T>, right:RFrame<T>) {
    let chi_sq:f64 = 0.0;
    let sum = left.vec.iter().zip(right.vec.iter()).map ( |x| {
      let (right, left) = ( (*x.0).clone(), (*x.1).clone());
      let right:f64 = cast(right).unwrap();
      let left:f64 = cast(left).unwrap();
      let ent = (right - left) * (right - left) / left;
      ent
    }).fold(0.0, |acc, x| acc + x);
    println!("sum {}", sum);
  }
 
  // 対立仮説との差を見る
  pub fn compareBase<T:Clone+Num+NumCast>(observe:RFrame<T>) {
    let chi_sq:f64 = 0.0;
    let size:f64 = observe.vec.len() as f64;
    let base:f64 = observe.vec.iter().map(|x| { 
      let c:f64 = cast( x.clone() ).unwrap();
      c 
    } ).fold(0.0, |acc,x| acc+ x) as f64 / size;

  }
}
