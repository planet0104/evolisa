use ::drawing::DnaDrawing;
use ::point::DnaPoint;
use ::brush::DnaBrush;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::{min, max};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
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
    DRAWING_POINTS_MAX
};

pub struct DnaPolygon{
    brush: DnaBrush,
    pub points: Vec<DnaPoint>
}

impl DnaPolygon{
    pub fn new() -> DnaPolygon{
        DnaPolygon{
            brush: DnaBrush::new(),
            points: vec![]
        }
    }

    pub fn init(&mut self, rng:&mut ThreadRng, width:i32, height:i32){
        self.brush.color.r = rng.gen_range(BRUSH_RED_RANGE_MIN, BRUSH_RED_RANGE_MAX);
        self.brush.color.g = rng.gen_range(BRUSH_GREEN_RANGE_MIN, BRUSH_GREEN_RANGE_MAX);
        self.brush.color.b = rng.gen_range(BRUSH_BLUE_RANGE_MIN, BRUSH_BLUE_RANGE_MAX);
        self.brush.color.a = rng.gen_range(BRUSH_ALPHA_RANGE_MIN, BRUSH_ALPHA_RANGE_MAX);

        let (origin_x, origin_y) = (rng.gen_range(0, width), rng.gen_range(0, height));
        for _ in 0..POLYGON_POINTS_PER_POLYGON_MIN{
            let mut point = DnaPoint::new();
            point.x = min(max(0, origin_x + rng.gen_range(0, 30)-30), width);
            point.y = min(max(0, origin_y + rng.gen_range(0, 30)-30), height);
            self.points.push(point);
        }
    }

    pub fn mutate<F: Fn()->i32>(&mut self, total_points: F, rng:&mut ThreadRng) -> bool{
        let mut dirty = false;
        // if rng.gen_range(0, POLYGON_ADD_POINT_MUTATION_RATE)==1{
        //     if self.add_point(total_points, rng){
        //         dirty = true;
        //     }
        // }
        // if rng.gen_range(0, POLYGON_REMOVE_POINT_MUTATION_RATE)==1{
        //     if self.remove_point(total_points, rng){
        //         dirty = true;
        //     }
        // }
        // if self.brush.mutate(rng){
        //     dirty = true;
        // }
        // for point in &mut self.points{
        //     if point.mutate(drawing, rng){
        //         dirty = true;
        //     }
        // }

        dirty
    }

    pub fn add_point<F: Fn()->i32>(&mut self, total_points: F, rng:&mut ThreadRng) -> bool{
        let mut dirty = false;
        if self.points.len() < POLYGON_POINTS_PER_POLYGON_MAX as usize{
            if total_points() < DRAWING_POINTS_MAX{
                let mut new_point = DnaPoint::new();
                let index = rng.gen_range(1, self.points.len()-1);

                {
                    let prev = &self.points[index-1];
                    let next = &self.points[index];

                    new_point.x = (prev.x + next.x)/2;
                    new_point.y = (prev.y + next.y)/2;
                }

                self.points.insert(index, new_point);
                dirty = true;
            }
        }
        dirty
    }

    pub fn remove_point<F: Fn()->i32>(&mut self, total_points: F, rng:&mut ThreadRng) -> bool{
        let mut dirty = false;
        if self.points.len() > POLYGON_POINTS_PER_POLYGON_MIN as usize{
            if total_points() > DRAWING_POINTS_MIN{
                let index = rng.gen_range(0, self.points.len());
                self.points.remove(index);
                dirty = true;
            }
        }
        dirty
    }

    pub fn render(&self, canvas: Canvas<Surface>) -> Result<(), String>{
        let vx:Vec<i16> = self.points.iter().map(|point|{ point.x as i16 }).collect();
        let vy:Vec<i16> = self.points.iter().map(|point|{ point.y as i16 }).collect();
        canvas.filled_polygon(vx.as_slice(), vy.as_slice(), self.brush.color)
    }
}

impl Clone for DnaPolygon{
    fn clone(&self) -> DnaPolygon{
        DnaPolygon{
            brush: self.brush.clone(),
            points: self.points.clone()
        }
    }
}