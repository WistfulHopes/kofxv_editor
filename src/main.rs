use std::{env, fs::{File, self}, io::{Write}};

use binrw::BinReaderExt;
use cact::CharaActionData;
use editor::Editor;
use eframe::{egui::{self}, emath::Vec2};

mod cfile;
mod cact;
mod editor;

fn parse_cact(path: &String) -> std::io::Result<()> {
    let mut file = File::open(path).unwrap();
    let cact: CharaActionData = file.read_le().unwrap();

    let serialized = serde_json::to_string_pretty(&cact)?;
    let new_path = path.to_owned() + ".json";
    let mut output = File::create(new_path)?;
    write!(output, "{}", serialized)
}

fn rebuild_cact(path: &String) -> std::io::Result<()> {
    let mut cact: CharaActionData = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    cact.write(&mut buf);

    let new_path: String = path.clone().drain(0..path.len() - 5).collect();
    let mut output = File::create(new_path).unwrap();
    output.write_all(&buf)
}

fn help() {
    println!("Incorrect arguments!");
    println!("The first argument should be -p to parse, or -r to rebuild.");
    println!("If parsing, the second argument should be the path to the .cact file.");
    println!("Extract this from the character's CACT uasset with UAssetGUI.");
    println!("If rebuilding, the second argument should be the path to the .json file.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        3 => {
            let path = &args[2];
            if args[1] == "-p"
            {
                match parse_cact(path)
                {
                    Ok(()) => println!("Successfully parsed .cact file!"),
                    Err(e) => println!("Failed to parse .cact file! Error: {}", e)
                }
            }
            else if args[1] == "-r"
            {
                match rebuild_cact(path)
                {
                    Ok(()) => println!("Successfully rebuilt .cact file!"),
                    Err(e) => println!("Failed to rebuild .cact file! Error: {}", e)
                }
            }
            else {
                help()
            }
        }
        _ => {
            let options = eframe::NativeOptions {
                drag_and_drop_support: true,
                initial_window_size: Some(Vec2{x: 1280.0, y: 720.0}),
                ..Default::default()
            };
            eframe::run_native(
                "The King of Fighters XV - Editor",
                options,
                Box::new(|_cc| Box::new(KOFXVEditor::new(_cc))),
            ).expect("Failed to start GUI!");
        }
    }
}


fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "MS Gothic".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "C:\\WINDOWS\\FONTS\\MSGOTHIC.TTC"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "MS Gothic".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("MS Gothic".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

#[derive(Default)]
struct KOFXVEditor {
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
    success: bool,
    editor: Editor,
}

impl KOFXVEditor {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            dropped_files: Default::default(),
            picked_path: None,
            success: false,
            editor: Default::default(),
        }
    }
}

impl eframe::App for KOFXVEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    self.file_menu(ui)
                });
                let mut visuals = ui.ctx().style().visuals.clone();
                visuals.light_dark_radio_buttons(ui);
                ui.ctx().set_visuals(visuals);
            });    

            ui.label("Open from the File menu.");

            if let Some(picked_path) = &self.picked_path {
                if self.success == false{
                    ui.horizontal(|ui| {
                        ui.label("Failed to open file!
Make sure that your file is a valid CACT from THe King of Fighters XV.
CACT files from Samurai Shodown (2019) or The King of Fighters XIV are not compatible!
                        ");
                    });        
                }
                else {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });
                    self.editor.ui(ui);
                }
            }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                for file in &self.dropped_files {
                    let &path = &file.path.as_ref().unwrap();
                    self.success = self.editor.open_cact(path);
                    self.picked_path = Some(path.display().to_string());
                }
                self.dropped_files.clear();
            }
        });
    }
}

impl KOFXVEditor {
    fn file_menu(&mut self, ui: &mut egui::Ui) {
        if ui.button("Open").clicked() {
            if let Some(path) = rfd::FileDialog::new()
            .add_filter("CACT File", &["cact"])
            .pick_file() {
                self.success = self.editor.open_cact(&path);
                self.picked_path = Some(path.display().to_string());
            };
            ui.close_menu();
        }
        if ui.button("Save").clicked() {
            match self.editor.cact {
                Some(_) => {
                    if let Some(path) = rfd::FileDialog::new()
                    .add_filter("CACT File", &["cact"])
                    .save_file() {
                        self.success = self.editor.save_cact(&path);
                    };
                    ui.close_menu();    
                }
                None => ()
            }
        }
    }
}
