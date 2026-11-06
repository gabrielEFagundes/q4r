use quac_core::{header::SignatureMounter, lexer::Lexer, signature::Signature};

const INPUT: &[u8] = b"
    float golden_ratio = 1.61803398875

    f sum(int v1, int v2) int{
        return v1+v2
    }

    f sub(float v1, float v2) float{
        return v1-v2
    }

    f mult(float v1, float v2) float{
        return v1*v2
    }

    f div(int v1, int v2) int{
        return v1/v2
    }

    f main() int{
        int sum = sum(10, 20)

        return sum
    }
";

const EXTERNS: &[u8] = b"
    extern f exit(int code)

    extern{
        f printf(char* v, ...)
        f scanf(char* sc, ...)
    }

    f main() int{
        return 0
    }
";

#[test]
pub fn test_table_mounter(){
    let (tokens, _) = Lexer::new(INPUT.to_vec()).lexerize();

    let mut binding = SignatureMounter::new(INPUT, &tokens);
    let signatures = binding.mount();

    assert!(signatures.get_key_value("golden_ratio").is_some());
    assert!(signatures.get_key_value("sum").is_some());

    // make sure it isn't accidentally taking the 'f' keyword
    assert!(signatures.get_key_value("f").is_none());
}

#[test]
pub fn test_extern_headers(){
    let (tokens, _) = Lexer::new(EXTERNS.to_vec()).lexerize();

    let mut binding = SignatureMounter::new(EXTERNS, &tokens);
    let signatures = binding.mount();

    assert!(signatures.get_key_value("exit").is_some_and(|s|{ is_signature_extern(s.1) }));
    assert!(signatures.get_key_value("printf").is_some());
    assert!(signatures.get_key_value("scanf").is_some());

    assert!(signatures.get_key_value("main").is_some_and(|s|{ !is_signature_extern(s.1) }))
}

fn is_signature_extern(signature: &Signature) -> bool{
    if let Signature::Function { returns: _, params: _, is_extern } = signature
        && *is_extern
    {
        return true;
    }
    false
}