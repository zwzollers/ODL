use lalrpop_util::lalrpop_mod;
pub mod ast;
pub mod runtime_graph;

lalrpop_mod!(pub syntax); // synthesized by LALRPOP