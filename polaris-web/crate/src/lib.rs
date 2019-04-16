use wasm_bindgen::prelude::*;

use polaris::{
    Map as PolarisMap,
    Point as PolarisPoint,
};

#[wasm_bindgen]
pub struct Point {
    point: PolarisPoint,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Point {
            point: PolarisPoint::new(x, y),
        }
    }
	
	pub fn get_x(&self) -> f32{
		self.point.x
	}
	
	pub fn get_y(&self) -> f32{
		self.point.y
	}
}

impl From<PolarisPoint> for Point {
    fn from(point: PolarisPoint) -> Point {
        Point { point }
    }
}

#[wasm_bindgen]
pub struct Map {
    map: PolarisMap,
}

#[wasm_bindgen]
impl Map {
    #[wasm_bindgen(constructor)]
    pub fn new(start: Point, target: Point, radius: f32) -> Self {
        Map {
            map: PolarisMap::new(start.point, target.point, radius),
        }
    }
	
	#[wasm_bindgen]
	pub fn set_limit(&mut self, n: usize){
		self.map.set_limit(n);
	}
	
	#[wasm_bindgen]
	pub fn add_obstacle(&mut self, p: Point){
		self.map.add_obstacle(p.point);
	}

    #[wasm_bindgen]
    pub fn compile(&mut self) -> JsValue {
		let compiled = self.map.compile().map(|c| JsValue::from_serde(&c).unwrap());
		match compiled {
			Ok(c) => c,
			Err(_) => JsValue::null(),
		}
    }
}
