use super::LineKind;

pub struct ConstraintSolver {
    pub vars: Vec<Variable>,
    equations: Vec<Equation>,
}

impl ConstraintSolver {
    pub fn new() -> ConstraintSolver {
        ConstraintSolver { vars: Vec::new(), equations: Vec::new() }
    }

    pub fn add_var(&mut self) -> VarRef {
        self.vars.push(Variable::new());
        self.vars.len()-1
    }

    pub fn add_var_value(&mut self, value: f32) -> VarRef {
        self.vars.push(Variable::Value(value));
        self.vars.len()-1
    }

    pub fn add_point(&mut self) -> Point {
        self.vars.push(Variable::new());
        self.vars.push(Variable::new());
        let idx = self.vars.len();

        Point { x: idx-2, y: idx-1 }
    }

    pub fn add_polyline(&mut self, pl: Vec<LineKind>) {
        for line in pl {
            match line {
                LineKind::Arc => {
                    
                }
                LineKind::Line => {

                }
            }
        }
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.equations.append(&mut constraint.to_equations());
    }

    pub fn solve(&mut self) -> bool {
        let mut progress = true;

        while progress && !self.is_solved() {
            progress = false;

            for equ in self.equations.iter_mut() {
                if equ.solved {
                    continue;
                }

                let solved = equ.solve(&mut self.vars);
                if solved {
                    progress = true;
                    equ.solved = true;
                    //println!("solved: {equ:?}");
                    //println!("vars: {:?}", self.vars);
                }
            }

            if !progress {
                return false;
            }
        }
        true
    }

    pub fn is_solved (&self) -> bool {
        for eq in &self.equations {
            if !eq.solved {
                return false;
            }
        }
        for var in &self.vars {
            if let Variable::Value(_) = var {

            }
            else {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Equation {
    kind: EquationKind,
    vars: Box<[VarRef]>,
    solved: bool,
}

impl Equation {
    fn solve (&mut self, vars: &mut Vec<Variable>) -> bool {

        let unknown = self.unknown(vars);

        if unknown.is_none() {
            return false;
        }

        let unknown = unknown.unwrap();

        match self.kind {
            EquationKind::Equal => {
                match unknown {
                    0 => vars[self.vars[unknown]] = Variable::Value(vars[self.vars[1]].get_value()),
                    1 => vars[self.vars[unknown]] = Variable::Value(vars[self.vars[0]].get_value()),
                    _ => ()
                }
            }
            EquationKind::Less => {
                match unknown {
                    0 => vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_range((vars[self.vars[1]].get_value(), f32::INFINITY)),
                    1 => vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_range((f32::NEG_INFINITY, vars[self.vars[0]].get_value())),
                    _ => ()
                }
            }
            EquationKind::Greater => {
                match unknown {
                    0 => vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_range((f32::NEG_INFINITY, vars[self.vars[1]].get_value())),
                    1 => vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_range((vars[self.vars[0]].get_value(), f32::INFINITY)),
                    _ => ()
                }
            }
            EquationKind::EqualDistance => {
                let eq1 = |a:f32,b:f32,c:f32,d:f32,e:f32,f:f32| 
                    f32::sqrt(-(a.powi(2)) + (2.0 * a * b) - (b.powi(2)) + (c.powi(2)) - (2.0 * d * c) + (c.powi(2)) + (e.powi(2)) - (2.0 * e * f) + (f.powi(2))); 
                let eq2 = |a:f32,b:f32,c:f32,d:f32,e:f32,f:f32| 
                    f32::sqrt((a.powi(2)) - (2.0 * a * b) + (b.powi(2)) + (c.powi(2)) - (2.0 * d * c) + (c.powi(2)) - (e.powi(2)) + (2.0 * e * f) - (f.powi(2))); 

                match unknown {
                    0 => {
                        let num = eq1(vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[6]].get_value(), vars[self.vars[4]].get_value(), vars[self.vars[7]].get_value(), vars[self.vars[5]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[2]].get_value(), (-num) + vars[self.vars[2]].get_value()])
                    }
                    1 => {
                        let num = eq1(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[6]].get_value(), vars[self.vars[4]].get_value(), vars[self.vars[7]].get_value(), vars[self.vars[5]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[3]].get_value(), (-num) + vars[self.vars[3]].get_value()])
                    }
                    2 => {
                        let num = eq1(vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[6]].get_value(), vars[self.vars[4]].get_value(), vars[self.vars[7]].get_value(), vars[self.vars[5]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[0]].get_value(), (-num) + vars[self.vars[0]].get_value()])
                    }
                    3 => {
                        let num = eq1(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[6]].get_value(), vars[self.vars[4]].get_value(), vars[self.vars[7]].get_value(), vars[self.vars[5]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[1]].get_value(), (-num) + vars[self.vars[1]].get_value()])
                    }
                    4 => {
                        let num = eq2(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[7]].get_value(), vars[self.vars[5]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[6]].get_value(), (-num) + vars[self.vars[6]].get_value()])
                    }
                    5 => {
                        let num = eq2(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[6]].get_value(), vars[self.vars[4]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[7]].get_value(), (-num) + vars[self.vars[7]].get_value()])
                    }
                    6 => {
                        let num = eq2(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[7]].get_value(), vars[self.vars[5]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[4]].get_value(), (-num) + vars[self.vars[4]].get_value()])
                    }
                    7 => {
                        let num = eq2(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[6]].get_value(), vars[self.vars[4]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[5]].get_value(), (-num) + vars[self.vars[5]].get_value()])
                    }
                    _ => ()
                }
            }
            EquationKind::Distance => {
                let eq = |a:f32,b:f32,c:f32| f32::sqrt((-1.0 * a.powi(2)) + (2.0 * a * b) - (b.powi(2)) + (c.powi(2))); 
                match unknown {
                    0 => {
                        let num = eq(vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[4]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[2]].get_value(), (-num) + vars[self.vars[2]].get_value()])
                    }
                    1 => {
                        let num = eq(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[4]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[3]].get_value(), (-num) + vars[self.vars[3]].get_value()])
                    }
                    2 => {
                        let num = eq(vars[self.vars[3]].get_value(), vars[self.vars[1]].get_value(), vars[self.vars[4]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[0]].get_value(), (-num) + vars[self.vars[0]].get_value()])
                    }
                    3 => {
                        let num = eq(vars[self.vars[2]].get_value(), vars[self.vars[0]].get_value(), vars[self.vars[4]].get_value());
                        vars[self.vars[unknown]] = vars[self.vars[unknown]].apply_set(vec![num + vars[self.vars[1]].get_value(), (-num) + vars[self.vars[1]].get_value()])
                    }
                    4 => {
                        let num = f32::sqrt( (vars[self.vars[2]].get_value() - vars[self.vars[0]].get_value()).powi(2) + (vars[self.vars[3]].get_value() - vars[self.vars[1]].get_value()).powi(2));
                        vars[self.vars[unknown]] = Variable::Value(num);
                    }
                    _ => ()
                }
            }
        }
        true
    }

    fn unknown (&self, vars: &mut Vec<Variable>) -> Option<usize> {
        let mut unknown = Some(0);
        let mut count = 0;
        for (i, arg) in self.vars.iter().enumerate() {
            match vars[*arg] {
                Variable::Value(_) => (),
                _ => 
                    if count > 0 {
                        return None;
                    }
                    else {
                        unknown = Some(i);
                        count += 1;
                    }
            }
        }
        unknown
    }
}

