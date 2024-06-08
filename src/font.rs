#[derive(Debug)]
pub enum ParseError {
    WrongSize,
    NoMoreData,
    FailToParse,
    WrongMagicNumber,
    InvalidValue,
    NotMonospaced,
    LibTrueTypeNotFound,
}

pub struct GlyphMetrics {
    pub width: u32,
    pub x_offset: u32,
    pub y_offset: u32,
}

pub struct TrueTypeFont {
    pub texture_atlas: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub metrics: Vec<GlyphMetrics>,
    pub line_height: u32,
    pub x_ratio: f32,
    pub scale: f32,
}

fn add_bitmap_to_atlas(
    x_offset: usize,
    y_offset: usize,
    bitmap: &[u8],
    texture_width: usize,
    width: usize,
    height: usize,
    atlas: &mut [u8]
) {
    for j in 0..height {
        let line_offset = (j + y_offset) * texture_width;
        for i in 0..width {
            atlas[x_offset + i + line_offset] = bitmap[i + j * width];
        }
    }
}

const PADDING: u32 = 2;

pub fn init(path: &str, code_points: &[u8], size: u8) -> Result<TrueTypeFont, ParseError> {
    let lib = freetype::Library::init().unwrap();
    let face = lib.new_face(path, 0).unwrap();

    face.set_pixel_sizes(size as u32, 0).unwrap();

    let scale = size as f32 / face.height() as f32;
    let ascender = (face.ascender() as f32 * scale).round() as i32;
    let max_advance = (face.max_advance_width() as f32 * scale).round() as u32;
    let line_height = (scale * face.height() as f32) as u32;

    let x_ratio = max_advance as f32 / line_height as f32;

    let (glyphs_per_row, line_count) = {
        let len = code_points.len() as f32;
        let height: f32 = len.sqrt().floor();
        ((len as f32 / height).ceil() as u32, height as u32)
    };

    let texture_width = (glyphs_per_row + PADDING) * max_advance;
    let texture_height = (line_height + PADDING) * line_count;

    let mut metrics = Vec::new();
    let mut texture: Vec<u8> = vec![0; (texture_width * texture_height) as usize];

    let mut x_offset = 0;
    let mut y_offset = 0;
    let mut i = 0;

    for code_point in code_points.iter() {
        face.load_char(*code_point as usize, freetype::face::LoadFlag::RENDER).unwrap();

        let glyph = face.glyph();
        let bitmap = glyph.bitmap();
        let buffer = bitmap.buffer();
        let width = bitmap.width();
        let height = bitmap.rows();
        let metric = glyph.metrics();
        let advance = max_advance;

        let left_bearing = (metric.horiBearingX as f32 * scale).round() as u32;
        let topmost = glyph.bitmap_top();
        let top_offset = ascender - topmost;

        add_bitmap_to_atlas(
            (left_bearing + x_offset) as usize,
            (top_offset + y_offset as i32) as usize,
            &buffer,
            texture_width as usize,
            width as usize,
            height as usize,
            &mut texture,
        );

        metrics.push(GlyphMetrics {
            width: advance,
            x_offset,
            y_offset,
        });

        x_offset += advance + PADDING;

        i += 1;
        if i == glyphs_per_row {
            i = 0;

            y_offset += line_height + PADDING;
            x_offset = 0;
        }
    }

    Ok(TrueTypeFont {
        texture_atlas: texture,
        width: texture_width,
        height: texture_height,
        metrics,
        line_height,
        x_ratio,
        scale
    })
}
// use std::io::{ Seek, Read, BufReader};
// use crate::binding::dl;

// const INTERPOLATIONS: u32 = 4;

// #[allow(dead_code)]
// struct Table {
//     name: [u8; 4],
//     checksum: u32,
//     offset: u32,
//     length: u32,
// }

// struct Box {
//     x_min: i16,
//     x_max: i16,
//     y_min: i16,
//     y_max: i16,
// }

// #[derive(Clone)]
// struct Contour {
//     lines: Vec<ContourLine>,
//     clock_wise_winding: bool,
// }

// #[derive(Clone)]
// struct ContourLine {
//     start: [u32; 2],
//     end: [u32; 2],
// }

// struct Glyph {
//     contours: Vec<Contour>,
//     boundary: Box,
//     left_bearing: u32,
//     advance: u32,
// }

// #[derive(Clone)]
// struct Point {
//     pos: [i16; 2],
//     on_curve: bool,
// }

// struct Header {
//     units_pem: u32,
//     index_to_loc_format: u32,
//     boundary: Box,
// }

// struct Info {
//     index_to_loc: u32,
//     loca_offset: u32,
//     glyf_offset: u32,
//     hmtx_offset: u32,
//     h_metrics_count: u32,
//     cvt: Vec<u32>,
// }

// enum TableType {
//     Map,
//     Glyph,
//     Header,
//     Location,
//     HorizontalHeader,
//     HorizontalMetrics,
//     ControlValue,
// }

// impl Table {
//     fn new(reader: &mut BufReader<std::fs::File>) -> Result<Table, ParseError> {
//         let mut name: [u8; 4] = [0; 4];
//         if 4 != reader.read(&mut name).map_err(|_| ParseError::NoMoreData)? {
//             return Err(ParseError::NoMoreData);
//         }

//         Ok(Table {
//             name,
//             checksum: read(reader, 4)?,
//             offset: read(reader, 4)?,
//             length: read(reader, 4)?,
//         })
//     }
// }


