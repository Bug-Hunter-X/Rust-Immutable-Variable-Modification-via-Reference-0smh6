fn main() { let x = 5; let y = &x; *y = 6; println!("{}", x); }