#[derive(Debug)]
enum EquationKind {
    Equal,            // v0 - v1 = 0
    Greater,          // v0 - v1 > 0
    Less,             // v0 - v1 < 0
    EqualDistance,    // (v2-v0)^2 + (v3-v1)^2 - (v6-v4)^2 - (v7-v5)^2 = 0
    Distance,         // ((v2-v0)^2 + (v3-v1)^2 - v4^2 = 0
}

#[derive(Debug, Clone)]
pub enum Variable {
    Value (f32),
    Set (Vec<f32>),
    Range (f32,f32),
    Unknown,
    Conflict,
}

impl Variable {
    fn new() -> Variable {
        Variable::Unknown
    }

    fn get_value(&self) -> f32 {
        if let Variable::Value(x) = self {
            *x
        }
        else {
            f32::NAN
        }
    }

    fn apply_range(&self, range: (f32,f32)) -> Variable {
        match self {
            Variable::Value(v) => {
                if *v > range.0 && *v < range.1 {
                    self.clone()
                }
                else {
                    Variable::Conflict
                }
            }
            Variable::Set(s) => {
                let ns: Vec<f32> = s.iter().filter_map(|x| if *x >= range.0 && *x <= range.1 {Some(*x)} else {None}).collect();
                match ns.len() {
                    0 => Variable::Conflict,
                    1 => Variable::Value(ns[0]),
                    _ => Variable::Set(ns),
                }
            }
            Variable::Range(l, u) => {
                if *l == range.1 {
                    Variable::Value(*l)
                } else if *u == range.0 {
                    Variable::Value(*u)
                } else if *l >= range.1 || *u <= range.0 {
                    Variable::Conflict
                } else {
                    Variable::Range(l.min(range.0), u.max(range.1))
                }
            }
            Variable::Unknown => {
                Variable::Range(range.0, range.1)
            }
            _ => Variable::Conflict
        }
    }

    fn apply_set(&self, set: Vec<f32>) -> Variable {
        match self {
            Variable::Value(v) => {
                if set.contains(v) {
                    self.clone()
                }
                else {
                    Variable::Conflict
                }
            }
            Variable::Set(s) => {
                let ns: Vec<f32> = s.iter().filter_map(|x| if s.contains(x) {Some(*x)} else {None}).collect();
                match ns.len() {
                    0 => Variable::Conflict,
                    1 => Variable::Value(ns[0]),
                    _ => Variable::Set(ns),
                }
            }
            Variable::Range(l, u) => {
                let ns: Vec<f32> = set.iter().filter_map(|x| if *x > *l && *x < *u {Some(*x)} else {None}).collect();
                match ns.len() {
                    0 => Variable::Conflict,
                    1 => Variable::Value(ns[0]),
                    _ => Variable::Set(ns),
                }
            }
            Variable::Unknown => {
                Variable::Set(set)
            }
            _ => Variable::Conflict
        }
    }
}


