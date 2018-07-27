use polygon::Polygon;
use painter::Params;
use rand::rngs::ThreadRng;
use image::{RgbaImage, Rgba};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;

pub struct Drawing{
    pub fitness: f64,
    pub polygons: Vec<Polygon>
}

impl Drawing{
    pub fn new() -> Drawing{
        Drawing{ fitness: 0.0, polygons: vec![] }
    }

    pub fn init(&mut self, params: &Params, rng: &mut ThreadRng){
        self.polygons.clear();
        for _ in 0..params.polygons_num{
            let mut polygon = Polygon::new();
            polygon.init(rng, params);
            self.polygons.push(polygon);
        }
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng, params: &Params){
        for i in 0..self.polygons.len(){
            self.polygons[i].mutate(rng, params);
        }
    }

    pub fn render(&self, image: &mut RgbaImage){
        //填充黑色背景
        let (width, height) = (image.width(), image.height());
        draw_filled_rect_mut(image, Rect::at(0, 0).of_size(width, height), Rgba([0u8, 0u8, 0u8, 255u8]));

        for polygon in &self.polygons{
            polygon.render(image);
        }
    }
}

impl Clone for Drawing{
    fn clone(&self) -> Drawing{
        Drawing{
            fitness: self.fitness,
            polygons: self.polygons.clone()
        }
    }
}