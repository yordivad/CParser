use lang::math::{whitespaces, literal, identifier, Formula};
use lang::core::{Parser, bind};


#[test]
fn whitespace_test() {

    let input = "   abc";
   if let Ok((next, result)) = whitespaces().parse(input) {
       println!("{}", next);
   }


}

#[test]
fn literal_test() {


    let let_parser = literal("let");

    let text = "let x = 2;";

    if let Ok( (next, result)) = let_parser.parse(text) {

        println!("{}", result);
        println!("{}", next)
    }

}

#[test]
fn valid_identifier_test() {
    let text = "name1 = 2";

    if let Ok((last, (result,spaces))) = bind(identifier(), whitespaces() ).parse(text) {
        println!("{}", result)
    }
}


#[test]
fn invalid_identifier_test() {
    let text = "2name = 2";

    if let Err(error) = identifier().parse(text) {
        println!("{}", error)
    }
}