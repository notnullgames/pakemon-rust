use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Image, VectorFont},
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
    let mut time:f32 = 0.0;
    let mut step = 0;
    let image = Image::load(&gfx, "title.png").await?;
    let ttf = VectorFont::load("font.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 10.0)?;
    loop {
        time = time + 1.0;
        while let Some(_) = input.next_event().await {}

        gfx.clear(Color::BLACK);
        
        if step == 0 {
            let y = time % 40.0;
            if y == ( 39.0 ) {
                step = 1;
            }
            
            let region = Rectangle::new(Vector::new(80.0, y), image.size());
            gfx.draw_image(&image, region);
        }

        if step == 1 {
            let s = time % 60.0;
            let region = Rectangle::new(Vector::new(80.0, 40.0), image.size());
            gfx.draw_image(&image, region);
            if s > 20.0 {
                font.draw(
                    &mut gfx,
                    "START",
                    Color::WHITE,
                    Vector::new(140.0, 110.0),
                )?;
            }
        }

        gfx.present(&window)?;
    }
}