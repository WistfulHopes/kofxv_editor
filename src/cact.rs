use binrw::{BinRead};
use byteorder::{LE, WriteBytesExt};
use serde::{Serialize, Deserialize};
use crate::cfile::FileHeader;
use std::{str, mem, io::Write};

#[derive(BinRead, Serialize, Deserialize)]
struct CharaActionDataActionHeader {
    data_version: u32,
    #[serde(skip_serializing, default)]
    action_count: i32,
    #[serde(skip_serializing, default)]
    anim_offset: u32,
    #[serde(skip_serializing, default)]
    anim_offset_2: u32,
    #[serde(skip_serializing, default)]
    camera_offset: u32,
    #[serde(skip_serializing, default)]
    bone_offset: u32,
}

impl CharaActionDataActionHeader {
    fn write(&self, buf: &mut Vec<u8>) {
        buf.write_u32::<LE>(self.data_version).unwrap();
        buf.write_i32::<LE>(self.action_count).unwrap();
        buf.write_u32::<LE>(self.anim_offset).unwrap();
        buf.write_u32::<LE>(self.anim_offset_2).unwrap();
        buf.write_u32::<LE>(self.camera_offset).unwrap();
        buf.write_u32::<LE>(self.bone_offset).unwrap();
    }
}

#[derive(BinRead, Serialize, Deserialize)]
pub struct CharaActionDataActionDataInfo {
    pub category_id: i32,
    pub sub_category_id: i32,
    pub end_frame: i32,
    pub loop_back_frame: i32,
    pub option_flag: i32,
    reserve1: i32,
    reserve2: i32,
    reserve3: i32,
    reserve4: i32,
    #[serde(skip_serializing, default)]
    line_count: i32,
}

