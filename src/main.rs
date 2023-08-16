fn main() {
  // not Copy
  let item = Some(String::from("item"));
  match item {
      // 使用ref引用了Option里的值，所以并没有消耗item
      Some(ref it) => println!("{it}"),
      None => unreachable!(),
  }
  // item在match后仍可使用
  println!("{item:?}")
}