use niri_ipc::Action::ScreenshotScreen;
use sdl3::event::Event;
use sdl3::image::LoadTexture;
use sdl3::keyboard::Keycode;
use std::path::Path;

fn main()  {
    let mut tempdir = std::env::temp_dir();
    tempdir.push("whatever");

    let amoguspath = tempdir.as_path().to_str().expect("amogus is sussy").to_string();

    let amogus = ScreenshotScreen { write_to_disk: true, show_pointer: false, path: Some(amoguspath) };

    let mut socket = niri_ipc::socket::Socket::connect().expect("Failed talking to niri socket");

    socket.send(niri_ipc::Request::Action(amogus)).expect("whatever?").expect("fucking i dont know");

    sdl_run(tempdir.as_path()).expect("whate the sigma");
}


fn sdl_run(png: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl3::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl3 demo: Video", 3440, 1440)
        .position_centered()
        .fullscreen()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(png)?;

    canvas.copy(&texture, None, None)?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}
