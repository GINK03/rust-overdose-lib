use std::{thread,time};
use std::sync::Arc;
use std::fmt::{Debug, Display};
use std::marker::{Send,Sync,Sized};

pub struct Concurrent {
}

impl Concurrent {
  pub fn map<T:Clone+Send+Debug+'static+Sized, F:Send+Sized, OUTPUT:Clone+Send+Sized+Debug+'static>( vec: Vec<T>, functor: F ) 
  -> Vec<OUTPUT> where F: Send + 'static + Fn(T) -> OUTPUT + Sync{
    let functor = Arc::new(functor);
    let ten_millis = time::Duration::from_millis(10);
    let mut handlers:Vec<_> = Vec::new(); 
    
    for v in vec { 
      let v = v.clone();
      let f = functor.clone();
      let handler = thread::Builder::new()
        .name( "threads".to_string() )
        .spawn(move || {
          f(v)
        });
      match handler.ok() {
        Some(h) => handlers.push(h),
        None => { println!("Exceed Thread  Number"); /*thread::sleep(ten_millis)*/;}
      }
    }
    
    let mut res:Vec<OUTPUT> = Vec::new();
    for handle in handlers { 
      match handle.join().ok() {
        Some(r) => { res.push(r);},
        None => {},
      }
    }
    res
  }
  // OSのリソースを使い切ってしまうので、chunkedMapを実装して負荷分散する
  pub fn chunkedMap<T:Clone+Send+Debug+'static+Sized, F:Send+Sized, OUTPUT:Clone+Send+Sized+Debug+'static>( vec: Vec<T>, functor: F ) 
  -> Vec<OUTPUT> where F: Send + 'static + Fn(T) -> OUTPUT + Sync{
    let functor = Arc::new(functor);
    // default chunk size = 32
    let CHUNK_SIZE = 32;
    // make chunks
    let mut chunk:Vec<Vec<T>> = vec![Vec::new(); CHUNK_SIZE];
    let mut i = 0;
    for v  in vec {
      let cursol = i%CHUNK_SIZE;
      chunk[cursol].push(v);
      i += 1;
    }


    let mut handlers:Vec<_> = Vec::new(); 
    
    for ch in chunk { 
      let ch = ch.clone();
      let f = functor.clone();
      let handler = thread::Builder::new()
        .name( "threads".to_string() )
        .spawn(move || {
          let mut tmp:Vec<OUTPUT> = Vec::new();
          for v in ch {
            let r = f(v);
            tmp.push(r);
          }
          tmp
        });
      match handler.ok() {
        Some(h) => handlers.push(h),
        None => { println!("Exceed Thread  Number"); /*thread::sleep(ten_millis)*/;}
      }
    }

    // 結果を集めて統合
    let mut res:Vec<OUTPUT> = Vec::new();
    for handle in handlers { 
      match handle.join().ok() {
        Some(rs) => { 
          for r in rs {
            res.push(r);
          }
        ;},
        None => {},
      }
    }
    res

  }
}
