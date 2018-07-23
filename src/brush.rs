use ::drawing::DnaDrawing;
use std::cell::RefCell;
use std::rc::Rc;
use rand::rngs::ThreadRng;
use rand::Rng;
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
    pub parent: Rc<RefCell<DnaDrawing>>,
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl DnaBrush{
    pub fn new(parent: Rc<RefCell<DnaDrawing>>) -> DnaBrush{
        DnaBrush{
            parent,
            r: 0,
            g: 0,
            b: 0,
            a: 0
        }
    }

    pub fn mutate(&mut self, rng:&mut ThreadRng){
        if rng.gen_range(0, BRUSH_RED_MUTATION_RATE)==1{
            self.r = rng.gen_range(BRUSH_RED_RANGE_MIN, BRUSH_RED_RANGE_MAX);
            self.parent.borrow_mut().dirty = true;
        }

        if rng.gen_range(0, BRUSH_GREEN_MUTATION_RATE)==1{
            self.g = rng.gen_range(BRUSH_GREEN_RANGE_MIN, BRUSH_GREEN_RANGE_MAX);
            self.parent.borrow_mut().dirty = true;
        }

        if rng.gen_range(0, BRUSH_BLUE_MUTATION_RATE)==1{
            self.b = rng.gen_range(BRUSH_BLUE_RANGE_MIN, BRUSH_BLUE_RANGE_MAX);
            self.parent.borrow_mut().dirty = true;
        }

        if rng.gen_range(0, BRUSH_ALPHA_MUTATION_RATE)==1{
            self.a = rng.gen_range(BRUSH_ALPHA_RANGE_MIN, BRUSH_ALPHA_RANGE_MAX);
            self.parent.borrow_mut().dirty = true;
        }
    }
}

impl Clone for DnaBrush{
    fn clone(&self) -> DnaBrush{
        DnaBrush{
            parent: self.parent.clone(),
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a
        }
    }
}