// fn new_header(reader: &mut BufReader<std::fs::File>) -> Result<Header, ParseError> {
//     reader.seek(std::io::SeekFrom::Current(12)).map_err(|_| ParseError::WrongSize)?;
//     let magic_number = read(reader, 4)?;
//     reader.seek(std::io::SeekFrom::Current(2)).map_err(|_| ParseError::WrongSize)?;
//     let units_pem = read(reader, 2)? as u32;
//     reader.seek(std::io::SeekFrom::Current(16)).map_err(|_| ParseError::WrongSize)?;

//     let x_min = read(reader, 2)? as i16;
//     let y_min = read(reader, 2)? as i16;
//     let x_max = read(reader, 2)? as i16;
//     let y_max = read(reader, 2)? as i16;

//     reader.seek(std::io::SeekFrom::Current(6)).map_err(|_| ParseError::WrongSize)?;

//     let index_to_loc_format = read(reader, 2)? as u32;

//     if 0x5f0f3cf5 != magic_number {
//         Err(ParseError::WrongMagicNumber)
//     } else {
//         Ok(Header {
//             units_pem,
//             index_to_loc_format,
//             boundary: Box {
//                 x_min,
//                 x_max,
//                 y_min,
//                 y_max,
//             }
//         })
//     }
// }

// struct Map {
//     start_code: Vec<u32>,
//     end_code: Vec<u32>,
//     glyph_code: Vec<u32>,
// }

// fn new_map(reader: &mut BufReader<std::fs::File>) -> Result<Map, ParseError> {
//     if 0 != read(reader, 2).unwrap() {
//         return Err(ParseError::WrongMagicNumber);
//     }

//     reader.seek(std::io::SeekFrom::Current(8)).map_err(|_| ParseError::NoMoreData)?;
//     let group_count = read(reader, 4)?;

//     let mut start_code = Vec::with_capacity(group_count as usize);
//     let mut end_code = Vec::with_capacity(group_count as usize);
//     let mut glyph_code = Vec::with_capacity(group_count as usize);

//     for _ in 0..group_count {
//         start_code.push(read(reader, 4)?);
//         end_code.push(read(reader, 4)?);
//         glyph_code.push(read(reader, 4)?);
//     }

//     Ok(Map {
//         start_code,
//         end_code,
//         glyph_code,
//     })
// }

// fn get_glyph_index(cmap: &Map, code_point: u8) -> u32 {
//     let code_point = code_point as u32;

//     for i in 0..cmap.start_code.len() {
//         if cmap.start_code[i] <= code_point && cmap.end_code[i] >= code_point {
//             let offset = code_point - cmap.start_code[i];
//             return cmap.glyph_code[i] + offset;
//         }
//     }

//     0
// }

// fn get_name_type(name: &[u8; 4]) -> Result<TableType, ParseError> {
//     match name {
//         [b'c', b'm', b'a', b'p'] => Ok(TableType::Map),
//         [b'g', b'l', b'y', b'f'] => Ok(TableType::Glyph),
//         [b'h', b'e', b'a', b'd'] => Ok(TableType::Header),
//         [b'h', b'h', b'e', b'a'] => Ok(TableType::HorizontalHeader),
//         [b'h', b'm', b't', b'x'] => Ok(TableType::HorizontalMetrics),
//         [b'c', b'v', b't', b' '] => Ok(TableType::ControlValue),
//         [b'l', b'o', b'c', b'a'] => Ok(TableType::Location),
//         _ => Err(ParseError::FailToParse),
//     }
// }

// fn read(reader: &mut BufReader<std::fs::File>, len: usize) -> Result<u32, ParseError> {
//     let mut array: [u8; 4] = [0; 4];

//     if len != reader.read(&mut array[..len]).map_err(|_| ParseError::NoMoreData)? {
//         return Err(ParseError::NoMoreData);
//     }

//     let mut variable: u32 = 0;

//     for i in 0..len {
//         variable += (array[i] as u32) << (8 * (len - i - 1));
//     }

//     Ok(variable)
// }

// // pub fn init(file_path: &str, code_points: &[u8], size: u8) -> Result<TrueTypeFont, ParseError> {
// //     let lib = freetype::Library::init().unwrap();
// //     let face = lib.new_face(file_path, 0).unwrap();
// //     face.set_pixel_sizes(size as u32, 0).unwrap();
// //     let scale = size as f32 / face.em_size() as f32;
//     // for code_point in code_points.iter() {
//     //     face.load_char('A' as usize, freetype::face::LoadFlag::RENDER).unwrap();
//     //     let mut metrics = Vec::new();
//     //     let glyph = face.glyph();
//     //     let bitmap = glyph.bitmap();
//     //     let buffer = bitmap.buffer();
//     //     let width = bitmap.width();
//     //     let height = bitmap.rows();
//     //     let line_height = (scale * face.height() as f32) as u32;
//     //     let advance = (glyph.metrics().horiAdvance as u32 as f32 * scale) as u32;
//     //     println!("{} {} {}, scale: {}, line height: {} ?", width, height, advance, scale, line_height);
//     //     metrics.push(GlyphMetrics {
//     //         width: advance,
//     //         x_offset: 0,
//     //         y_offset: 0,
//     //     });
//     // }
//     // println!("width: {}, height: {}", width, height);
//     // Err(ParseError::FailToParse)
//     // let file = std::fs::File::open(file_path).map_err(|_| ParseError::FailToParse)?;
//     // let mut reader = BufReader::new(file);

//     // let mut cmap: Option<Map> = None;
//     // let mut header: Option<Header> = None;
//     // let mut info = Info {
//     //     index_to_loc: 0,
//     //     loca_offset: 0,
//     //     glyf_offset: 0,
//     //     hmtx_offset: 0,
//     //     h_metrics_count: 0,
//     //     cvt: Vec::with_capacity(10),
//     // };

