use eframe::egui::{self, Vec2};
use egui::Painter;
use std::collections::LinkedList;

fn main() -> eframe::Result {
    let board = Board::new(
        Circle::new_blackhole(
            Board::X as f32 / 2.0,
            Board::Y as f32 / 2.0,
            50.0
        ),
    );
    let mut options = eframe::NativeOptions::default();
    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([Board::X as f32, Board::Y as f32]);

    eframe::run_native(
        "App",
        options,
        Box::new(|_cc| Ok(Box::new(board))),
    )
}

enum CircleType {
    Normal,
    BlackHole,
}

struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    _type: CircleType,
    trail: LinkedList<(f32, f32)>,
    dx: f32,
    dy: f32,
    alive: bool,

}

impl Circle {
    fn new(x: f32, y: f32, radius: f32) -> Self {
        Self { x, y, radius, _type: CircleType::Normal, trail: LinkedList::new(), dx: 1.0, dy: 0.0, alive: true }
    }

    fn new_blackhole(x: f32, y: f32, radius: f32) -> Self {
        Self { x, y, radius, _type: CircleType::BlackHole, trail: LinkedList::new(), dx: 0.0, dy: 0.0, alive: true }
    }

    fn get_color(&self) -> egui::Color32 {
        match self._type {
            CircleType::Normal => egui::Color32::WHITE,
            CircleType::BlackHole => egui::Color32::BLACK
        }
    }
}

impl Component for Circle {
    fn draw(&self, painter: &mut Painter) {
        painter.circle_filled(
            egui::pos2(self.x, self.y),
            self.radius,
            self.get_color(),
        );

        let mut alpha = 1.0;
        for (tx, ty) in self.trail.iter().rev() {
            alpha = alpha * 0.95; // Fade out the trail
            if alpha < 0.01 {
                break; // Stop drawing if the trail is too faint
            }

            painter.circle_filled(
                egui::pos2(*tx, *ty),
                self.radius,
                egui::Color32::WHITE.linear_multiply(alpha as f32),
            );

        }

    }

    fn update(&mut self, blackhole: &Circle) {
        if let CircleType::BlackHole = self._type {
            return
        }
        self.trail.push_back((self.x, self.y));
        if self.trail.len() > 50 {
            self.trail.pop_front();
        }

        let (dx, dy) = (blackhole.x - self.x, blackhole.y - self.y);
        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt();

        const MASS: f32 = 100.0;
        let rs = blackhole.radius;

        if dist < rs {
            self.alive = false;
            return;
        }

        let pull_strength = MASS / dist_sq;

        let pull_x = pull_strength * dx / dist;
        let pull_y = pull_strength * dy / dist;

        self.dx += pull_x;
        self.dy += pull_y;

        let curr_speed = (self.dx * self.dx + self.dy * self.dy).sqrt();

        self.dx = self.dx / curr_speed;
        self.dy = self.dy / curr_speed;
        self.x += self.dx;
        self.y += self.dy;

    }

    fn keep(&self) -> bool {
        if !self.alive {
            return false;
        }

        self.x <= Board::X as f32 && self.x >= 0.0 && self.y <= Board::Y as f32 && self.y >= 0.0
    }
}

trait Component {
    fn draw(&self, painter: &mut Painter);
    fn update(&mut self, blackhole: &Circle) { }
    fn keep(&self) -> bool { true }
}

struct Board{
    obj: Vec<Box<dyn Component>>,
    blackhole: Circle,
    last_generate: f64,
    pixels: Vec<egui::Color32>,
}

impl Component for Box<dyn Component> {
    fn draw(&self, painter: &mut Painter) {
        (**self).draw(painter);
    }


    fn update(&mut self, blackhole: &Circle) {
        (**self).update(blackhole);
    }

    fn keep(&self) -> bool {
        (**self).keep()
    }
}

impl Board
{
    const GENERATE_INTERVAL: f64 = 0.02;
    const X: f64 = 1024.0;
    const Y: f64 = 768.0;

    fn new(blackhole: Circle) -> Self {
        Self { blackhole, obj: Vec::new(), last_generate: 0.0, pixels: vec![egui::Color32::RED; (Board::X * Board::Y) as usize] }
    }

    fn add(&mut self, o: Box<dyn Component>) {
        self.obj.push(o);
    }

    fn update(&mut self) {
        for o in self.obj.iter_mut() {
            o.update(&self.blackhole);
            // Update logic for each object if needed
        }
    }

    fn generate(&mut self, time: f64) {
        if time - self.last_generate > Self::GENERATE_INTERVAL {
            // Generate new objects or update existing ones based on time
            let random_y: f64 = rand::random::<f64>() * Self::Y; // Random y position
            self.add(
                Box::new(Circle::new(0.0, random_y as f32, 0.5))
            );
            self.last_generate = time;
        }
    }

    fn render(&mut self, time: f64, painter: &mut Painter) {
        self.obj.retain(|o| o.keep());

        // Filter out
        self.generate(time);
        self.update();

        self.blackhole.draw(painter);
        for o in self.obj.iter() {
            o.draw(painter);
        }
    }
}

impl eframe::App for Board {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let image = egui::ColorImage {
                size: [Board::X as usize, Board::Y as usize],
                source_size: Vec2::new(Board::X as f32, Board::Y as f32),
                pixels: self.pixels,
            };
            
            ctx.request_repaint();

        });
    }
}
