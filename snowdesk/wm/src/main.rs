use std::collections::VecDeque;
use std::thread;
use std::time::Duration;
use ui::aqua;

#[derive(Debug, Clone, Copy)]
enum Backend {
    X11,
    Wayland,
}

impl Backend {
    fn detect() -> Self {
        if std::env::var_os("WAYLAND_DISPLAY").is_some() {
            Backend::Wayland
        } else {
            Backend::X11
        }
    }
}

#[derive(Debug, Clone)]
struct Window {
    id: u32,
    title: String,
    focused: bool,
}

#[derive(Debug)]
enum WmEvent {
    OpenWindow { title: String },
    CloseWindow { id: u32 },
    ToggleExpose,
}

#[derive(Debug)]
struct Animation {
    label: &'static str,
    duration_ms: u16,
}

#[derive(Debug, Default)]
struct CompositorState {
    windows: Vec<Window>,
    expose_mode: bool,
    queue: VecDeque<WmEvent>,
    next_id: u32,
}

impl CompositorState {
    fn push_event(&mut self, event: WmEvent) {
        self.queue.push_back(event);
    }

    fn process(&mut self) {
        while let Some(event) = self.queue.pop_front() {
            match event {
                WmEvent::OpenWindow { title } => {
                    self.next_id += 1;
                    for w in &mut self.windows {
                        w.focused = false;
                    }
                    self.windows.push(Window {
                        id: self.next_id,
                        title,
                        focused: true,
                    });
                    self.animate("open-window", aqua::NORMAL_ANIMATION.duration_ms);
                }
                WmEvent::CloseWindow { id } => {
                    self.windows.retain(|w| w.id != id);
                    if let Some(last) = self.windows.last_mut() {
                        last.focused = true;
                    }
                    self.animate("close-window", aqua::QUICK_ANIMATION.duration_ms);
                }
                WmEvent::ToggleExpose => {
                    self.expose_mode = !self.expose_mode;
                    self.animate("expose-grid", aqua::NORMAL_ANIMATION.duration_ms);
                    self.print_expose_layout();
                }
            }
        }
    }

    fn animate(&self, label: &'static str, duration_ms: u16) {
        let anim = Animation { label, duration_ms };
        println!(
            "[wm] animation={} duration={}ms",
            anim.label, anim.duration_ms
        );
        println!(
            "[wm] blur+shadow pass scheduled with radius {:?}",
            aqua::WINDOW_CORNER_RADIUS
        );
    }

    fn print_expose_layout(&self) {
        if !self.expose_mode {
            println!("[wm] Exposé off");
            return;
        }
        let count = self.windows.len().max(1) as f32;
        let cols = count.sqrt().ceil() as usize;
        println!("[wm] Exposé on: {} windows -> {} columns", self.windows.len(), cols);
        for (idx, w) in self.windows.iter().enumerate() {
            let row = idx / cols;
            let col = idx % cols;
            println!("[wm] tile row={} col={} id={} title={}", row, col, w.id, w.title);
        }
    }
}

fn run_event_loop(backend: Backend, oneshot: bool) {
    let mut wm = CompositorState::default();
    wm.push_event(WmEvent::OpenWindow {
        title: "Finder".into(),
    });
    wm.push_event(WmEvent::OpenWindow {
        title: "Terminal".into(),
    });
    wm.push_event(WmEvent::ToggleExpose);
    wm.push_event(WmEvent::CloseWindow { id: 1 });
    wm.process();

    if oneshot {
        println!("[wm] oneshot mode complete");
        return;
    }

    println!("[wm] entering persistent session loop on {:?}", backend);
    loop {
        thread::sleep(Duration::from_secs(5));
        println!("[wm] heartbeat: session alive");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let oneshot = args.iter().any(|arg| arg == "--oneshot");
    let backend = if args.iter().any(|arg| arg == "--x11") {
        Backend::X11
    } else if args.iter().any(|arg| arg == "--wayland") {
        Backend::Wayland
    } else {
        Backend::detect()
    };

    println!("SnowDesk WM booting with backend: {:?}", backend);
    run_event_loop(backend, oneshot);
}
