use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::{min, max};
use painter::Params;
use image::{RgbaImage, Rgba};
use imageproc::drawing::{draw_convex_polygon_mut, Point};

pub struct Polygon{
    pub points: Vec<(i32, i32)>,
    pub color: Rgba<u8>
}

impl Polygon{
    pub fn new() -> Polygon{
        Polygon{
            points: vec![],
            color: Rgba([0, 0, 0, 0])
        }
    }

    pub fn init(&mut self, rng: &mut ThreadRng, params: &Params){
        self.color[0] = rng.gen_range(*params.red_range.start(), *params.red_range.end());
        self.color[1] = rng.gen_range(*params.green_range.start(), *params.green_range.end());
        self.color[2] = rng.gen_range(*params.blue_range.start(), *params.blue_range.end());
        self.color[3] = rng.gen_range(*params.alpha_range.start(), *params.alpha_range.end());

        let (origin_x, origin_y) = (rng.gen_range(0, params.width), rng.gen_range(0, params.height));
        self.points.clear();
        for _ in 0..params.vertex_num_range.start{
            self.points.push((min(max(0, origin_x + rng.gen_range(0, 30)-30), params.width), min(max(0, origin_y + rng.gen_range(0, 30)-30), params.height)));
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
            self.color[0] = rng.gen_range(*params.red_range.start(), *params.red_range.end());
        }

        if rng.gen::<f32>()<params.mutation_rate{
            self.color[1] = rng.gen_range(*params.green_range.start(), *params.green_range.end());
        }

        if rng.gen::<f32>()<params.mutation_rate{
            self.color[2] = rng.gen_range(*params.blue_range.start(), *params.blue_range.end());
        }

        if rng.gen::<f32>()<params.mutation_rate{
            self.color[3] = rng.gen_range(*params.alpha_range.start(), *params.alpha_range.end());
        }

        //顶点变异
        for i in 0..self.points.len(){
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
                self.points[i].0 = min(max(0, self.points[i].0 + (rng.gen_range(0, move_range*2)-move_range)), params.width);
                self.points[i].1 = min(max(0, self.points[i].1 + (rng.gen_range(0, move_range*2)-move_range)), params.height);
            }
        }
    }

    pub fn add_point(&mut self, rng: &mut ThreadRng, params: &Params){
        if self.points.len() < params.vertex_num_range.end{
            //随机选择一个位置增加点
            let index = rng.gen_range(1, self.points.len()-1);

            let (prev_x, prev_y) = (self.points[index-1].0, self.points[index-1].1);
            let (next_x, next_y) = (self.points[index].0, self.points[index].1);
            //在中间插入一个点
            self.points.insert(index, ((prev_x-next_x)/2, (prev_y-next_y)/2));
        }
    }

    pub fn remove_point(&mut self, rng: &mut ThreadRng, params: &Params){
        if self.points.len() > params.vertex_num_range.start{
            let index = rng.gen_range(0, self.points.len());
            self.points.remove(index);
        }
    }

    pub fn render(&self, image: &mut RgbaImage){
        if self.points[0] != self.points[self.points.len()-1]{
            let poly:Vec<Point<i32>> = self.points.iter().map(|point|  Point::new(point.0, point.1) ).collect();
            draw_convex_polygon_mut(image, poly.as_slice(), self.color);
        }
    }
}

impl Clone for Polygon{
    fn clone(&self) -> Polygon{
        Polygon{
            points: self.points.clone(),
            color: self.color
        }
    }
}