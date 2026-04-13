pub mod entities;
pub mod value_objects;

pub use entities::*;
pub use value_objects::*;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum AppState {
    #[default]
    Menu,
    TaskInput,
    Running,
    Paused,
}

pub struct App {
    pub frame_count: usize,
    pub should_quit: bool,
    pub state: AppState,
    pub phase: Phase,
    pub configs: Vec<SessionConfig>,
    pub selected_index: usize,
    pub timer: Timer,
    pub task_name: TaskName,
}

impl App {
    pub fn new() -> Self {
        let configs = vec![
            SessionConfig {
                label: String::from("Classic (25/5)"),
                work_duration_min: 25,
                break_duration_min: 5,
            },
            SessionConfig {
                label: String::from("Focus (50/10)"),
                work_duration_min: 50,
                break_duration_min: 10,
            },
            SessionConfig {
                label: String::from("Quick (15/5)"),
                work_duration_min: 15,
                break_duration_min: 5,
            },
        ];

        Self {
            frame_count: 0,
            should_quit: false,
            state: AppState::Menu,
            phase: Phase::Work,
            configs,
            selected_index: 0,
            timer: Timer::default(),
            task_name: TaskName::default(),
        }
    }

    pub fn next_option(&mut self) {
        if self.configs.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.configs.len();
    }

    pub fn prev_option(&mut self) {
        if self.configs.is_empty() {
            return;
        }
        if self.selected_index == 0 {
            self.selected_index = self.configs.len() - 1;
        } else {
            self.selected_index -= 1;
        }
    }

    pub fn enter_task_input(&mut self) {
        self.state = AppState::TaskInput;
    }

    pub fn start_session(&mut self) {
        let config = &self.configs[self.selected_index];
        self.phase = Phase::Work;
        self.timer.reset(config.work_duration_min * 60);
        self.state = AppState::Running;
    }

    pub fn tick(&mut self) {
        if self.state == AppState::Running && self.timer.tick() {
            self.transition_phase();
        }
    }

    fn transition_phase(&mut self) {
        let config = &self.configs[self.selected_index];
        match self.phase {
            Phase::Work => {
                self.phase = Phase::Break;
                self.timer.reset(config.break_duration_min * 60);
            }
            Phase::Break => {
                self.phase = Phase::Work;
                self.timer.reset(config.work_duration_min * 60);
            }
        }
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            AppState::Running => self.state = AppState::Paused,
            AppState::Paused => self.state = AppState::Running,
            _ => {}
        }
    }

    pub fn update_frame(&mut self) {
        self.frame_count += 1;
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
