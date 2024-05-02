use std::io::{ Seek, Read, BufReader};

const INTERPOLATIONS: u32 = 5;

#[derive(Debug)]
pub enum ParseError {
    WrongSize,
    NoMoreData,
    FailToParse,
    WrongMagicNumber,
    InvalidValue,
}

#[allow(dead_code)]
struct Table {
    name: [u8; 4],
    checksum: u32,
    offset: u32,
    length: u32,
}

pub struct TrueTypeFont {
    pub texture: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub glyph_width: u32,
    pub glyph_height: u32,
    pub glyphs_per_row: u32,
}

struct Glyph {
    contour_ends: Vec<u16>,
    points: Vec<Point>,
}

struct Point {
    x: i16,
    y: i16,
    on_curve: bool,
}

struct Header {
    units_pem: u16,
    x_min: i16,
    x_max: i16,
    y_min: i16,
    y_max: i16,
    // font_direction_hint: u16,
    index_to_loc_format: u16,
}

fn new_header(reader: &mut BufReader<std::fs::File>) -> Result<Header, ParseError> {
    // let version = read(reader, 4)?;
    // let font_revision = read(reader, 4)?;
    // let checksum_adjustment = read(reader, 4)?;
    reader.seek(std::io::SeekFrom::Current(12)).map_err(|_| ParseError::WrongSize)?;
    let magic_number = read(reader, 4)?;
    reader.seek(std::io::SeekFrom::Current(2)).map_err(|_| ParseError::WrongSize)?;
    // let flags = read(reader, 2)? as u16;
    let units_pem = read(reader, 2)? as u16;

    reader.seek(std::io::SeekFrom::Current(16)).map_err(|_| ParseError::WrongSize)?;

    let x_min = read(reader, 2)? as i16;
    let y_min = read(reader, 2)? as i16;
    let x_max = read(reader, 2)? as i16;
    let y_max = read(reader, 2)? as i16;
    reader.seek(std::io::SeekFrom::Current(6)).map_err(|_| ParseError::WrongSize)?;
    // let mac_style = read(reader, 2)? as u16;
    // let lowest_rec_ppem = read(reader, 2)? as u16;
    // let font_direction_hint = read(reader, 2)? as u16;
    let index_to_loc_format = read(reader, 2)? as u16;
    // let glyph_data_format = read(reader, 2)? as u16;

    if 0x5f0f3cf5 != magic_number {
        Err(ParseError::WrongMagicNumber)
    } else {
        Ok(Header {
            units_pem,
            x_min,
            x_max,
            y_min,
            y_max,
            index_to_loc_format,
        })
    }
}

struct Map {
    start_code: Vec<u32>,
    end_code: Vec<u32>,
    glyph_code: Vec<u32>,
}

fn construct_cmap(reader: &mut BufReader<std::fs::File>) -> Result<Map, ParseError> {
    if 0 != read(reader, 2).unwrap() {
        return Err(ParseError::WrongMagicNumber);
    }

    reader.seek(std::io::SeekFrom::Current(8)).map_err(|_| ParseError::NoMoreData)?;
    let group_count = read(reader, 4)?;

    let mut start_code = Vec::with_capacity(group_count as usize);
    let mut end_code = Vec::with_capacity(group_count as usize);
    let mut glyph_code = Vec::with_capacity(group_count as usize);

    for _ in 0..group_count {
        start_code.push(read(reader, 4)?);
        end_code.push(read(reader, 4)?);
        glyph_code.push(read(reader, 4)?);
    }

    Ok(Map {
        start_code,
        end_code,
        glyph_code,
    })
}

fn get_index(cmap: &Map, code_point: u8) -> u32 {
    let code_point = code_point as u32;

    for i in 0..cmap.start_code.len() {
        if cmap.start_code[i] <= code_point && cmap.end_code[i] >= code_point {
            let offset = code_point - cmap.start_code[i];
            return cmap.glyph_code[i] + offset;
        }
    }

    0
}

enum TableType {
    Map,
    Max,
    Glyph,
    Header,
    Location,
}

