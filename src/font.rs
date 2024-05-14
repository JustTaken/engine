use std::io::{ Seek, Read, BufReader};

const INTERPOLATIONS: u32 = 4;

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

pub struct GlyphMetrics {
    pub width: usize,
    pub x_offset: usize,
    pub y_offset: usize,
}

pub struct TrueTypeFont {
    pub texture: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub metrics: Vec<GlyphMetrics>,
    pub line_height: usize,
    pub scale: f32,
}

#[derive(Clone)]
struct Box {
    x_min: i16,
    x_max: i16,
    y_min: i16,
    y_max: i16,
}

#[derive(Clone)]
struct Glyph {
    contour_ends: Vec<u16>,
    points: Vec<Point>,
    boundary: Box,
    left_bearing: usize,
    advance: usize,
}

#[derive(Debug, Clone)]
struct Point {
    x: i16,
    y: i16,
    on_curve: bool,
}

struct Header {
    units_pem: u16,
    index_to_loc_format: u16,
    boundary: Box,
}

fn new_header(reader: &mut BufReader<std::fs::File>) -> Result<Header, ParseError> {
    reader.seek(std::io::SeekFrom::Current(12)).map_err(|_| ParseError::WrongSize)?;
    let magic_number = read(reader, 4)?;
    reader.seek(std::io::SeekFrom::Current(2)).map_err(|_| ParseError::WrongSize)?;
    let units_pem = read(reader, 2)? as u16;
    reader.seek(std::io::SeekFrom::Current(16)).map_err(|_| ParseError::WrongSize)?;

    let x_min = read(reader, 2).unwrap() as i16;
    let y_min = read(reader, 2).unwrap() as i16;
    let x_max = read(reader, 2).unwrap() as i16;
    let y_max = read(reader, 2).unwrap() as i16;

    reader.seek(std::io::SeekFrom::Current(6)).map_err(|_| ParseError::WrongSize)?;

    let index_to_loc_format = read(reader, 2)? as u16;

    if 0x5f0f3cf5 != magic_number {
        Err(ParseError::WrongMagicNumber)
    } else {
        Ok(Header {
            units_pem,
            index_to_loc_format,
            boundary: Box {
                x_min,
                x_max,
                y_min,
                y_max,
            }
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
    HorizontalHeader,
    HorizontalMetrics,
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
        [b'h', b'h', b'e', b'a'] => Ok(TableType::HorizontalHeader),
        [b'h', b'm', b't', b'x'] => Ok(TableType::HorizontalMetrics),
        [b'l', b'o', b'c', b'a'] => Ok(TableType::Location),
        [b'm', b'a', b'x', b'p'] => Ok(TableType::Max),
        _ => Err(ParseError::FailToParse),
    }
}

fn read(reader: &mut BufReader<std::fs::File>, len: usize) -> Result<u32, ParseError> {
    if len > 4 {
        return Err(ParseError::InvalidValue);
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

    let mut location_table_offset: u32 = 0;
    let mut glyph_table_offset: u32 = 0;
    let mut horizontal_metrics_table_offset: u32 = 0;
    let mut num_of_long_h_metrics: u32 = 0;

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
                location_table_offset = table.offset;
            } else if let TableType::Glyph = typ {
                glyph_table_offset = table.offset;
            } else if let TableType::HorizontalHeader = typ {
                reader.seek(std::io::SeekFrom::Start(table.offset as u64 + 34)).unwrap();
                num_of_long_h_metrics = read(&mut reader, 2).unwrap();
            } else if let TableType::HorizontalMetrics = typ {
                horizontal_metrics_table_offset = table.offset;
            }
        }
    }

    if let (Some(cmap), Some(header)) = (cmap, header) {
        let scale = size as f32 / header.units_pem as f32;
        let line_height = ((header.boundary.y_max - header.boundary.y_min) as f32 * scale) as usize;
        let index_to_loc = header.index_to_loc_format as u32;

        let (glyphs_per_row, line_count) = {
            let len = code_points.len() as f32;
            let height: f32 = len.sqrt().floor();
            ((len as f32 / height).ceil() as u32, height as u32)
        };

        let mut texture_width = 0;
        // let mut texture_height = 0;

        let mut x_offset = 0;
        let mut y_offset = 0;

        let mut max_width = 0;

        let mut metrics: Vec<GlyphMetrics> = Vec::with_capacity(code_points.len());
        let mut glyphs: Vec<Glyph> = Vec::with_capacity(code_points.len());

        let mut i = 0;
        for code_point in code_points.iter() {
            let index = get_index(&cmap, *code_point);

            let glyph = new_glyph(
                &mut reader,
                index_to_loc,
                location_table_offset,
                glyph_table_offset,
                horizontal_metrics_table_offset,
                num_of_long_h_metrics,
                index,
                scale,
            );

            let glyph_width = ((glyph.boundary.x_max - glyph.boundary.x_min) as f32 * scale) as usize;

            let box_width = glyph_width + glyph.left_bearing + glyph.advance;

            metrics.push(GlyphMetrics {
                width: box_width,
                x_offset,
                y_offset,
            });

            // if glyph_height > max_height {
            //     max_height = glyph_height;
            //     texture_height = y_offset + glyph_height;
            // }

            x_offset += box_width;

            i += 1;
            if i == glyphs_per_row {
                i = 0;

                if x_offset > max_width {
                    max_width = x_offset;
                    texture_width = max_width;
                }

                y_offset += line_height;
                x_offset = 0;
            }

            glyphs.push(glyph);
        }

        let texture_height = line_count as usize * line_height;
        let mut texture: Vec<u8> = vec![0; texture_width * texture_height];

        for (i, glyph) in glyphs.iter().enumerate() {
            let bottom_padding = ((glyph.boundary.y_min - header.boundary.y_min) as f32 * scale) as usize;

            modify_texture(
                &mut texture,
                texture_width,
                metrics[i].width,
                line_height,
                &glyph.contour_ends,
                &glyph.points,
                [(metrics[i].x_offset + glyph.left_bearing) as u32, (metrics[i].y_offset + bottom_padding) as u32]
            );
        }

        Ok(TrueTypeFont {
            texture,
            width: texture_width,
            height: texture_height,
            metrics,
            line_height,
            scale,
        })
    } else {
        Err(ParseError::FailToParse)
    }
}

