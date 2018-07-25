use polygon::Polygon;
use painter::Params;
use rand::rngs::ThreadRng;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::rect::Rect;

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

    pub fn render(&self, canvas: &mut Canvas<Surface>) -> Result<(), String>{
        //填充白色背景
        let size = canvas.output_size().unwrap();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(Rect::new(0, 0, size.0, size.1)).unwrap();

        for polygon in &self.polygons{
            let r = polygon.render(canvas);
            if r.is_err(){
                return r;
            }
        }
        Ok(())
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