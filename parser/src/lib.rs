use lalrpop_util::lalrpop_mod;
pub mod ast;
pub mod runtime;

lalrpop_mod!(pub syntax); // synthesized by LALRPOP