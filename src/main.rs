#![allow(non_snake_case)]
use std::f32::consts::PI;
use std::ops::Mul;

struct Dot {
    X:f32,
    Y:f32
}

struct Segment {
    A:Dot,
    B:Dot
}

fn main() {
    // println!("{}", Length(&Dot{X:2.0,Y:2.0},&Dot{X:3.0,Y:3.0}));
    // println!("{}", RadianToDeg(35.0));
    // println!("{}", ComputeAngleInDegree(Dot{X:2.0,Y:2.0}));
    // println!("{}",f32::EPSILON);
    task1(Dot{X:2.0,Y:-2.0},Dot{X:0.0,Y:2.0})
}

fn ComputeAngleInDegree(a:Dot) -> f32 {
    let res:f32 = RadianToDeg(Cos(&a).acos());
    if a.Y < 0.0 { 360.0 - res } else { res }
}

fn task1(a:Dot, b:Dot) {
    if ComputeAngleInDegree(a) > ComputeAngleInDegree(b) { println!("a>b") } else { println!("b>a")}
}

fn task2(a:Dot, s:Segment) -> bool {
    (Length(&a,&s.A) + Length(&a, &s.B)).abs() < f32::EPSILON
}

fn task3(a:Dot, s:Segment) -> bool {

    true
}

fn length(a:&Dot, b:&Dot) -> f32 {
    ((a.X - b.X).powi(2) + (a.Y - b.Y).powi(2)).sqrt()
}

fn RadianToDeg(r:f32) -> f32 {
    r.mul(180.0 / PI)
}

fn Cos(a:&Dot) -> f32 {
    a.X / Length(&Dot{X:0.0,Y:0.0},a)
}