//     // reader.seek(std::io::SeekFrom::Current(4)).unwrap();
//     // let num_tables = read(&mut reader, 2)?;

//     // reader.seek(std::io::SeekFrom::Current(6)).unwrap();
//     // let pos = reader.stream_position().unwrap();

//     // for i in 0..num_tables {
//     //     reader.seek(std::io::SeekFrom::Start(pos + (i as usize * std::mem::size_of::<Table>()) as u64)).unwrap();

//     //     let table = Table::new(&mut reader)?;

//     //     if let Ok(typ) = get_name_type(&table.name) {
//     //         match typ {
//     //             TableType::Map => {
//     //                 reader.seek(std::io::SeekFrom::Start(table.offset as u64 + 2)).unwrap();
//     //                 let number_subtables = read(&mut reader, 2)?;

//     //                 for k in 0..number_subtables {
//     //                     reader.seek(std::io::SeekFrom::Start(table.offset as u64 + 8 * k as u64 + 4)).unwrap();

//     //                     let id = read(&mut reader, 2)?;
//     //                     let specific_id = read(&mut reader, 2)?;
//     //                     let offset = read(&mut reader, 4)?;

//     //                     if id != 0 && specific_id != 0 && specific_id != 4 && specific_id != 3 {
//     //                         continue;
//     //                     }

//     //                     reader.seek(std::io::SeekFrom::Start(table.offset as u64 + offset as u64)).unwrap();

//     //                     if read(&mut reader, 2)? == 12 {
//     //                         cmap = new_map(&mut reader).ok();
//     //                         break;
//     //                     }
//     //                 }
//     //             },
//     //             TableType::Header => {
//     //                 reader.seek(std::io::SeekFrom::Start(table.offset as u64)).unwrap();
//     //                 header = new_header(&mut reader).ok();
//     //             },
//     //             TableType::Location => {
//     //                 info.loca_offset = table.offset;
//     //             },
//     //             TableType::Glyph => {
//     //                 info.glyf_offset = table.offset;
//     //             },
//     //             TableType::HorizontalHeader => {
//     //                 reader.seek(std::io::SeekFrom::Start(table.offset as u64 + 34)).unwrap();
//     //                 info.h_metrics_count = read(&mut reader, 2).unwrap();
//     //             },
//     //             TableType::HorizontalMetrics => {
//     //                 info.hmtx_offset = table.offset;
//     //             },
//     //             TableType::ControlValue => {
//     //                 reader.seek(std::io::SeekFrom::Start(table.offset as u64)).unwrap();

//     //                 for _ in 0..table.length / 4 {
//     //                     info.cvt.push(read(&mut reader, 4).unwrap());
//     //                 }
//     //             },
//     //         }
//     //     }
//     // }

//     // if let (Some(cmap), Some(header)) = (cmap, header) {
//     //     let scale = size as f32 / header.units_pem as f32;
//     //     let line_height = (header.boundary.y_max - header.boundary.y_min) as u32;

//     //     info.index_to_loc = header.index_to_loc_format;

//         // let (glyphs_per_row, line_count) = {
//         //     let len = code_points.len() as f32;
//         //     let height: f32 = len.sqrt().floor();
//         //     ((len as f32 / height).ceil() as u32, height as u32)
//         // };

//         // let mut texture_width = 0;

//         // let mut x_offset = 0;
//         // let mut y_offset = 0;
//         // let mut max_width = 0;
//         // let mut i = 0;

//     //     let mut metrics: Vec<GlyphMetrics> = Vec::with_capacity(code_points.len());
//     //     let mut glyphs: Vec<Glyph> = Vec::with_capacity(code_points.len());


//         // let line_height = (scale * face.height() as f32) as u32;
//         // face.load_char('a' as usize, freetype::face::LoadFlag::RENDER).unwrap();
//         // let max_advance = (face.glyph().metrics().horiAdvance as f32 * scale) as u32;

//         // let mut metrics = Vec::new();
//         // let mut texture: Vec<u8> = vec![0; code_points.len() * (max_advance * line_height) as usize];
//         // let descender = (face.descender() as f32 * scale) as i32;
//         // let ascender = (face.ascender() as f32 * scale) as i32;

//         // for code_point in code_points.iter() {
//             // let index = get_glyph_index(&cmap, *code_point);
//         //     face.load_char(*code_point as usize, freetype::face::LoadFlag::RENDER).unwrap();
//         //     let glyph = face.glyph();
//         //     let bitmap = glyph.bitmap();
//         //     let buffer = bitmap.buffer();
//         //     let width = bitmap.width();
//         //     let height = bitmap.rows();
//         //     let scale = size as f32 / face.em_size() as f32;
//         //     let metric = glyph.metrics();
//         //     let advance = (metric.horiAdvance as f32 * scale) as u32;
//         //     let left_bearing = (metric.horiBearingX as f32 * scale) as u32;
//             // println!("{} {} {}, scale: {}, line height: {} ?", width, height, advance, scale, line_height);
//             // metrics.push(GlyphMetrics {
//             //     width: advance,
//             //     x_offset: 0,
//             //     y_offset: 0,
//             // });

//             // let glyph = new_glyph(
//             //     &mut reader,
//             //     &info,
//             //     index,
//             //     scale,
//             // );

