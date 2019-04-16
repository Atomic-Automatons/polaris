extern crate polaris;

use polaris::{
    Circle,
    Collider,
    Map,
    Point,
};
#[test]
fn common() {
    let start = Point::new(25.0, 50.0);
    let end = Point::new(600.0, 400.0);

    let obstacle = Point::new(450.0, 300.0);
    let obstacle1 = Point::new(400.0, 350.0);
    let obstacle2 = Point::new(300.0, 350.0);

    let mut m = Map::new(start, end, 50.0);
    m.add_obstacle(obstacle);
    m.add_obstacle(obstacle1);
    //m.add_obstacle(obstacle2);
    let p = m.compile().unwrap();
    dbg!(p);
}

#[test]
fn circle_point_true() {
    let center = Point::new(2.0, 4.0);
    let radius = 4.0;
    let c = Circle::new(center, radius);
    let p = Point::new(3.0, 4.0);
    assert!(p.collides_with(&c));
}

#[test]
fn circle_point_false() {
    let center = Point::new(2.0, 4.0);
    let radius = 1.0;
    let c = Circle::new(center, radius);
    let p = Point::new(3.0, 4.0);
    assert!(!p.collides_with(&c));
}