impl CharaActionDataActionDataInfo {
    fn write(&self, buf: &mut Vec<u8>) {
        buf.write_i32::<LE>(self.category_id).unwrap();
        buf.write_i32::<LE>(self.sub_category_id).unwrap();
        buf.write_i32::<LE>(self.end_frame).unwrap();
        buf.write_i32::<LE>(self.loop_back_frame).unwrap();
        buf.write_i32::<LE>(self.option_flag).unwrap();
        buf.write_i32::<LE>(self.reserve1).unwrap();
        buf.write_i32::<LE>(self.reserve2).unwrap();
        buf.write_i32::<LE>(self.reserve3).unwrap();
        buf.write_i32::<LE>(self.reserve4).unwrap();
        buf.write_i32::<LE>(self.line_count).unwrap();
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataBaseAnime
{
    pub motion_file_id: i32,
    pub motion_id: i32,
    pub motion_frame: f32,
    pub blend: f32,
    pub transparent: f32, 
    pub unk1: f32, 
    pub unk2: u32, 
    pub unk3: u32, 
    pub unk4: f32, 
    #[serde(skip_serializing, default)]
    padding: u32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataFaceAnime
{
    pub motion_file_id: i32,
    pub motion_id: i32,
    pub motion_frame: f32,
    pub blend: f32,
    #[serde(skip_serializing, default)]
    padding: [u32; 6],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataUnkAnime
{
    pub motion_file_id: i32,
    pub motion_id: i32,
    pub motion_frame: f32,
    pub blend: f32,
    #[serde(skip_serializing, default)]
    padding: [u32; 6],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataCollision
{
    pub rect_id: i32,
    pub rect_attr: i32,
    pub flag: i32,
    pub branch_key: i32,
    pub bind_index: i32,
    pub push_rate: f32,
    pub rect: CollisionHitRect,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataUnkCollision
{
    pub rect_id: i32,
    pub rect_attr: i32,
    pub flag: i32,
    pub branch_key: i32,
    pub bind_index: i32,
    pub push_rate: f32,
    pub rect: CollisionHitRect,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct CollisionHitRect
{
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataAttack
{
    pub data_id: i32,
    pub group_id: i32,
    pub rect: CollisionHitRect,
    pub flag: i32,
    #[serde(skip_serializing, default)]
    padding: [u32; 3],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataCancel
{
    pub flg1: i32,
    pub flg2: i32,
    pub flg3: i32,
    pub terms: i32,
    pub flg4: i32,
    pub precede_frame: i32,
    #[serde(skip_serializing, default)]
    padding: [u32; 4],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataBranch
{
    pub type_1: i32,
    pub param_1: f32,
    pub action_id: i32,
    pub action_frame: i32,
    pub type_2: i32,
    pub param_2: f32,
    #[serde(skip_serializing, default)]
    padding: [u32; 4],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataMove
{
    pub x: f32,
    pub y: f32,
    pub col_x: f32,
    pub col_y: f32,
    pub option_flag: i32,
    #[serde(skip_serializing, default)]
    padding: [u32; 5],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataOffset
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[serde(skip_serializing, default)]
    padding: [u32; 7],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataSpeed
{
    pub set_flag: i32,
    pub move_x: f32,
    pub move_y: f32,
    pub add_x: f32,
    pub add_y: f32,
    pub option_flag: i32,
    #[serde(skip_serializing, default)]
    padding: [u32; 4],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataActionFlag
{
    pub flag: i32,
    pub param1: i32,
    #[serde(skip_serializing, default)]
    padding: [u32; 8],
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EffectTypeSet {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub pos_base: i16,
    pub act_file_id: i16,
    pub term_flag: i16,
    pub act_number: i16,
    pub option_flag: i32,
    pub unk_x: f32,
    pub unk_y: f32,
    pub unk_z: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EffectTypeControl {
    pub reserve1: i32,
    pub reserve2: i32,
    pub reserve3: i32,
    pub pos_base: i16,
    pub act_file_id: i16,
    pub term_flag: i16,
    pub act_number: i16,
    pub option_flag: i32,
    padding2: [u32; 3],
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum EffectType {
    Set(EffectTypeSet),
    Control(EffectTypeControl),
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct CharaActionDataEffect
{
    pub eff_type: EffectType,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct SoundPlay
{
    pub volume: f32,
    pub common_id: i32,
    pub unique_id: i16,
    pub group_id: i16,
    pub priority: i32,
    pub option_flag: i16,
    pub tag_id: i16,
    pub option_param: f32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct SoundStop
{
    pub fade_sec: f32,
    pub reserve1: i16,
    pub tag_id: i16,
    pub reserve2: i32,
    pub reserve3: i32,
    pub reserve4: i32,
    pub reserve5: i32,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct SoundUnk
{
    pub float: f32,
    pub reserve1: i32,
    pub reserve2: i32,
    pub reserve3: i32,
    pub reserve4: i32,
    pub reserve5: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum SoundType {
    Play(SoundPlay),
    Stop(SoundStop),
    Unk(SoundUnk),
    DefaultSound([u32; 6]),
}

impl Default for SoundType {
    fn default() -> Self {
        SoundType::DefaultSound([ 0u32; 6 ]) 
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CharaActionDataSound
{
    #[serde(skip_serializing, default)]
    data: [u32; 6],
    #[serde(skip_serializing, default)]
    sound_type: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    #[serde(skip_serializing, default)]
    padding: [u32; 3],
    pub sound: SoundType,
}

#[binrw::parser(reader, endian)]
fn line_parser(count: usize, _: u8) -> binrw::BinResult<Vec<CharaActionDataActionLine>> {
    let mut line_vec: Vec<CharaActionDataActionLine> = Vec::new();
    for _ in 0..count {
        let mut line = <CharaActionDataActionLine>::read_options(reader, endian, ()).unwrap();
        for mut frame in &mut line.frame {
            match line.action_line_id {
                0 => {
                    let base_anime: CharaActionDataBaseAnime;
                    unsafe {
                        base_anime = mem::transmute::<[u32; 10], CharaActionDataBaseAnime>(frame.data);
                    }                
                    frame.line = Line::BaseAnime(base_anime)
                },
                1 => {
                    let face_anime: CharaActionDataFaceAnime;
                    unsafe {
                        face_anime = mem::transmute::<[u32; 10], CharaActionDataFaceAnime>(frame.data);
                    }                
                    frame.line = Line::FaceAnime(face_anime)
                },
                2 => {
                    let unk_anime: CharaActionDataUnkAnime;
                    unsafe {
                        unk_anime = mem::transmute::<[u32; 10], CharaActionDataUnkAnime>(frame.data);
                    }                
                    frame.line = Line::UnkAnime(unk_anime)
                },
                3 => {
                    let collision: CharaActionDataCollision;
                    unsafe {
                        collision = mem::transmute::<[u32; 10], CharaActionDataCollision>(frame.data);
                    }                
                    frame.line = Line::Collision(collision)
                },
                4 => {
                    let unk_collision: CharaActionDataUnkCollision;
                    unsafe {
                        unk_collision = mem::transmute::<[u32; 10], CharaActionDataUnkCollision>(frame.data);
                    }                
                    frame.line = Line::UnkCollision(unk_collision)
                },
                5 => {
                    let attack: CharaActionDataAttack;
                    unsafe {
                        attack = mem::transmute::<[u32; 10], CharaActionDataAttack>(frame.data);
                    }                
                    frame.line = Line::Attack(attack)
                },
                6 => {
                    let cancel: CharaActionDataCancel;
                    unsafe {
                        cancel = mem::transmute::<[u32; 10], CharaActionDataCancel>(frame.data);
                    }                
                    frame.line = Line::Cancel(cancel)
                },
                7 => {
                    let branch: CharaActionDataBranch;
                    unsafe {
                        branch = mem::transmute::<[u32; 10], CharaActionDataBranch>(frame.data);
                    }                
                    frame.line = Line::Branch(branch)
                },
                8 => {
                    let line_move: CharaActionDataMove;
                    unsafe {
                        line_move = mem::transmute::<[u32; 10], CharaActionDataMove>(frame.data);
                    }                
                    frame.line = Line::Move(line_move)
                },
                9 => {
                    let offset: CharaActionDataOffset;
                    unsafe {
                        offset = mem::transmute::<[u32; 10], CharaActionDataOffset>(frame.data);
                    }                
                    frame.line = Line::Offset(offset)
                },
                10 => {
                    let speed: CharaActionDataSpeed;
                    unsafe {
                        speed = mem::transmute::<[u32; 10], CharaActionDataSpeed>(frame.data);
                    }                
                    frame.line = Line::Speed(speed)
                },
                11 => {
                    let action_flag: CharaActionDataActionFlag;
                    unsafe {
                        action_flag = mem::transmute::<[u32; 10], CharaActionDataActionFlag>(frame.data);
                    }                
                    frame.line = Line::ActionFlag(action_flag)
                },
                14 => {
                    let effect: CharaActionDataEffect;
                    unsafe {
                        effect = mem::transmute::<[u32; 10], CharaActionDataEffect>(frame.data);
                    }
                    frame.line = Line::Effect(effect)
                },
                15 => {
                    let mut sound: CharaActionDataSound = unsafe { mem::zeroed() };
                    let mut buf: [u32; 6] = [0; 6];
                    buf.clone_from_slice(&frame.data[..6]);
                    sound.data = buf;
                    sound.sound_type = frame.data[6];
                    sound.unk1 = frame.data[7];
                    sound.unk2 = frame.data[8];
                    sound.unk3 = frame.data[9];
                    match sound.sound_type
                    {
                        0 => {
                            let play: SoundPlay;
                            unsafe {
                                play = mem::transmute::<[u32; 6], SoundPlay>(sound.data);
                            }
                            sound.sound = SoundType::Play(play)
                        },
                        1 => {
                            let stop: SoundStop;
                            unsafe {
                                stop = mem::transmute::<[u32; 6], SoundStop>(sound.data);
                            }
                            sound.sound = SoundType::Stop(stop)
                        },
                        2 => {
                            let unk: SoundUnk;
                            unsafe {
                                unk = mem::transmute::<[u32; 6], SoundUnk>(sound.data);
                            }
                            sound.sound = SoundType::Unk(unk)
                        },
                        _ => sound.sound = SoundType::DefaultSound(sound.data),
                    }
                    frame.line = Line::Sound(sound)
                },
                _ => frame.line = Line::DefaultLine(frame.data),
            }
        }
        line_vec.push(line);
    }
    Ok(line_vec)
}

#[derive(Serialize, Deserialize)]
pub enum Line {
    BaseAnime(CharaActionDataBaseAnime),
    FaceAnime(CharaActionDataFaceAnime),
    UnkAnime(CharaActionDataUnkAnime),
    Collision(CharaActionDataCollision),
    UnkCollision(CharaActionDataUnkCollision),
    Attack(CharaActionDataAttack),
    Cancel(CharaActionDataCancel),
    Branch(CharaActionDataBranch),
    Move(CharaActionDataMove),
    Offset(CharaActionDataOffset),
    Speed(CharaActionDataSpeed),
    ActionFlag(CharaActionDataActionFlag),
    Effect(CharaActionDataEffect),
    Sound(CharaActionDataSound),
    DefaultLine([u32; 10]),
}

impl Default for Line {
    fn default() -> Self {
        Line::DefaultLine([ 0u32; 10 ]) 
    }
}

#[derive(BinRead, Serialize, Deserialize)]
pub struct CharaActionDataActionLineFrame {
    pub frame: i32,
    #[serde(skip_serializing, default)]
    data: [u32; 10],
    #[br(ignore)]
    pub line: Line,
}

fn vec_u32_to_u8(vec32: Vec<u32>) -> Vec<u8> {
    let mut vec8: Vec<u8> = vec![];
    for elem in vec32 {
        vec8.write_u32::<LE>(elem).unwrap();
    }
    vec8
}

impl CharaActionDataActionLineFrame {
    fn write(&self, buf: &mut Vec<u8>) {
        buf.write_i32::<LE>(self.frame).unwrap();
        let vec32 = self.data.to_vec();
        let vec8 = vec_u32_to_u8(vec32);
        buf.write_all(&vec8).unwrap();
    }
}

#[derive(BinRead, Serialize, Deserialize)]
pub struct CharaActionDataActionLine {
    #[serde(skip_serializing, default)]
    key_frame_count: i32,
    pub action_line_id: i32,
    #[br(count = key_frame_count)]
    pub frame: Vec<CharaActionDataActionLineFrame>,
}

impl CharaActionDataActionLine {
    fn write(&self, buf: &mut Vec<u8>) {
        buf.write_i32::<LE>(self.key_frame_count).unwrap();
        buf.write_i32::<LE>(self.action_line_id).unwrap();
        for frame in &self.frame {
            frame.write(buf);
        }
    }
}

#[derive(BinRead, Serialize, Deserialize)]
pub struct CharaActionDataActionData {
    pub info: CharaActionDataActionDataInfo,
    #[br(args(info.line_count as usize, 0))]
    #[br(parse_with = line_parser)]
    pub frame: Vec<CharaActionDataActionLine>,
}

impl CharaActionDataActionData {
    fn write(&self, buf: &mut Vec<u8>) {
        self.info.write(buf);
        for frame in &self.frame {
            frame.write(buf);
        }
    }
}

#[binrw::parser(reader, endian)]
fn name_parser(count: usize, _: u8) -> binrw::BinResult<Vec<Name>> {
    let mut name_vec: Vec<Name> = Vec::new();
    for _ in 0..count {
        let mut name = Name::read_options(reader, endian, ()).unwrap();
        name.pretty_name = match str::from_utf8(&name.name) {
            Ok(pretty) => pretty.to_owned(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        name_vec.push(name);
    }
    Ok(name_vec)
}

#[derive(BinRead, Serialize, Deserialize)]
pub struct Name {
    #[serde(skip_serializing, default)]
    name_size: u32,
    #[br(count = name_size)]
    #[serde(skip_serializing, default)]
    name: Vec<u8>,
    #[br(ignore)]
    pub pretty_name: String,
}

impl Name {
    fn prepare_write(&mut self) {
        self.name = self.pretty_name.as_bytes().to_vec();
        self.name_size = self.name.len() as u32;
    }
    fn write(&self, buf: &mut Vec<u8>) {
        buf.write_u32::<LE>(self.name_size).unwrap();
        buf.write_all(&self.name).unwrap();
    }
}

#[derive(BinRead, Serialize, Deserialize, Default)]
struct NameRepeat {
    #[serde(skip_serializing, default)]
    name_count: u32,
    #[br(args(name_count as usize, 0))]
    #[br(parse_with = name_parser)]
    #[serde(skip_serializing, default)]
    names: Vec<Name>,
}

#[derive(BinRead, Serialize, Deserialize)]
struct NameList {
    #[serde(skip_serializing, default)]
    demo_count: u32,
    #[br(count(demo_count as usize + 1))]
    unk1: Vec<u32>,
    #[br(args(demo_count as usize, 0))]
    #[br(parse_with = name_parser)]
    demo_names: Vec<Name>,
    unk2: u32,
    #[br(ignore)]
    #[serde(skip_serializing, default)]
    anim_offset: u32,
    #[serde(skip_serializing, default)]
    chara_anim_count: u32,
    #[br(args(chara_anim_count as usize, 0))]
    #[br(parse_with = name_parser)]
    chara_anim_names: Vec<Name>,
    #[serde(skip_serializing, default)]
    cmn_anim_count: u32,
    #[br(args(cmn_anim_count as usize, 0))]
    #[br(parse_with = name_parser)]
    cmn_anim_names: Vec<Name>,
    #[serde(skip_serializing, default)]
    face_count: u32,
    #[br(args(face_count as usize, 0))]
    #[br(parse_with = name_parser)]
    face_names: Vec<Name>, 
    #[serde(skip_serializing, default)]
    face_count_2: u32,
    #[br(args(face_count_2 as usize, 0))]
    #[br(parse_with = name_parser)]
    #[serde(skip_serializing, default)]
    face_names_2: Vec<Name>,
    unk3: u32,
    chara_anim_repeat_count: u32,
    #[br(count(chara_anim_repeat_count as usize))]
    #[serde(skip_serializing, default)]
    chara_anim_repeat: Vec<NameRepeat>,
    mat_repeat_count: u32,
    #[br(count(mat_repeat_count as usize - 1))]
    #[serde(skip_serializing, default)]
    mat_names: Vec<NameRepeat>,
    unk5: u32,
    #[br(ignore)]
    #[serde(skip_serializing, default)]
    camera_offset: u32,
    #[serde(skip_serializing, default)]
    camera_count: u32,
    #[br(args(camera_count as usize, 0))]
    #[br(parse_with = name_parser)]
    camera_names: Vec<Name>,
    #[br(ignore)]
    #[serde(skip_serializing, default)]
    bone_offset: u32,
    #[serde(skip_serializing, default)]
    bone_count: u32,
    #[br(args(bone_count as usize, 0))]
    #[br(parse_with = name_parser)]
    bone_names: Vec<Name>,
}

impl NameList {
    fn prepare_write(&mut self) {
        self.demo_count = self.demo_names.len() as u32;
        for name in &mut self.demo_names {
            name.prepare_write();
        }
        self.chara_anim_count = self.chara_anim_names.len() as u32;
        for name in &mut self.chara_anim_names {
            name.prepare_write();
        }
        self.cmn_anim_count = self.cmn_anim_names.len() as u32;
        for name in &mut self.cmn_anim_names {
            name.prepare_write();
        }
        self.face_count = self.face_names.len() as u32;
        for name in &mut self.face_names {
            name.prepare_write();
        }
        for index in 0..self.mat_names.len()  {
            self.mat_names[index].name_count = self.mat_names[index].names.len() as u32;
            for name in &mut self.mat_names[index].names {
                name.prepare_write();
            }
        }
        self.camera_count = self.camera_names.len() as u32;
        for name in &mut self.camera_names {
            name.prepare_write();
        }
        self.bone_count = self.bone_names.len() as u32;
        for name in &mut self.bone_names {
            name.prepare_write();
        }
    }
    fn write(&mut self, buf: &mut Vec<u8>) {
        buf.write_u32::<LE>(self.demo_count).unwrap();
        let unk1 = vec_u32_to_u8(self.unk1.to_vec());
        buf.write_all(&unk1).unwrap();
        for name in &self.demo_names {
            name.write(buf);
        }
        buf.write_u32::<LE>(self.unk2).unwrap();
        self.anim_offset = buf.len() as u32 - 8;
        buf.write_u32::<LE>(self.chara_anim_count).unwrap();
        for name in &self.chara_anim_names {
            name.write(buf);
        }
        buf.write_u32::<LE>(self.cmn_anim_count).unwrap();
        for name in &self.cmn_anim_names {
            name.write(buf);
        }
        for _ in 0..2
        {
            buf.write_u32::<LE>(self.face_count).unwrap();
            for name in &self.face_names {
                name.write(buf);
            }
        }
        buf.write_u32::<LE>(self.unk3).unwrap();
        buf.write_u32::<LE>(self.chara_anim_repeat_count).unwrap();
        for _ in 0..self.chara_anim_repeat_count
        {
            buf.write_u32::<LE>(self.chara_anim_count).unwrap();
            for name in &self.chara_anim_names {
                name.write(buf);
            }
        }
        buf.write_u32::<LE>(self.mat_repeat_count).unwrap();
        for index in 0..self.mat_repeat_count - 1
        {
            buf.write_u32::<LE>(self.mat_names[index as usize].name_count).unwrap();
            for name in &self.mat_names[index as usize].names {
                name.write(buf);
            }
        }
        buf.write_u32::<LE>(self.unk5).unwrap();
        self.camera_offset = buf.len() as u32 - 4;
        buf.write_u32::<LE>(self.camera_count).unwrap();
        for name in &self.camera_names {
            name.write(buf);
        }
        self.bone_offset = buf.len() as u32 - 4;
        buf.write_u32::<LE>(self.bone_count).unwrap();
        for name in &self.bone_names {
            name.write(buf);
        }
    }
}

#[derive(BinRead, Serialize, Deserialize)]
#[br(magic = b"\x00\x00\x00\x00")]
pub struct CharaActionData {
    file_header: FileHeader,
    data_header: CharaActionDataActionHeader,

    #[br(count = data_header.action_count)]
    pub frame: Vec<CharaActionDataActionData>,

    #[br(args(data_header.action_count as usize, 0))]
    #[br(parse_with = name_parser)]
    pub act_names: Vec<Name>,

    name_list: NameList,
}

impl CharaActionData {
    fn write_action_count(&mut self) {
        self.data_header.action_count = self.frame.len() as i32;
    }
    fn write_action_lines(&mut self) {
        for action_data in &mut self.frame {
            action_data.info.line_count = action_data.frame.len() as i32;
            for frame in &mut action_data.frame {
                frame.key_frame_count = frame.frame.len() as i32;
                for line in &mut frame.frame {
                    match &mut line.line {
                        Line::BaseAnime(base_anime) => {
                            let v = bincode::serialize(&base_anime).unwrap();
                            let buf: [u8; 36] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..36].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::FaceAnime(face_anime) => {
                            let v = bincode::serialize(&face_anime).unwrap();
                            let buf: [u8; 16] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..16].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::UnkAnime(unk_anime) => {
                            let v = bincode::serialize(&unk_anime).unwrap();
                            let buf: [u8; 16] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..16].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::Collision(collision) => {
                            let v = bincode::serialize(&collision).unwrap();
                            let buf: [u8; 40] = v.try_into().expect("incorrect size!");
                            line.data = unsafe { std::mem::transmute(buf) };
                        }
                        Line::UnkCollision(unk_collision) => {
                            let v = bincode::serialize(&unk_collision).unwrap();
                            let buf: [u8; 40] = v.try_into().expect("incorrect size!");
                            line.data = unsafe { std::mem::transmute(buf) };
                        }
                        Line::Attack(attack) => {
                            let v = bincode::serialize(&attack).unwrap();
                            let buf: [u8; 28] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..28].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::Cancel(cancel) => {
                            let v = bincode::serialize(&cancel).unwrap();
                            let buf: [u8; 24] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..24].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::Branch(branch) => {
                            let v = bincode::serialize(&branch).unwrap();
                            let buf: [u8; 24] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..24].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::Move(line_move) => {
                            let v = bincode::serialize(&line_move).unwrap();
                            let buf: [u8; 20] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..20].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::Offset(offset) => {
                            let v = bincode::serialize(&offset).unwrap();
                            let buf: [u8; 12] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..12].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::Speed(offset) => {
                            let v = bincode::serialize(&offset).unwrap();
                            let buf: [u8; 24] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..24].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::ActionFlag(action_flag) => {
                            let v = bincode::serialize(&action_flag).unwrap();
                            let buf: [u8; 8] = v.try_into().expect("incorrect size!");
                            let mut new_buf: [u8; 40] = [0; 40];
                            new_buf[..8].clone_from_slice(&buf);
                            line.data = unsafe { std::mem::transmute(new_buf) };
                        }
                        Line::Effect(effect) => {
                            match &mut effect.eff_type {
                                EffectType::Set(set) => {
                                    let mut v = Vec::new();
                                    for _ in 0..4 {
                                        v.push(0);
                                    }
                                    v.append(&mut bincode::serialize(&set).unwrap());
                                    let buf: [u8; 40] = v.try_into().expect("incorrect size!");
                                    line.data = unsafe { std::mem::transmute(buf) };        
                                }
                                EffectType::Control(control) => {
                                    let mut v = Vec::new();
                                    v.push(1);
                                    for _ in 0..3 {
                                        v.push(0);
                                    }
                                    v.append(&mut bincode::serialize(&control).unwrap());
                                    let buf: [u8; 40] = v.try_into().expect("incorrect size!");
                                    line.data = unsafe { std::mem::transmute(buf) };        
                                }
                            }
                        }
                        Line::Sound(sound) => {
                            match sound.sound {
                                SoundType::Play(play) => {
                                    let v = bincode::serialize(&play).unwrap();
                                    let buf: [u8; 24] = v.try_into().expect("incorrect size!");
                                    sound.data = unsafe { std::mem::transmute(buf) };
                                    sound.sound_type = 0;
                                }
                                SoundType::Stop(stop) => {
                                    let v = bincode::serialize(&stop).unwrap();
                                    let buf: [u8; 24] = v.try_into().expect("incorrect size!");
                                    sound.data = unsafe { std::mem::transmute(buf) };
                                    sound.sound_type = 1;
                                }
                                SoundType::Unk(unk) => {
                                    let v = bincode::serialize(&unk).unwrap();
                                    let buf: [u8; 24] = v.try_into().expect("incorrect size!");
                                    sound.data = unsafe { std::mem::transmute(buf) };
                                    sound.sound_type = 2;
                                }
                                SoundType::DefaultSound(default) => {
                                    let v = bincode::serialize(&default).unwrap();
                                    let buf: [u8; 24] = v.try_into().expect("incorrect size!");
                                    sound.data = unsafe { std::mem::transmute(buf) };
                                    sound.sound_type = 0;
                                }
                            }
                            let data: [u32; 6] = sound.data;
                            let mut v: Vec<u8> = vec_u32_to_u8(data.to_vec());
                            let mut sound_type: Vec<u8> = sound.sound_type.to_le_bytes().to_vec();
                            v.append(&mut sound_type);
                            let mut unk1: Vec<u8> = sound.unk1.to_le_bytes().to_vec();
                            v.append(&mut unk1);
                            let mut unk2: Vec<u8> = sound.unk2.to_le_bytes().to_vec();
                            v.append(&mut unk2);
                            let mut unk3: Vec<u8> = sound.unk3.to_le_bytes().to_vec();
                            v.append(&mut unk3);
                            let buf: [u8; 40] = v.try_into().expect("incorrect size!");
                            line.data = unsafe { std::mem::transmute(buf) };
                        }
                        Line::DefaultLine(data) => {
                            line.data = *data;
                        }
                    }
                }
            }
        }
    }
    fn prepare_write(&mut self) {
        self.write_action_count();
        self.write_action_lines();
        self.name_list.prepare_write();
    }
    pub fn write(&mut self, buf: &mut Vec<u8>) {
        self.prepare_write();
        buf.write_u32::<LE>(0).unwrap();
        self.file_header.write(buf);
        self.data_header.write(buf);
        for frame in &self.frame {
            frame.write(buf);
        }
        for name in &mut self.act_names {
            name.prepare_write();
            name.write(buf);
        }
        self.name_list.write(buf);
        self.data_header.anim_offset = self.name_list.anim_offset;
        self.data_header.anim_offset_2 = self.name_list.anim_offset;
        self.data_header.camera_offset = self.name_list.camera_offset;
        self.data_header.bone_offset = self.name_list.bone_offset;

        let mut new_data_header: Vec<u8> = Vec::new();
        self.data_header.write(&mut new_data_header);
        buf.splice(0xC..0x24, new_data_header);
    }
}