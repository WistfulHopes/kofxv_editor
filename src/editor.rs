use std::{path::PathBuf, fs::File, io::Write};
use binrw::BinReaderExt;

use eframe::egui::{ComboBox, self, Response};

use crate::cact::{CharaActionData, Line, EffectType, SoundType};

pub struct Editor {
    pub cact: Option<CharaActionData>,
    selected: String,
    action_index: usize,
    current_frame: i32,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            cact: None,
            selected: Default::default(),
            action_index: 0,
            current_frame: 0,
        }
    }
}

pub fn render_line(ui: &mut egui::Ui, line: &mut Line) {
    match line {
        Line::BaseAnime(base_anime) => {
            ui.horizontal(|ui| {
                ui.label("Base Anime");
            });
            ui.horizontal(|ui| {
                ui.label("Motion File ID");
                let motion_file_id = &mut base_anime.motion_file_id.to_string();
                ui.text_edit_singleline(motion_file_id);
                base_anime.motion_file_id = motion_file_id.parse::<i32>().unwrap_or(base_anime.motion_file_id);
            });
            ui.horizontal(|ui| {
                ui.label("Motion ID");
                let motion_id = &mut base_anime.motion_id.to_string();
                ui.text_edit_singleline(motion_id);
                base_anime.motion_id = motion_id.parse::<i32>().unwrap_or(base_anime.motion_id);
            });
            ui.horizontal(|ui| {
                ui.label("Motion Frame");
                let motion_frame = &mut base_anime.motion_frame.to_string();
                ui.text_edit_singleline(motion_frame);
                base_anime.motion_frame = motion_frame.parse::<f32>().unwrap_or(base_anime.motion_frame);
            });
            ui.horizontal(|ui| {
                ui.label("Blend");
                let blend = &mut base_anime.blend.to_string();
                ui.text_edit_singleline(blend);
                base_anime.blend = blend.parse::<f32>().unwrap_or(base_anime.blend);
            });
            ui.horizontal(|ui| {
                ui.label("Transparent");
                let transparent = &mut base_anime.transparent.to_string();
                ui.text_edit_singleline(transparent);
                base_anime.transparent = transparent.parse::<f32>().unwrap_or(base_anime.transparent);
            });
            ui.horizontal(|ui| {
                ui.label("Unknown 1");
                let unk1 = &mut base_anime.unk1.to_string();
                ui.text_edit_singleline(unk1);
                base_anime.unk1 = unk1.parse::<f32>().unwrap_or(base_anime.unk1);
            });
            ui.horizontal(|ui| {
                ui.label("Unknown 2");
                let unk2 = &mut base_anime.unk2.to_string();
                ui.text_edit_singleline(unk2);
                base_anime.unk2 = unk2.parse::<u32>().unwrap_or(base_anime.unk2);
            });
            ui.horizontal(|ui| {
                ui.label("Unknown 3");
                let unk3 = &mut base_anime.unk3.to_string();
                ui.text_edit_singleline(unk3);
                base_anime.unk3 = unk3.parse::<u32>().unwrap_or(base_anime.unk3);
            });            
            ui.horizontal(|ui| {
                ui.label("Unknown 4");
                let unk4 = &mut base_anime.unk4.to_string();
                ui.text_edit_singleline(unk4);
                base_anime.unk4 = unk4.parse::<f32>().unwrap_or(base_anime.unk4);
            });
        }
        Line::FaceAnime(face_anime) => {
            ui.horizontal(|ui| {
                ui.label("Face Anime");
            });
            ui.horizontal(|ui| {
                ui.label("Motion File ID");
                let motion_file_id = &mut face_anime.motion_file_id.to_string();
                ui.text_edit_singleline(motion_file_id);
                face_anime.motion_file_id = motion_file_id.parse::<i32>().unwrap_or(face_anime.motion_file_id);
            });
            ui.horizontal(|ui| {
                ui.label("Motion ID");
                let motion_id = &mut face_anime.motion_id.to_string();
                ui.text_edit_singleline(motion_id);
                face_anime.motion_id = motion_id.parse::<i32>().unwrap_or(face_anime.motion_id);
            });
            ui.horizontal(|ui| {
                ui.label("Motion Frame");
                let motion_frame = &mut face_anime.motion_frame.to_string();
                ui.text_edit_singleline(motion_frame);
                face_anime.motion_frame = motion_frame.parse::<f32>().unwrap_or(face_anime.motion_frame);
            });
            ui.horizontal(|ui| {
                ui.label("Blend");
                let blend = &mut face_anime.blend.to_string();
                ui.text_edit_singleline(blend);
                face_anime.blend = blend.parse::<f32>().unwrap_or(face_anime.blend);
            });
        }
        Line::UnkAnime(unk_anime) => {
            ui.horizontal(|ui| {
                ui.label("Unknown Anime");
            });
            ui.horizontal(|ui| {
                ui.label("Motion File ID");
                let motion_file_id = &mut unk_anime.motion_file_id.to_string();
                ui.text_edit_singleline(motion_file_id);
                unk_anime.motion_file_id = motion_file_id.parse::<i32>().unwrap_or(unk_anime.motion_file_id);
            });
            ui.horizontal(|ui| {
                ui.label("Motion ID");
                let motion_id = &mut unk_anime.motion_id.to_string();
                ui.text_edit_singleline(motion_id);
                unk_anime.motion_id = motion_id.parse::<i32>().unwrap_or(unk_anime.motion_id);
            });
            ui.horizontal(|ui| {
                ui.label("Motion Frame");
                let motion_frame = &mut unk_anime.motion_frame.to_string();
                ui.text_edit_singleline(motion_frame);
                unk_anime.motion_frame = motion_frame.parse::<f32>().unwrap_or(unk_anime.motion_frame);
            });
            ui.horizontal(|ui| {
                ui.label("Blend");
                let blend = &mut unk_anime.blend.to_string();
                ui.text_edit_singleline(blend);
                unk_anime.blend = blend.parse::<f32>().unwrap_or(unk_anime.blend);
            });
        }
        Line::Collision(collision) => {
            ui.horizontal(|ui| {
                ui.label("Collision");
            });
            ui.horizontal(|ui| {
                ui.label("Box ID");
                let rect_id = &mut collision.rect_id.to_string();
                ui.text_edit_singleline(rect_id);
                collision.rect_id = rect_id.parse::<i32>().unwrap_or(collision.rect_id);
            });
            ui.horizontal(|ui| {
                ui.label("Box Attribute");
                let rect_attr = &mut collision.rect_attr.to_string();
                ui.text_edit_singleline(rect_attr);
                collision.rect_attr = rect_attr.parse::<i32>().unwrap_or(collision.rect_attr);
            });
            ui.horizontal(|ui| {
                ui.label("Flag");
                let flag = &mut collision.flag.to_string();
                ui.text_edit_singleline(flag);
                collision.flag = flag.parse::<i32>().unwrap_or(collision.flag);
            });
            ui.horizontal(|ui| {
                ui.label("Branch Key");
                let branch_key = &mut collision.branch_key.to_string();
                ui.text_edit_singleline(branch_key);
                collision.branch_key = branch_key.parse::<i32>().unwrap_or(collision.branch_key);
            });
            ui.horizontal(|ui| {
                ui.label("Bind Index");
                let bind_index = &mut collision.bind_index.to_string();
                ui.text_edit_singleline(bind_index);
                collision.bind_index = bind_index.parse::<i32>().unwrap_or(collision.bind_index);
            });
            ui.horizontal(|ui| {
                ui.label("Push Rate");
                let push_rate = &mut collision.push_rate.to_string();
                ui.text_edit_singleline(push_rate);
                collision.push_rate = push_rate.parse::<f32>().unwrap_or(collision.push_rate);
            });
            ui.horizontal(|ui| {
                ui.label("Box X");
                let x = &mut collision.rect.x.to_string();
                ui.text_edit_singleline(x);
                collision.rect.x = x.parse::<f32>().unwrap_or(collision.rect.x);
            });
            ui.horizontal(|ui| {
                ui.label("Box Y");
                let y = &mut collision.rect.y.to_string();
                ui.text_edit_singleline(y);
                collision.rect.y = y.parse::<f32>().unwrap_or(collision.rect.y);
            });
            ui.horizontal(|ui| {
                ui.label("Box Width");
                let w = &mut collision.rect.w.to_string();
                ui.text_edit_singleline(w);
                collision.rect.w = w.parse::<f32>().unwrap_or(collision.rect.w);
            });
            ui.horizontal(|ui| {
                ui.label("Box Height");
                let h = &mut collision.rect.h.to_string();
                ui.text_edit_singleline(h);
                collision.rect.h = h.parse::<f32>().unwrap_or(collision.rect.h);
            });
        }
        Line::UnkCollision(unk_collision) => {
            ui.horizontal(|ui| {
                ui.label("Unknown Collision");
            });
            ui.horizontal(|ui| {
                ui.label("Box ID");
                let rect_id = &mut unk_collision.rect_id.to_string();
                ui.text_edit_singleline(rect_id);
                unk_collision.rect_id = rect_id.parse::<i32>().unwrap_or(unk_collision.rect_id);
            });
            ui.horizontal(|ui| {
                ui.label("Box Attribute");
                let rect_attr = &mut unk_collision.rect_attr.to_string();
                ui.text_edit_singleline(rect_attr);
                unk_collision.rect_attr = rect_attr.parse::<i32>().unwrap_or(unk_collision.rect_attr);
            });
            ui.horizontal(|ui| {
                ui.label("Flag");
                let flag = &mut unk_collision.flag.to_string();
                ui.text_edit_singleline(flag);
                unk_collision.flag = flag.parse::<i32>().unwrap_or(unk_collision.flag);
            });
            ui.horizontal(|ui| {
                ui.label("Branch Key");
                let branch_key = &mut unk_collision.branch_key.to_string();
                ui.text_edit_singleline(branch_key);
                unk_collision.branch_key = branch_key.parse::<i32>().unwrap_or(unk_collision.branch_key);
            });
            ui.horizontal(|ui| {
                ui.label("Bind Index");
                let bind_index = &mut unk_collision.bind_index.to_string();
                ui.text_edit_singleline(bind_index);
                unk_collision.bind_index = bind_index.parse::<i32>().unwrap_or(unk_collision.bind_index);
            });
            ui.horizontal(|ui| {
                ui.label("Push Rate");
                let push_rate = &mut unk_collision.push_rate.to_string();
                ui.text_edit_singleline(push_rate);
                unk_collision.push_rate = push_rate.parse::<f32>().unwrap_or(unk_collision.push_rate);
            });
            ui.horizontal(|ui| {
                ui.label("Box X");
                let x = &mut unk_collision.rect.x.to_string();
                ui.text_edit_singleline(x);
                unk_collision.rect.x = x.parse::<f32>().unwrap_or(unk_collision.rect.x);
            });
            ui.horizontal(|ui| {
                ui.label("Box Y");
                let y = &mut unk_collision.rect.y.to_string();
                ui.text_edit_singleline(y);
                unk_collision.rect.y = y.parse::<f32>().unwrap_or(unk_collision.rect.y);
            });
            ui.horizontal(|ui| {
                ui.label("Box Width");
                let w = &mut unk_collision.rect.w.to_string();
                ui.text_edit_singleline(w);
                unk_collision.rect.w = w.parse::<f32>().unwrap_or(unk_collision.rect.w);
            });
            ui.horizontal(|ui| {
                ui.label("Box Height");
                let h = &mut unk_collision.rect.h.to_string();
                ui.text_edit_singleline(h);
                unk_collision.rect.h = h.parse::<f32>().unwrap_or(unk_collision.rect.h);
            });
        }
        Line::Attack(attack) => {
            ui.horizontal(|ui| {
                ui.label("Attack");
            });
            ui.horizontal(|ui| {
                ui.label("Data ID");
                let data_id = &mut attack.data_id.to_string();
                ui.text_edit_singleline(data_id);
                attack.data_id = data_id.parse::<i32>().unwrap_or(attack.data_id);
            });
            ui.horizontal(|ui| {
                ui.label("Group ID");
                let group_id = &mut attack.group_id.to_string();
                ui.text_edit_singleline(group_id);
                attack.group_id = group_id.parse::<i32>().unwrap_or(attack.group_id);
                });
            ui.horizontal(|ui| {
                ui.label("Box X");
                let x = &mut attack.rect.x.to_string();
                ui.text_edit_singleline(x);
                attack.rect.x = x.parse::<f32>().unwrap_or(attack.rect.x);
            });
            ui.horizontal(|ui| {
                ui.label("Box Y");
                let y = &mut attack.rect.y.to_string();
                ui.text_edit_singleline(y);
                attack.rect.y = y.parse::<f32>().unwrap_or(attack.rect.y);
            });
            ui.horizontal(|ui| {
                ui.label("Box Width");
                let w = &mut attack.rect.w.to_string();
                ui.text_edit_singleline(w);
                attack.rect.w = w.parse::<f32>().unwrap_or(attack.rect.w);
            });
            ui.horizontal(|ui| {
                ui.label("Box Height");
                let h = &mut attack.rect.h.to_string();
                ui.text_edit_singleline(h);
                attack.rect.h = h.parse::<f32>().unwrap_or(attack.rect.h);
            });
            ui.horizontal(|ui| {
                ui.label("Flag");
                let flag = &mut attack.flag.to_string();
                ui.text_edit_singleline(flag);
                attack.flag = flag.parse::<i32>().unwrap_or(attack.flag);
            });
        }
        Line::Cancel(cancel) => {
            ui.horizontal(|ui| {
                ui.label("Cancel");
            });
            ui.horizontal(|ui| {
                ui.label("Flag 1");
                let flg1 = &mut cancel.flg1.to_string();
                ui.text_edit_singleline(flg1);
                cancel.flg1 = flg1.parse::<i32>().unwrap_or(cancel.flg1);
            });
            ui.horizontal(|ui| {
                ui.label("Flag 2");
                let flg2 = &mut cancel.flg2.to_string();
                ui.text_edit_singleline(flg2);
                cancel.flg2 = flg2.parse::<i32>().unwrap_or(cancel.flg2);
            });
            ui.horizontal(|ui| {
                ui.label("Flag 3");
                let flg3 = &mut cancel.flg3.to_string();
                ui.text_edit_singleline(flg3);
                cancel.flg3 = flg3.parse::<i32>().unwrap_or(cancel.flg3);
            });
            ui.horizontal(|ui| {
                ui.label("Terms");
                let terms = &mut cancel.terms.to_string();
                ui.text_edit_singleline(terms);
                cancel.terms = terms.parse::<i32>().unwrap_or(cancel.terms);
            });
            ui.horizontal(|ui| {
                ui.label("Flag 4");
                let flg4 = &mut cancel.flg4.to_string();
                ui.text_edit_singleline(flg4);
                cancel.flg4 = flg4.parse::<i32>().unwrap_or(cancel.flg4);
            });
            ui.horizontal(|ui| {
                ui.label("Precede Frame");
                let precede_frame = &mut cancel.precede_frame.to_string();
                ui.text_edit_singleline(precede_frame);
                cancel.precede_frame = precede_frame.parse::<i32>().unwrap_or(cancel.precede_frame);
            });
        }
        Line::Branch(branch) => {
            ui.horizontal(|ui| {
                ui.label("Branch");
            });
            ui.horizontal(|ui| {
                ui.label("Type 1");
                let type_1 = &mut branch.type_1.to_string();
                ui.text_edit_singleline(type_1);
                branch.type_1 = type_1.parse::<i32>().unwrap_or(branch.type_1);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 1");
                let param_1 = &mut branch.param_1.to_string();
                ui.text_edit_singleline(param_1);
                branch.param_1 = param_1.parse::<f32>().unwrap_or(branch.param_1);
            });
            ui.horizontal(|ui| {
                ui.label("Action ID");
                let action_id = &mut branch.action_id.to_string();
                ui.text_edit_singleline(action_id);
                branch.action_id = action_id.parse::<i32>().unwrap_or(branch.action_id);
            });
            ui.horizontal(|ui| {
                ui.label("Action Frame");
                let action_frame = &mut branch.action_frame.to_string();
                ui.text_edit_singleline(action_frame);
                branch.action_frame = action_frame.parse::<i32>().unwrap_or(branch.action_frame);
            });
            ui.horizontal(|ui| {
                ui.label("Type 2");
                let type_2 = &mut branch.type_2.to_string();
                ui.text_edit_singleline(type_2);
                branch.type_2 = type_2.parse::<i32>().unwrap_or(branch.type_2);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 2");
                let param_2 = &mut branch.param_2.to_string();
                ui.text_edit_singleline(param_2);
                branch.param_2 = param_2.parse::<f32>().unwrap_or(branch.param_2);
            });
        }
        Line::Move(line_move) => {
            ui.horizontal(|ui| {
                ui.label("Move");
            });
            ui.horizontal(|ui| {
                ui.label("X");
                let x = &mut line_move.x.to_string();
                ui.text_edit_singleline(x);
                line_move.x = x.parse::<f32>().unwrap_or(line_move.x);
            });
            ui.horizontal(|ui| {
                ui.label("Y");
                let y = &mut line_move.y.to_string();
                ui.text_edit_singleline(y);
                line_move.y = y.parse::<f32>().unwrap_or(line_move.y);
                });
            ui.horizontal(|ui| {
                ui.label("Collision X");
                let col_x = &mut line_move.col_x.to_string();
                ui.text_edit_singleline(col_x);
                line_move.col_x = col_x.parse::<f32>().unwrap_or(line_move.col_x);
            });
            ui.horizontal(|ui| {
                ui.label("Collision Y");
                let col_y = &mut line_move.col_y.to_string();
                ui.text_edit_singleline(col_y);
                line_move.col_y = col_y.parse::<f32>().unwrap_or(line_move.col_y);
            });
            ui.horizontal(|ui| {
                ui.label("Option Flag");
                let option_flag = &mut line_move.option_flag.to_string();
                ui.text_edit_singleline(option_flag);
                line_move.option_flag = option_flag.parse::<i32>().unwrap_or(line_move.option_flag);
            });
        }
        Line::Offset(offset) => {
            ui.horizontal(|ui| {
                ui.label("Offset");
            });
            ui.horizontal(|ui| {
                ui.label("X");
                let x = &mut offset.x.to_string();
                ui.text_edit_singleline(x);
                offset.x = x.parse::<f32>().unwrap_or(offset.x);
            });
            ui.horizontal(|ui| {
                ui.label("Y");
                let y = &mut offset.y.to_string();
                ui.text_edit_singleline(y);
                offset.y = y.parse::<f32>().unwrap_or(offset.y);
            });
            ui.horizontal(|ui| {
                ui.label("Z");
                let z = &mut offset.z.to_string();
                ui.text_edit_singleline(z);
                offset.z = z.parse::<f32>().unwrap_or(offset.z);
            });
        }
        Line::Speed(speed) => {
            ui.horizontal(|ui| {
                ui.label("Speed");
            });
            ui.horizontal(|ui| {
                ui.label("Set Flag");
                let set_flag = &mut speed.set_flag.to_string();
                ui.text_edit_singleline(set_flag);
                speed.set_flag = set_flag.parse::<i32>().unwrap_or(speed.set_flag);
            });
            ui.horizontal(|ui| {
                ui.label("Move X");
                let move_x = &mut speed.move_x.to_string();
                ui.text_edit_singleline(move_x);
                speed.move_x = move_x.parse::<f32>().unwrap_or(speed.move_x);
            });
            ui.horizontal(|ui| {
                ui.label("Move Y");
                let move_y = &mut speed.move_y.to_string();
                ui.text_edit_singleline(move_y);
                speed.move_y = move_y.parse::<f32>().unwrap_or(speed.move_y);
            });
            ui.horizontal(|ui| {
                ui.label("Add X");
                let add_x = &mut speed.add_x.to_string();
                ui.text_edit_singleline(add_x);
                speed.add_x = add_x.parse::<f32>().unwrap_or(speed.add_x);
            });
            ui.horizontal(|ui| {
                ui.label("Add Y");
                let add_y = &mut speed.add_y.to_string();
                ui.text_edit_singleline(add_y);
                speed.add_y = add_y.parse::<f32>().unwrap_or(speed.add_y);
            });
            ui.horizontal(|ui| {
                ui.label("Option Flag");
                let option_flag = &mut speed.option_flag.to_string();
                ui.text_edit_singleline(option_flag);
                speed.option_flag = option_flag.parse::<i32>().unwrap_or(speed.option_flag);
            });
        }
        Line::ActionFlag(action_flag) => {
            ui.horizontal(|ui| {
                ui.label("Action Flag");
            });
            ui.horizontal(|ui| {
                ui.label("Flag");
                let flag = &mut action_flag.flag.to_string();
                ui.text_edit_singleline(flag);
                action_flag.flag = flag.parse::<i32>().unwrap_or(action_flag.flag);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 1");
                let param1 = &mut action_flag.param1.to_string();
                ui.text_edit_singleline(param1);
                action_flag.param1 = param1.parse::<i32>().unwrap_or(action_flag.param1);
            });
        }
        Line::Effect(effect) => {
            match &mut effect.eff_type {
                EffectType::Set(set) => {
                    ui.horizontal(|ui| {
                        ui.label("Effect Set");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Position X");
                        let pos_x = &mut set.pos_x.to_string();
                        ui.text_edit_singleline(pos_x);
                        set.pos_x = pos_x.parse::<f32>().unwrap_or(set.pos_x);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Position Y");
                        let pos_y = &mut set.pos_y.to_string();
                        ui.text_edit_singleline(pos_y);
                        set.pos_y = pos_y.parse::<f32>().unwrap_or(set.pos_y);
                    });        
                    ui.horizontal(|ui| {
                        ui.label("Position Z");
                        let pos_z = &mut set.pos_z.to_string();
                        ui.text_edit_singleline(pos_z);
                        set.pos_z = pos_z.parse::<f32>().unwrap_or(set.pos_z);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Position Base");
                        let pos_base = &mut set.pos_base.to_string();
                        ui.text_edit_singleline(pos_base);
                        set.pos_base = pos_base.parse::<i16>().unwrap_or(set.pos_base);
                    });        
                    ui.horizontal(|ui| {
                        ui.label("Action File ID");
                        let act_file_id = &mut set.act_file_id.to_string();
                        ui.text_edit_singleline(act_file_id);
                        set.act_file_id = act_file_id.parse::<i16>().unwrap_or(set.act_file_id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Term Flag");
                        let term_flag = &mut set.term_flag.to_string();
                        ui.text_edit_singleline(term_flag);
                        set.term_flag = term_flag.parse::<i16>().unwrap_or(set.term_flag);
                    });        
                    ui.horizontal(|ui| {
                        ui.label("Action Number");
                        let act_number = &mut set.act_number.to_string();
                        ui.text_edit_singleline(act_number);
                        set.act_number = act_number.parse::<i16>().unwrap_or(set.act_number);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Option Flag");
                        let option_flag = &mut set.option_flag.to_string();
                        ui.text_edit_singleline(option_flag);
                        set.option_flag = option_flag.parse::<i32>().unwrap_or(set.option_flag);
                    });        
                    ui.horizontal(|ui| {
                        ui.label("Unknown X");
                        let unk_x = &mut set.unk_x.to_string();
                        ui.text_edit_singleline(unk_x);
                        set.unk_x = unk_x.parse::<f32>().unwrap_or(set.unk_x);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Unknown Y");
                        let unk_y = &mut set.unk_y.to_string();
                        ui.text_edit_singleline(unk_y);
                        set.unk_y = unk_y.parse::<f32>().unwrap_or(set.unk_y);
                    });        
                    ui.horizontal(|ui| {
                        ui.label("Unknown Z");
                        let unk_z = &mut set.unk_z.to_string();
                        ui.text_edit_singleline(unk_z);
                        set.unk_z = unk_z.parse::<f32>().unwrap_or(set.unk_z);
                    });
                }
                EffectType::Control(control) => {
                    ui.horizontal(|ui| {
                        ui.label("Effect Control");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 1");
                        let reserve1 = &mut control.reserve1.to_string();
                        ui.text_edit_singleline(reserve1);
                        control.reserve1 = reserve1.parse::<i32>().unwrap_or(control.reserve1);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 2");
                        let reserve2 = &mut control.reserve2.to_string();
                        ui.text_edit_singleline(reserve2);
                        control.reserve2 = reserve2.parse::<i32>().unwrap_or(control.reserve2);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 3");
                        let reserve3 = &mut control.reserve3.to_string();
                        ui.text_edit_singleline(reserve3);
                        control.reserve3 = reserve3.parse::<i32>().unwrap_or(control.reserve3);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Position Base");
                        let pos_base = &mut control.pos_base.to_string();
                        ui.text_edit_singleline(pos_base);
                        control.pos_base = pos_base.parse::<i16>().unwrap_or(control.pos_base);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Term Flag");
                        let term_flag = &mut control.term_flag.to_string();
                        ui.text_edit_singleline(term_flag);
                        control.term_flag = term_flag.parse::<i16>().unwrap_or(control.term_flag);
                    });        
                    ui.horizontal(|ui| {
                        ui.label("Action Number");
                        let act_number = &mut control.act_number.to_string();
                        ui.text_edit_singleline(act_number);
                        control.act_number = act_number.parse::<i16>().unwrap_or(control.act_number);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Option Flag");
                        let option_flag = &mut control.option_flag.to_string();
                        ui.text_edit_singleline(option_flag);
                        control.option_flag = option_flag.parse::<i32>().unwrap_or(control.option_flag);
                    });        
                }
            }
        }
        Line::Sound(sound) => {
            match &mut sound.sound {
                SoundType::Play(play) => {
                    ui.horizontal(|ui| {
                        ui.label("Sound Play");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Volume");
                        let volume = &mut play.volume.to_string();
                        ui.text_edit_singleline(volume);
                        play.volume = volume.parse::<f32>().unwrap_or(play.volume);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Common ID");
                        let common_id = &mut play.common_id.to_string();
                        ui.text_edit_singleline(common_id);
                        play.common_id = common_id.parse::<i32>().unwrap_or(play.common_id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Unique ID");
                        let unique_id = &mut play.unique_id.to_string();
                        ui.text_edit_singleline(unique_id);
                        play.unique_id = unique_id.parse::<i16>().unwrap_or(play.unique_id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Group ID");
                        let group_id = &mut play.group_id.to_string();
                        ui.text_edit_singleline(group_id);
                        play.group_id = group_id.parse::<i16>().unwrap_or(play.group_id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Priority");
                        let priority = &mut play.priority.to_string();
                        ui.text_edit_singleline(priority);
                        play.priority = priority.parse::<i32>().unwrap_or(play.priority);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Option Flag");
                        let option_flag = &mut play.option_flag.to_string();
                        ui.text_edit_singleline(option_flag);
                        play.option_flag = option_flag.parse::<i16>().unwrap_or(play.option_flag);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Tag ID");
                        let tag_id = &mut play.tag_id.to_string();
                        ui.text_edit_singleline(tag_id);
                        play.tag_id = tag_id.parse::<i16>().unwrap_or(play.tag_id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Tag ID");
                        let option_param = &mut play.option_param.to_string();
                        ui.text_edit_singleline(option_param);
                        play.option_param = option_param.parse::<f32>().unwrap_or(play.option_param);
                    });
                }
                SoundType::Stop(stop) => {
                    ui.horizontal(|ui| {
                        ui.label("Sound Stop");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Fade Seconds");
                        let fade_sec = &mut stop.fade_sec.to_string();
                        ui.text_edit_singleline(fade_sec);
                        stop.fade_sec = fade_sec.parse::<f32>().unwrap_or(stop.fade_sec);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 1");
                        let reserve1 = &mut stop.reserve1.to_string();
                        ui.text_edit_singleline(reserve1);
                        stop.reserve1 = reserve1.parse::<i16>().unwrap_or(stop.reserve1);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Tag ID");
                        let tag_id = &mut stop.tag_id.to_string();
                        ui.text_edit_singleline(tag_id);
                        stop.tag_id = tag_id.parse::<i16>().unwrap_or(stop.tag_id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 2");
                        let reserve2 = &mut stop.reserve2.to_string();
                        ui.text_edit_singleline(reserve2);
                        stop.reserve2 = reserve2.parse::<i32>().unwrap_or(stop.reserve2);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 3");
                        let reserve3 = &mut stop.reserve3.to_string();
                        ui.text_edit_singleline(reserve3);
                        stop.reserve3 = reserve3.parse::<i32>().unwrap_or(stop.reserve3);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 4");
                        let reserve4 = &mut stop.reserve4.to_string();
                        ui.text_edit_singleline(reserve4);
                        stop.reserve4 = reserve4.parse::<i32>().unwrap_or(stop.reserve4);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 5");
                        let reserve5 = &mut stop.reserve5.to_string();
                        ui.text_edit_singleline(reserve5);
                        stop.reserve5 = reserve5.parse::<i32>().unwrap_or(stop.reserve5);
                    });
                }
                SoundType::Unk(unk) => {
                    ui.horizontal(|ui| {
                        ui.label("Sound Unknown");
                    });
                    ui.horizontal(|ui| {
                        ui.label("Float");
                        let float = &mut unk.float.to_string();
                        ui.text_edit_singleline(float);
                        unk.float = float.parse::<f32>().unwrap_or(unk.float);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 1");
                        let reserve1 = &mut unk.reserve1.to_string();
                        ui.text_edit_singleline(reserve1);
                        unk.reserve1 = reserve1.parse::<i32>().unwrap_or(unk.reserve1);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 2");
                        let reserve2 = &mut unk.reserve2.to_string();
                        ui.text_edit_singleline(reserve2);
                        unk.reserve2 = reserve2.parse::<i32>().unwrap_or(unk.reserve2);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 3");
                        let reserve3 = &mut unk.reserve3.to_string();
                        ui.text_edit_singleline(reserve3);
                        unk.reserve3 = reserve3.parse::<i32>().unwrap_or(unk.reserve3);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 4");
                        let reserve4 = &mut unk.reserve4.to_string();
                        ui.text_edit_singleline(reserve4);
                        unk.reserve4 = reserve4.parse::<i32>().unwrap_or(unk.reserve4);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Reserve 5");
                        let reserve5 = &mut unk.reserve5.to_string();
                        ui.text_edit_singleline(reserve5);
                        unk.reserve5 = reserve5.parse::<i32>().unwrap_or(unk.reserve5);
                    });
                }
                SoundType::DefaultSound(_) => {
                    ui.horizontal(|ui| {
                        ui.label("Invalid sound type! 
                        You should never see this!");
                    });
                },
            }
        }
        Line::DefaultLine(default_line) => {
            ui.horizontal(|ui| {
                ui.label("Unknown Line Type");
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 1");
                let param0 = &mut default_line[0].to_string();
                ui.text_edit_singleline(param0);
                default_line[0] = param0.parse::<u32>().unwrap_or(default_line[0]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 2");
                let param1 = &mut default_line[1].to_string();
                ui.text_edit_singleline(param1);
                default_line[1] = param1.parse::<u32>().unwrap_or(default_line[1]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 3");
                let param2 = &mut default_line[2].to_string();
                ui.text_edit_singleline(param2);
                default_line[2] = param2.parse::<u32>().unwrap_or(default_line[2]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 4");
                let param3 = &mut default_line[3].to_string();
                ui.text_edit_singleline(param3);
                default_line[3] = param3.parse::<u32>().unwrap_or(default_line[3]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 5");
                let param4 = &mut default_line[4].to_string();
                ui.text_edit_singleline(param4);
                default_line[4] = param4.parse::<u32>().unwrap_or(default_line[4]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 6");
                let param5 = &mut default_line[5].to_string();
                ui.text_edit_singleline(param5);
                default_line[5] = param5.parse::<u32>().unwrap_or(default_line[5]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 7");
                let param6 = &mut default_line[6].to_string();
                ui.text_edit_singleline(param6);
                default_line[6] = param6.parse::<u32>().unwrap_or(default_line[6]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 8");
                let param7 = &mut default_line[7].to_string();
                ui.text_edit_singleline(param7);
                default_line[7] = param7.parse::<u32>().unwrap_or(default_line[7]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 9");
                let param8 = &mut default_line[8].to_string();
                ui.text_edit_singleline(param8);
                default_line[8] = param8.parse::<u32>().unwrap_or(default_line[8]);
            });
            ui.horizontal(|ui| {
                ui.label("Parameter 10");
                let param9 = &mut default_line[9].to_string();
                ui.text_edit_singleline(param9);
                default_line[9] = param9.parse::<u32>().unwrap_or(default_line[9]);
            });
        }
    }
}

impl Editor {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Response {
        ComboBox::from_label("Action List")
        .selected_text(format!("{:?}", self.selected))
        .width(150.0)
        .show_ui(ui, |ui| {
            match &self.cact {
                Some(cact) => {
                    for (index, name) in cact.act_names.iter().enumerate() {
                        let mut name = name.pretty_name.clone();
                        if name == "" {
                            name = "Action ".to_owned() + &index.to_string();
                        }
                        if ui.selectable_label(true, name.clone())
                        .clicked()
                        {
                            if self.selected != ""
                            {
                                self.action_index = index;
                            }
                            self.selected = name.clone();
                        };
                    }        
                }
                None => ()
            }
        });
        if self.selected != ""{
            self.action_timeline(ui)
        }
        else {
            ui.horizontal(|ui| {
                ui.label("Select an action from the action list!");
            }).response
        }
    }
    pub fn action_timeline(&mut self, ui: &mut egui::Ui) -> Response {
        match &mut self.cact 
        {
            Some(cact) => {
                let act = &mut cact.frame[self.action_index];
                ui.vertical(|ui| {
                    ui.label("Category ID");
                    let category_id = &mut act.info.category_id.to_string();
                    ui.text_edit_singleline(category_id);
                    act.info.category_id = category_id.parse::<i32>().unwrap_or(act.info.category_id);
                });
                ui.vertical(|ui| {
                    ui.label("Sub Category ID");
                    let sub_category_id = &mut act.info.sub_category_id.to_string();
                    ui.text_edit_singleline(sub_category_id);
                    act.info.sub_category_id = sub_category_id.parse::<i32>().unwrap_or(act.info.sub_category_id);
                });
                ui.vertical(|ui| {
                    ui.label("End Frame");
                    let end_frame = &mut act.info.end_frame.to_string();
                    ui.text_edit_singleline(end_frame);
                    act.info.end_frame = end_frame.parse::<i32>().unwrap_or(act.info.end_frame);
                });
                ui.vertical(|ui| {
                    ui.label("Loop To");
                    let loop_back_frame = &mut act.info.loop_back_frame.to_string();
                    ui.text_edit_singleline(loop_back_frame);
                    act.info.loop_back_frame = loop_back_frame.parse::<i32>().unwrap_or(act.info.loop_back_frame);
                });            
                ui.vertical(|ui| {
                    ui.label("Option Flag");
                    let option_flag = &mut act.info.option_flag.to_string();
                    ui.text_edit_singleline(option_flag);
                    act.info.option_flag = option_flag.parse::<i32>().unwrap_or(act.info.option_flag);
                });
                egui::ScrollArea::vertical().show(ui, |ui|{
                    if self.current_frame < 0
                    {
                        self.current_frame = 0;
                    }
                    else if self.current_frame > act.info.end_frame
                    {
                        self.current_frame = act.info.end_frame;
                    }
                    for (index, line) in &mut act.frame.iter_mut().enumerate()
                    {
                        for line_frame in &mut line.frame {
                            if line_frame.frame == self.current_frame {
                                egui::ScrollArea::horizontal()
                                .id_source(index)
                                .show(ui, |ui|{
                                    ui.horizontal(|ui| {
                                        render_line(ui, &mut line_frame.line);
                                    });
                                });
                            }
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Current Frame");
                    let current_frame = &mut self.current_frame.to_string();
                    ui.text_edit_singleline(current_frame);
                    self.current_frame = current_frame.parse::<i32>().unwrap_or(self.current_frame);
                }).response
            }
            None => ui.horizontal(|ui| {
                ui.label("Select an action from the action list!");
            }).response
        }
    }
    pub fn open_cact(&mut self, path: &PathBuf) -> bool {
        let file = File::open(&path);
        match file {
            Ok(mut file) => {
                let cact = file.read_le::<CharaActionData>();
                let success: bool;
                match cact {
                    Ok(cact) => {
                        self.cact = Some(cact);
                        success = true
                    }
                    Err(_) => success = false
                }
                success
            }
            Err(_) => false
        }
    }
    pub fn save_cact(&mut self, path: &PathBuf) -> bool {
        match &mut self.cact {
            Some(cact) => {
                let mut buf: Vec<u8> = Vec::new();
                cact.write(&mut buf);
            
                let output = File::create(&path);
                let success: bool;
                match output {
                    Ok(mut output) => {        
                        let result = output.write_all(&buf);
                        match result {
                            Ok(_) => success = true,
                            Err(_) => success = false
                        }
                    }
                    Err(_) => success = false
                }
                success
            }
            None => false
        }
    }
}