//         //     let bearing = (metric.horiBearingY as f32 * scale) as usize;
//         //     println!("ascender: {}, height: {}, horiBearing: {}, advance: {}", descender, height, bearing, advance);
//         //      let l = (bearing as i32 - height  as i32) - descender;
//         //     add_bitmap_to_atlas(
//         //         (left_bearing + x_offset) as usize,
//         //         l as usize + y_offset as usize,
//         //         &buffer,
//         //         width as usize,
//         //         height as usize,
//         //         &mut texture,
//         //     );
//         //     metrics.push(GlyphMetrics {
//         //         width: advance,
//         //         x_offset,
//         //         y_offset,
//         //     });

//         //     x_offset += advance;

//         //     i += 1;
//         //     if i == glyphs_per_row {
//         //         i = 0;

//         //         if x_offset > max_width {
//         //             max_width = x_offset;
//         //             texture_width = max_width;
//         //         }

//         //         y_offset += line_height;
//         //         x_offset = 0;
//         //     }

//         // }

//         // Ok(TrueTypeFont {
//         //     texture_atlas: texture,
//         //     width: texture_width,
//         //     height: line_height * line_count,
//         //     metrics,
//         //     line_height,
//         //     x_ratio: 0.5,
//         //     scale
//         // })

//     //     let texture_width = (texture_width as f32 * scale) as u32;
//     //     let texture_height = ((line_count * line_height ) as f32 * scale) as u32;

//     //     let mut texture: Vec<u8> = vec![0; (texture_width * texture_height) as usize];

//     //     for (i, glyph) in glyphs.iter().enumerate() {
//     //         let bottom_padding = (glyph.boundary.y_min - header.boundary.y_min) as u32;

//     //         add_glyph_to_bitmap(
//     //             &mut texture,
//     //             texture_width,
//     //             glyph,
//     //             scale,
//     //             [
//     //                 metrics[i].x_offset + glyph.left_bearing,
//     //                 metrics[i].y_offset + bottom_padding
//     //             ]
//     //         );
//     //     }

//     //     let x_ratio = metrics[0].width as f32 / line_height as f32;

//     //     Ok(TrueTypeFont {
//     //         texture_atlas: texture,
//     //         width: texture_width,
//     //         height: texture_height,
//     //         metrics,
//     //         line_height,
//     //         x_ratio,
//     //         scale,
//     //     })
//     // } else {
//     //     Err(ParseError::FailToParse)
//     // }
// // }

// fn goto_glyph_offset(reader: &mut BufReader<std::fs::File>, info: &Info, code_point: u32) {
//     let translate = info.index_to_loc * 2;
//     reader.seek(std::io::SeekFrom::Start((info.loca_offset + code_point * (translate + 2)) as u64)).unwrap();
//     let offset = read(reader, 2 + translate as usize).unwrap() * (((info.index_to_loc + 1) % 2) + 1);
//     reader.seek(std::io::SeekFrom::Start((offset + info.glyf_offset) as u64)).unwrap();
// }

// fn get_padding_metrics(reader: &mut BufReader<std::fs::File>, info: &Info, index: u32) -> [u32; 2] {
//     if index < info.h_metrics_count {
//         reader.seek(std::io::SeekFrom::Start((info.hmtx_offset + 4 * index) as u64)).unwrap();
//         let advance = read(reader, 2).unwrap();
//         let left_bearing = read(reader, 2).unwrap();

//         [advance, left_bearing]
//     } else {
//         reader.seek(std::io::SeekFrom::Start(info.hmtx_offset as u64 + 4 * (info.h_metrics_count - 1) as u64)).unwrap();
//         let advance = read(reader, 2).unwrap();
//         reader.seek(std::io::SeekFrom::Current(2)).unwrap();
//         reader.seek(std::io::SeekFrom::Current(2 * (index - info.h_metrics_count) as i64)).unwrap();
//         let left_bearing = read(reader, 2).unwrap();

//         [advance, left_bearing]
//     }
// }

// fn new_glyph(
//     reader: &mut BufReader<std::fs::File>,
//     info: &Info,
//     code_point: u32,
//     scale: f32,
// ) -> Glyph {
//     let padding_metrics = get_padding_metrics(reader, info, code_point);

//     goto_glyph_offset(reader, info, code_point);
//     let number_of_contours = read(reader, 2).unwrap() as i16;

//     let boundary = Box {
//         x_min: read(reader, 2).unwrap() as i16,
//         y_min: read(reader, 2).unwrap() as i16,
//         x_max: read(reader, 2).unwrap() as i16,
//         y_max: read(reader, 2).unwrap() as i16,
//     };

//     if number_of_contours < 0 {
//         let mut flag = MORE_COMPONENTS;

//         let mut simple_glyph = Glyph {
//             contours: Vec::new(),
//             boundary: Box {
//                 x_min: boundary.x_min,
//                 y_min: boundary.y_min,
//                 x_max: boundary.x_max,
//                 y_max: boundary.y_max,
//             },
//             advance: padding_metrics[0],
//             left_bearing: padding_metrics[1],
//         };

//         while flag & MORE_COMPONENTS != 0 {
//             flag = read(reader, 2).unwrap() as u16;

//             let index = read(reader, 2).unwrap();
//             let matrix = read_compound_glyph(reader, flag);
//             let pos = reader.stream_position().unwrap();

//             goto_glyph_offset(reader, info, index);
//             let number_of_contours = read(reader, 2).unwrap() as i16;
//             reader.seek(std::io::SeekFrom::Current(8)).unwrap();

//             if number_of_contours > 0 {
//                 let center_offset = if flag & USE_MY_METRICS != 0 {
//                     [0, 0]
//                 } else {
//                     [matrix[4], matrix[5]]
//                 };

//                 let glyph = read_simple_glyph(




//                     reader,
//                     number_of_contours,
//                     &boundary,
//                     center_offset,
//                     &info.cvt,
//                     scale,
//                 ).unwrap();

