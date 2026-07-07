use cli_clipboard::{ClipboardContext, ClipboardProvider};
use eframe::egui;
use eframe::egui::Color32;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

use crate::CachedUserList;
use crate::NO_BOT_TOKEN;
use crate::parse_file;

pub struct LookupApp<'a> {
    current_display_list: String,
    picked_file: Option<PathBuf>,
    user_list_dialog: FileDialog,
    last_process_result: &'a str,
    last_process_color: Color32,
    bot_token: String,
    cached_list: CachedUserList,
}

impl eframe::App for LookupApp<'_> {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Discord User Lookup App");
            });
            if self.bot_token == NO_BOT_TOKEN {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Missing Bot Token! Cannot contact the Discord API in this state.\nPlease create a file named BOT_TOKEN.txt with a proper Discord Bot Token.").color(Color32::RED).underline());
                });
            }
            ui.add_space(30.0);
            // Use columns for spacing
            ui.columns_const(|[col_1, col_2, _col_3]| {
                col_1.label("Input");
                col_1.separator();
                col_1.vertical_centered_justified(|ui| {
                    ui.label("Select a CSV file");
                });
                col_1.add_space(15.0);
                col_1.vertical_centered(|ui| {
                    if ui.button("Pick a file").clicked() {
                        self.user_list_dialog.pick_file();
                    }
                    let label = match &self.picked_file {
                        Some(val) => val
                            .file_name()
                            .map_or("Failed to read file name", |filename| {
                                filename.to_str().unwrap_or("Failed to decode file name")
                            }),
                        None => "Nothing",
                    };
                    ui.label(format!("Currently opened: {}", label));
                    self.user_list_dialog.update(ui);

                    if let Some(path) = self.user_list_dialog.take_picked() {
                        self.picked_file = Some(path.to_path_buf());
                    }
                    ui.add_space(40.0);
                    if ui.button("Process").clicked() {
                        // Attempt to process the file
                        match &self.picked_file {
                            Some(file) => {
                                // TODO: This can fail. Change to result and handle appropriately
                                let result = parse_file(&mut self.cached_list, &self.bot_token, file);
                                self.current_display_list = result.into_iter().collect::<Vec<String>>().join("\n");
                                self.last_process_color = Color32::GREEN;
                                self.last_process_result = "Success!";
                            }
                            None => {
                                self.last_process_color = Color32::WHITE;
                                self.last_process_result = "Please choose a file";
                            }
                        }

                    }
                    ui.label(egui::RichText::new(self.last_process_result).color(self.last_process_color));
                });

                col_2.vertical_centered_justified(|ui| {
                    ui.label("Results");
                    ui.separator();
                    ui.add_space(40.0);
                    ui.label(&self.current_display_list);
                    if &self.current_display_list != &"Nothing so far..." {
                        if ui.button("Copy to Clipboard").clicked() {
                            let mut ctx = ClipboardContext::new().unwrap();
                            ctx.set_contents(self.current_display_list.to_owned()).unwrap();
                        }
                    }
                });
            });
        });
    }
}

impl LookupApp<'_> {
    pub fn new(bot_token: String) -> Self {
        Self {
            current_display_list: "Nothing so far...".to_owned(),
            picked_file: None,
            user_list_dialog: FileDialog::new(),
            bot_token: bot_token,
            last_process_result: "",
            last_process_color: Color32::GREEN,
            cached_list: CachedUserList::new(),
        }
    }
}
