use carbide_errors::reporter::ErrorReporter;
use carbide_lexer::lexer::CarbideLexer;
use carbide_parser::parser::CarbideParser;

fn main() {
    let src = r#"let x ="#;
    let mut lexer = CarbideLexer::from_src(src);
    let result = lexer.lex();

    let mut reporter = ErrorReporter::new();
    reporter.add_source("example.cb", src);
    reporter
        .print_errors("example.cb", &result.errors)
        .expect("Expected error printing to succeed");

    if result.has_errors() {
        return;
    }

    let mut parser = CarbideParser::new(result.tokens);
    let result = parser.parse();

    let mut reporter = ErrorReporter::new();
    reporter.add_source("example.cb", src);
    reporter
        .print_errors("example.cb", &result.errors)
        .expect("Expected error printing to succeed");

    println!("{:?}", result.ast);
}
