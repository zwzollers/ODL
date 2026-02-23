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

// syntax for a simple 50mm cube
let box = {L,L,L,L}.loop(origin.top);
{box.0}.h();
{box.2}.h();
{box.1}.v();
{box.2}.v();
{box.*}.e();
{box.0,box.1}.d(90_deg);
{box.0,^origin}.i();

let cube = {box}.extrude(box_extrude, +);

{box_extrude.dist,test.0}.e();

{box.0}.d(50_mm);

let circle = {A}.loop(cube.)