//                 simple_glyph.contours.extend_from_slice(&glyph.contours);
//             }

//             reader.seek(std::io::SeekFrom::Start(pos)).unwrap();
//         }

//         simple_glyph
//     } else {
//         let mut glyph = read_simple_glyph(
//             reader,
//             number_of_contours,
//             &boundary,
//             [0, 0],
//             &info.cvt,
//             scale,
//         ).unwrap();

//         glyph.left_bearing = padding_metrics[1];
//         glyph.advance = padding_metrics[0];

//         glyph
//     }
// }

// const ON_CURVE: u8 = 0x01;
// const X_IS_SHORT: u8 = 0x02;
// const Y_IS_SHORT: u8 = 0x04;
// const REPEAT: u8 = 0x08;
// const X_IS_SAME: u8 = 0x10;
// const Y_IS_SAME: u8 = 0x20;

// const RP0: usize = 0;
// const RP1: usize = 1;
// const RP2: usize = 2;

// const ZP0: usize = 3;
// const ZP1: usize = 4;
// const ZP2: usize = 5;

// struct Stack {
//     handle: [u32; 100],
//     registers: [u32; 6],
//     len: usize,
// }

// impl Stack {
//     fn pop(&mut self) -> u32 {
//         self.len -= 1;
//         self.handle[self.len]
//     }

//     fn push(&mut self, v: u32) {
//         self.handle[self.len] = v;
//         self.len += 1;
//     }
// }

// fn miap(stack: &mut Stack, control_value_table: &[u32], code: u32) {
//     let n = stack.pop();
//     let p = stack.pop();
//     let distance = control_value_table[n as usize] as f32 / 64 as f32;

//     let distance = if code & 0x01 != 0 {
//         distance.round() as i16
//     } else {
//         distance as i16
//     };
// }

// fn npushb(reader: &mut BufReader<std::fs::File>, stack: &mut Stack) {
//     let n = read(reader, 1).unwrap();

//     for _ in 0..n {
//         stack.push(read(reader, 1).unwrap());
//     }
// }

// fn pushb(reader: &mut BufReader<std::fs::File>, stack: &mut Stack, code: u32) {
//     let n = code & 0b111;
//     for _ in 0..n + 1 {
//         stack.push(read(reader, 1).unwrap());
//     }
// }

// fn pushw(reader: &mut BufReader<std::fs::File>, stack: &mut Stack, code: u32) {
//     let n = code & 0b111;

//     for _ in 0..n + 1 {
//         let byte = read(reader, 2).unwrap();
//         stack.push(byte);
//     }
// }

// fn mirp(
//     stack: &mut Stack,
//     code: u32,
//     control_value_table: &[u32],
//     points: &[Point],
// ) {
//     let n = stack.pop();
//     let p = stack.pop();

//     if code & 0b00010000 != 0 {
//         stack.registers[RP0] = p;
//     }

//     if code & 0b00001000 != 0 {
//         println!("do not keep distance greater that or equal to minimum distance");
//     } else {
//         println!("keep distance greater than or equal to minimum distance");
//     }

//     let distance = if code & 0b00000100 != 0 {
//         println!("do not round the distance and do not look at the control value cut-in");
//         code & 0b00000011
//     } else {
//         println!("round the distance and look at the control cut-in value");
//         control_value_table[n as usize]
//     };
// }

// fn svtca(
//     projection_vector_index: &mut usize,
//     freedom_vector_index: &mut usize,
//     code: u32,
// ) {
//     if code & 0x01 == 0 {
//         *projection_vector_index = 1;
//         *freedom_vector_index = 1;
//     } else {
//         *projection_vector_index = 0;
//         *freedom_vector_index = 0;
//     }
// }

// fn grid_fit(
//     reader: &mut BufReader<std::fs::File>,
//     points: &[Point],
//     control_value_table: &[u32],
//     instructions_length: u32,
// ) {
//     let mut freedom_vector_index: usize = 0;
//     let mut projection_vector_index: usize = 0;

//     let mut stack = Stack {
//         handle: [0; 100],
//         registers: [0; 6],
//         len: 0,
//     };

//     for i in 0..instructions_length {
//         let code = read(reader, 1).unwrap();
//         match code {
//             0x00 | 0x01 => svtca(&mut freedom_vector_index, &mut projection_vector_index, code),
//             0x3E..=0x3F => miap(&mut stack, control_value_table, code),
//             0x40 => npushb(reader, &mut stack),
//             0xB8..=0xBF => pushw(reader, &mut stack, code),
//             0xB0..=0xB7 => pushb(reader, &mut stack, code),
//             // 0xE0..=0xFF => mirp(&mut stack, code, control_value_table, points),
//             a => {
//                 println!("function: {:#x}", a);
//                 break;
//             }
//         }
//     }
// }

// fn read_simple_glyph(
//     reader: &mut BufReader<std::fs::File>,
//     number_of_contours: i16,
//     boundary: &Box,
//     center_offset: [i16; 2],
//     control_value_table: &[u32],
//     scale: f32,
// ) -> Result<Glyph, ParseError> {
//     let mut contour_ends: Vec<u16> = Vec::with_capacity(number_of_contours as usize);

//     for _ in 0..number_of_contours {
//         contour_ends.push(read(reader, 2)? as u16);
//     }

//     let contour_max: u16 = contour_ends[contour_ends.len() - 1] + 1;
//     let instructions_length = read(reader, 2)?;
//     let instructions_offset = reader.stream_position().unwrap();

//     reader.seek(std::io::SeekFrom::Current(instructions_length as i64)).unwrap();

