use RFrame;
extern crate num;
use self::num::Num;
use self::num::cast;
use self::num::ToPrimitive;
use self::num::NumCast;

pub fn fac(t:i64) -> f64 {
  let t = t as i64;
  let mut buff:f64 = 1.0;
  for a in (1..t+1) {
    buff *= a as f64;
  }
  buff
}
// Please Refere this URL
// https://ja.wikipedia.org/wiki/%E3%83%95%E3%82%A3%E3%83%83%E3%82%B7%E3%83%A3%E3%83%BC%E3%81%AE%E6%AD%A3%E7%A2%BA%E7%A2%BA%E7%8E%87%E6%A4%9C%E5%AE%9A
pub struct Fisher{
}

impl Fisher{
  pub fn compareNormal(observe: (i64, i64, i64, i64) ) -> (f64, Vec<(i64, f64)>) {
    let (a,b,c,d) = observe;    
    let min = 0;
    let max = a+c;
    let N = a+b+c+d;
    
    let pb = fac(a+b)*fac(c+d)*fac(a+c)*fac(b+d) / (fac(N)*fac(a)*fac(b)*fac(c)*fac(d));
   
    let mut buff:Vec<(i64, f64)> = Vec::new();
    for A in (min..max) {
      let C = a+c-A;
      let p = fac(A+b)*fac(C+d)*fac(A+C)*fac(b+d) / (fac(N)*fac(A)*fac(b)*fac(C)*fac(d));
      buff.push( (A, p) ); 
    }

    let total:f64 = buff.iter().map( |x| {
      let (a,p) = x.clone();
      p
    }).fold(0.0, |acc,x| acc + x);
    
    let mut inte:f64 = 0.0;
    for x in buff.iter() {
      let (a_,p) = x.clone();
      if a_ < a { 
        inte += p;
      }
    }
    let pval = inte/total;
    (pval, buff)
  }
}
