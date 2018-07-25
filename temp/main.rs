fn offscreen_test(){
    let surface = Surface::new(100, 100, PixelFormatEnum::RGB24).unwrap();

    let mut canvas = surface.into_canvas().unwrap();


    //-------------------------------------------------------------
    //使用纹理
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(texture_creator.default_pixel_format(), 100, 100)
        .unwrap();
    canvas.with_texture_canvas(&mut texture, |texture_canvas| {
        texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        texture_canvas.fill_rect(Rect::new(0, 0, 50, 50)).unwrap();
    }).unwrap();
    canvas.copy(&texture, None, None).expect("Render failed");
    //-------------------------------------------------------------

    //普通画布
    // canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
    // canvas.fill_rect(Rect::new(0, 0, 50, 50)).unwrap();
    
    canvas.present();

    println!("{:?}", canvas.output_size());

    //let image = canvas.read_pixels(Rect::new(0, 0, 512, 512), PixelFormatEnum::RGB888).unwrap();
    canvas.surface().save_bmp("test.bmp").unwrap();
}

fn test_window(){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("evolisa", 800, 600)
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        {
            // Update the window title.
            let window = canvas.window_mut();

            let position = window.position();
            let size = window.size();
            let title = format!("Window - pos({}x{}), size({}x{}): {}",
                                position.0,
                                position.1,
                                size.0,
                                size.1,
                                tick);
            window.set_title(&title).unwrap();

            tick += 1;
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.filled_polygon(&[100, 200, 0], &[0, 0, 100], Color::RGBA(255, 0, 0, 255)).unwrap();

        canvas.present();
    }
}