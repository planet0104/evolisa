use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;
use std::cmp::{min, max};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::gfx::primitives::DrawRenderer;
use painter::Params;

pub struct Polygon{
    vx: Vec<i16>,
    vy: Vec<i16>,
    color: Color
}

impl Polygon{
    pub fn new() -> Polygon{
        Polygon{
            vx: vec![],
            vy: vec![],
            color: Color::RGBA(0, 0, 0, 0)
        }
    }

    pub fn init(&mut self, rng: &mut ThreadRng, params: &Params){
        self.color.r = rng.gen_range(*params.red_range.start(), *params.red_range.end());
        self.color.g = rng.gen_range(*params.green_range.start(), *params.green_range.end());
        self.color.b = rng.gen_range(*params.blue_range.start(), *params.blue_range.end());
        self.color.a = rng.gen_range(*params.alpha_range.start(), *params.alpha_range.end());

        let (origin_x, origin_y) = (rng.gen_range(0, params.width), rng.gen_range(0, params.height));
        self.vx.clear();
        self.vy.clear();
        for _ in 0..params.vertex_num_range.start{
            self.vx.push(min(max(0, origin_x + rng.gen_range(0, 30)-30), params.width) as i16);
            self.vy.push(min(max(0, origin_y + rng.gen_range(0, 30)-30), params.height) as i16);
        }
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng, params: &Params){
        //添加顶点
        if rng.gen::<f32>()<params.mutation_rate{
            self.add_point(rng, params);
        }

        //删除顶点
        if rng.gen::<f32>()<params.mutation_rate{
            self.remove_point(rng, params);
        }

        //颜色变异
        if rng.gen::<f32>()<params.mutation_rate{
            self.color.r = rng.gen_range(*params.red_range.start(), *params.red_range.end());
        }

        if rng.gen::<f32>()<params.mutation_rate{
            self.color.g = rng.gen_range(*params.green_range.start(), *params.green_range.end());
        }

        if rng.gen::<f32>()<params.mutation_rate{
            self.color.b = rng.gen_range(*params.blue_range.start(), *params.blue_range.end());
        }

        if rng.gen::<f32>()<params.mutation_rate{
            self.color.a = rng.gen_range(*params.alpha_range.start(), *params.alpha_range.end());
        }

        //顶点变异
        for i in 0..self.vx.len(){
            //选择移动范围
            let move_range = if rng.gen::<f32>()<params.mutation_rate{
                Some(params.vertex_move_range[2])
            }else if rng.gen::<f32>()<params.mutation_rate{
                Some(params.vertex_move_range[1])
            }else if rng.gen::<f32>()<params.mutation_rate{
                Some(params.vertex_move_range[0])
            }else{
                None
            };

            if let Some(move_range) = move_range{
                self.vx[i] = min(max(0, self.vx[i] + (rng.gen_range(0, move_range*2)-move_range)), params.width);
                self.vy[i] = min(max(0, self.vy[i] + (rng.gen_range(0, move_range*2)-move_range)), params.height);
            }
        }
    }

    pub fn add_point(&mut self, rng: &mut ThreadRng, params: &Params){
        if self.vx.len() < params.vertex_num_range.end{
            //随机选择一个位置增加点
            let index = rng.gen_range(1, self.vx.len()-1);

            let (prev_x, prev_y) = (self.vx[index-1], self.vy[index-1]);
            let (next_x, next_y) = (self.vx[index], self.vy[index]);
            //在中间插入一个点
            self.vx.insert(index, (prev_x-next_x)/2);
            self.vy.insert(index, (prev_y-next_y)/2);
        }
    }

    pub fn remove_point(&mut self, rng: &mut ThreadRng, params: &Params){
        if self.vx.len() > params.vertex_num_range.start{
            let index = rng.gen_range(0, self.vx.len());
            self.vx.remove(index);
            self.vy.remove(index);
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Surface>) -> Result<(), String>{
        canvas.set_draw_color(self.color);
        canvas.filled_polygon(&self.vx, &self.vy, self.color)
    }
}

impl Clone for Polygon{
    fn clone(&self) -> Polygon{
        Polygon{
            vx: self.vx.clone(),
            vy: self.vy.clone(),
            color: self.color
        }
    }
}