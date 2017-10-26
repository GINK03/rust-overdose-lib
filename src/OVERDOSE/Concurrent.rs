use std::thread;
use std::sync::Arc;
use std::fmt::{Debug, Display};
use std::marker::{Send,Sync,Sized};

pub struct Concurrent {
}

impl Concurrent {
  pub fn map<T:Clone+Send+Debug+'static+Sized, F:Send+Sized, OUTPUT:Clone+Send+Sized+Debug+'static>( vec: Vec<T>, functor: F ) 
  -> Vec<OUTPUT> where F: Send + 'static + Fn(T) -> OUTPUT + Sync{
    let functor = Arc::new(functor);
    let handlers = vec.iter().map( |v| { 
      let v = v.clone();
      let f = functor.clone();
      let handler = thread::Builder::new()
        .name( "threads".to_string() )
        .spawn(move || {
          f(v)
        })
        .unwrap();
      handler
    }).collect::<Vec<_>>();
    
    let mut res:Vec<OUTPUT> = Vec::new();
    for handle in handlers { 
      match handle.join().ok() {
        Some(r) => { res.push(r);},
        None => {},
      }
    }
    res
  }
}
