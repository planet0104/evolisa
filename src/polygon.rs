use ::drawing::DnaDrawing;
use ::point::DnaPoint;
use ::brush::DnaBrush;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DnaPolygon{
    brush: DnaBrush,
    parent: Rc<RefCell<DnaDrawing>>,
    points: Vec<DnaPoint>
}

impl DnaPolygon{
    pub fn new(parent: Rc<RefCell<DnaDrawing>>) -> DnaPolygon{
        DnaPolygon{
            parent: parent.clone(),
            brush: DnaBrush::new(parent),
            points: vec![]
        }
    }

    pub fn init(&mut self){
        
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<DnaDrawing>>){
        for point in &mut self.points{
            point.parent = parent.clone();
        }
        self.brush.parent = parent.clone();
        self.parent = parent;
    }

    pub fn parent(&self) -> &Rc<RefCell<DnaDrawing>>{
        &self.parent
    }
}