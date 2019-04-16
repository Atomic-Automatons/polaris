mod point;

pub use point::Point;

use serde::{
    Deserialize,
    Serialize,
};
use std::collections::{
    BTreeMap,
    VecDeque,
};

#[derive(Debug)]
pub enum PolarisError {
    ProcessedLimit,
}

pub type PolarisResult<T> = Result<T, PolarisError>;

pub struct Quadtree<T> {
    quads: Option<[Box<Quadtree<T>>; 4]>,
    children: Vec<T>,
}

impl<T> Quadtree<T> {
    pub fn new() -> Self {
        Quadtree {
            quads: None,
            children: Vec::new(),
        }
    }
}

pub struct SpatialPartition {
    //nodes: Vec<Vec<
}

pub struct Circle {
    center: Point,
    radius: f32,
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Circle { center, radius }
    }
}

pub trait Collider {
    type Target;
    fn collides_with(&self, target: &Self::Target) -> bool;
}

impl Collider for Point {
    type Target = Circle;
    fn collides_with(&self, target: &Self::Target) -> bool {
        //let dx = (target.center.x - self.x).abs();
        //let dy = (target.center.y - self.y).abs();
        //let mag = dx.powi(2) + dy.powi(2);
        (&target.center - self).mag() < target.radius
    }
}

#[derive(Debug)]
pub struct Arc {
    start: f32,
    end: f32,
}

impl Arc {
    pub fn new(mut start: f32, mut end: f32) -> Self {
        assert!(start < end);
        //start = Self::normalize(start);
        //end = Self::normalize(end);

        Arc { start, end }
    }

    pub fn normalize(mut value: f32) -> f32 {
        value.sin().atan2(value.cos())
    }

    pub fn contains(&self, mut angle: f32) -> bool {
        //angle = Self::normalize(angle);
        angle >= self.start && angle <= self.end
    }
}

pub struct Node {
	parents: Vec<usize>,
	children: Vec<usize>,
	point: Point,
}

pub struct Map {
    current: Point,
    target: Point,
    radius: f32,
    obstacles: Vec<Point>,

    path: Vec<Point>,
    queue: VecDeque<Point>,
    processed_limit: usize,
}

impl Map {
    pub fn new(current: Point, target: Point, radius: f32) -> Self {
        Map {
            current,
            target,
            radius,
            obstacles: Vec::new(),
            path: Vec::new(),

            queue: VecDeque::new(),
            processed_limit: 500,
        }
    }

    pub fn set_limit(&mut self, n: usize) {
        self.processed_limit = n;
    }

    pub fn add_obstacle(&mut self, p: Point) {
        self.obstacles.push(p);
    }

    pub fn add_obstacles(&mut self, points: &[Point]) {
        self.obstacles.extend_from_slice(points);
    }

    pub fn get_obstacle_collisions(&self, c: &Circle) -> Vec<&Point> {
        self.obstacles
            .iter()
            .filter(|el| el.collides_with(c))
            .collect()
    }

    fn calculate_point(&mut self, current_point: &Point, delta: &Point, theta: f32) -> Point {
        let mut travel_x = self.radius * theta.cos();
        let mut travel_y = self.radius * theta.sin();

        if delta.mag() < self.radius {
            travel_x = delta.x;
            travel_y = delta.y;
        }

        Point::new(current_point.x + travel_x, current_point.y + travel_y)
    }

    fn get_denial_arcs(&self, point: &Point) -> Vec<Arc> {
        self.get_obstacle_collisions(&Circle::new(point.clone(), self.radius * 2.0))
            .iter()
            .map(|p| {
                let delta = *p - point;
                let theta = delta.theta();
                //let denial = 0.5_f32.atan();
                let denial = 60.0_f32.to_radians();
                Arc::new(theta - denial, theta + denial)
            })
            .fold(Vec::new(), |mut denial_arcs: Vec<Arc>, new_arc: Arc| {
                //dbg!(&new_arc);
                let mut added = false;
                denial_arcs.iter_mut().for_each(|total_arc| {
                    if new_arc.start < new_arc.end && total_arc.start < total_arc.end {
                        if total_arc.contains(new_arc.start) && !total_arc.contains(new_arc.end) {
                            total_arc.end = new_arc.end;
                            added = true;
                        } else if total_arc.contains(new_arc.end)
                            && !total_arc.contains(new_arc.start)
                        {
                            total_arc.start = new_arc.start;
                            added = true;
                        }
                    }
                });

                if !added {
                    denial_arcs.push(new_arc);
                }

                denial_arcs
            })
    }

    fn add_if_valid(&mut self, p: Point) {
        if self.path.iter().all(|n| (n - &p).mag() > self.radius * 0.5) && self.obstacles.iter().all(|o| (o - &p).mag() > self.radius){
            self.queue.push_back(p.clone());
            self.path.push(p);
        }
    }

    pub fn compile(&mut self) -> PolarisResult<Vec<Point>> {
        self.queue.truncate(0);
        self.path.truncate(0);

        self.path.push(self.current.clone());
        let mut processed = 0;

        let nodes = vec![self.current.clone()];

        self.queue.push_back(self.current.clone());

        while self.queue.front().is_some()
            && self.queue.front().expect("No data in queue") != &self.target
            && processed < self.processed_limit
        {
            let current = self.queue.pop_front().unwrap();
            let denial_arcs = self.get_denial_arcs(&current);

            dbg!(processed);
            dbg!(&denial_arcs);

            //find new points, one for now
            //Breath first for shortest path?

            let delta = &self.target - &current;
            let mut theta = delta.theta();

            //let thetas: Vec<_> = denial_arcs.iter().filter(|arc| arc.contains(theta)).collect();

            if denial_arcs.is_empty() || denial_arcs.iter().all(|a| !a.contains(theta)) {
                let new_point = self.calculate_point(&current, &delta, theta);
                self.add_if_valid(new_point.clone());
            }

            for arc in denial_arcs.iter() {
                if arc.contains(theta) {
                    let start_point = self.calculate_point(&current, &delta, arc.start);
                    let end_point = self.calculate_point(&current, &delta, arc.end);

                    self.add_if_valid(start_point.clone());
                    self.add_if_valid(end_point.clone());

                    if (&self.target - &start_point).mag() > (&self.target - &end_point).mag() {
                        theta = arc.end;
                    } else {
                        theta = arc.start;
                    }
                }
            }

            //let new_point = self.calculate_point(&current, &delta, theta);
            //self.queue.push_back(new_point.clone());
            //self.add_to_queue_if_valid(new_point.clone());

            //self.path.push(new_point);
            processed += 1;
        }

        if self.processed_limit == processed {
            //return Err(PolarisError::ProcessedLimit);
        }

        let mut path = Vec::new();
        std::mem::swap(&mut path, &mut self.path);

        Ok(path)
    }
}
