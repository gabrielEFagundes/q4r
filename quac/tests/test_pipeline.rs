use quac_core::{backend::Backend, codegen::Codegen, header::SignatureMounter, lexer::Lexer};
use quaoar_c::emitter::CCompiler;

const INPUT: &[u8] = br#"
    /*
    Smaller version of the _pipeline.qua testing file, 
    without comments and stdout.

    original file can be found at `~/examples/stress_tests/_pipeline.qua`
    */

    extern {
        f printf(*char s, ...)
        f scanf(*char fmt, ...)
    }

    extern f exit(int code)

    int passed = 0
    int failed = 0

    f main() int{
        testArithmetic()
        testComparisons()
        testPointers()
        testNestedCalls()
        testWhileLoop()
        testForLoop()
        testChars()
        testBools()
        testCompoundAssignment()
        testNestedIf()
        testForwardRef()

        if failed > 0{
            return 1
        }
        return 0
    }

    f check(*char label, int got, int expected){
        if got == expected{
            printf("[PASS] %s: got %d\n", label, got)
            passed += 1
        } else {
            printf("[FAIL] %s: expected %d, got %d\n", label, expected, got)
            failed += 1
        }
    }

    f testArithmetic(){
        int a = 10
        int b = 3
        check("add", a + b, 13)
        check("sub", a - b, 7)
        check("mul", a * b, 30)
        check("div", a / b, 3)
        check("nested expr", (a + b) * (a - b), 91)
        check("unary minus", 0 - a, -10)
    }

    f isGreater(int x, int y) int{
        if x > y{
            return 1
        }
        return 0
    }

    f isEqual(int x, int y) int{
        if x == y{
            return 1
        }
        return 0
    }

    f testComparisons(){
        check("10 > 3", isGreater(10, 3), 1)
        check("3 > 10", isGreater(3, 10), 0)
        check("5 == 5", isEqual(5, 5), 1)
        check("5 == 6", isEqual(5, 6), 0)
    }

    f testPointers(){
        int val = 42
        *int p = &val
        check("deref read", *p, 42)

        *p = 99
        check("deref write", val, 99)

        int other = 7
        p = &other
        check("pointer reassign", *p, 7)
    }

    f doubled(int x) int{
        return x * 2
    }

    f addOne(int x) int{
        return x + 1
    }

    f testNestedCalls(){
        check("double(5)", doubled(5), 10)
        check("addOne(double(3))", addOne(doubled(3)), 7)
        check("double(addOne(4))", doubled(addOne(4)), 10)
    }

    f testWhileLoop(){
        int sum = 0
        int i = 1
        for i <= 10{
            sum += i
            i += 1
        }
        check("while sum 1..10", sum, 55)
    }

    f testForLoop(){
        int total = 0
        for i = 0; i < 5; +1{
            total += i
        }
        check("for sum 0..4", total, 10)
    }

    f testChars(){
        char a = 'A'
        char b = 'B'
        check("char A", a, 65)
        check("char B", b, 66)
        check("char cmp", isEqual(a, a), 1)
    }

    f boolToInt(bool v) int{
        if v{
            return 1
        }
        return 0
    }

    f testBools(){
        check("true", boolToInt(true), 1)
        check("false", boolToInt(false), 0)
    }

    f testCompoundAssignment(){
        int x = 10
        x += 5
        check("+= 5", x, 15)
        x -= 3
        check("-= 3", x, 12)
        x *= 2
        check("*= 2", x, 24)
        x /= 6
        check("/= 6", x, 4)
    }

    f classify(int x) int{
        if x > 100{
            return 3
        } else if x > 50{
            return 2
        } else if x > 0{
            return 1
        } else {
            return 0
        }
    }

    f testNestedIf(){
        check("classify(200)", classify(200), 3)
        check("classify(75)", classify(75), 2)
        check("classify(25)", classify(25), 1)
        check("classify(-5)", classify(-5), 0)
    }

    f testForwardRef(){
        check("forward ref", helperForward(), 42)
    }

    f helperForward() int{
        return 42
    }
"#;

#[test]
pub fn stress_test_pipeline(){
    let (tokens, errors) = Lexer::new(INPUT.to_vec()).lexerize();
    assert!(errors.is_empty());

    let mut binding = SignatureMounter::new(INPUT, tokens.as_slice());
    let signatures = binding.mount();

    let mut backend = CCompiler::new();
    let mut c_compiler: Backend<'_> = Backend::new(INPUT, &tokens, signatures, false);

    let (bytes, errors) = backend
        .generate_headers(&c_compiler.signatures)
        .generate(&mut c_compiler);

    assert!(errors.is_empty());
    assert!(bytes[bytes.len()-1] == b'}');

    assert!(quaoar_c::exec_test(&bytes))
}