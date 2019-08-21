#[derive(Clone)]
#[allow(dead_code)]
pub enum Answer {
    None,
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Text(String),
} 

impl std::fmt::Display for Answer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str({
                    match self{
                        Answer::None => "\u{001b}[0m\u{001b}[33;7m[???]\u{001b}[0m".to_string(),
                        Answer::Unsigned(x) => format!("\u{001b}[0m\u{001b}[32;7m[{}]\u{001b}[0m", x.to_string()),
                        Answer::Signed(x) => format!("\u{001b}[0m\u{001b}[32;7m[{}]\u{001b}[0m", x.to_string()),
                        Answer::Float(x) => format!("\u{001b}[0m\u{001b}[32;7m[{}]\u{001b}[0m", x.to_string()),
                        Answer::Text(x) => format!("\u{001b}[0m\u{001b}[32;7m[{}]\u{001b}[0m", x.clone()),
                    }
                }.as_str())
    }
}
