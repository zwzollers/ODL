# ODL Language Specification

This repositry contains the language spec for ODL and a parser to convert ODL to stl

## Encoding

ODL is a text based language that uses UTF-8 encoding

## file extension

use the .odl file extension

## usage

while ODL can be written in any text based editor the nature of designing 3D objects sometimes requires feedback to the user to ensure the shape or size of something matches the users idea of what it should look like. Becasue of this ODL is accompanied by a interactive IDE that helps you write the code and keep track of 
objects in space.

# Whats makes ODL better

## Scripting

becasue of the text based nature of ODL its only natural to include a builting scripting language that allows you to do more complex and powerful operations in your 3D CAD software. The scripting language has similuar syntax to rust but is much more lightweight and interpreted.

## Version Control

text based version control solutions have been around for decades so there is no need to reinvent the wheel with ODL. keep using your favorite version control software and you will get all the benifits of reversable and understandable changes.

## Low Computing Requirements

ODL is fully offline and requires very little computing power. You decide if you want to reload after every change or every 5 minuets. This allows you to talor your expirence to the computing power you have available.


# Terminology

## The Timeline

ODL is like most modern CAD softwares it has a timeline that contains a list of operations to complete in order. After all these operations have completed you should have your finished object.

## Primitive Types

the most basic types of ODL are lines, arcs and points. You might think that points should be the only primitive type but that loses information when it comes to curves. a collections of lines and arcs forms a polyline. if the polyline connects to itself it is considered a loop. Loops are the most common usecase for 3d design as they are the initial point for all 2d->3d operations like extrude and loft. 

## Constraints

Constriants are a way of solifiying measurments of a design without having to calculate every dimension beteen points. The following is a list of supported contraints
- Horizontal (h)
- Vertical (v)
- Parallel (p)
- Perpendicular (!p)
- Coincident (i)
- Tangent (t)
- Normal (n)
- Midpoint (m)
- Equal (e)
- Dimension (d)
- Above (a)
- Below (b)


# Syntax
```
Capsule (diameter: dist, length: dist) {}
  let pill = loop({a:line, b:arc, c:line, d:arc}, origin.top);
  h({pill[a]}, {pill[c]});
  t(
    {pill[a], pill[b]}, 
    {pill[a], pill[d]}, 
    {pill[c], pill[a]}, 
    {pill[c], pill[d]}
  );
  let x = dist({pill[b][0], pill[b][1]});
  eq({x,pill[a]});
  
  let cline = pl({a:line});
  i({cline[0],pill[b][0.5]}, {cline[1],pill[d][0.5]})
  
  let sections = pill.split(cline);
  
  let dpill = revolve({sections[0]}, cline);
  
  dim({pill[b][0.5], pill[d][0.5]}, length);
  dim({x}, diameter);
}
```

```
Gear (face: plane, center_point: point, pitch_diameter: dist, teeth: number, modulus: number) {

  let m = (pitch_diameter/teeth)/PI;\
  let tooth_height = m * 2;
  let half_tooth = pl({base: arc, inv: equation, top:line}, face);
  
  let side_line = pl({l:line}, face);
  i({side_line[l][0], center_point});
  i({side_line[l][1], half_tooth[base][0]});
  n({side_line[l], half_tooth[base]});
  
  let center_line = pl({cl:line}, face);
  v({center_line[cl]});
  i({center_line[cl][0], center_point});
  i({center_line[cl][1], half_tooth[top][1]);
  dim({center_line}, (pitch_diameter/2)+tooth_height);
  
  let r = pitch_diameter;
  parametric({inv}, r*cos(t+a)+t*sin(t+a), r*sin(t+a)-t*cos(t+a));
  
  h({half_tooth[top]});
  
  t({half_tooth[base], half_tooth[inv]});
  
}
```

```
let sq = loop([
  a: line,
  b: 1ine,
  c: line,
  d: line
]);
eq([sq[a][0],origin]);
eq([sq[a],sq[b]]);
h([sq[a]]);
h([sq[c]]);
v[sq[b]];
v[sq[d]];
east([sq[a]]);
north[sq[b]];
dim([sq[a]], 5.0);


4 points P0, P1, P2, P3
P0.x = 0
P0.y = 0
(P1.x-P0.x)^2 + (P1.y-P0.y)^2 - (P2.x-P1.x)^2 - (P2.y-P1.y)^2 = 0
P0.y - P1.y = 0
P2.y - P3.y = 0
P1.x - P2.x = 0
P0.x - P3.x = 0
P1.x - P0.x > 0
P2.y - P1.y > 0
sqrt((P1.x-P0.x)^2 + (P1.y-P0.y)^2) - 5 = 0

P0 = (0,0)
P1 = (,0)
P2 = (,)
P3 = (0,)
```


## Keywords

### ```Let```

let is used to name things

```
let name = thing;

let x = 5;
let loop = loop([
  a: line,
  b: arc,
  c: line,
  d: arc
]);
```

### ```tuples```

{} denotes a tuple

### implicit naming

```
let x: num; // explicit naming

[x: num]; // implicit naming
```


## primitive types

```
number -> num
distance -> dist
angle -> angle
area -> area
boolean -> bool

named type -> name: type
```