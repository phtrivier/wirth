use egui::{self, Color32, FontId, RichText};
use simulator::{Execution, Simulator};
use std::cell::RefCell;

pub struct SimulatorGui {
    pub simulator: RefCell<simulator::Simulator>,
    pub memory_dump_from: usize,
    pub memory_dump_count: usize,
}

pub const W: f32 = 1600.0;
pub const H: f32 = 900.0;

impl eframe::App for SimulatorGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut simulator = self.simulator.borrow_mut();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                program_column(ui, &simulator);

                ui.separator();

                memory_column(ui, &simulator, self);

                ui.separator();

                register_column(ui, &mut simulator);
            });
        });
    }
}

#[cfg(not(tarpaulin_include))]
fn program_column(ui: &mut egui::Ui, simulator: &Simulator) {
    ui.vertical(|ui| {
        ui.set_width(W * 0.33);

        ui.vertical(|ui| {
            ui.set_height(H * 0.85);
            title(ui, "Program");

            let mem = simulator.memory(0, 100);
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                egui::Grid::new("program").num_columns(3).striped(true).show(ui, |grid| {
                    let mut i = 0;
                    while i < 100 {
                        code_memory_row(grid, i, mem[i], simulator.pc());
                        i += 1;
                    }
                });
            });
        });
    });
}

#[cfg(not(tarpaulin_include))]
fn memory_column(ui: &mut egui::Ui, simulator: &Simulator, model: &SimulatorGui) {
    ui.vertical(|ui| {
        ui.set_width(W * 0.33);
        title(ui, "Memory");

        let mem = simulator.memory(model.memory_dump_from, model.memory_dump_count);
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            egui::Grid::new("code").num_columns(3).striped(true).show(ui, |grid| {
                let mut i = 0;
                while i < 100 {
                    main_memory_row(grid, model.memory_dump_from + i, mem[i]);
                    i += 1;
                }
            });
        });
    });
}

#[cfg(not(tarpaulin_include))]
fn register_column(ui: &mut egui::Ui, simulator: &mut Simulator) {
    ui.vertical(|ui| {
        ui.set_width(W * 0.30);
        title(ui, "Registers");

        let registers = simulator.registers();
        egui::Grid::new("registers").num_columns(3).striped(true).show(ui, |grid| {
            let mut i = 0;
            while i < 15 {
                register_row(grid, i, registers[i]);
                i += 1;
            }
        });

        ui.separator();

        ui.vertical(|ui| {
            ui.set_height(H * 0.15);
            title(ui, "Controls");

            ui.horizontal(|ui| {
                if ui.button("Next").clicked() {
                    simulator.execute_next();
                }

                if ui.button("Run").clicked() {
                    match simulator.execute(Execution {
                        max_cycles: 9999,
                        stack_base: 1000,
                    }) {
                        Ok(_) => {}
                        Err(_err) => {
                            // TODO(pht) report error somewhere
                            println!("Max execution reached");
                        }
                    }
                }
            })
        })
    });
}

#[cfg(not(tarpaulin_include))]
fn title(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).font(FontId::proportional(16.0)));
    ui.add_space(8.0);
}

#[cfg(not(tarpaulin_include))]
fn code_memory_row(grid: &mut egui::Ui, i: usize, mem: i32, pc: usize) {
    code_memory_text(grid, &format!("0x{:04}", i), i == pc);
    code_memory_text(grid, &format!("0b{:032b}", mem), i == pc);
    code_memory_text(grid, &format!("0x{:04x}", mem), i == pc);
    grid.end_row();
}
#[cfg(not(tarpaulin_include))]
fn main_memory_row(grid: &mut egui::Ui, i: usize, mem: i32) {
    mono_text(grid, &format!("0x{:04}", i));
    mono_text(grid, &format!("0b{:032b}", mem));
    mono_text(grid, &format!("0x{:04x}", mem));
    mono_text(grid, &format!("{}", mem));
    grid.end_row();
}
#[cfg(not(tarpaulin_include))]
fn register_row(grid: &mut egui::Ui, i: usize, register: i32) {
    mono_text(grid, &format!("R{:02}", i));
    mono_text(grid, &format!("0b{:032b}", register));
    mono_text(grid, &format!("0x{:04x}", register));
    mono_text(grid, &format!("{}", register));
    grid.end_row();
}
#[cfg(not(tarpaulin_include))]
fn code_memory_text(ui: &mut egui::Ui, text: &str, current: bool) {
    if current {
        ui.label(RichText::new(text).background_color(Color32::DARK_BLUE).color(Color32::WHITE).font(FontId::monospace(13.0)));
    } else {
        ui.label(RichText::new(text).font(FontId::monospace(13.0)));
    }
}
#[cfg(not(tarpaulin_include))]
fn mono_text(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).font(FontId::monospace(13.0)));
}