impl Table {
    fn new(reader: &mut BufReader<std::fs::File>) -> Result<Table, ParseError> {
        let mut name: [u8; 4] = [0; 4];
        if 4 != reader.read(&mut name).map_err(|_| ParseError::NoMoreData)? {
            return Err(ParseError::NoMoreData);
        }

        Ok(Table {
            name,
            checksum: read(reader, 4)?,
            offset: read(reader, 4)?,
            length: read(reader, 4)?,
        })
    }
}

fn get_name_type(name: &[u8; 4]) -> Result<TableType, ParseError> {
    match name {
        [b'c', b'm', b'a', b'p'] => Ok(TableType::Map),
        [b'g', b'l', b'y', b'f'] => Ok(TableType::Glyph),
        [b'h', b'e', b'a', b'd'] => Ok(TableType::Header),
        [b'l', b'o', b'c', b'a'] => Ok(TableType::Location),
        [b'm', b'a', b'x', b'p'] => Ok(TableType::Max),
        _ => Err(ParseError::FailToParse),
    }
}

fn read(reader: &mut BufReader<std::fs::File>, len: usize) -> Result<u32, ParseError> {
    if len > 4 {
        return Err(ParseError::WrongSize);
    }

    let mut array: [u8; 4] = [0; 4];

    if len != reader.read(&mut array[..len]).unwrap() {
        return Err(ParseError::NoMoreData);
    }

    let mut variable: u32 = 0;

    for i in 0..len {
        variable += (array[i] as u32) << (8 * (len - i - 1));
    }

    Ok(variable)
}

pub fn init(file_path: &str, code_points: &[u8], size: u8) -> Result<TrueTypeFont, ParseError> {
    let file = std::fs::File::open(file_path).map_err(|_| ParseError::FailToParse)?;
    let mut reader = BufReader::new(file);

    reader.seek(std::io::SeekFrom::Current(4)).unwrap();
    let num_tables = read(&mut reader, 2)?;

    reader.seek(std::io::SeekFrom::Current(6)).unwrap();
    let pos = reader.stream_position().unwrap();

    let mut cmap: Option<Map> = None;
    let mut header: Option<Header> = None;

    let mut location_offset: u32 = 0;
    let mut glyph_table_offset: u32 = 0;

    for i in 0..num_tables {
        reader.seek(std::io::SeekFrom::Start(pos + (i as usize * std::mem::size_of::<Table>()) as u64)).unwrap();

        let table = Table::new(&mut reader)?;

        if let Ok(typ) = get_name_type(&table.name) {
            if let TableType::Map = typ {
                reader.seek(std::io::SeekFrom::Start(table.offset as u64 + 2)).unwrap();
                let number_subtables = read(&mut reader, 2)?;

                for k in 0..number_subtables {
                    reader.seek(std::io::SeekFrom::Start(table.offset as u64 + 8 * k as u64 + 4)).unwrap();

                    let id = read(&mut reader, 2)?;
                    let specific_id = read(&mut reader, 2)?;
                    let offset = read(&mut reader, 4)?;

                    if id != 0 && specific_id != 0 && specific_id != 4 && specific_id != 3 {
                        continue;
                    }

                    reader.seek(std::io::SeekFrom::Start(table.offset as u64 + offset as u64)).unwrap();

                    let format = read(&mut reader, 2)?;

                    if format == 12 {
                        cmap = construct_cmap(&mut reader).ok();
                        break;
                    }
                }
            } else if let TableType::Max = typ {
                reader.seek(std::io::SeekFrom::Start(table.offset as u64 + 4)).unwrap();
                //let glyphs_count = read(&mut reader, 2)?;
            } else if let TableType::Header = typ {
                reader.seek(std::io::SeekFrom::Start(table.offset as u64)).unwrap();
                header = new_header(&mut reader).ok();
            } else if let TableType::Location = typ {
                location_offset = table.offset;
            } else if let TableType::Glyph = typ {
                glyph_table_offset = table.offset;
            }
        }
    }

    if let (Some(cmap), Some(header)) = (cmap, header) {
        let factor = size as f32 / header.units_pem as f32;

        let x_max = (header.x_max as f32 * factor) as i16;
        let x_min = (header.x_min as f32 * factor) as i16;
        let y_max = (header.y_max as f32 * factor) as i16;
        let y_min = (header.y_min as f32 * factor) as i16;

        let glyph_width: u32 = (x_max - x_min) as u32 + 1;
        let glyph_height: u32 = (y_max - y_min) as u32 + 1;

        let (glyphs_per_row, glyphs_per_coloum) = {
            let len = code_points.len() as f32;
            let height: f32 = len.sqrt().floor();
            let width = (len as f32 / height).ceil() as u32;

            (width, height as u32)
        };

        let width = glyph_width * glyphs_per_row;
        let height = glyph_height * glyphs_per_coloum;

        let texture_size: u32 = width * height;
        let mut texture: Vec<u8> = vec![0; texture_size as usize];

        let boundary = [x_min, x_max, y_min, y_max];

        // let indices = [18, 18];
        for (i, code_point) in code_points.iter().enumerate() {
            // let index = indices[i as usize];
            let index = get_index(&cmap, *code_point);
            let x_offset: u32 = glyph_width * (i as u32 % glyphs_per_row) as u32;
            let y_offset: u32 = glyph_height * (i as u32 / glyphs_per_row) as u32;

            add_glyph(
                &mut texture,
                &mut reader,
                header.index_to_loc_format as u32,
                location_offset,
                glyph_table_offset,
                index,
                width,
                [x_offset, y_offset],
                [0, 0],
                [factor, 0.0, 0.0, factor],
                boundary,
            );
        }

        Ok(TrueTypeFont {
            texture,
            width,
            height,
            glyph_width,
            glyph_height,
            glyphs_per_row: glyphs_per_row,
        })
    } else {
        Err(ParseError::FailToParse)
    }
}

