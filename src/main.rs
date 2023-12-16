mod core;

use eframe::egui;
use egui::UserAttentionType;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SplitJoin",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {
    picked_path: Option<String>,
    filesize: FileSize,
    size: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum FileSize {
    KiloBytes,
    #[default]
    MegaBytes,
    GigaBytes,
}

fn repr(filesize: FileSize) -> String {
    format!("{filesize:?}")
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Pick a file.");
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.picked_path = Some(path.display().to_string());
                    }
                }
            });

            if let Some(picked_path) = &self.picked_path {
                ui.monospace(picked_path);

                ui.separator();
                ui.horizontal(|ui| {
                    egui::ComboBox::new("filesize", "")
                        .selected_text(repr(self.filesize))
                        .show_ui(ui, |ui| {
                            for kind in [
                                FileSize::MegaBytes,
                                FileSize::GigaBytes,
                                FileSize::KiloBytes,
                            ] {
                                ui.selectable_value(&mut self.filesize, kind, repr(kind));
                            }
                        });
                        ui.text_edit_singleline(&mut self.size);
                        if ui.button("Split").clicked() {}
                });
                ui.separator();
                if ui.button("Join").clicked() {}
            }
        });
    }
}

// println!("Shifted 0: {}", shift_by('.', "filename.zip", 0));
// println!("Shifted 1: {}", shift_by('.', "filename.zip.part1", 1));
// println!("Shifted 2: {}", shift_by('.', "filename.zip.part1.split", 2));

// let path = Path::new("C:/Users/Clark/Desktop/files/hjsplit.exe");
// split(path, FileSize::of_kilo_bytes(40)).unwrap();

// let base = Path::new("C:/Users/Clark/Desktop/files");
// join(base);

#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
}
