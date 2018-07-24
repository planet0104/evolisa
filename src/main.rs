//#![no_main]
extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;

mod point;
mod polygon;
mod drawing;
mod brush;

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


const DRAWING_ADD_POLYGON_MUTATION_RATE:i32 = 700;
const DRAWING_REMOVE_POLYGON_MUTATION_RATE:i32 = 1500;
const DRAWING_POLYGONS_MIN:i32 = 1;
const DRAWING_POLYGONS_MAX:i32 = 255;
const DRAWING_POINTS_MIN:i32 = 3;
const DRAWING_POINTS_MAX:i32 = 255;

const POLYGON_ADD_POINT_MUTATION_RATE:i32 = 1500;
const POLYGON_MOVE_POINT_MUTATION_RATE:i32 = 1500;
const POLYGON_REMOVE_POINT_MUTATION_RATE:i32 = 1500;
const POLYGON_POINTS_PER_POLYGON_MIN:i32 = 3;
const POLYGON_POINTS_PER_POLYGON_MAX:i32 = 10;

const POINT_MOVE_POINT_MUTATION_RATE:i32 = 1500;
const POINT_MOVE_POINT_MIN_MUTATION_RATE:i32 = 1500;
const POINT_MOVE_POINT_MID_MUTATION_RATE:i32 = 1500;
const POINT_MOVE_POINT_MAX_MUTATION_RATE:i32 = 1500;
const POINT_MOVE_POINT_RANGE_MIN:i32 = 3;
const POINT_MOVE_POINT_RANGE_MID:i32 = 20;

const BRUSH_ALPHA_MUTATION_RATE:i32 = 1500;
const BRUSH_ALPHA_RANGE_MIN:u8 = 30;
const BRUSH_ALPHA_RANGE_MAX:u8 = 60;
const BRUSH_RED_MUTATION_RATE:i32 = 1500;
const BRUSH_RED_RANGE_MIN:u8 = 0;
const BRUSH_RED_RANGE_MAX:u8 = 255;
const BRUSH_GREEN_MUTATION_RATE:i32 = 1500;
const BRUSH_GREEN_RANGE_MIN:u8 = 0;
const BRUSH_GREEN_RANGE_MAX:u8 = 255;
const BRUSH_BLUE_MUTATION_RATE:i32 = 1500;
const BRUSH_BLUE_RANGE_MIN:u8 = 0;
const BRUSH_BLUE_RANGE_MAX:u8 = 255;


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
    println!("{:?}", canvas.logical_size());

    //let image = canvas.read_pixels(Rect::new(0, 0, 512, 512), PixelFormatEnum::RGB888).unwrap();
    canvas.surface().save_bmp("test.bmp").unwrap();
}

//源码 https://github.com/jhrdt/evolisa.js

fn main(){

    offscreen_test();
    return;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("evolisa", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        {
            // Update the window title.
            let window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!("Window - pos({}x{}), size({}x{}): {}",
                                position.0,
                                position.1,
                                size.0,
                                size.1,
                                tick);
            window.set_title(&title).unwrap();

            tick += 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.filled_polygon(&[100, 200, 0], &[0, 0, 100], Color::RGBA(255, 0, 0, 255)).unwrap();

        canvas.present();
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    main();
    0
}