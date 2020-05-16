use piston_window::*;
use rand::*;

const HEIGHT: f64 = 720.0;
const WIDTH: f64 = 1280.0;

struct Bubble {
  speed: f64,
  radius: f64,
  x: f64,
  y: f64,
}

impl Bubble {
  pub fn new(num: Option<f64>) -> Bubble {
    let radius = (random::<f64>() * (WIDTH / 8.0)) + 5.0;

    let mut bubble: Bubble = Bubble {
      speed: (random::<f64>() * 90.0) + 10.0,
      x: random::<f64>() * WIDTH,
      y: random::<f64>() * (HEIGHT + radius),
      radius: radius,
    };

    if let Some(y) = num {
      bubble.speed = 0.0;
      bubble.y = y
    }

    return bubble;
  }
}

fn get_bubbles() -> Vec<Bubble> {
  let mut bubbles = Vec::new();
  let count = (random::<u64>() % 15) + 10;
  for _ in 0..count {
    bubbles.push(Bubble::new(Some(HEIGHT)));
    bubbles.push(Bubble::new(Some(0.0)));
    bubbles.push(Bubble::new(None));
  }

  bubbles
}

fn main() {
  let bubble_color = [1.0, 97.0 / 255.0, 0.0, 1.0];
  let bg_color = [104.0 / 255.0, 221.0 / 255.0, 19.0 / 255.0, 1.0];
  let mut bubbles: Vec<Bubble> = get_bubbles();

  let mut window: PistonWindow =
    WindowSettings::new("Lava Lamp", [WIDTH, HEIGHT])
      .exit_on_esc(true)
      .build()
      .unwrap();

  let mut events = window.events;
  while let Some(e) = events.next(&mut window) {
    if let Some(_) = e.render_args() {
      let bubbs = &bubbles;
      window.draw_2d(&e, |context, graphics, _| {
        clear(bg_color, graphics);

        for bubble in bubbs {
          ellipse(
            bubble_color,
            [
              bubble.x - bubble.radius,
              bubble.y - bubble.radius,
              bubble.radius * 2.0,
              bubble.radius * 2.0,
            ],
            context.transform,
            graphics,
          )
        }
      });
    };

    if let Some(update) = e.update_args() {
      let bubbs = &mut bubbles;
      for bubble in bubbs {
        bubble.y -= bubble.speed * update.dt;
        if bubble.y + bubble.radius <= 0.0 {
          bubble.y = HEIGHT + bubble.radius
        }
      }
    }
  }
}
