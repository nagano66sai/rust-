fn  main(){
println!("2つの整数を入力してください");
let  mut  s=String::new();
let  mut  ss=String::new();
std::io::stdin().read_line(&mut s).ok();
std::io::stdin().read_line(&mut ss).ok();
let n:i32 =s.trim().parse().ok().unwrap();
let nn:i32 =ss.trim().parse().ok().unwrap();
let  m:i32=n+nn;

println!("入力された2数{}と{}の和は{}です。",n,nn,m);
}