pub enum Constraint<'a> {
    LHorizontal {l: &'a Line<'a>},
    LVertical {l: &'a Line<'a>},
    LNorth {l: &'a Line<'a>},
    LEast {l: &'a Line<'a>},
    LSouth {l: &'a Line<'a>},
    LWest {l: &'a Line<'a>},
    LEqual {l0: &'a Line<'a>, l1: &'a Line<'a>},
    LDimension {l: &'a Line<'a>, d: VarRef},

    PHorizontal {p0: &'a Point, p1: &'a Point},
    PVertical {p0: &'a Point, p1: &'a Point},
    PNorth {p0: &'a Point, p1: &'a Point},
    PEast {p0: &'a Point, p1: &'a Point},
    PSouth {p0: &'a Point, p1: &'a Point},
    PWest {p0: &'a Point, p1: &'a Point},
    PEqual {p0: &'a Point, p1: &'a Point},
    PDimension {p0: &'a Point, x: VarRef, y:VarRef},
}

impl<'a> Constraint<'a> {
    fn to_equations(&self) -> Vec<Equation> {
        match self {
            Constraint::LHorizontal { l } => {vec![
                Equation { kind: EquationKind::Equal, vars:Box::new([l.p0.y, l.p1.y]), solved: false},
            ]}
            Constraint::LVertical { l } => {vec![
                Equation { kind: EquationKind::Equal, vars:Box::new([l.p0.x, l.p1.x ]), solved: false},
            ]}
            Constraint::LNorth { l } => {vec![
                Equation { kind: EquationKind::Greater, vars:Box::new([l.p0.y, l.p1.y ]), solved: false},
            ]}
            Constraint::LEast { l } => {vec![
                Equation { kind: EquationKind::Greater, vars:Box::new([l.p0.x, l.p1.x ]), solved: false},
            ]}
            Constraint::LSouth { l } => {vec![
                Equation { kind: EquationKind::Less, vars:Box::new([l.p0.y, l.p1.y ]), solved: false},
            ]}
            Constraint::LWest { l } => {vec![
                Equation { kind: EquationKind::Less, vars:Box::new([l.p0.x, l.p1.x ]), solved: false},
            ]}
            Constraint::LEqual { l0, l1 } => {vec![
                Equation { kind: EquationKind::EqualDistance, vars:Box::new([l0.p0.x, l0.p0.y, l0.p1.x, l0.p1.y, l1.p0.x, l1.p0.y, l1.p1.x, l1.p1.y ]), solved: false},
            ]}
            Constraint::LDimension { l, d } => {vec![
                Equation { kind: EquationKind::Distance, vars:Box::new([l.p0.x, l.p0.y, l.p1.x, l.p1.y, *d ]), solved: false},
            ]}
            Constraint::PHorizontal { p0, p1 } => {vec![
                Equation { kind: EquationKind::Equal, vars:Box::new([p0.y, p1.y ]), solved: false},
            ]}
            Constraint::PVertical { p0, p1 } => {vec![
                Equation { kind: EquationKind::Equal, vars:Box::new([p0.y, p1.y ]), solved: false},
            ]}
            Constraint::PNorth { p0, p1 } => {vec![
                Equation { kind: EquationKind::Greater, vars:Box::new([p0.y, p1.y ]), solved: false},
            ]}
            Constraint::PEast { p0, p1 } => {vec![
                Equation { kind: EquationKind::Greater, vars:Box::new([p0.x, p1.x ]), solved: false},
            ]}
            Constraint::PSouth { p0, p1 } => {vec![
                Equation { kind: EquationKind::Less, vars:Box::new([p0.y, p1.y ]), solved: false},
            ]}
            Constraint::PWest { p0, p1 } => {vec![
                Equation { kind: EquationKind::Less, vars:Box::new([p0.y, p1.y ]), solved: false},
            ]}
            Constraint::PEqual { p0, p1 } => {vec![
                Equation { kind: EquationKind::Equal, vars:Box::new([p0.x, p1.x ]), solved: false},
                Equation { kind: EquationKind::Equal, vars:Box::new([p0.y, p1.y ]), solved: false},
            ]}
            Constraint::PDimension { p0, x, y } => {vec![
                Equation { kind: EquationKind::Equal, vars:Box::new([p0.x, *x ]), solved: false},
                Equation { kind: EquationKind::Equal, vars:Box::new([p0.y, *y ]), solved: false},
            ]}
        }
    }
}

type VarRef: = usize;

pub struct Point {
    x: VarRef,
    y: VarRef,
}

impl Point {
    pub fn new(x: VarRef, y:VarRef) -> Point {
        Point { x, y }
    }
}

pub struct Line<'a> {
    p0: &'a Point,
    p1: &'a Point,
}

impl<'a> Line<'a> {
    pub fn new(p0: &'a Point, p1: &'a Point) -> Line<'a> {
        Line{p0, p1}
    }
}
