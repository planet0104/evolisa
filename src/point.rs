use ::drawing::DnaDrawing;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::{min, max};
use  ::{POINT_MOVE_POINT_MUTATION_RATE,
        POINT_MOVE_POINT_MIN_MUTATION_RATE,
        POINT_MOVE_POINT_MID_MUTATION_RATE,
        POINT_MOVE_POINT_MAX_MUTATION_RATE,
        POINT_MOVE_POINT_RANGE_MIN,
        POINT_MOVE_POINT_RANGE_MID};

pub struct DnaPoint{
    pub parent: Rc<RefCell<DnaDrawing>>,
    pub x: i32,
    pub y: i32,
}

impl DnaPoint{
    pub fn mutate(&mut self, rng:&mut ThreadRng, width:i32, height:i32){
        if rng.gen_range(0, POINT_MOVE_POINT_MAX_MUTATION_RATE)==1{
            self.x = rng.gen_range(0, width);
            self.y = rng.gen_range(0, height);
            self.parent.borrow_mut().dirty = true;
        }

        if rng.gen_range(0, POINT_MOVE_POINT_MID_MUTATION_RATE)==1{
            self.x = min(max(0, self.x + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MID*2)-POINT_MOVE_POINT_RANGE_MID)), width);
            self.y = min(max(0, self.y + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MID*2)-POINT_MOVE_POINT_RANGE_MID)), height);
            self.parent.borrow_mut().dirty = true;
        }

        if rng.gen_range(0, POINT_MOVE_POINT_MIN_MUTATION_RATE)==1{
            self.x = min(max(0, self.x + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MIN*2)-POINT_MOVE_POINT_RANGE_MIN)), width);
            self.y = min(max(0, self.y + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MIN*2)-POINT_MOVE_POINT_RANGE_MIN)), height);
            self.parent.borrow_mut().dirty = true;
        }
    }
}

impl Clone for DnaPoint{
    fn clone(&self) -> DnaPoint{
        DnaPoint{
            parent: self.parent.clone(),
            x: self.x,
            y: self.y,
        }
    }
}