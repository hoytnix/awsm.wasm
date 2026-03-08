use std::collections::HashMap;

// Assuming Lexer/Parser code copied over or just test the array tokenization

fn main() {
    let script = r#"
    (awsm:translate-dialect 
      :target @processData 
      :enforce-explicit-type "DataConfig" 
      :generate-interface ["id: string" "isActive: boolean"] 
      :target-param-index 0)
    "#;
    // Just mock enough lexer rules to see the problem
    println!("Testing lexer with brackets");
}
