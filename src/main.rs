//#![no_main]
extern crate sdl2;
extern crate rand;
extern crate image;
extern crate imageproc;

use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rwops::RWops;
use std::path::Path;
use std::time::{Duration, Instant};
use sdl2::gfx::primitives::DrawRenderer;
use image::RgbaImage;
use imageproc::drawing::draw_convex_polygon_mut;
use imageproc::drawing::draw_filled_rect_mut;
use image::Rgba;
use imageproc::drawing::Point;

mod polygon;
mod drawing;
mod painter;

use painter::{Painter, Params};

//opengl库
/*

1. glium
2. gfx
3. gl-rs

 */

//opencl GPU图像处理:
//https://www.evl.uic.edu/kreda/gpu/image-convolution/

/* rust缩小体积
https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html

//SDL2_gfx http://www.ferzkopp.net/wordpress/2016/01/02/sdl_gfx-sdl2_gfx/
/*

windows下 编译 SDL2_gfx:
1、安装 Visual Studio Community 2017
2、解压从以上网站下载的SDL2_gfx源码
3、将SDL2 2.0.8 源码中include文件夹下的所有.h文件复制到SDL2_gfx源码目录
4、用Visual Studio打开SDL2_gfx.sln1
5、右键SDL2_gfx，选择"属性"
    配置:Release 平台:x64
    项目默认值 配置类型:静态库(.lib)
    字符集: 
6、右键SDL2_gfx，选择"属性"，点击右上角"配置管理器"，选择 release x64
7、右键SDL2_gfx生成， 即可得到x64\Release\SDL2_gfx.lib


 */

crate-type = cdylib 使用 lto

[profile.release]
lto = true
panic = 'abort'

*/

//源码 https://github.com/jhrdt/evolisa.js

fn main(){

    //60个半透明多边形
    let params = Params{
        width: 120,
        height: 145,
        num_elite: 2,
        num_copies_elite: 2,
        polygons_num: 50, //多边形数量
        vertex_num_range: 3..10, //多边形顶点数量 3~10
        mutation_rate: 0.004, //变异率 0.004
        crossover_rate: 0.5, //杂交率 0.5
        vertex_move_range: [200, 20, 3], //顶点移动范围类型(值越小的概率越高) [0~200; 0~20; 0~3]
        alpha_range: 30..=60, //颜色取值范围
        red_range: 0..=255,
        green_range: 0..=255,
        blue_range: 0..=255,
    };

    let (width, height) = (params.width as u32, params.height as u32);

    let mut painter = Painter::new(100, "girl.jpg", params);

    let surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
    let mut canvas = surface.into_canvas().unwrap();

    // for g in 0..500000{
    //     painter.epoch(&mut canvas);
    //     if g%1000 == 0{
    //         let drawings = painter.drawings();
    //         println!("代数:{} 最高分:{} 最低分:{}", painter.generation(), drawings[0].fitness, drawings[drawings.len()-1].fitness);
    //     }
    // }

    //painter.epoch(&mut canvas);

    //渲染速度测试
    println!("开始测试.");

    let start_time = Instant::now();

    let drawings = painter.drawings();
    let mut image = RgbaImage::new(120, 145);

    let mut total = 0;

    for drawing in drawings{
        let polygons = &drawing.polygons;
        
        //SDL2渲染bitmap
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_target(texture_creator.default_pixel_format(), 120, 145)
            .unwrap();
        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            let size = texture_canvas.output_size().unwrap();
            texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
            texture_canvas.fill_rect(Rect::new(0, 0, size.0, size.1)).unwrap();

            for polygon in polygons{
                texture_canvas.set_draw_color(polygon.color);
                texture_canvas.filled_polygon(&polygon.vx, &polygon.vy, polygon.color).unwrap();
            }
        }).unwrap();
        canvas.copy(&texture, None, None).expect("Render failed");
        canvas.present();
        total += canvas.read_pixels(Rect::new(0, 0, 120, 145), PixelFormatEnum::RGB24).unwrap().len();

        //imageproc渲染bitmap
        draw_filled_rect_mut(&mut image, imageproc::rect::Rect::at(0, 0).of_size(120, 145), Rgba([0u8, 0u8, 0u8, 255u8]));
        for polygon in polygons{
            let mut poly = vec![];
            for i in 0..polygon.vx.len(){
                poly.push(Point::new(polygon.vx[i], polygon.vy[i]));
            }
            draw_convex_polygon_mut(&mut image, &poly, Rgba([polygon.color.r as u8, polygon.color.g as u8, polygon.color.b as u8, polygon.color.a as u8]));
        }
    }

    println!("total={} render耗时:{}ms", total, duration_to_milis(&start_time.elapsed()));
    //25ms渲染100个
    //1秒渲染4000个(200代左右)

    //canvas.surface().save_bmp("ok.bmp").unwrap();
    

    // println!("OK.");

    // let sdl_context = sdl2::init().unwrap();
    // let video_subsystem = sdl_context.video().unwrap();

    // let window = video_subsystem
    //     .window("evolisa", 800, 600)
    //     .resizable()
    //     .build()
    //     .unwrap();

    // let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    // let texture_creator = canvas.texture_creator();
    
    // let target_surface = Surface::load_bmp("girl.bmp").unwrap();
    // let target_texture = texture_creator.create_texture_from_surface(&target_surface).unwrap();

    // let mut tick = 0;

    // let mut event_pump = sdl_context.event_pump().unwrap();

    // 'running: loop {
    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. } |
    //             Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
    //             _ => {}
    //         }
    //     }

    //     {
    //         // Update the window title.
    //         let window = canvas.window_mut();

    //         let position = window.position();
    //         let size = window.size();
    //         let title = format!("Window - pos({}x{}), size({}x{}): {}",
    //                             position.0,
    //                             position.1,
    //                             size.0,
    //                             size.1,
    //                             tick);
    //         window.set_title(&title).unwrap();

    //         tick += 1;
    //     }

    //     painter.epoch();
    //     // if tick%100 == 0{
    //     //     let drawings = painter.drawings();
    //     //     println!("代数{} 最高分:{} 最低分:{}", painter.generation(), drawings[0].fitness, drawings[drawings.len()-1].fitness);
    //     // }
    //     let render_canvas = painter.render_drawing(&painter.drawings()[0]);
    //     let texture = texture_creator.create_texture_from_surface(render_canvas.surface()).unwrap();
    //     let size = render_canvas.output_size().unwrap();
    //     canvas.copy(&texture, Rect::new(0, 0, size.0, size.1), Rect::new(0, 0, size.0, size.1)).expect("Render failed");

    //     canvas.copy(&target_texture, Rect::new(0, 0, size.0, size.1), Rect::new(200, 0, size.0, size.1)).expect("Render failed");

    //     canvas.present();
    // }
}

pub fn duration_to_milis(duration: &Duration) -> f64 {
    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 / 1_000_000.0
}

// pub fn render_drawing(&self, drawing:&Drawing) -> Canvas<Surface>{
    //     let (width, height) = (self.params.width as u32, self.params.height as u32);
    //     let surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
    //     let mut canvas = surface.into_canvas().unwrap();

    //     let texture_creator = canvas.texture_creator();
    //     let mut texture = texture_creator
    //         .create_texture_target(texture_creator.default_pixel_format(), 100, 100)
    //         .unwrap();
    //     canvas.with_texture_canvas(&mut texture, |mut texture_canvas| {
    //         drawing.render(&mut texture_canvas).unwrap();
    //     }).unwrap();
    //     canvas.copy(&texture, None, None).expect("Render failed");
    //     canvas.present();

    //     canvas
    // }


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    main();
    0
}