fn add_glyph(
    texture: &mut Vec<u8>,
    reader: &mut BufReader<std::fs::File>,
    index_to_loc: u32,
    location_offset: u32,
    glyph_table_offset: u32,
    code_point: u32,
    texture_width: u32,
    quad_offset: [u32; 2],
    center_offset: [i16; 2],
    factor_matrix: [f32; 4],
    boundary: [i16; 4],
) {
    let translate = index_to_loc * 2;
    reader.seek(std::io::SeekFrom::Start((location_offset + code_point * (translate + 2)) as u64)).unwrap();
    let offset = read(reader, 2 + translate as usize).unwrap() * (((index_to_loc + 1) % 2) + 1);
    reader.seek(std::io::SeekFrom::Start((offset + glyph_table_offset) as u64)).unwrap();

    let width: u32 = try_usize(boundary[1] - boundary[0]).unwrap() + 1;
    let height: u32 = try_usize(boundary[3] - boundary[2]).unwrap() + 1;

    let number_of_contours = read(reader, 2).unwrap() as i16;
    reader.seek(std::io::SeekFrom::Current(8)).unwrap();

    if number_of_contours < 0 {
        let mut flag = MORE_COMPONENTS;
        let mut last_offset = [0.0, 0.0];

        while flag & MORE_COMPONENTS != 0 {
            flag = read(reader, 2).unwrap() as u16;

            let index = read(reader, 2).unwrap();
            let matrix = read_compound_glyph(reader, factor_matrix[0], &mut last_offset, flag);
            // println!("{}", index);
            // println!("{:#018b}", flag);
            // println!("{:?}", matrix);
            let pos = reader.stream_position().unwrap();

            add_glyph(
                texture,
                reader,
                index_to_loc,
                location_offset,
                glyph_table_offset,
                index,
                texture_width,
                quad_offset,
                [matrix[4] as i16, matrix[5] as i16],
                [matrix[0], matrix[1], matrix[2], matrix[3]],
                boundary,
            );

            reader.seek(std::io::SeekFrom::Start(pos)).unwrap();
        }
    } else {
        let glyph = read_simple_glyph(
            reader,
            number_of_contours.try_into().unwrap(),
            factor_matrix,
            center_offset,
        ).unwrap();

        modify_texture(
            width,
            height,
            boundary[0],
            boundary[2],
            texture_width,
            quad_offset,
            glyph,
            texture
        );
    }
}

