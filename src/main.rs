use niri_ipc::Action::ScreenshotScreen;
use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::render::FRect;
use std::collections::HashMap;
use zbus::zvariant::OwnedObjectPath;
use zbus::zvariant::Value;
use zbus::{Connection, proxy};

const MOVEMENT_SPEED: f32 = 0.5;

#[proxy(
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop",
    interface = "org.freedesktop.portal.Screenshot"
)]
trait ScreenshotPortal {
    async fn screenshot(
        &self,
        parent_window: &str,
        options: HashMap<String, Value<'static>>,
    ) -> zbus::Result<OwnedObjectPath>;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection = zbus::Connection::session().await?;
    // busctl --user call org.freedesktop.portal.Desktop /org/freedesktop/portal/desktop org.freedesktop.portal.Screenshot Screenshot sa{sv} '' 2 handle_token s portal1894239812 interactive b false

    let proxy = ScreenshotPortalProxy::new(&connection).await?;
    let mut options: HashMap<String, Value<'static>> = HashMap::new();
    options.insert(
        "handle_token".to_string(),
        Value::from("portal1894239812".to_string()),
    );
    options.insert("interactive".to_string(), Value::from(false));
    let return_skibidi = proxy.screenshot("", options).await?;
    println!("{return_skibidi}");

    // busctl --user wait :1.28 /org/freedesktop/portal/desktop/request/1_715/portal1894239812 org.freedesktop.portal.Request Response

    let mut tempdir = std::env::temp_dir();
    tempdir.push("whatever");

    let amoguspath = tempdir
        .as_path()
        .to_str()
        .expect("amogus is sussy")
        .to_string();

    let amogus = ScreenshotScreen {
        write_to_disk: true,
        show_pointer: false,
        path: Some(amoguspath.clone()),
    };

    let mut socket = niri_ipc::socket::Socket::connect().expect("Failed talking to niri socket");

    socket
        .send(niri_ipc::Request::Action(amogus))
        .expect("whatever?")
        .expect("fucking i dont know");

    let sdl_context = sdl3::init()?;

    let video_subsystem = sdl_context.video()?;
    let mouse_subsystem = sdl_context.mouse();

    let display_mode = video_subsystem.get_primary_display()?.get_mode()?;

    let window = video_subsystem
        .window(
            "Example Renderer Scaling & Moving Textures",
            display_mode.w as u32,
            display_mode.h as u32,
        )
        .position_centered()
        .input_grabbed()
        .build()?;

    mouse_subsystem.relative_mouse_mode(&window);
    mouse_subsystem.warp_mouse_in_window(
        &window,
        display_mode.w as f32 / 2.0,
        display_mode.h as f32 / 2.0,
    );
    mouse_subsystem.capture(true);

    // x is (x_aspect) times bigger than y
    let x_aspect: f32 = display_mode.w as f32 / display_mode.h as f32;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .load_texture(amoguspath)
        .expect("Failed loading texture");

    let texture_width = texture.width();
    let texture_height = texture.height();

    let mut event_pump = sdl_context.event_pump()?;
    let mut wheel = 0.0;
    let mut offset_x_movement = 0.0;
    let mut offset_y_movement = 0.0;
    let mut holding_mouse = false;

    loop {
        for event in event_pump.poll_iter() {
            if let Event::MouseMotion { xrel, yrel, .. } = event {
                offset_x_movement -= xrel;
                offset_y_movement -= yrel;
            }

            if let Event::MouseWheel { y, .. } = event {
                if y >= 0.0 {
                    wheel += 0.05;
                } else if (wheel - 0.05) > 0.0 {
                    wheel -= 0.05;
                } else {
                    wheel = 0.0;
                }
            }

            if let Event::MouseButtonUp { .. } = event {
                holding_mouse = false;
            }
            if let Event::MouseButtonDown { .. } = event {
                holding_mouse = true;
            }

            if let Event::Quit { .. } = event {
                return Ok(());
            }
        }

        // Determine direction and scale
        let scale = 1.0 + wheel;

        // Calculate horizontal movement
        // let movement = ((now % 4000) as f32 / 4000.0) * display_mode.w as f32;

        // Clear the canvas
        canvas.set_draw_color(sdl3::pixels::Color::RGB(0, 0, 0));
        canvas.clear();

        // Calculate the destination rectangle with scaling and horizontal movement
        let width = texture_width as f32 * scale;
        let height = texture_height as f32 * scale;

        // Start at 0, dont move if USER scaling is none
        // When zooming, make sure that we are zooming towards the center and offsetting by the opposite of the offset that the user has from when they first clicked on the screen so that when they move their cursor the focus goes towards it
        // Also ensure that it follows the aspect ratio of the screen
        let x = wheel
            * (offset_x_movement * MOVEMENT_SPEED
                - ((texture_width as f32 / 2.0)
                    - (offset_x_movement / (2.0 * x_aspect)) * x_aspect));
        let y = wheel
            * (offset_y_movement * MOVEMENT_SPEED
                - ((texture_height as f32 / 2.0) - (offset_y_movement / 2.0) * 1.0));

        let dst_rect = FRect::new(x, y, width, height);

        // Render the texture
        canvas.copy(&texture, None, Some(dst_rect))?;

        // Present the updated canvas
        canvas.present();
    }
}
