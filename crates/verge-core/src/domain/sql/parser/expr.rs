//! File: `expr.rs`
//!
//! Deskripsi: Parser ekspresi WHERE — AND, OR, dan perbandingan.
//! Layer: domain/sql/parser
//! Tanggung jawab: `parse_disjunction`, `parse_conjunction`, `parse_comparison`.
//!
//! Author: Miruameli
//! Created: 2026-10-08
//! Modified: 2026-10-08
//! Version: 0.1.0
//! License: Apache-2.0
//!
//! Dependencies:
//!   - `super::mod.rs` (Parser), `ast.rs`, `token.rs`
//!
//! Related issues:
//!   - #30 (Milestone 5)
//!
//! Related ADR:
//!   - ADR-0012 (SQL parser semantics and limits)

use super::super::ast::{Expr, Op, SqlError, SqlValue};
use super::super::token::{Keyword, Operator, TokenKind};
use super::Parser;

/// Konversi `Operator` lexer ke `Op` AST.
fn op_to_ast(op: Operator) -> Op {
    match op {
        Operator::Equal => Op::Equal,
        Operator::NotEqual => Op::NotEqual,
        Operator::LessThan => Op::LessThan,
        Operator::LessEqual => Op::LessEqual,
        Operator::GreaterThan => Op::GreaterThan,
        Operator::GreaterEqual => Op::GreaterEqual,
    }
}

impl Parser<'_> {
    /// Mem-parse disjungsi: `conjunction (OR conjunction)*`.
    pub(super) fn parse_disjunction(&mut self, depth: i32) -> Result<Expr, SqlError> {
        let mut expr = self.parse_conjunction(depth)?;
        while self.peek().kind == TokenKind::Keyword(Keyword::Or) {
            self.advance();
            let right = self.parse_conjunction(depth)?;
            expr = Expr::BinaryOp {
                op: Op::Or,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Mem-parse konjungsi: `comparison (AND comparison)*`.
    fn parse_conjunction(&mut self, depth: i32) -> Result<Expr, SqlError> {
        let mut expr = self.parse_comparison(depth)?;
        while self.peek().kind == TokenKind::Keyword(Keyword::And) {
            self.advance();
            let right = self.parse_comparison(depth)?;
            expr = Expr::BinaryOp {
                op: Op::And,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    /// Mem-parse perbandingan: `value_expr comp_op value_expr`.
    fn parse_comparison(&mut self, _depth: i32) -> Result<Expr, SqlError> {
        let left = self.parse_value_expr()?;
        let tok = self.peek();
        if let TokenKind::Operator(op) = &tok.kind {
            let ast_op = op_to_ast(*op);
            self.advance();
            let right = self.parse_value_expr()?;
            Ok(Expr::BinaryOp {
                op: ast_op,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Err(SqlError::at(
                tok.offset,
                format!("operator perbandingan diharapkan; diperoleh `{}`", tok.text),
            ))
        }
    }

    /// Mem-parse primary: kolom, string, angka, atau parameter.
    fn parse_value_expr(&mut self) -> Result<Expr, SqlError> {
        let tok = self.peek();
        match &tok.kind {
            TokenKind::Identifier(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::Column(s))
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expr::Value(SqlValue::Text(s)))
            }
            TokenKind::Number(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::Value(SqlValue::Number(n)))
            }
            TokenKind::Parameter(p) => {
                let p = p.clone();
                self.advance();
                Ok(Expr::Value(SqlValue::Text(p)))
            }
            _ => Err(SqlError::at(
                tok.offset,
                "diperlukan kolom, string, atau angka",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::domain::sql::ast::{Expr, Op, SqlValue};

    fn expr_of(sql: &str) -> Expr {
        let stmt = parse_sql(sql).unwrap();
        stmt.where_clause.expect("WHERE harus ada")
    }

    #[test]
    fn or_or_and_precedence() {
        let e = expr_of("SELECT * FROM t WHERE a = 1 OR b = 2 AND c = 3");
        match e {
            Expr::BinaryOp { op, .. } => assert_eq!(op, Op::Or),
            _ => panic!("harus OR di atas"),
        }
    }

    #[test]
    fn not_equal_operator() {
        let e = expr_of("SELECT * FROM t WHERE name != 'admin'");
        match e {
            Expr::BinaryOp { op, left, right } => {
                assert_eq!(op, Op::NotEqual);
                match (&*left, &*right) {
                    (Expr::Column(n), Expr::Value(SqlValue::Text(s))) => {
                        assert_eq!(n, "name");
                        assert_eq!(s, "admin");
                    }
                    _ => panic!("bentuk operand salah"),
                }
            }
            _ => panic!("harus BinaryOp"),
        }
    }

    #[test]
    fn less_than_operator() {
        let e = expr_of("SELECT * FROM t WHERE age < 18");
        match e {
            Expr::BinaryOp { op, .. } => assert_eq!(op, Op::LessThan),
            _ => panic!("harus BinaryOp"),
        }
    }
}