fn goto_glyph_offset(reader: &mut BufReader<std::fs::File>, index_to_loc: u32, location_table_offset: u32, code_point: u32, glyph_table_offset: u32) {
    let translate = index_to_loc * 2;
    reader.seek(std::io::SeekFrom::Start((location_table_offset + code_point * (translate + 2)) as u64)).unwrap();
    let offset = read(reader, 2 + translate as usize).unwrap() * (((index_to_loc + 1) % 2) + 1);
    reader.seek(std::io::SeekFrom::Start((offset + glyph_table_offset) as u64)).unwrap();
}

fn new_glyph(
    reader: &mut BufReader<std::fs::File>,
    index_to_loc: u32,
    location_table_offset: u32,
    glyph_table_offset: u32,
    hhea_table_offset: u32,
    long_h_metrics_count: u32,
    code_point: u32,
    scale: f32,
) -> Glyph {
    let (advance, left_bearing) = if code_point < long_h_metrics_count {
        reader.seek(std::io::SeekFrom::Start((hhea_table_offset + 4 * code_point) as u64)).unwrap();
        let advance = read(reader, 2).unwrap() as f32;
        let left_bearing = read(reader, 2).unwrap() as f32;
        ((advance * scale) as usize, (left_bearing * scale) as usize)
    } else {
        reader.seek(std::io::SeekFrom::Start(hhea_table_offset as u64 + 4 * (long_h_metrics_count - 1) as u64)).unwrap();
        let advance = read(reader, 2).unwrap() as f32;
        reader.seek(std::io::SeekFrom::Current(2 * (code_point - long_h_metrics_count) as i64)).unwrap();
        let left_bearing = read(reader, 2).unwrap() as f32;
        ((advance * scale) as usize, (left_bearing * scale) as usize)
    };

    goto_glyph_offset(reader, index_to_loc, location_table_offset, code_point, glyph_table_offset);
    let number_of_contours = read(reader, 2).unwrap() as i16;

    let boundary = Box {
        x_min: read(reader, 2).unwrap() as i16,
        y_min: read(reader, 2).unwrap() as i16,
        x_max: read(reader, 2).unwrap() as i16,
        y_max: read(reader, 2).unwrap() as i16,
    };

    if number_of_contours < 0 {
        let mut flag = MORE_COMPONENTS;

        let mut simple_glyph = Glyph {
            contour_ends: Vec::new(),
            points: Vec::new(),
            boundary: boundary.clone(),
            left_bearing,
            advance,
        };

        while flag & MORE_COMPONENTS != 0 {
            flag = read(reader, 2).unwrap() as u16;

            let index = read(reader, 2).unwrap();
            let matrix = read_compound_glyph(reader, flag);
            let pos = reader.stream_position().unwrap();

            goto_glyph_offset(reader, index_to_loc, location_table_offset, index, glyph_table_offset);
            let number_of_contours = read(reader, 2).unwrap() as i16;
            reader.seek(std::io::SeekFrom::Current(8)).unwrap();

            if number_of_contours > 0 {
                let offset_quad = if flag & USE_MY_METRICS != 0 {
                    [0, 0]
                } else {
                    [(matrix[4] as f32 * scale) as i16, (matrix[5] as f32 * scale) as i16]
                };

                let mut glyph = read_simple_glyph(
                    reader,
                    number_of_contours,
                    &boundary,
                    [matrix[0] as f32 * scale, matrix[1] as f32 * scale, matrix[2] as f32 * scale, matrix[3] as f32 * scale],
                    offset_quad,
                ).unwrap();

                for i in 0..glyph.contour_ends.len() {
                    glyph.contour_ends[i] += simple_glyph.points.len() as u16;
                }

                simple_glyph.contour_ends.extend_from_slice(&glyph.contour_ends);
                simple_glyph.points.extend_from_slice(&glyph.points);
            }

            reader.seek(std::io::SeekFrom::Start(pos)).unwrap();
        }

        simple_glyph
    } else {
        let mut glyph = read_simple_glyph(
            reader,
            number_of_contours,
            &boundary,
            [scale, 0.0, 0.0, scale],
            [0, 0],
        ).unwrap();

        glyph.left_bearing = left_bearing;
        glyph.advance = advance;

        glyph
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
    number_of_contours: i16,
    boundary: &Box,
    factor_matrix: [f32; 4],
    center_offset: [i16; 2],
) -> Result<Glyph, ParseError> {
    let mut contour_ends: Vec<u16> = Vec::with_capacity(number_of_contours as usize);

    for _ in 0..number_of_contours {
        contour_ends.push(read(reader, 2)? as u16);
    }

    let contour_max: u16 = contour_ends[contour_ends.len() - 1] + 1;

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
            x: x_value,
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

        points[i].y = ((y_value - boundary.y_min) as f32 * factor_matrix[3] + points[i].x as f32 * factor_matrix[1]) as i16 + center_offset[1];
        points[i].x = ((points[i].x - boundary.x_min) as f32 * factor_matrix[0] + points[i].y as f32 * factor_matrix[2]) as i16 + center_offset[0];
    }

    Ok(Glyph {
        contour_ends,
        points,
        boundary: Box {
            x_min: boundary.x_min,
            y_min: boundary.y_min,
            x_max: boundary.x_max,
            y_max: boundary.y_max,
        },
        left_bearing: 0,
        advance: 0,
    })
}