//     let mut flags: Vec<u8> = Vec::with_capacity(contour_max as usize);
//     let mut i: u16 = 0;

//     while i < contour_max {
//         let flag = read(reader, 1).unwrap() as u8;

//         if flag & REPEAT != 0 {
//             let repeat_count = read(reader, 1).unwrap() as u8;

//             flags.extend_from_slice(&vec![flag; repeat_count as usize + 1]);
//             i += repeat_count as u16;
//         } else {
//             flags.push(flag)
//         }

//         i += 1;
//     }

//     let mut points: Vec<Point> = Vec::with_capacity(contour_max as usize);
//     let mut x_value: i16 = 0;

//     for i in 0..contour_max {
//         let i = i as usize;

//         if flags[i] & X_IS_SHORT != 0 {
//             let value = read(reader, 1).unwrap() as u8;

//             if flags[i] & X_IS_SAME != 0 {
//                 x_value += value as i16;
//             } else {
//                 x_value -= value as i16;
//             }
//         } else if flags[i] & X_IS_SAME == 0 {
//             x_value += read(reader, 2).unwrap() as i16;
//         }

//         points.push(Point {
//             pos: [((x_value - boundary.x_min + center_offset[0]) as f32 * scale) as i16, 0],
//             on_curve: flags[i] & ON_CURVE != 0,
//         });
//     }

//     let mut y_value: i16 = 0;
//     for i in 0..contour_max {
//         let i = i as usize;

//         if flags[i] & Y_IS_SHORT != 0 {
//             let value = read(reader, 1).unwrap() as u8;

//             if flags[i] & Y_IS_SAME != 0 {
//                 y_value += value as i16;
//             } else {
//                 y_value -= value as i16;
//             }
//         } else if flags[i] & Y_IS_SAME == 0 {
//             y_value += read(reader, 2).unwrap() as i16;
//         }

//         points[i].pos[1] = ((y_value - boundary.y_min + center_offset[1]) as f32 * scale) as i16;
//     }

//     reader.seek(std::io::SeekFrom::Start(instructions_offset as u64)).unwrap();
//     grid_fit(reader, &mut points, control_value_table, instructions_length);

//     let mut control_points: [[u32; 2]; 10] = [[0; 2]; 10];
//     let mut contour_start: u8 = 0;
//     let mut contours: Vec<Contour> = Vec::with_capacity(contour_ends.len());

//     for contour_end in contour_ends.iter() {
//         let mut contour_winding_sum: i32 = 0;
//         let mut lines: Vec<ContourLine> = Vec::with_capacity(points.len());

//         for i in contour_start..*contour_end as u8 + 1 {
//             if !points[i as usize].on_curve {
//                 continue;
//             }

//             let mut index_of_next = if i == *contour_end as u8 {
//                 contour_start
//             } else {
//                 i + 1
//             };

//             let mut control_points_count: usize = 0;
//             while !points[index_of_next as usize].on_curve {
//                 control_points[control_points_count] = [
//                     points[index_of_next as usize].pos[0] as u32,
//                     points[index_of_next as usize].pos[1] as u32,
//                 ];

//                 control_points_count += 1;

//                 if index_of_next >= *contour_end as u8 {
//                     index_of_next = contour_start;
//                 } else {
//                     index_of_next += 1;
//                 }
//             }

//             let line = ContourLine {
//                 start: [points[i as usize].pos[0] as u32, points[i as usize].pos[1] as u32],
//                 end: [points[index_of_next as usize].pos[0] as u32, points[index_of_next as usize].pos[1] as u32],
//             };

//             contour_winding_sum += shoelace(&line);

//             if control_points_count == 0 {
//                 lines.push(line);
//             } else {
//                 extend_lines_from_bezier_curve(&line, &control_points[0..control_points_count], &mut lines);
//             }
//         }

//         let contour = Contour {
//             clock_wise_winding: contour_winding_sum > 0,
//             lines,
//         };

//         contours.push(contour);
//         contour_start = *contour_end as u8 + 1;
//     }

//     Ok(Glyph {
//         contours,
//         boundary: Box {
//             x_min: boundary.x_min,
//             y_min: boundary.y_min,
//             x_max: boundary.x_max,
//             y_max: boundary.y_max,
//         },
//         left_bearing: 0,
//         advance: 0,
//     })
// }

// const ARG_1_AND_2_ARE_WORDS: u16 = 0x0001;
// const ARGS_ARE_XY_VALUES: u16 = 0x0002;
// const WE_HAVE_A_SCALE: u16 = 0x0008;
// const MORE_COMPONENTS: u16 = 0x0020;
// const WE_HAVE_AN_X_AND_Y_SCALE: u16 = 0x0040;
// const WE_HAVE_A_TWO_BY_TWO: u16 = 0x0080;
// const WE_HAVE_INSTRUCTIONS: u16 = 0x0100;
// const USE_MY_METRICS: u16 = 0x0200;

// fn read_compound_glyph(
//     reader: &mut BufReader<std::fs::File>,
//     flag: u16,
// ) -> [i16; 6] {
//     let mut matrix: [i16; 6] = [1, 0, 0, 1, 0, 0];

//     if flag & ARGS_ARE_XY_VALUES == 0 {
//         return matrix;
//     }

//     if flag & ARG_1_AND_2_ARE_WORDS != 0 {
//         matrix[4] = read(reader, 2).unwrap() as i16;
//         matrix[5] = read(reader, 2).unwrap() as i16;
//     } else {
//         matrix[4] = read(reader, 1).unwrap() as i16;
//         matrix[5] = read(reader, 1).unwrap() as i16;
//     }

