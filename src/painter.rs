use drawing::Drawing;
use rand::rngs::ThreadRng;
use rand::Rng;
use rand::thread_rng;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
use image;
use image::DynamicImage;

/*
参数:

多边形顶点数量 3~10
增加顶点、移动顶点、删除顶点、色值改变 的概率 0.0007
顶点移动范围 3~20、0~width
Alpha取值范围 30~60
Red取值范围 0~255
Green取值范围 0~255
Blue取值范围 0~255
*/

//参数
pub struct Params{
    pub width: i16,
    pub height: i16,
    pub num_elite: usize,
    pub num_copies_elite: usize,
    pub polygons_num: usize, //多边形数量
    pub vertex_num_range: ::std::ops::Range<usize>, //多边形顶点数量 3~10
    pub mutation_rate: f32, //变异率 0.007
    pub crossover_rate: f32, //杂交率 0.7
    pub vertex_move_range: [i16; 3], //顶点移动范围类型(值越小的概率越高) [0~200; 0~20; 0~3]
    pub alpha_range: ::std::ops::RangeInclusive<u8>, //颜色取值范围
    pub red_range: ::std::ops::RangeInclusive<u8>,
    pub green_range: ::std::ops::RangeInclusive<u8>,
    pub blue_range: ::std::ops::RangeInclusive<u8>,
}

pub struct Painter{
    target_pixels: Vec<u8>,
    params: Params,
    drawings: Vec<Drawing>,
    generation: usize,
    total_fitness: f64,
    rng: ThreadRng,
}

impl Painter{
    //创建初始群体
    pub fn new(pop_size: usize, target_file: &str, params: Params) ->Painter{
        let mut drawings = vec![];
        let mut rng = thread_rng();

        for _ in 0..pop_size{
            let mut drawing = Drawing::new();
            drawing.init(&params, &mut rng);
            drawings.push(drawing);
        }

        //读取目标图片像素
        let img = image::open(target_file).unwrap();
        Painter{
            rng,
            target_pixels: img.raw_pixels(),
            generation: 0,
            drawings,
            params,
            total_fitness: 0.0,
        }
    }

    pub fn epoch(&mut self){

        //计算总适应分
        self.total_fitness = 0.0;
        for p in &mut self.drawings{
            self.total_fitness += p.fitness;
        }

        //按照得分排序
        self.drawings.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let mut new_pop = vec![];
        //精英选择
        self.grab_n_best(self.params.num_elite, self.params.num_copies_elite, &mut new_pop);

        while new_pop.len() < self.drawings.len(){
            println!(">>>len={}", new_pop.len());
            let (id1, id2) = (self.get_chromo_roulette(), self.get_chromo_roulette());
            //杂交
            let (mut baby1, mut baby2) = Painter::crossover(self.params.crossover_rate, &self.drawings[id1], &self.drawings[id2], &mut self.rng);
            //变异
            baby1.mutate(&mut self.rng, &self.params);
            baby1.mutate(&mut self.rng, &self.params);

            //计算适应分
            baby1.fitness = self.calculate_fitness(&baby1);
            baby2.fitness = self.calculate_fitness(&baby2);

            new_pop.push(baby1);
            new_pop.push(baby2);
        }

        self.generation += 1;
    }

    //精英选择
    pub fn grab_n_best(&self, num_elite: usize, num_copies: usize, pop:&mut Vec<Drawing>){
        for i in 0..num_elite{
            for _ in 0..num_copies{
                pop.push(self.drawings[i].clone());
            }
        }
    }

    //赌轮选择
    pub fn get_chromo_roulette(&mut self) -> usize{
        //生成0和总体适应分之间的随机数
        let slice = self.rng.gen::<f64>() * self.total_fitness;
        let mut fitness_so_far = 0.0;
        let mut the_choose_one = 0;
        for i in 0..self.drawings.len(){
            fitness_so_far += self.drawings[i].fitness;
            //如果当前适应分>随机数，返回此处的染色体
            if fitness_so_far > slice{
                the_choose_one = i;
                break;
            }
        }
        the_choose_one
    }

    pub fn crossover(crossover_rate: f32, mum: &Drawing, dad: &Drawing, rng: &mut ThreadRng) -> (Drawing, Drawing){

        if rng.gen::<f32>()>crossover_rate{
            return (mum.clone(), dad.clone());
        }

        let len = mum.polygons.len();
        let index = rng.gen_range(0, len);

        let mut baby1 = vec![];
        let mut baby2 = vec![];
        baby1.extend_from_slice(mum.polygons.get(0..index).unwrap());
        baby1.extend_from_slice(dad.polygons.get(index..len).unwrap());
        baby2.extend_from_slice(dad.polygons.get(0..index).unwrap());
        baby2.extend_from_slice(mum.polygons.get(index..len).unwrap());

        (Drawing{
            fitness: 0.0,
            polygons: baby1    
        }, Drawing{
            fitness: 0.0,
            polygons: baby2
        })
    }

    pub fn calculate_fitness(&mut self, drawing:&Drawing) -> f64{
        let (width, height) = (self.params.width as u32, self.params.height as u32);
        let surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
        let mut canvas = surface.into_canvas().unwrap();
        drawing.render(&mut canvas).unwrap();
        canvas.present();
        let drawing_pixels = canvas.read_pixels(Rect::new(0, 0, width, height), PixelFormatEnum::RGB24).unwrap();
        
        let mut error = 0.0;
        
        let mut i = 0;
        while i<drawing_pixels.len(){        
            let r = self.target_pixels[i] as f64 - drawing_pixels[i] as f64;
            let g = self.target_pixels[i+1] as f64 - drawing_pixels[i+1] as f64;
            let b = self.target_pixels[i+2] as f64 - drawing_pixels[i+2] as f64;
            let a = self.target_pixels[i+3] as f64 - drawing_pixels[i+3] as f64;
            let e = r*r + g*g + b*b + a*a;

            //Alternative error functions
            //e = sqrt(r*r + g*g + b*b + a*a);
            //e = abs(r) + abs(g) + abs(b) + abs(a);

            error += e;
            i += 4;
        }
        
        error
    }
}