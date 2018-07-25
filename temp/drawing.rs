use ::polygon::DnaPolygon;
use ::point::DnaPoint;
use ::brush::DnaBrush;
use std::cell::RefCell;
use std::rc::Rc;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::{min, max};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;

use ::{
    BRUSH_ALPHA_MUTATION_RATE,
    BRUSH_ALPHA_RANGE_MIN,
    BRUSH_ALPHA_RANGE_MAX,
    BRUSH_RED_MUTATION_RATE,
    BRUSH_RED_RANGE_MIN,
    BRUSH_RED_RANGE_MAX,
    BRUSH_GREEN_MUTATION_RATE,
    BRUSH_GREEN_RANGE_MIN,
    BRUSH_GREEN_RANGE_MAX,
    BRUSH_BLUE_MUTATION_RATE,
    BRUSH_BLUE_RANGE_MIN,
    BRUSH_BLUE_RANGE_MAX,

    POLYGON_ADD_POINT_MUTATION_RATE,
    POLYGON_MOVE_POINT_MUTATION_RATE,
    POLYGON_REMOVE_POINT_MUTATION_RATE,
    POLYGON_POINTS_PER_POLYGON_MIN,
    POLYGON_POINTS_PER_POLYGON_MAX,

    DRAWING_POINTS_MIN,
    DRAWING_POINTS_MAX,
    DRAWING_POLYGONS_MIN,
    DRAWING_POLYGONS_MAX,
    DRAWING_ADD_POLYGON_MUTATION_RATE,
    DRAWING_REMOVE_POLYGON_MUTATION_RATE
};

pub struct DnaDrawing{
    polygons: Vec<DnaPolygon>,
    pub dirty: bool,
    pub width: i32,
    pub height: i32,
}

impl DnaDrawing{
    pub fn new(width: i32, height:i32) -> DnaDrawing{
        DnaDrawing{
            polygons: vec![],
            dirty: false,
            width,
            height
        }
    }

    pub fn init(&mut self, rng: &mut ThreadRng){
        for _ in 0..DRAWING_POLYGONS_MIN{
            self.add_polygon(rng);
        }
        self.dirty = true;
    }

    pub fn add_polygon(&mut self, rng:&mut ThreadRng){
        if self.polygons.len() < DRAWING_POLYGONS_MAX as usize{
            let mut new_ploygon = DnaPolygon::new();
            new_ploygon.init(rng, self.width, self.height);
            self.polygons.push(new_ploygon);
            self.dirty = true;
        }
    }

    pub fn remove_polygon(&mut self, rng:&mut ThreadRng){
        let len = self.polygons.len();
        if len>DRAWING_POLYGONS_MIN as usize{
            self.polygons.remove(rng.gen_range(0, len));
            self.dirty = true;
        }
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng){
        if rng.gen_range(0, DRAWING_ADD_POLYGON_MUTATION_RATE)==1{
            self.add_polygon(rng);
        }

        if rng.gen_range(0, DRAWING_REMOVE_POLYGON_MUTATION_RATE)==1{
            self.remove_polygon(rng);
        }

        // for i in 0..self.polygons.len(){
        //     self.polygons[i].mutate(&*self, rng);
        // }
    }

    pub fn render(canvas: Canvas<Surface>) -> Result<(), String>{
        //填充白色背景
        //logical_size

        Ok(())
    }

    pub fn total_points(&self) -> i32{
        let mut total = 0;
        for polygon in &self.polygons{
            total += polygon.points.len() as i32;
        }
        total
    }
}

impl Clone for DnaDrawing{
    fn clone(&self) -> DnaDrawing{
        DnaDrawing{
            polygons: self.polygons.clone(),
            dirty: false,
            width: self.width,
            height: self.height
        }
    }
}