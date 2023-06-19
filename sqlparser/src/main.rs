use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    let dialect = GenericDialect {}; // or AnsiDialect
    let sql = "SELECT a, b, 123, myfunc(b) \
         FROM table_1 \
           WHERE a > b AND b < 100 \
           ORDER BY a DESC, b";
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    println!("AST: {:?}", ast);
}
