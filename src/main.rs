//#![no_main]
extern crate piston_window;
extern crate rand;
mod point;
mod polygon;
mod drawing;
mod brush;

use piston_window::*;

const DRAWING_ADD_POLYGON_MUTATION_RATE:i32 = 700;
const DRAWING_REMOVE_POLYGON_MUTATION_RATE:i32 = 1500;
const DRAWING_POLYGONS_MIN:i32 = 1;
const DRAWING_POLYGONS_MAX:i32 = 255;
const DRAWING_POINTS_MIN:i32 = 3;
const DRAWING_POINTS_MAX:i32 = 255;

const POLYGON_ADD_POINT_MUTATION_RATE:i32 = 1500;
const POLYGON_MOVE_POINT_MUTATION_RATE:i32 = 1500;
const POLYGON_REMOVE_POINT_MUTATION_RATE:i32 = 1500;
const POLYGON_POINT_PER_POLYGON_MIN:i32 = 3;
const POLYGON_POINT_PER_POLYGON_MAX:i32 = 10;

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

fn main(){
    let mut window: PistonWindow = 
            WindowSettings::new("evolisa", [640, 640])
            .exit_on_esc(true).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0; 4], g);

            let mut x = 0.0;
            while x<2.0*std::f64::consts::PI*2.0{
                let ysin = 1.0-x.sin();
                let ycos = 1.0-x.cos();
                rectangle([1.0, 1.0, 1.0, 1.0],
                    [50.0*x, 99.0*ysin, 1.0, 1.0],
                    c.transform, g);
                rectangle([1.0, 0.0, 0.0, 1.0],
                    [50.0*x, 99.0*ycos, 1.0, 1.0],
                    c.transform, g);
                x += 0.01;
            }

        /*
            //画点
            rectangle([1.0, 1.0, 1.0, 1.0],
                    [10.0, 10.0, 1.0, 1.0],
                    c.transform, g);
            
            //画线
            line([1.0, 1.0, 1.0, 1.0],
                    0.5,
                    [50.0, 50.0, 180.0, 150.0],
                    c.transform, g);
            //正方形
            let border = Rectangle::new_border([1.0, 1.1, 1.0, 1.0], 0.5);
            border.draw([190.0, 190.0, 70.0, 70.0],
                                &DrawState::new_outside(),
                                c.transform,
                                g);
            //方块
            rectangle([1.0, 0.0, 0.0, 1.0],
                    [150.0, 150.0, 30.0, 30.0],
                    c.transform, g);
            //圆圈
            circle_arc([1.0, 1.0, 1.0, 1.0],//color
                    0.5,//线宽
                    0.0001,//起始
                    std::f64::consts::PI*2.0,//结束
                    [200.0, 200.0, 50.0, 50.0],
                    c.transform, g);
            //圆形/椭圆
            ellipse([1.0, 1.0, 0.0, 1.0],
                    [300.0, 300.0, 60.0, 60.0],
                    c.transform, g);
            
            //多边形
            let polygon = Polygon::new([1.0; 4]);
            polygon.draw(
                    &vec![[100.0, 100.0],[120.0, 100.0],[120.0, 120.0], [110.0, 120.0]],
                    &DrawState::new_alpha(),
                    c.transform,
                    g);
                    */
        });
    }
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern fn WinMain() -> i32 {
    main();
    0
}