const ARG_1_AND_2_ARE_WORDS: u16 = 0x0001; // 0
const ARGS_ARE_XY_VALUES: u16 = 0x0002; // 1
// const ROUND_XY_TO_GRID: u16 = 0x0004; // 2
const WE_HAVE_A_SCALE: u16 = 0x0008; // 3
// const RESERVED: u16 = 0x0010; // 4
const MORE_COMPONENTS: u16 = 0x0020; // 5
const WE_HAVE_AN_X_AND_Y_SCALE: u16 = 0x0040; // 6
const WE_HAVE_A_TWO_BY_TWO: u16 = 0x0080; // 7
const WE_HAVE_INSTRUCTIONS: u16 = 0x0100; // 8
const USE_MY_METRICS: u16 = 0x0200; // 9
// const OVERLAP_COMPONENT: u16 = 0x0400; // 10

fn read_compound_glyph(
    reader: &mut BufReader<std::fs::File>,
    flag: u16,
) -> [i16; 6] {
    let mut matrix: [i16; 6] = [1, 0, 0, 1, 0, 0];

    if flag & ARGS_ARE_XY_VALUES == 0 {
        return matrix;
    }

    if flag & ARG_1_AND_2_ARE_WORDS != 0 {
        matrix[4] = read(reader, 2).unwrap() as i16;
        matrix[5] = read(reader, 2).unwrap() as i16;
    } else {
        matrix[4] = read(reader, 1).unwrap() as i16;
        matrix[5] = read(reader, 1).unwrap() as i16;
    }

    if flag & WE_HAVE_A_SCALE != 0 {
        matrix[0] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
        matrix[3] = matrix[0];
    } else if flag & WE_HAVE_AN_X_AND_Y_SCALE != 0{
        matrix[0] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
        matrix[3] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
    } else if flag & WE_HAVE_A_TWO_BY_TWO != 0 {
        matrix[0] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
        matrix[1] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
        matrix[2] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
        matrix[3] = (read(reader, 2).unwrap() as u16 / (1 as u16) << 14) as i16;
    }

    if flag & WE_HAVE_INSTRUCTIONS != 0 {
        reader.seek(std::io::SeekFrom::Current(2)).unwrap();
    }

    matrix
}

