use carbide_errors::{reporter::ErrorReporter};
use carbide_lexer::lexer::CarbideLexer;

fn main() {
    let src = r#"
    let x = @invalid;
    let hex = 0x;
    let λ = 42;
    let name = "John
    let age = 25;
    "#;
    let mut lexer = CarbideLexer::from_src(src);
    let result = lexer.lex();

    let mut reporter = ErrorReporter::new();
    reporter.add_source("example.cb", src);
    reporter.print_errors("example.cb", &result.errors);
}
