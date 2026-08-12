mod common;

#[test]
fn nested_conditionals() {
  let source_code: String = "
    perhaps (1) {
      perhaps (2) {
        #nothing
      }
      otherwise {
        perhaps (3) {
          #nothing
        }
      }
    }".to_string();
  let ast = common::string_to_ast(source_code);
  assert!(ast.is_ok());
}
