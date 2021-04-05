use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Color,
    run, Graphics, Input, Result, Settings, Window,
};

fn main() {
    run(
        Settings {
            title: "Pakémon",
            size: Vector::new(320.0, 240.0),
            ..Settings::default()
        },
        app,
    );
}

async fn app(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    gfx.clear(Color::BLACK);
    let rect = Rectangle::new(Vector::new(10.0, 10.0), Vector::new(100.0, 100.0));
    gfx.fill_rect(&rect, Color::BLUE);
    gfx.stroke_rect(&rect, Color::RED);
    gfx.present(&window)?;
    
    loop {
        while let Some(_) = input.next_event().await {}
    }
}