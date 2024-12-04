fn main() {
  let mut parser = tree_sitter::Parser::new();
  parser.set_language(&tree_sitter_swift::LANGUAGE.into()).unwrap();

  let my_source_code = "let x = 5";

  let tree = parser.parse(&my_source_code, None);
  println!("{:?}", tree.unwrap().root_node().to_sexp());
}
