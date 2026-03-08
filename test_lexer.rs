mod main_awsm {
    include!("src/main.rs");
}

fn main() {
    let mut lexer = main_awsm::Lexer::new(r#"[:generate-interface ["id: string" "isActive: boolean"] ]"#);
    println!("{:#?}", lexer.tokenize());
}
