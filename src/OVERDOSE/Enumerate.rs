
use List;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{BufReader, BufWriter};
use std::collections::btree_map;
use std::collections::HashMap;
use std::ops::Shr;
use std::fmt::Debug;


pub fn Enumerate<A, I:Iterator<Item=A>>(i:I) -> Vec<(i32,A)> {
  let mut index:i32 = 0;
  let mut tmp:Vec<(i32,A)> = Vec::new();
  for a in i {
    tmp.push((index, a));
  }
  tmp
}

