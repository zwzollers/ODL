use parser::{syntax, runtime::Runtime};
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

}
