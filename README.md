d7engine
A project by Markus Dick
d7engine is a homemade games engine for fun.

Basic setup:

use d7engine::prelude::*;

struct Runt {
}

impl Runtime for Runt {
    fn load(&mut self, _camera: &mut Camera) {
        
    }

    fn inputs(&mut self, _event: Event) {
        
    }

    fn draw(&mut self, _delta: f32, _camera: &mut Camera, _mouse: &Mouse) {
       
    }
}

fn main() {
    let mut runt = Runt{};

    init(Config::default(), &mut runt);
}