const ON_CURVE: u8 = 0x01;
const X_IS_SHORT: u8 = 0x02;
const Y_IS_SHORT: u8 = 0x04;
const REPEAT: u8 = 0x08;
const X_IS_SAME: u8 = 0x10;
const Y_IS_SAME: u8 = 0x20;

fn read_simple_glyph(
    reader: &mut BufReader<std::fs::File>,
    number_of_contours: u16,
    factor_matrix: [f32; 4],
    center_offset: [i16; 2],
    // x_factor: f32,
    // y_factor: f32,
) -> Result<Glyph, ParseError> {
    let mut contour_ends: Vec<u16> = Vec::with_capacity(number_of_contours as usize);
    let mut contour_max: u16 = 0;

    for _ in 0..number_of_contours {
        let contour_end = read(reader, 2)? as u16;

        if contour_end + 1 > contour_max {
            contour_max = contour_end + 1;
        }

        contour_ends.push(contour_end);
    }

    let offset = read(reader, 2)?;
    reader.seek(std::io::SeekFrom::Current(offset as i64)).unwrap();

    let mut flags: Vec<u8> = Vec::with_capacity(contour_max as usize);
    let mut i: u16 = 0;

    while i < contour_max {
        let flag = read(reader, 1).unwrap() as u8;

        if flag & REPEAT != 0 {
            let repeat_count = read(reader, 1).unwrap() as u8;

            flags.extend_from_slice(&vec![flag; repeat_count as usize + 1]);
            i += repeat_count as u16;
        } else {
            flags.push(flag)
        }

        i += 1;
    }

    let mut points: Vec<Point> = Vec::with_capacity(contour_max as usize);
    let mut x_value: i16 = 0;

    for i in 0..contour_max {
        let i = i as usize;

        if flags[i] & X_IS_SHORT != 0 {
            let value = read(reader, 1).unwrap() as u8;

            if flags[i] & X_IS_SAME != 0 {
                x_value += value as i16;
            } else {
                x_value -= value as i16;
            }
        } else if flags[i] & X_IS_SAME == 0 {
            x_value += read(reader, 2).unwrap() as i16;
        }

        points.push(Point {
            x: (x_value as f32 * factor_matrix[0]) as i16,
            y: 0,
            on_curve: flags[i] & ON_CURVE != 0,
        });
    }

    let mut y_value: i16 = 0;
    for i in 0..contour_max {
        let i = i as usize;
        if flags[i] & Y_IS_SHORT != 0 {
            let value = read(reader, 1).unwrap() as u8;

            if flags[i] & Y_IS_SAME != 0 {
                y_value += value as i16;
            } else {
                y_value -= value as i16;
            }
        } else if flags[i] & Y_IS_SAME == 0 {
            y_value += read(reader, 2).unwrap() as i16;
        }

        points[i].y += (y_value as f32 * factor_matrix[3] + points[i].x as f32 * factor_matrix[1]) as i16 + center_offset[1];
        points[i].x += (y_value as f32 * factor_matrix[2]) as i16 + center_offset[0];
    }

    Ok(Glyph {
        contour_ends,
        points,
    })
}

const ARG_1_AND_2_ARE_WORDS: u16 = 0x0001;
const ARGS_ARE_XY_VALUES: u16 = 0x0002;
// const ROUND_XY_TO_GRID: u16 = 0x0004;
const WE_HAVE_A_SCALE: u16 = 0x0008;
const MORE_COMPONENTS: u16 = 0x0020;
const WE_HAVE_AN_X_AND_Y_SCALE: u16 = 0x0040;
const WE_HAVE_A_TWO_BY_TWO: u16 = 0x0080;
const WE_HAVE_INSTRUCTIONS: u16 = 0x0100;
// const USE_MY_METRICS: u16 = 0x0200;
// const OVERLAP_COMPONENT: u16 = 0x0400;

