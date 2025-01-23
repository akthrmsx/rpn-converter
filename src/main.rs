use colored::Colorize;
use rpn_converter::convert;
use rustyline::DefaultEditor;

fn main() {
    let mut editor = DefaultEditor::new().unwrap();

    loop {
        match editor.readline(">> ") {
            Ok(line) => match convert(line) {
                Ok(s) => println!("{}", s.green()),
                Err(e) => eprintln!("{}", e.red()),
            },
            Err(e) => {
                eprintln!("{}", format!("{}", e).to_lowercase());
                break;
            }
        }
    }
}
