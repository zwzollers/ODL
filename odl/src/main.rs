use parser::{syntax, runtime::{Runtime, Instr, HeapKind}};
use petgraph::csr::Csr;
use std::time::Instant;

fn main() {
    let mut errors = Vec::new();

    let now = Instant::now();

    let expr = syntax::ASTParser::new()
        .parse( &mut errors,
            r#"
            mod top (
                a: in bool,
                x: out num,
                y: out num,
            ) {
                let i = 6;
                if a {
                    foo(a:6, x:k);
                    let x = (j * i) + j * k;
                    let j = 10;
                } -> (x:x)
                else {
                    let x = 10;
                } -> (x:x)

                y = i;
            }

            mod foo (
                a: in num,
                x: out num,
            ) {
                x = a + 4;
            }
            "#);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    //println!("{expr:#?}, {errors:#?}");


    let mut g = Csr::<Instr, ()>::new();

    // FOO
    let foo_x = g.add_node(Instr::LOA { addr: 1 });
    let add_0 = g.add_node(Instr::ADD);
    g.add_edge(foo_x, add_0, ());
    let foo_a = g.add_node(Instr::LOA { addr: 0 });
    g.add_edge(add_0, foo_a, ());
    let psh_4 = g.add_node(Instr::PSH { data: 4 });
    g.add_edge(add_0, psh_4, ());

    // TOP
    let top_x = g.add_node(Instr::LOA { addr: 3 });
    let if_c = g.add_node(Instr::SCP { addr: 1, alloc: 1 });
    g.add_edge(top_x, if_c, ());
    

    let speek_0 = g.add_node(Instr::SPK { n: 1 });
    let top_a = g.add_node(Instr::LOA { addr: 0 });
    g.add_edge(speek_0, top_a, ());

    let if_f = g.add_node(Instr::SCP { addr: 0, alloc: 1 });
    let if_x = g.add_node(Instr::LOA { addr: 0 });
    g.add_edge(if_f, if_x, ());
    let psh_10 = g.add_node(Instr::PSH { data: 10 });
    g.add_edge(if_x, psh_10, ());

    let if_t = g.add_node(Instr::SCP { addr: 0, alloc: 4 });
    let if_x = g.add_node(Instr::LOA { addr: 0 });
    g.add_edge(if_t, if_x, ());
    let add_0 = g.add_node(Instr::ADD);
    g.add_edge(if_x, add_0, ());

    let mul_0 = g.add_node(Instr::MUL);
    g.add_edge(add_0, mul_0, ());

    let speek = g.add_node(Instr::SPK { n: 2 });
    g.add_edge(mul_0, speek, ());
    let top_i = g.add_node(Instr::LOA { addr: 2 });
    g.add_edge(speek, top_i, ());
    let psh_6 = g.add_node(Instr::PSH { data: 6 });
    g.add_edge(top_i, psh_6, ());

    let if_j = g.add_node(Instr::LOA { addr: 2 });
    g.add_edge(mul_0, if_j, ());
    let psh_10 = g.add_node(Instr::PSH { data: 10 });
    g.add_edge(if_j, psh_10, ());

    let mul_1 = g.add_node(Instr::MUL);
    g.add_edge(add_0, mul_1, ());
    g.add_edge(mul_1, if_j, ());

    let if_k = g.add_node(Instr::LOA { addr: 3 });
    g.add_edge(mul_1, if_k, ());
    let foo = g.add_node(Instr::SCP { addr: 1, alloc: 2 });
    g.add_edge(if_k, foo, ());
    let cal = g.add_node(Instr::CALL { node: foo_x });
    g.add_edge(foo, cal, ());
    let pop = g.add_node(Instr::POP);
    g.add_edge(cal, pop, ());
    let foo_a = g.add_node(Instr::LOA { addr: 0 });
    g.add_edge(pop, foo_a, ());
    let psh_6 = g.add_node(Instr::PSH { data: 6 });
    g.add_edge(foo_a, psh_6, ());

    let _if = g.add_node(Instr::IF { cond: speek_0, t: if_t, f: if_f });
    g.add_edge(if_c, _if, ());
    let top_y = g.add_node(Instr::LOA { addr: 4 });
    g.add_edge(top_y, top_i, ());

    let mut rt = Runtime::new(g);

    rt.alloc(5);
    rt.mem_wr(0, HeapKind::Num(1));
    rt.entry_point = top_x;


    let now = Instant::now();

    rt.run();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    

    println!("{:?}", rt.mem_rd(3));

    println!("{:?}", rt.heap);

}
