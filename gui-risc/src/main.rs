use raylib::prelude::*;

use assembler;

const INTERLINE: f32 = 20.0;

fn main() {
    let (mut rl, thread) = raylib::init()
        .resizable()
        .size(640 * 2, 480 * 2)
        .title("Bye, World")
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

    // TODO(pht) load file instead
    // Assemble a program
    let mut a = assembler::Assembler::new();

    let program = "
    * A program that increments R0 until it's 3
    #FOO    3            ; The number of iterations
            MOVI R0,0,0  ; R0 <- 0
    @LOOP   ADDI R0,R0,1  ; R0 <- R0 + 1
            CMPI R0,#FOO ; If iteration done ?
    * Note that the loop is treated as a displacement
            BNE  @LOOP      
    @END    RET  R14    ; Exits since R14 is null in our case
    ";
    a.assemble(program).expect("Unable to parse program !");

    // Load instructions
    let mut c = risc::computer::Computer::new();
    c.load_instructions(a.instructions);
    c.execute(99, true);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(background);

        d.draw_text_ex(
            &font,
            "Registers",
            Vector2::new(10.0, 20.0),
            32.0,
            1.0,
            foreground,
        );

        let size = font.base_size() as f32 / 2.0;

        let mut i = 0;
        let mut y: f32 = 32.0;
        while i < 15 {
            y = y + 1.5 * INTERLINE;

            let reg = c.regs[i];
            let text = format!("REG {:02}: 0x{:04X} {:032b}", i, reg, reg);

            d.draw_text_ex(&font, &text, Vector2::new(10.0, y), size, 1.0, foreground);
            i = i + 1;
        }
    }
}