fn modify_texture(
    texture: &mut Vec<u8>,
    texture_width: usize,
    width: usize,
    height: usize,
    contour_ends: &[u16],
    points: &[Point],
    quad_offset: [u32; 2],
) {
    let mut out_points: [[u32; 2]; 10] = [[0; 2]; 10];
    let mut contour_start: u8 = 0;
    let mut first_line: [u32; 4] = [0; 4];
    let mut contours_inner_points: Vec<[usize; 2]> = Vec::with_capacity(contour_ends.len());

    for contour_end in contour_ends.iter() {
        let mut points_first_line: [[u32; 2]; 10] = [[0; 2]; 10];
        let mut points_first_line_count: usize = 0;
        let mut first_point_flag = false;
        let mut delta_y: i32 = 0;
        let mut delta_x: i32 = 0;
        let mut clock_wise_sum: u32 = 0;
        let mut counter_clock_wise_sum: u32 = 0;

        let mut py = 0;
        let mut px = 0;

        for i in contour_start..*contour_end as u8 + 1 {
            if !points[i as usize].on_curve {
                continue;
            }

            let mut index_of_next = if i == *contour_end as u8 {
                contour_start
            } else {
                i + 1
            };

            let mut out_points_count: usize = 0;
            while !points[index_of_next as usize].on_curve {
                out_points[out_points_count] = [
                    points[index_of_next as usize].x as u32 + quad_offset[0],
                    points[index_of_next as usize].y as u32 + quad_offset[1],
                ];

                out_points_count += 1;

                if index_of_next >= *contour_end as u8 {
                    index_of_next = contour_start;
                } else {
                    index_of_next += 1;
                }
            }
            let x0: u32 = points[i as usize].x as u32 + quad_offset[0];
            let y0: u32 = points[i as usize].y as u32 + quad_offset[1];

            let x1: u32 = points[index_of_next as usize].x as u32 + quad_offset[0];
            let y1: u32 = points[index_of_next as usize].y as u32 + quad_offset[1];

            if out_points_count == 0 {
                draw_line([x0, y0], [x1, y1], texture_width as u32, texture);
            } else {
                let mut previous_x: u32 = x0;
                let mut previous_y: u32 = y0;
                let mut coeficients: [[u32; 2]; 12] = [[0; 2]; 12];

                if !first_point_flag {
                    points_first_line[0] = [x0, y0];
                    points_first_line_count = 1;
                }

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

                    if !first_point_flag {
                        points_first_line[points_first_line_count] = [ptx, pty];
                        points_first_line_count += 1;
                    }

                    draw_line([previous_x, previous_y], [ptx, pty], texture_width as u32, texture);

                    previous_x = ptx;
                    previous_y = pty;
                }
            }

            if !first_point_flag {
                first_line = [x0, y0, x1, y1];

                delta_x = points[index_of_next as usize].x as i32 - points[i as usize].x as i32;
                delta_y = points[index_of_next as usize].y as i32 - points[i as usize].y as i32;

                let y_mean = delta_y / 2;
                let x_mean = delta_x / 2;

                py = points[i as usize].y + y_mean as i16;
                px = points[i as usize].x + x_mean as i16;

                if y_mean != 0 {
                    first_point_flag = true;
                }
            } else {
                while py == points[index_of_next as usize].y {
                    index_of_next = if index_of_next == *contour_end as u8 {
                        contour_start
                    } else {
                        index_of_next + 1
                    };

                    while !points[index_of_next as usize].on_curve {
                        if index_of_next >= *contour_end as u8 {
                            index_of_next = contour_start;
                        } else {
                            index_of_next += 1;
                        }
                    }
                }

                if (py < points[index_of_next as usize].y && py > points[i as usize].y)
                    || (py > points[index_of_next as usize].y && py < points[i as usize].y) {
                    let dx = points[index_of_next as usize].x as i32 - points[i as usize].x as i32;
                    let dy = points[index_of_next as usize].y as i32 - points[i as usize].y as i32;

                    if dy != 0 {
                        let dh: f32 = py as f32 - points[i as usize].y as f32;
                        let c = dh / dy as f32;
                        let x = ((dx as f32 * c) + points[i as usize].x as f32) as i16;

                        if delta_y > 0 {
                            if x >= px {
                                clock_wise_sum += 1;
                            } else {
                                counter_clock_wise_sum += 1;
                            }
                        } else if delta_y < 0 {
                            if x > px {
                                counter_clock_wise_sum += 1;
                            } else {
                                clock_wise_sum += 1;
                            }
                        }
                    }
                }
            }
        }

        if clock_wise_sum % 2 == 1 && counter_clock_wise_sum % 2 == 0 {
            let uy = if delta_x == 0 {
                0
            } else if (delta_y / delta_x).abs() <= 2 {
                if delta_x > 0 {
                    - 1
                } else {
                    1
                }
            } else {
                0
            };

            let ux: i32 = if delta_y > 0 {
                1
            } else {
                - 1
            };

            let mut xm = (first_line[2] + first_line[0])/ 2;
            let ym = (first_line[3] + first_line[1])/ 2;

            if 0 < points_first_line_count {
                for outsider in 0..points_first_line_count - 1 {
                    if ym <= points_first_line[outsider][1] && ym >= points_first_line[outsider + 1][1]
                        || ym >= points_first_line[outsider][1] && ym <= points_first_line[outsider + 1][1] {
                        let dx = points_first_line[outsider + 1][0] as i32 - points_first_line[outsider][0] as i32;
                        let dy = points_first_line[outsider + 1][1] as i32 - points_first_line[outsider][1] as i32;

                        if dy != 0 {
                            let dh: f32 = ym as f32 - points_first_line[outsider][1] as f32;
                            let c = dh / dy as f32;
                            let x = ((dx as f32 * c) + points_first_line[outsider][0] as f32) as u32;
                            xm = x;
                        } else {
                            xm = points_first_line[outsider][0]
                        }

                        break;
                    }
                }
            }

            let mut y_pos = (ym as i32 + uy) as usize * texture_width;

            while texture[(xm as i32 + ux) as usize + y_pos as usize] != 0 {
                xm = (xm as i32 + ux) as u32;
                y_pos = (y_pos as i32 + uy * texture_width as i32) as usize;
            }

            contours_inner_points.push([(xm as i32 + ux) as usize, y_pos as usize]);

        } else if clock_wise_sum % 2 == 0 && counter_clock_wise_sum % 2 == 1 {
        } else {
            println!("failed to determine the clock orientation with: {} {}", clock_wise_sum, counter_clock_wise_sum);
            return;
        }

        contour_start = *contour_end as u8 + 1;
    }

    for point in contours_inner_points.iter() {
        let mut points_to_fill: Vec<[usize; 2]> = Vec::with_capacity(width * height);
        let mut last: usize = 0;

        points_to_fill.push(*point);
        texture[point[0] + point[1]] = 255;

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
        let x: u32 = (start[0] as f32 + i as f32 * x_m).round() as u32;
        let y: u32 = (start[1] as f32 + i as f32 * y_m).round() as u32;

        texture[(x + y * width) as usize] = 255;
    }
}
