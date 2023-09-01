use crate::proto::equation::CalculationResponse;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tonic::Status;

/// AST for the math operations covered in this challege
/// Inspired by the new defunct [math-ast](https://crates.io/crates/math-ast)
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    async fn add(&self, first: i32, second: i32) -> Result<i32, E>;
    async fn subtract(&self, first: i32, second: i32) -> Result<i32, E>;
    async fn multiply(&self, first: i32, second: i32) -> Result<i32, E>;
    async fn divide(&self, first: i32, second: i32) -> Result<i32, E>;

    async fn eval(&self, ast: MathAST) -> Result<MathAST, E> {
        match ast {
            MathAST::Value(_) => Ok(ast),
            MathAST::Add(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(self.add(first, second).await?))
                }
                [first, second] => Ok(MathAST::Add(
                    Box::new(self.eval(first).await?),
                    Box::new(self.eval(second).await?),
                )),
            },
            MathAST::Subtract(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(self.subtract(first, second).await?))
                }
                [first, second] => Ok(MathAST::Subtract(
                    Box::new(self.eval(first).await?),
                    Box::new(self.eval(second).await?),
                )),
            },
            MathAST::Multiply(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(self.multiply(first, second).await?))
                }
                [first, second] => Ok(MathAST::Multiply(
                    Box::new(self.eval(first).await?),
                    Box::new(self.eval(second).await?),
                )),
            },
            MathAST::Divide(f, s) => match [*f, *s] {
                [MathAST::Value(first), MathAST::Value(second)] => {
                    Ok(MathAST::Value(self.divide(first, second).await?))
                }
                [first, second] => Ok(MathAST::Divide(
                    Box::new(self.eval(first).await?),
                    Box::new(self.eval(second).await?),
                )),
            },
        }
    }
}

impl TryFrom<MathAST> for CalculationResponse {
    type Error = Status;

    fn try_from(value: MathAST) -> Result<Self, Self::Error> {
        if let MathAST::Value(v) = value {
            Ok(CalculationResponse { result: v })
        } else {
            Err(Status::invalid_argument("MathAST with No Value"))
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[derive(Default)]
    struct TestASTEvaluator {}

    #[async_trait]
    impl MathASTEvaluator<()> for TestASTEvaluator {
        async fn add(&self, first: i32, second: i32) -> Result<i32, ()> {
            Ok(first + second)
        }
        async fn subtract(&self, first: i32, second: i32) -> Result<i32, ()> {
            Ok(first - second)
        }
        async fn multiply(&self, first: i32, second: i32) -> Result<i32, ()> {
            Ok(first * second)
        }
        async fn divide(&self, first: i32, second: i32) -> Result<i32, ()> {
            Ok(first / second)
        }
    }

    #[actix_rt::test]
    async fn test_ast_eval() {
        let mut ast = test_value();
        let mut evaluator = TestASTEvaluator::default();

        let depth = 10;

        for _ in 0..depth {
            ast = evaluator.eval(ast.clone()).await.unwrap();

            if let MathAST::Value(_) = ast {
                break;
            }
        }

        assert_matches!(
            ast,
            MathAST::Value(v) if v == 1
        );
    }
}
