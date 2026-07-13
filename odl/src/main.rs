use parser::{syntax, runtime_graph};
use petgraph::graph::DiGraph;

fn main() {
    let mut errors = Vec::new();

    let expr = syntax::ASTParser::new()
        .parse( &mut errors,
            r#"
            //struct point {
            //    x: u32,
            //    y: u32,
            //}

            enum dir {
                North,
                East,
                South,
                West,
            }

            mod test (

            ) {
                //let x = [hi, ho, 5.5*5, "4"];
            }     
            "#);

    println!("{expr:#?}, {errors:#?}");


    let mut g = DiGraph::<String, ()>::new();

    let n0 = g.add_node("Code".into());
    let n1 = g.add_node("Code".into());

    g.add_edge(n0, n1, ());

    let c: i32 = 6;

    println!("{g:?}");
}
