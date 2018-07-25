use rand::rngs::ThreadRng;
use rand::Rng;
use sdl2::pixels::Color;

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
};

pub struct DnaBrush{
    pub color: Color,
}

impl DnaBrush{
    pub fn new() -> DnaBrush{
        DnaBrush{
            color: Color::RGBA(0, 0, 0, 0)
        }
    }

    pub fn mutate(&mut self, rng:&mut ThreadRng) -> bool{
        let mut dirty = false;
        if rng.gen_range(0, BRUSH_RED_MUTATION_RATE)==1{
            self.color.r = rng.gen_range(BRUSH_RED_RANGE_MIN, BRUSH_RED_RANGE_MAX);
            dirty = true;
        }

        if rng.gen_range(0, BRUSH_GREEN_MUTATION_RATE)==1{
            self.color.g = rng.gen_range(BRUSH_GREEN_RANGE_MIN, BRUSH_GREEN_RANGE_MAX);
            dirty = true;
        }

        if rng.gen_range(0, BRUSH_BLUE_MUTATION_RATE)==1{
            self.color.b = rng.gen_range(BRUSH_BLUE_RANGE_MIN, BRUSH_BLUE_RANGE_MAX);
            dirty = true;
        }

        if rng.gen_range(0, BRUSH_ALPHA_MUTATION_RATE)==1{
            self.color.a = rng.gen_range(BRUSH_ALPHA_RANGE_MIN, BRUSH_ALPHA_RANGE_MAX);
            dirty = true;
        }

        dirty
    }
}

impl Clone for DnaBrush{
    fn clone(&self) -> DnaBrush{
        DnaBrush{
            color: self.color
        }
    }
}