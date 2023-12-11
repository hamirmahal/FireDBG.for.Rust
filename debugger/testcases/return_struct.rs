use std::hint::black_box as bb;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
}
#[derive(Debug)]
struct Coeff(f32, f64);
#[derive(Debug)]
struct Mixed {
    x: i32,
    y: f64,
}
#[derive(Debug)]
struct Wrapper<T> {
    i: T,
}
#[derive(Debug)]
struct Anchor<T>(T);
#[derive(Debug)]
struct MapPoint {
    u: Anchor<i64>,
    v: Anchor<i64>,
}
#[derive(Debug)]
struct Label {
    s: &'static str,
}
#[derive(Debug)]
struct Long(i128);

fn ret_1() -> Point { bb(Point { x: 1, y: 2 }) }
fn ret_2() -> (Point, i32) { bb((Point { x: 1, y: 2 }, 3)) }
fn ret_3() -> Vector { bb(Vector { x: 1.1, y: 2.1 }) }
fn ret_4() -> Mixed { bb(Mixed { x: 4, y: 0.1 }) }
fn ret_5() -> Wrapper<Point> { bb(Wrapper{ i: Point { x: 3, y: 4 } }) }
fn ret_6() -> Coeff { bb(Coeff(1.1, 2.2)) }
fn ret_7() -> MapPoint { bb(MapPoint { u: Anchor(-22), v: Anchor(44) }) }
fn ret_8() -> Label { bb(Label { s: "hello" }) }
fn ret_9() -> Long { bb(Long(22_222_222_222_222_222_222)) }

fn main() {
    let r = ret_1(); dbg!(r);
    let r = ret_2(); dbg!(r);
    let r = ret_3(); dbg!(r);
    let r = ret_4(); dbg!(r);
    let r = ret_5(); dbg!(r);
    let r = ret_6(); dbg!(r);
    let r = ret_7(); dbg!(r);
    let r = ret_8(); dbg!(r);
    let r = ret_9(); dbg!(r);
    println!();
}
