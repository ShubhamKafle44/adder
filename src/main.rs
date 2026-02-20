use std::env;
use std::fs::File;
use std::io::prelude::*;

use sexp::*;
use sexp::Atom::*;

// =======================
// Abstract Syntax Tree
// =======================

#[derive(Debug)]
enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>),
}

// =======================
// Parser
// =======================

fn parse_expr(s: &Sexp) -> Expr {
    match s {
        // Parse numbers
        Sexp::Atom(I(n)) => {
            Expr::Num(i32::try_from(*n).expect("Invalid 32-bit integer"))
        }

        // Parse lists like (add1 e)
        Sexp::List(vec) => {
            match &vec[..] {
                [Sexp::Atom(S(op)), e] if op == "add1" => {
                    Expr::Add1(Box::new(parse_expr(e)))
                }
                [Sexp::Atom(S(op)), e] if op == "sub1" => {
                    Expr::Sub1(Box::new(parse_expr(e)))
                }
                [Sexp::Atom(S(op)), e] if op == "negate" => {
                    Expr::Negate(Box::new(parse_expr(e)))
                }
                _ => panic!("Invalid expression"),
            }
        }

        _ => panic!("Invalid expression"),
    }
}

// =======================
// Code Generator
// =======================

fn compile_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => {
            format!("mov rax, {}", *n)
        }

        Expr::Add1(subexpr) => {
            let compiled = compile_expr(subexpr);
            format!("{compiled}\nadd rax, 1")
        }

        Expr::Sub1(subexpr) => {
            let compiled = compile_expr(subexpr);
            format!("{compiled}\nsub rax, 1")
        }

        Expr::Negate(subexpr) => {
            let compiled = compile_expr(subexpr);
            format!("{compiled}\nneg rax")
        }
    }
}

// =======================
// Main
// =======================

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: cargo run <input.snek> <output.s>");
    }

    let in_name = &args[1];
    let out_name = &args[2];

    // Read input file
    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    // Parse program
    let parsed = parse(&in_contents).expect("Invalid S-expression");
    let expr = parse_expr(&parsed);

    // Compile to assembly
    let result = compile_expr(&expr);

    // Wrap with assembly boilerplate
    let asm_program = format!(
"section .text
global our_code_starts_here
our_code_starts_here:
  {}
  ret
", result);

    // Write assembly file
    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}