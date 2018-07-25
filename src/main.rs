//#![no_main]
extern crate sdl2;
extern crate rand;
extern crate image;

use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::path::Path;

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

    offscreen_test();

    //60个半透明多边形
    // println!("start..");
    // let params = Params{
    //     width: 120,
    //     height: 145,
    //     num_elite: 2,
    //     num_copies_elite: 2,
    //     polygons_num: 60, //多边形数量
    //     vertex_num_range: 3..10, //多边形顶点数量 3~10
    //     mutation_rate: 0.007, //变异率 0.007
    //     crossover_rate: 0.7, //杂交率 0.7
    //     vertex_move_range: [200, 20, 3], //顶点移动范围类型(值越小的概率越高) [0~200; 0~20; 0~3]
    //     alpha_range: 30..=60, //颜色取值范围
    //     red_range: 0..=255,
    //     green_range: 0..=255,
    //     blue_range: 0..=255,
    // };
    // println!("params..");

    // let mut painter = Painter::new(20, "girl.jpg", params);

    // println!("new ok.");

    // painter.epoch();

    // println!("OK.");
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    main();
    0
}

fn offscreen_test(){

    let surface = Surface::new(100, 100, PixelFormatEnum::RGB24).unwrap();

    let mut canvas = surface.into_canvas().unwrap();


    //-------------------------------------------------------------
    //使用纹理
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(texture_creator.default_pixel_format(), 100, 100)
        .unwrap();
    canvas.with_texture_canvas(&mut texture, |texture_canvas| {
        texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        texture_canvas.fill_rect(Rect::new(0, 0, 50, 50)).unwrap();
    }).unwrap();
    canvas.copy(&texture, None, None).expect("Render failed");
    //-------------------------------------------------------------

    //普通画布
    // canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
    // canvas.fill_rect(Rect::new(0, 0, 50, 50)).unwrap();
    
    canvas.present();

    println!("{:?}", canvas.output_size());

    //let image = canvas.read_pixels(Rect::new(0, 0, 512, 512), PixelFormatEnum::RGB888).unwrap();
    canvas.surface().save_bmp("test.bmp").unwrap();
}