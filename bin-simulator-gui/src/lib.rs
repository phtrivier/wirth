use egui::{self, RichText, FontId};
use eframe;
use std::cell::RefCell;

pub struct SimulatorGui{
    pub simulator: RefCell<simulator::Simulator>
}

pub const W : f32 = 1280.0;
pub const H : f32 = 720.0;

impl eframe::App for SimulatorGui {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let simulator = self.simulator.borrow();

        egui::CentralPanel::default().show(&ctx, |ui| {

            ui.horizontal_centered(|ui| {

                ui.vertical(|ui| {
                    ui.set_width(W / 3.0);

                    ui.vertical(|ui| {
                        ui.set_height(H * 0.85);
                        title(ui, "Code");

                        let mem = simulator.memory(0, 100);
                        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {

                            egui::Grid::new("code").num_columns(3).striped(true).show(ui, |grid| {
                                let mut i = 0;
                                while i < 100 {
                                    code_memory_row(grid, i, mem[i]);
                                    i = i + 1;
                                }
                            });

                        });

                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.set_height(H * 0.15);
                        title(ui, "Controls");
                    })

                });

                ui.separator();

                ui.vertical(|ui| {
                    ui.set_width(W / 3.0);
                    title(ui, "Registers");
                });

                ui.separator();

                ui.vertical(|ui| {
                    title(ui, "Memory");
                });

            });

            // ui.label("A shorter and more convenient way to add a label.");
            // if ui.button("Click me").clicked() {
            //     // take some action here
            // }
        });
    }

}

fn title(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).font(FontId::proportional(16.0)));
    ui.add_space(8.0);
}


fn code_memory_row(grid: &mut egui::Ui, i: usize, mem: i32) {
    mono_text(grid, &format!("{:04}", i));
    mono_text(grid, &format!("0b{:032b}", mem));
    mono_text(grid, &format!("0x{:04x}", mem));
    grid.end_row();
}

fn mono_text(ui: &mut egui::Ui, text: &str) {
    ui.label(RichText::new(text).font(FontId::monospace(13.0)));
}
