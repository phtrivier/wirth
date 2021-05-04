use raylib::prelude::*;

use simulator::Execution;

const INTERLINE: f32 = 20.0;

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
    #[structopt(short="m", name="max-cyles", default_value = "9999")]
    execution_max_cycles: u32,

    /// Stack base address when simulating process
    #[structopt(short="s", name="stack-base", long, default_value = "1000")]
    execution_stack_base: usize,

    /// Memory position to dump data from
    #[structopt(long, default_value = "30")]
    memory_dump_from: usize,

    /// Number of memory position to dump data
    #[structopt(long, default_value = "15")]
    memory_dump_count: usize,
    
    /// Debug mode
    #[structopt(short, long)]
    debug: bool,
}

fn main() {

    let opt = Opt::from_args();
    let filename = opt.input.into_os_string().into_string().expect("Filename is malformed.");

    let content = std::fs::read_to_string(filename).expect("Unable to read from input file.");

    let mut simulator;
    if opt.compile {
        simulator = simulator::Simulator::from_oberon(&content).unwrap();
    } else {
        simulator = simulator::Simulator::from_assembler(&content).unwrap();
    };

    let (mut rl, thread) = raylib::init()
        .resizable()
        .size(640 * 2, 480 * 2)
        .title("Wirth Simulator")
        .build();

    // https://en.wikipedia.org/wiki/Wikipedia:Zenburn
    let background = Color::from_hex("3F3F3F").unwrap();
    let foreground = Color::from_hex("DCDCCC").unwrap();

    let font = rl
        .load_font_ex(
            &thread,
            "fonts/DroidSansMono.ttf",
            32,
            FontLoadEx::Default(256),
        )
        .expect("couldn't load font");


    match simulator.execute(Execution{
        program_address: 0,
        max_cycles: opt.execution_max_cycles,
        stack_base: opt.execution_stack_base,
    }) {
        Ok(_) => (),
        Err(_) => ()
    };

    let size = font.base_size() as f32 / 2.0;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(background);

        // Draw registers
        d.draw_text_ex(
            &font,
            "Registers",
            Vector2::new(10.0, 20.0),
            32.0,
            1.0,
            foreground,
        );

        let mut y: f32 = 32.0;
        for (i, reg) in simulator.registers().iter().enumerate() {
            y = y + 1.5 * INTERLINE;
            let text = format!("REG {:02}: 0x{:04X} {:032b}", i, reg, reg);
            d.draw_text_ex(&font, &text, Vector2::new(10.0, y), size, 1.0, foreground);
        }

        // Draw memory
        d.draw_text_ex(
            &font,
            "Memory",
            Vector2::new(700.0, 20.0),
            32.0,
            1.0,
            foreground,
        );
        let mut y: f32 = 32.0;
        for (i, mem) in simulator.memory(opt.memory_dump_from, opt.memory_dump_from).iter().enumerate() {
            y = y + 1.5 * INTERLINE;
            let text = format!("MEM {:02}: 0x{:04X} {:032b} {:?}", i, mem, mem, mem);
            d.draw_text_ex(&font, &text, Vector2::new(700.0, y), size, 1.0, foreground);
        }
    }
}
