fn main() {
    println!("Hello, world!");
}

struct NNet {
    ws: Vec<f64>,
    bs: Vec<f64>,
    ns: usize,
}

impl NNet {
    
    pub fn new(ns: usize) -> NNet {
        let size = ns * 2;
        let ws: Vec<f64> = vec![0.0; size];
        let bs: Vec<f64> = vec![0.0; size];

        NNet { ws, bs, ns }
    }

    pub fn pt(self: &Self, x: usize, y: usize, z: usize) -> usize {
        match x {
            0 if z == 0 && y < self.ns => y,
            1 if y == 0 && z < self.ns => self.ns + z,
            _ => panic!("Invalid location: {}, {}, {}", x, y, z),
        }
    }
}