//     if flag & WE_HAVE_A_SCALE != 0 {
//         matrix[0] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
//         matrix[3] = matrix[0];
//     } else if flag & WE_HAVE_AN_X_AND_Y_SCALE != 0{
//         matrix[0] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
//         matrix[3] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
//     } else if flag & WE_HAVE_A_TWO_BY_TWO != 0 {
//         matrix[0] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
//         matrix[1] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
//         matrix[2] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
//         matrix[3] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
//     }

//     if flag & WE_HAVE_INSTRUCTIONS != 0 {
//         reader.seek(std::io::SeekFrom::Current(2)).unwrap();
//     }

//     matrix
// }

// fn add_glyph_to_bitmap(
//     bitmap: &mut Vec<u8>,
//     bitmap_width: u32,
//     glyph: &Glyph,
//     scale: f32,
//     quad_offset: [u32; 2],
// ) {
//     let offset = [(quad_offset[0] as f32 * scale).round() as u32, (quad_offset[1] as f32 * scale).round() as u32];
//     for contour in glyph.contours.iter() {
//         for line in contour.lines.iter() {
//             draw_line(line, bitmap_width, bitmap, offset);
//         }
//     }

//     // let mut points_to_fill: Vec<[usize; 2]> = Vec::with_capacity((width * height) as usize);
//     // for point in contours_inner_points.iter() {
//     //     points_to_fill.clear();
//     //     let mut last: usize = 0;

//     //     points_to_fill.push([point[0], point[1]]);
//     //     bitmap[point[0] + point[1]] = 255;

//     //     loop {
//     //         let right = [points_to_fill[last][0] + 1, points_to_fill[last][1]];
//     //         if bitmap[right[0] + right[1]] == 0 {
//     //             points_to_fill.push(right);
//     //             bitmap[right[0] + right[1]] = 255;
//     //         }

//     //         let left = [points_to_fill[last][0] - 1, points_to_fill[last][1]];
//     //         if bitmap[left[0] + left[1]] == 0 {
//     //             points_to_fill.push(left);
//     //             bitmap[left[0] + left[1]] = 255;
//     //         }

//     //         let down = [points_to_fill[last][0], points_to_fill[last][1] + bitmap_width as usize];
//     //         if bitmap[down[0] + down[1]] == 0 {
//     //             points_to_fill.push(down);
//     //             bitmap[down[0] + down[1]] = 255;
//     //         }

//     //         let up = [points_to_fill[last][0], points_to_fill[last][1] - bitmap_width as usize];
//     //         if bitmap[up[0] + up[1]] == 0 {
//     //             points_to_fill.push(up);
//     //             bitmap[up[0] + up[1]] = 255;
//     //         }

//     //         last += 1;

//     //         if last >= points_to_fill.len() {
//     //             break;
//     //         }
//     //     }
//     // }
// }

// #[inline(always)]
// fn pow(base: f32, expoent: u32) -> f32 {
//     if expoent == 0 {
//         1.0
//     } else {
//         base.powf(expoent as f32)
//     }
// }

// #[inline(always)]
// fn factorial(n: u32) -> u32 {
//     if n <= 1 {
//         1
//     } else {
//         n * factorial(n - 1)
//     }
// }

// #[inline(always)]
// fn shoelace(line: &ContourLine) -> i32 {
//     (line.end[0] as i32 - line.start[0] as i32) * (line.start[1] as i32 + line.end[1] as i32)
// }

// // #[inline(always)]
// // fn contour_winding(lines: &[ContourLine]) -> Winding {
// //     let first_line = &lines[0];

// //     let vec_line: [i32; 2] = [
// //         first_line.end[0] as i32 - first_line.start[0] as i32,
// //         first_line.end[1] as i32 - first_line.start[1] as i32,
// //     ];

// //     let mid_point_first_line = [
// //         ((first_line.start[0] + first_line.end[0]) as f32 / 2.0).round() as u32,
// //         ((first_line.start[1] + first_line.end[1]) as f32 / 2.0).round() as u32
// //     ];

// //     let right_vec_line: [i32; 2] = [
// //         vec_line[1],
// //         - vec_line[0],
// //     ];

// //     let right_len = vec_len(right_vec_line);

// //     let mut cross_end_of_line: Option<i32> = None;
// //     let mut cross_count = 0;
// //     for line in lines[1..].iter() {
// //         let from_middle_to_start = [line.start[0] as i32 - mid_point_first_line[0] as i32, line.start[1] as i32 - mid_point_first_line[1] as i32, ];
// //         let from_middle_to_end = [line.end[0] as i32 - mid_point_first_line[0] as i32, line.end[1] as i32 - mid_point_first_line[1] as i32, ];

// //         let start_cos_theta = right_vec_line[0] * from_middle_to_start[0] + right_vec_line[1] * from_middle_to_start[1];
// //         let end_cos_theta = right_vec_line[0] * from_middle_to_end[0] + right_vec_line[1] * from_middle_to_end[1];

// //         let start_sin_theta = vec_line[0] * from_middle_to_start[0] + vec_line[1] * from_middle_to_start[1];
// //         let end_sin_theta = vec_line[0] * from_middle_to_end[0] + vec_line[1] * from_middle_to_end[1];

// //         if start_cos_theta <= 0 || end_cos_theta <= 0 {
// //             let sin_theta_start = start_sin_theta as f32 / (right_len * vec_len(from_middle_to_start)) as f32;
// //             let start_theta = find_theta(start_cos_theta, sin_theta_start);