fn read_compound_glyph(
    reader: &mut BufReader<std::fs::File>,
    factor: f32,
    last_offset: &mut [f32],
    flag: u16,
) -> [f32; 6] {
        let mut matrix: [f32; 6] = [factor, 0.0, 0.0, factor, 0.0, 0.0];

        if flag & ARG_1_AND_2_ARE_WORDS != 0 && flag & ARGS_ARE_XY_VALUES != 0 {
            matrix[4] = (read(reader, 2).unwrap() as f32) * factor;
            matrix[5] = (read(reader, 2).unwrap() as f32) * factor;
        } else if flag & ARG_1_AND_2_ARE_WORDS == 0 && flag & ARGS_ARE_XY_VALUES != 0{
            matrix[4] = (read(reader, 1).unwrap() as f32) * factor;
            matrix[5] = (read(reader, 1).unwrap() as f32) * factor;
        }

        if flag & WE_HAVE_A_SCALE != 0 {
            matrix[0] = (read(reader, 2).unwrap() as f32 / ((1 as u16) << 14) as f32) * factor;
            matrix[3] = matrix[0];
        } else if flag & WE_HAVE_AN_X_AND_Y_SCALE != 0{
            matrix[0] = (read(reader, 2).unwrap() as f32 / ((1 as u16) << 14) as f32) * factor;
            matrix[3] = (read(reader, 2).unwrap() as f32 / ((1 as u16) << 14) as f32) * factor;
        } else if flag & WE_HAVE_A_TWO_BY_TWO != 0 {
            matrix[0] = (read(reader, 2).unwrap() as f32 / ((1 as u16) << 14) as f32) * factor;
            matrix[1] = (read(reader, 2).unwrap() as f32 / ((1 as u16) << 14) as f32) * factor;
            matrix[2] = (read(reader, 2).unwrap() as f32 / ((1 as u16) << 14) as f32) * factor;
            matrix[3] = (read(reader, 2).unwrap() as f32 / ((1 as u16) << 14) as f32) * factor;
        }

        if flag & WE_HAVE_INSTRUCTIONS != 0 {
            matrix[4] += last_offset[0];
            matrix[5] += last_offset[1];
        }

        last_offset[0] = matrix[4];
        last_offset[1] = matrix[5];

        matrix
}

fn try_usize(integer: i16) -> Result<u32, ParseError> {
    if integer < 0 {
        Err(ParseError::WrongSize)
    } else {
        Ok(integer as u32)
    }
}

const MIN_LINE_MATCHES: u8 = 4;
const HALF_MATCHES: u8 = MIN_LINE_MATCHES / 2;

fn search_blank_point(point: [u32; 2], texture_width: u32, quad_offset: [u32; 2], glyph_boundary: [u32; 2], texture: &mut Vec<u8>) -> Result<[usize; 2], ParseError> {
    let texture_width: usize = texture_width as usize;
    let point = [point[0] as usize, point[1] as usize * texture_width];

    let mut down_count: u8 = 0;
    let mut up_count: u8 = 0;

    let mut down = point[1] + texture_width;
    let mut up = point[1] - texture_width;

    let mut middle_down: Option<[usize; 2]> = None;
    let mut middle_up: Option<[usize; 2]> = None;

    let mut up_is_playing = true;
    let mut down_is_playing = true;

    let mut i: usize = 0;
    while i < 20 {
        i += 1;
        let mut last_x_down: u32 = 0;
        let mut last_x_up: u32 = 0;

        let mut down_interceptions: u8 = 0;
        let mut up_interceptions: u8 = 0;

        for i in quad_offset[0]..glyph_boundary[0] + quad_offset[0] {
            if texture[i as usize + down] != 0 && i - last_x_down > 1 {
                down_interceptions += 1;
                last_x_down = i;
                if down_count == HALF_MATCHES {
                    middle_down = Some([i as usize - 1, down as usize]);
                }
            }

            if texture[i as usize + up] != 0 && i - last_x_up > 1 {
                up_interceptions += 1;
                last_x_up = i;
                if up_count == HALF_MATCHES {
                    middle_up = Some([i as usize - 1, up as usize]);
                }
            }
        }

        if up_is_playing {
            if last_x_up > 0 {
                if up_interceptions % 2 == 0 {
                    if up_count >= MIN_LINE_MATCHES {
                        return middle_up.ok_or(ParseError::NoMoreData);
                    }

                    up_count += 1;
                } else {
                    up_count = 0;
                }

                up -= texture_width;
            } else {
                up_is_playing = false;
            }
        }

        if down_is_playing {
            if last_x_down > 0 {
                if down_interceptions % 2 == 0 {
                    if down_count >= MIN_LINE_MATCHES {
                        // return Ok([last_x_down as usize - 1, down]);
                        return middle_down.ok_or(ParseError::NoMoreData);
                    }

                    down_count += 1;
                } else {
                    down_count = 0;
                }

                down += texture_width;

            } else {
                down_is_playing = false;
            }
        }
    }

    Err(ParseError::NoMoreData)
}

