use async_trait::async_trait;

/// AST for the math operations covered in this challege
/// Inspired by the new defunct [math-ast](https://crates.io/crates/math-ast)
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
trait MathASTEvaluator<E: Send + Sync> {
    async fn add(first: i32, second: i32) -> Result<i32, E>;
    async fn subtract(first: i32, second: i32) -> Result<i32, E>;
    async fn multiply(first: i32, second: i32) -> Result<i32, E>;
    async fn divide(first: i32, second: i32) -> Result<i32, E>;

    async fn eval(ast: MathAST) -> Result<i32, E> {
        match ast {
            MathAST::Value(v) => Ok(v),
            MathAST::Add(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => Self::add(first, second).await,
                [first, second] => {
                    Self::add(Self::eval(first).await?, Self::eval(second).await?).await
                }
            },
            MathAST::Subtract(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Self::subtract(first, second).await
                }
                [first, second] => {
                    Self::subtract(Self::eval(first).await?, Self::eval(second).await?).await
                }
            },
            MathAST::Multiply(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Self::multiply(first, second).await
                }
                [first, second] => {
                    Self::multiply(Self::eval(first).await?, Self::eval(second).await?).await
                }
            },
            MathAST::Divide(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Self::divide(first, second).await
                }
                [first, second] => {
                    Self::divide(Self::eval(first).await?, Self::eval(second).await?).await
                }
            },
        }
    }
}
