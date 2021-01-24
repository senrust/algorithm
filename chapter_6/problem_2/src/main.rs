struct Positon {
    x: f64,
    y: f64,
}

impl Positon {
    fn rotate(&self, center: &Positon, deg: f64) -> Positon{
        let rad = deg.to_radians();
        let x = self.x - center.x;
        let y = self.y - center.y;
        let rotate_x = x*rad.cos() - y*rad.sin() + center.x;
        let rotate_y = x*rad.sin() + y*rad.cos() + center.y;
        Positon {
            x: rotate_x,
            y: rotate_y,
        }
    }

    fn trichotomize(&self, base: &Positon) -> (Positon, Positon) {
        let first_position = Positon{x: (self.x - base.x)/3.0 + base.x, 
                y: (self.y - base.y)/3.0 + base.y,};
        let second_position = Positon{x: (self.x - base.x)*2.0/3.0 + base.x, 
            y: (self.y - base.y)*2.0/3.0 + base.y,};
        (first_position, second_position)
    }
}


fn konh(depth: i32, p1: &Positon, p2:&Positon) {
    if depth == 0 {
        return;
    }
    let (s, t) = p2.trichotomize(p1);
    let u = t.rotate(&s, 60.);
    konh(depth -1, p1, &s);
    println!("{:.3} {:.3}", s.x, s.y);
    konh(depth -1, &s, &u);
    println!("{:.3} {:.3}", u.x, u.y);
    konh(depth -1, &u, &t);
    println!("{:.3} {:.3}", t.x, t.y);
    konh(depth -1, &t, p2);
}

fn main() {
    let p1 = Positon{x:0.0 , y:0.0};
    let p2 = Positon{x:100.0 , y:0.0};
    let depth = input::read_number();
    println!("{:.3} {:.3}", p1.x, p1.y);
    konh(depth, &p1, &p2);
    println!("{:.3} {:.3}", p2.x, p2.y);
}