fn modify_texture(
    width: u32,
    height: u32,
    x_min: i16,
    y_min: i16,
    texture_width: u32,
    quad_offset: [u32; 2],
    glyph: Glyph,
    texture: &mut Vec<u8>
) {
    let mut point: Option<[usize; 2]> = None;
    let mut out_points: [[u32; 2]; 10] = [[0; 2]; 10];
    let mut contour_start: u8 = 0;
    let mut first_curve_point: [u32; 2] = [0; 2];

    for contour_end in glyph.contour_ends.iter() {
        for i in contour_start..*contour_end as u8 + 1 {
            if !glyph.points[i as usize].on_curve {
                continue;
            }

            // if i >= contour_start + 2 {
            //     break;
            // }

            let mut index_of_next = if i == *contour_end as u8 {
                contour_start
            } else {
                i + 1
            };

            let mut out_points_count: usize = 0;
            while !glyph.points[index_of_next as usize].on_curve {
                out_points[out_points_count] = [
                    try_usize(glyph.points[index_of_next as usize].x - x_min).unwrap() + quad_offset[0],
                    try_usize(glyph.points[index_of_next as usize].y - y_min).unwrap() + quad_offset[1],
                ];

                out_points_count += 1;

                if index_of_next >= *contour_end as u8 {
                    index_of_next = contour_start;
                } else {
                    index_of_next += 1;
                }
            }

            let x0: u32 = try_usize(glyph.points[i as usize].x - x_min).unwrap() + quad_offset[0];
            let y0: u32 = try_usize(glyph.points[i as usize].y - y_min).unwrap() + quad_offset[1];
            first_curve_point = [x0, y0];

            let x1: u32 = try_usize(glyph.points[index_of_next as usize].x - x_min).unwrap() + quad_offset[0];
            let y1: u32 = try_usize(glyph.points[index_of_next as usize].y - y_min).unwrap() + quad_offset[1];

            if out_points_count == 0 {
                draw_line([x0, y0], [x1, y1], texture_width, texture);
            } else {
                let mut previous_x: u32 = x0;
                let mut previous_y: u32 = y0;
                let mut coeficients: [[u32; 2]; 12] = [[0; 2]; 12];

                coeficients[0] = [x0, y0];
                coeficients[1..out_points_count as usize + 1].copy_from_slice(&out_points[0..out_points_count as usize]);
                coeficients[out_points_count + 1] = [x1, y1];

                let len = out_points_count + 2 - 1;

                for iter in 1..INTERPOLATIONS + 1 {
                    let t: f32 = iter as f32 / INTERPOLATIONS as f32;

                    let mut ptx: f32 = 0.0;
                    let mut pty: f32 = 0.0;

                    for index in 0..len + 1 {
                        let bin: f32 = factorial(len) as f32 / (factorial(index) * factorial(len - index)) as f32;
                        let tm: f32 = pow(1.0 - t, (len - index) as f32);
                        let tt: f32 = pow(t, index as f32);

                        ptx += bin * tm * tt * coeficients[index][0] as f32;
                        pty += bin * tm * tt * coeficients[index][1] as f32;
                    }

                    let ptx = ptx.round() as u32;
                    let pty = pty.round() as u32;

                    draw_line([previous_x, previous_y], [ptx, pty], texture_width, texture);

                    previous_x = ptx;
                    previous_y = pty;
                }
            }
        }

        if let None = point {
            point = search_blank_point(first_curve_point, texture_width, quad_offset, [width, height], texture).ok();
        }

        contour_start = *contour_end as u8 + 1;
    }

    // 'out: for i in 0..height {
    //     let mut count: usize = 0;
    //     let mut last_x: usize = 0;

    //     for j in 0..width {
    //         if texture[(j + quad_offset[0]) as usize + ((i + quad_offset[1]) * texture_width) as usize] != 0 {
    //             count += 1;

    //             if count == 2 {
    //                 if (j as usize - last_x) > 1 {
    //                     println!("x: {}, last: {}", j, last_x);
    //                     point = Some([quad_offset[0] as usize + last_x + 1, ((quad_offset[1] + i) * texture_width) as usize]);
    //                     break 'out;
    //                 } else {
    //                     break;
    //                 }
    //             }

    //             last_x = j as usize;
    //         }
    //     }
    // }

    if let None = point {
        return;
    }

    let point = point.unwrap();
    let mut points_to_fill: Vec<[usize; 2]> = Vec::with_capacity((width * height) as usize);

    points_to_fill.push(point);
    texture[point[0] + point[1]] = 255;
    // println!("pinto: {}, {}", point[0], point[1] / texture_width as usize);

    let mut last: usize = 0;
    let texture_width: usize = texture_width as usize;

    loop {
        let right = [points_to_fill[last][0] + 1, points_to_fill[last][1]];
        if texture[right[0] + right[1]] == 0 {
            points_to_fill.push(right);
            texture[right[0] + right[1]] = 255;
        }

        let left = [points_to_fill[last][0] - 1, points_to_fill[last][1]];
        if texture[left[0] + left[1]] == 0 {
            points_to_fill.push(left);
            texture[left[0] + left[1]] = 255;
        }

        let down = [points_to_fill[last][0], points_to_fill[last][1] + texture_width];
        if texture[down[0] + down[1]] == 0 {
            points_to_fill.push(down);
            texture[down[0] + down[1]] = 255;
        }

        let up = [points_to_fill[last][0], points_to_fill[last][1] - texture_width];
        if texture[up[0] + up[1]] == 0 {
            points_to_fill.push(up);
            texture[up[0] + up[1]] = 255;
        }

        last += 1;

        if last >= points_to_fill.len() {
            break;
        }
    }
}

fn pow(base: f32, expoent: f32) -> f32 {
    if expoent < 0.05 {
        1.0
    } else {
        base.powf(expoent)
    }
}

fn factorial(n: usize) -> u32 {
    if n <= 1 {
        1
    } else {
        n as u32 * factorial(n - 1)
    }
}

fn draw_line(start: [u32; 2], end: [u32; 2], width: u32, texture: &mut [u8]) {
    let first_x = std::cmp::min(start[0], end[0]);
    let first_y = std::cmp::min(start[1], end[1]);

    let last_x = start[0] + end[0] - first_x;
    let last_y = start[1] + end[1] - first_y;

    let iter_max = std::cmp::max(last_x - first_x, last_y - first_y);

    if 0 == iter_max {
        return;
    }

    let x_m: f32 = (end[0] as f32 - start[0] as f32) / iter_max as f32;
    let y_m: f32 = (end[1] as f32 - start[1] as f32) / iter_max as f32;

    for i in 0..iter_max {
        let x: u32 = (start[0] as f32 + i as f32 * x_m).round()as u32;
        let y: u32 = (start[1] as f32 + i as f32 * y_m).round()as u32;

        texture[(x + y * width) as usize] = 255;
    }
}