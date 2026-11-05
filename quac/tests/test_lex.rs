use quac_core::{lexer, tokens::VoidstarTokenTypes};

const INPUT: &[u8] = b"
    f main() int{
        return 0
    }
";

const PROBLEMATIC_INPUT: &[u8] = b"
    f main() int{
        return `0
    }`
";

#[test]
pub fn test_basic_tokenizer(){
    let (output, _) = lexer::Lexer::new(INPUT.to_vec()).lexerize();

    assert!(output.len() == 11);

    assert_eq!(output[0].token_type(), VoidstarTokenTypes::Function);
    assert_eq!(output[output.len()-1].token_type(), VoidstarTokenTypes::SemiColon);
}

#[test]
pub fn test_errors(){
    let (output, errors) = lexer::Lexer::new(PROBLEMATIC_INPUT.to_vec()).lexerize();

    assert!(output.len() == 9);
    assert!(errors.len() == 2);

    assert!(errors[0].msg.contains("impossible state"));
}