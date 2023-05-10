use simulator::Execution;

use gtk::prelude::*;
use relm4::prelude::*;

use std::path::PathBuf;
use structopt::StructOpt;

/// Load a binary file (compiled from assembly or oberon-0) and run it in the risc computer
#[derive(StructOpt, Debug)]
#[structopt(name = "cli-risc", version = "0.0.1")]
struct Opt {
    /// Assembly language file
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,

    /// Treat input as Oberon-0 code, and compile it before execution
    #[structopt(short, long)]
    compile: bool,

    /// Maximum number of cycles to run before failing execution
    #[structopt(short = "m", name = "max-cyles", default_value = "9999")]
    execution_max_cycles: u32,

    /// Stack base address when simulating process
    #[structopt(short = "s", name = "stack-base", long, default_value = "1000")]
    execution_stack_base: usize,

    /// Memory position to dump data from
    #[structopt(long, default_value = "1000")]
    memory_dump_from: usize,
    // / Number of memory position to dump data
    // #[structopt(long, default_value = "15")]
    // memory_dump_count: usize,

    // / Debug mode
    // #[structopt(short, long)]
    // debug: bool,
}

struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}


#[relm4::component]
impl SimpleComponent for AppModel {

    type Init = u8;

    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Wirth Simulator"),
            set_default_width: 1280,
            set_default_height: 720,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,
                set_margin_all: 5,
                set_homogeneous: true,

                // Code column
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Frame {
                        set_label: Some("Code"),
                        set_vexpand: true,
                        gtk::Box {
                            gtk::Label {
                                set_label: "TODO"
                            }
                        }

                    },

                    gtk::ActionBar {
                        pack_start = &gtk::Box {
                            set_spacing: 8,
                            set_orientation: gtk::Orientation::Horizontal,

                            gtk::Button {
                                set_label: "Step",
                                connect_clicked[sender] => move |_| {
                                    println!("TODO: Step")
                                },
                            },

                            gtk::Button {
                                set_label: "Run",
                                connect_clicked[sender] => move |_| {
                                    println!("TODO: Run")
                                }
                            }
                        }
                    }
                },

                // Registers column
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Frame {
                        set_label: Some("Registers"),
                        set_vexpand: true,
                        gtk::Box {
                            gtk::Label {
                                set_label: "TODO"
                            }
                        }

                    }
                },

                // Memory column
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Frame {
                        set_label: Some("Memory"),
                        set_vexpand: true,
                        gtk::Box {
                            gtk::Label {
                                set_label: "TODO"
                            }
                        }

                    }
                }


                /*
                gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    }
                },

                gtk::Button::with_label("Decrement") {
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Decrement);
                    }
                },

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                    set_margin_all: 5,
                }
                */
            }
        }
    }

    // Initialize the UI.
    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel { counter };

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}



#[cfg(not(tarpaulin_include))]
fn main() {
    let opt = Opt::from_args();
    let filename = opt.input.into_os_string().into_string().expect("Filename is malformed.");

    let content = std::fs::read_to_string(filename).expect("Unable to read from input file.");

    let mut sim = if opt.compile {
        simulator::Simulator::from_oberon(&content).unwrap()
    } else {
        simulator::Simulator::from_assembler(&content).unwrap()
    };

    let app = RelmApp::new("wirth.simulator.gui");
    app.run::<AppModel>(0);

    /*
    let (mut rl, thread) = raylib::init().resizable().size(640 * 2, 480 * 2).title("Wirth Simulator").build();

    // // https://en.wikipedia.org/wiki/Wikipedia:Zenburn
    let background = Color::from_hex("3F3F3F").unwrap();
    let foreground = Color::from_hex("DCDCCC").unwrap();

    let font = rl
        .load_font_ex(&thread, "fonts/DroidSansMono.ttf", 32, FontLoadEx::Default(256))
        .expect("couldn't load font");

    match sim.execute(Execution {
        program_address: 0,
        max_cycles: opt.execution_max_cycles,
        stack_base: opt.execution_stack_base,
    }) {
        Ok(_) => {}
        Err(err) => {
            println!("Error executing simulator code {:?}", err);
        }
    };

    let size = font.base_size() as f32 / 2.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(background);

        // Draw registers
        d.draw_text_ex(&font, "Registers", Vector2::new(10.0, 20.0), 32.0, 1.0, foreground);

        let mut y: f32 = 32.0;
        for (i, reg) in sim.registers().iter().enumerate() {
            y += 1.5 * INTERLINE;
            let text = format!("REG {:02}: 0x{:04X} {:032b}", i, reg, reg);
            d.draw_text_ex(&font, &text, Vector2::new(10.0, y), size, 1.0, foreground);
        }

        // Draw memory
        d.draw_text_ex(&font, "Memory", Vector2::new(700.0, 20.0), 32.0, 1.0, foreground);
        let mut y: f32 = 32.0;
        for (i, mem) in sim.memory(opt.memory_dump_from, opt.memory_dump_from).iter().enumerate() {
            y += 1.5 * INTERLINE;
            let text = format!("MEM {:02}: 0x{:04X} {:032b} {:?}", i, mem, mem, mem);
            d.draw_text_ex(&font, &text, Vector2::new(700.0, y), size, 1.0, foreground);
        }
    }
*/
}
