pub struct App {
    pub frame_count: usize,
    pub should_quit: bool,
    pub work_duration: u32,
    pub break_duration: u32,
    pub progress: u16, // 0 to 100
}

impl App {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            should_quit: false,
            work_duration: 25,
            break_duration: 5,
            progress: 40, // Static example for now
        }
    }

    pub fn update_animation(&mut self) {
        self.frame_count += 1;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