// //             let sin_theta_end = end_sin_theta as f32 / (right_len * vec_len(from_middle_to_end)) as f32;
// //             let end_theta = find_theta(end_cos_theta, sin_theta_end);

// //             if start_theta + end_theta >= PI {
// //                 continue;
// //             }
// //         }

// //         if end_sin_theta == 0 || start_sin_theta == 0 {
// //             let current_line_vec = [
// //                 line.end[0] as i32 - line.start[0] as i32,
// //                 line.end[1] as i32 - line.start[1] as i32,
// //             ];

// //             let current_line_right_vec = [
// //                 current_line_vec[1],
// //                 - current_line_vec[0],
// //             ];

// //             let current_line_cos = current_line_right_vec[0] * from_middle_to_end[0] + current_line_right_vec[1] * from_middle_to_end[1];

// //             if let Some(cos) = cross_end_of_line {
// //                 if current_line_cos.signum() + cos.signum() != 0 {
// //                     cross_count += 1;
// //                 }

// //                 cross_end_of_line = None;
// //             } else {
// //                 cross_end_of_line = Some(current_line_cos);
// //             }

// //         } else if start_sin_theta.signum() + end_sin_theta.signum() == 0 {
// //             cross_count += 1;
// //         }

// //         // if let None = point_inside {
// //         //     if end_sin_theta != 0 || start_sin_theta != 0 {
// //         //         let current_line_vec = [
// //         //             line.end[0] as i32 - line.start[0] as i32,
// //         //             line.end[1] as i32 - line.start[1] as i32,
// //         //         ];

// //         //         if vec_len(current_line_vec) < 2.0 {
// //         //             continue;
// //         //         }

// //         //         let current_line_right_vec = [
// //         //             current_line_vec[1],
// //         //             - current_line_vec[0],
// //         //         ];

// //         //         let start = [
// //         //             ((line.start[0] + line.end[0]) as f32 / 2.0).round() as u32,
// //         //             ((line.start[1] + line.end[1]) as f32 / 2.0).round() as u32
// //         //         ];

// //         //         let len = vec_len(current_line_right_vec);
// //         //         let xm = current_line_right_vec[0] as f32 / len;
// //         //         let ym = current_line_right_vec[1] as f32 / len;

// //         //         let mut i: f32 = 0.0;

// //         //         let mut point = [start[0] as usize, start[1] as usize * bitmap_width];
// //         //         while bitmap[point[0] + point[1]] != 0 {
// //         //             i += 1.0;
// //         //             point = [(start[0] as f32 + i * xm) as usize, (start[1] as f32 + i * ym) as usize * bitmap_width];
// //         //         }

// //         //         point_inside = Some(point);
// //         //     }
// //         // }
// //     }


// //     let winding = if cross_count % 2 == 0 {
// //         Winding::CounterClockWise
// //     } else {
// //         Winding::ClockWise
// //     };

// //     winding
// // }

// #[inline(always)]
// fn extend_lines_from_bezier_curve(line: &ContourLine, control_points: &[[u32; 2]], lines: &mut Vec<ContourLine>) {
//     let mut previous_x: u32 = line.start[0];
//     let mut previous_y: u32 = line.start[1];
//     let mut coeficients: [[u32; 2]; 12] = [[0; 2]; 12];

//     let len = control_points.len();

//     coeficients[0] = [previous_x, previous_y];
//     coeficients[1..len as usize + 1].copy_from_slice(&control_points[0..len]);
//     coeficients[len + 1] = [line.end[0], line.end[1]];

//     let len = len as u32 + 2 - 1;

//     let interpolations_f32 = INTERPOLATIONS as f32;

//     for iter in 1..INTERPOLATIONS + 1 {
//         let t: f32 = iter as f32 / interpolations_f32;

//         let mut ptx: f32 = 0.0;
//         let mut pty: f32 = 0.0;

//         for index in 0..len + 1 {
//             let bin: f32 = factorial(len) as f32 / (factorial(index) * factorial(len - index)) as f32;
//             let tm: f32 = pow(1.0 - t, len - index);
//             let tt: f32 = pow(t, index);

//             ptx += bin * tm * tt * coeficients[index as usize][0] as f32;
//             pty += bin * tm * tt * coeficients[index as usize][1] as f32;
//         }

//         let ptx = ptx.round() as u32;
//         let pty = pty.round() as u32;

//         lines.push(ContourLine {
//             start: [previous_x, previous_y],
//             end: [ptx, pty],
//         });

//         previous_x = ptx;
//         previous_y = pty;
//     }
// }

// #[inline(always)]
// fn draw_line(line: &ContourLine, bitmap_width: u32, bitmap: &mut [u8], quad_offset: [u32; 2]) {
//     let first_x = std::cmp::min(line.start[0], line.end[0]);
//     let first_y = std::cmp::min(line.start[1], line.end[1]);

//     let last_x = line.start[0] + line.end[0] - first_x;
//     let last_y = line.start[1] + line.end[1] - first_y;

//     let iter_max = std::cmp::max(last_x - first_x, last_y - first_y) * 2;

//     if 0 == iter_max {
//         return;
//     }

//     let x_m: f32 = (line.end[0] as f32 - line.start[0] as f32) / iter_max as f32;
//     let y_m: f32 = (line.end[1] as f32 - line.start[1] as f32) / iter_max as f32;

//     for i in 0..iter_max {
//         let x: u32 = (line.start[0] as f32 + i as f32 * x_m).round() as u32;
//         let y: u32 = (line.start[1] as f32 + i as f32 * y_m).round() as u32;

//         bitmap[(x + quad_offset[0] + (quad_offset[1] + y) * bitmap_width) as usize] = 255;
//     }
// }
