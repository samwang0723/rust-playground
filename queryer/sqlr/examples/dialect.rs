use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    tracing_subscriber::fmt::init();

    let sql = "SELECT count(a) \
                FROM data_source";

    let ast = Parser::parse_sql(&GenericDialect, sql);
    println!("{:#?}", ast);
}
