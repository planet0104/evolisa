use ::drawing::DnaDrawing;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::cmp::{min, max};
use  ::{POINT_MOVE_POINT_MUTATION_RATE,
        POINT_MOVE_POINT_MIN_MUTATION_RATE,
        POINT_MOVE_POINT_MID_MUTATION_RATE,
        POINT_MOVE_POINT_MAX_MUTATION_RATE,
        POINT_MOVE_POINT_RANGE_MIN,
        POINT_MOVE_POINT_RANGE_MID};

pub struct DnaPoint{
    pub x: i32,
    pub y: i32,
}

impl DnaPoint{
    pub fn new()->DnaPoint{
        DnaPoint{
            x: 0,
            y: 0,
        }
    }

    pub fn mutate(&mut self, drawing: &DnaDrawing, rng:&mut ThreadRng) -> bool{
        let mut dirty = false;
        if rng.gen_range(0, POINT_MOVE_POINT_MAX_MUTATION_RATE)==1{
            self.x = rng.gen_range(0, drawing.width);
            self.y = rng.gen_range(0, drawing.height);
            dirty = true;
        }

        if rng.gen_range(0, POINT_MOVE_POINT_MID_MUTATION_RATE)==1{
            self.x = min(max(0, self.x + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MID*2)-POINT_MOVE_POINT_RANGE_MID)), drawing.width);
            self.y = min(max(0, self.y + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MID*2)-POINT_MOVE_POINT_RANGE_MID)), drawing.height);
            dirty = true;
        }

        if rng.gen_range(0, POINT_MOVE_POINT_MIN_MUTATION_RATE)==1{
            self.x = min(max(0, self.x + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MIN*2)-POINT_MOVE_POINT_RANGE_MIN)), drawing.width);
            self.y = min(max(0, self.y + (rng.gen_range(0, POINT_MOVE_POINT_RANGE_MIN*2)-POINT_MOVE_POINT_RANGE_MIN)), drawing.height);
            dirty = true;
        }

        dirty
    }
}

impl Clone for DnaPoint{
    fn clone(&self) -> DnaPoint{
        DnaPoint{
            x: self.x,
            y: self.y,
        }
    }
}