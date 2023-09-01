use async_trait::async_trait;

/// AST for the math operations covered in this challege
/// Inspired by the new defunct [math-ast](https://crates.io/crates/math-ast)
#[derive(Debug, Clone)]
pub enum MathAST {
    Value(i32),
    Add(Box<MathAST>, Box<MathAST>),
    Subtract(Box<MathAST>, Box<MathAST>),
    Multiply(Box<MathAST>, Box<MathAST>),
    Divide(Box<MathAST>, Box<MathAST>),
}

/// AST Placeholder for ( ( (3 + 3)*2) /4) – 2 = X until we write a parser
pub fn test_value() -> MathAST {
    MathAST::Subtract(
        Box::new(MathAST::Divide(
            Box::new(MathAST::Multiply(
                Box::new(MathAST::Add(
                    Box::new(MathAST::Value(3)),
                    Box::new(MathAST::Value(3)),
                )),
                Box::new(MathAST::Value(2)),
            )),
            Box::new(MathAST::Value(4)),
        )),
        Box::new(MathAST::Value(2)),
    )
}

/// Implement an evaluator depding on the role of each micro service
/// For example your Adder service would evaluate whole values for first and second args
/// when adding but pass nested evaluations onto other services
#[async_trait]
pub trait MathASTEvaluator<E: Send + Sync> {
    async fn add(first: i32, second: i32) -> Result<i32, E>;
    async fn subtract(first: i32, second: i32) -> Result<i32, E>;
    async fn multiply(first: i32, second: i32) -> Result<i32, E>;
    async fn divide(first: i32, second: i32) -> Result<i32, E>;

    async fn eval(ast: MathAST) -> Result<MathAST, E> {
        match ast {
            MathAST::Value(_) => Ok(ast),
            MathAST::Add(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(Self::add(first, second).await?))
                }
                [first, second] => Ok(MathAST::Add(
                    Box::new(Self::eval(first).await?),
                    Box::new(Self::eval(second).await?),
                )),
            },
            MathAST::Subtract(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(Self::subtract(first, second).await?))
                }
                [first, second] => Ok(MathAST::Subtract(
                    Box::new(Self::eval(first).await?),
                    Box::new(Self::eval(second).await?),
                )),
            },
            MathAST::Multiply(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(Self::multiply(first, second).await?))
                }
                [first, second] => Ok(MathAST::Multiply(
                    Box::new(Self::eval(first).await?),
                    Box::new(Self::eval(second).await?),
                )),
            },
            MathAST::Divide(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(Self::divide(first, second).await?))
                }
                [first, second] => Ok(MathAST::Divide(
                    Box::new(Self::eval(first).await?),
                    Box::new(Self::eval(second).await?),
                )),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestASTEvaluator {}

    #[async_trait]
    impl MathASTEvaluator<()> for TestASTEvaluator {
        async fn add(first: i32, second: i32) -> Result<i32, ()> {
            Ok(first + second)
        }
        async fn subtract(first: i32, second: i32) -> Result<i32, ()> {
            Ok(first - second)
        }
        async fn multiply(first: i32, second: i32) -> Result<i32, ()> {
            Ok(first * second)
        }
        async fn divide(first: i32, second: i32) -> Result<i32, ()> {
            Ok(first / second)
        }
    }

    #[actix_rt::test]
    async fn test_ast_eval() {
        let mut ast = test_value();

        let mut depth = 10;

        for i in 0..depth {
            println!("Iteration {:?}", i);
            depth = i;

            ast = TestASTEvaluator::eval(ast.clone()).await.unwrap();
            if let MathAST::Value(_) = ast {
                break;
            }
        }

        println!("Result: {:#?} at depth {:?}", ast, depth);
    }
}
