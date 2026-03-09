#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod constraints;
use constraints::*;

mod render;
use render::run;


fn main() {

    run();

    let mut solver = ConstraintSolver::new();

    let ox = solver.add_var_value(0.0);
    let oy = solver.add_var_value(0.0);

    let origin = Point::new(ox, oy);

    let p0 = solver.add_point();
    let p1 = solver.add_point();
    let p2 = solver.add_point();
    let p3 = solver.add_point();

    let v0 = solver.add_var_value(10.0);
    let v1 = solver.add_var_value(5.0);

    let v2 = solver.add_var();

    let l0 = Line::new(&p0, &p1);
    let l1 = Line::new(&p1, &p2);
    let l2 = Line::new(&p2, &p3);
    let l3 = Line::new(&p3, &p0);

    solver.add_constraint(Constraint::LHorizontal { l: &l0 });
    solver.add_constraint(Constraint::LHorizontal { l: &l2 });

    solver.add_constraint(Constraint::LVertical { l: &l1 });
    solver.add_constraint(Constraint::LVertical { l: &l3 });

    solver.add_constraint(Constraint::LEast { l: &l0 });
    solver.add_constraint(Constraint::LNorth { l: &l1 });

    solver.add_constraint(Constraint::PEqual { p0: &p0, p1: &origin });

    solver.add_constraint(Constraint::LDimension { l: &l0, d:v0 });

    solver.add_constraint(Constraint::LDimension { l: &l1, d:v1 });

    solver.add_constraint(Constraint::LDimension { l: &l1, d:v2 });


    let solved = solver.solve();
    
    if solved {
        println!("Solved: {:?}", solver.vars);
    } else {
        println!("Not Solved: {:?}", solver.vars);
    }
    
    
}

enum LineKind {
    Line,
    Arc
}
