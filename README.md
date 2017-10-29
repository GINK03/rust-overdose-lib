# Rust Overdose - 優しいRustのデータ分析 -

## 項目
- 四則演算とベクトル

## 四則演算とベクトル
### ベクトルの初期化
[1, 2, 3, 4, 5]というベクトルが欲しい場合、このように初期化することができます  
- 標準関数で行う場合
```rust
let bs = (1..6).collect::<Vec<i32>>(); 
println!("{:?}", bs);
[1, 2, 3, 4, 5]
```
- OVERDOSEで行う場合
```rust
let bs = RFrame::withRange(1,6);
println!("{:?}", bs.vec);
[1, 2, 3, 4, 5]
```

### ベクトル演算
[1, 2, 3, 4, 5]のすべての要素に3をかける  
- 標準関数で行い場合
```rust
let bs = (1..6).map(|x| x*3).collect::<Vec<i32>>();  
println!("{:?}", bs); 
[3, 6, 9, 12, 15]
```
- OVERDOSEで行う場合   
(参照でクロージャーを受け取るときはシングルプロセスで動作します)
```rust
let bs = RFrame::withRange(1,6).map( &|x| {x*3} ); 
println!("{:?}", bs.vec); 
[3, 6, 9, 12, 15] 
```

### ベクトル同士の演算
xs = [1, 2, 3, 4, 5]  
ys = [6, 7, 8, 9, 10]  
この二つの演算で、足し算をします  
- 標準関数で行う場合
```rust
let bs = RFrame::withRange(1,6).vec.iter().zip( RFrame::withRange(6,11).vec.iter() ).map( |x| (x.0.clone()+x.1.clone()) ).collect::<Vec<i32>>(); 
println!("{:?}", bs);
[7, 9, 11, 13, 15]
```

### ベクトルのサイズの確認
xs = [1, 2, 3, 4, 5]  
これのサイズを求める
- 標準関数で行う場合
```rust
let size = (1..6).into_iter().collect::<Vec<i32>>().len();   
println!("{:?}", size);
5
```
- OVERDOSEで行う場合
```rust
let size = RFrame::withRange(1,6).vec.len(); 
println!("{:?}", size);
5
```
