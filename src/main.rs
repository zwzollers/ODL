
trait VariableTrait {
    
}

#[derive(Clone)]
enum Variable<T> {
    Value (T),
    Range (Vec<(T,T)>),
    Unknown,
    Conflict,
}

impl<T> VariableTrait for Variable<T> {
    
}

enum Constraint<'a, T: VariableTrait> {
    Horizontal (&'a Point<'a, T>, &'a Point<'a, T>),
    Vertical (&'a Point<'a, T>, &'a Point<'a, T>),
    North (&'a Point<'a, T>, &'a Point<'a, T>),
    East (&'a Point<'a, T>, &'a Point<'a, T>),
    South (&'a Point<'a, T>, &'a Point<'a, T>),
    West (&'a Point<'a, T>, &'a Point<'a, T>),
    
    EqualPoint (&'a Point<'a, T>, &'a Point<'a, T>),
    EqualLine (&'a Line<'a, T>, &'a Line<'a, T>),
    
    DimensionLength (&'a Line<'a, T>, T),
    DimensionPoint (&'a Point<'a, T>, (T,T)),
    DimensionAngle (&'a Line<'a, T>, &'a Line<'a, T>, T),
}

impl<'a, T: VariableTrait> Constraint<'a, T> {

}

#[derive(Clone)]
struct Point<'a, T> {
    x: &'a T,
    y: &'a T,
}
#[derive(Clone)]
struct Line<'a, T> {
    p0: &'a Point<'a, T>,
    p1: &'a Point<'a, T>,
}


fn main() {
    let mut x0 = Variable::<f32>::Unknown;
    let mut y0 = Variable::<f32>::Unknown;
    let p0 = Point::<Variable::<f32>> { x:&x0, y:&y0 };
    
    let mut x1 = Variable::<f32>::Unknown;
    let mut y1 = Variable::<f32>::Unknown;
    let p1 = Point::<Variable::<f32>> { x:&x1, y:&y1 };
    
    let mut x2 = Variable::<f32>::Unknown;
    let mut y2 = Variable::<f32>::Unknown;
    let p2 = Point::<Variable::<f32>> { x:&x2, y:&y2 };
    
    let mut x3 = Variable::<f32>::Unknown;
    let mut y3 = Variable::<f32>::Unknown;
    let p3 = Point::<Variable::<f32>> { x:&x3, y:&y3 };
    
    let l0 = Line::<Variable<f32>> { p0:&p0, p1:&p1};
    let l1 = Line::<Variable<f32>> { p0:&p1, p1:&p2};
    let l2 = Line::<Variable<f32>> { p0:&p2, p1:&p3};
    let l3 = Line::<Variable<f32>> { p0:&p3, p1:&p0};
    
    let mut unknowns = vec![
        x0, y0  
    ];
    let constraints = vec![
        Constraint::<Variable<f32>>::Horizontal(&p0, &p1),
    ];
    
    
    
    
}
