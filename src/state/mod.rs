use ratatui::widgets::canvas::Circle;

pub struct AppState {
    objects: Vec<Object>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self) {
        let o = Object::new();
        self.objects.push(o);
    }

    pub fn objects_to_circle(&self) -> Vec<Circle> {
        self.objects
            .iter()
            .map(|o| Circle {
                x: o.x,
                y: o.y,
                radius: o.r,
                color: ratatui::style::Color::LightGreen,
            })
            .collect()
    }

    pub fn update(&mut self) {
        self.grow_radius();
        self.prune();
    }

    fn grow_radius(&mut self) {
        for o in self.objects.iter_mut() {
            o.r += 1.0;
        }
    }

    fn prune(&mut self) {
        self.objects.retain(|o| o.r <= 200.0);
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

struct Object {
    x: f64,
    y: f64,
    r: f64,
}

impl Object {
    pub fn new() -> Self {
        Object {
            x: 100.0,
            y: 100.0,
            r: 5.0,
        }
    }
}
