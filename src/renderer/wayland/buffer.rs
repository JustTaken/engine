use super::Core;

pub struct Line {
    pub content: Vec<u8>,
    pub indent: u32,
}

pub struct UniqueChars {
    pub positions: Vec<Vec<[u8; 2]>>,
}

pub struct ModeLine {
    left: Vec<u8>,
    middle: Vec<u8>,
    right: Vec<u8>,
}

pub struct Buffer {
    pub cursors: Vec<Cursor>,
    pub offset: Offset,
    pub lines: Vec<Line>,
    pub file_name: Option<Vec<u8>>,
    pub unique_chars: UniqueChars,
    pub mode_line: ModeLine,
    pub main_cursor_index: u32,
}

#[derive(Clone)]
pub struct Cursor {
    pub position: Position,
    pub selection: Option<Position>,
}

#[derive(Clone)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

pub struct Offset {
    pub x: u32,
    pub y: u32,
}

pub fn empty_buffer(chars_per_row: u32, chars_per_coloumn: u32) -> Buffer {
    let mode_line = ModeLine {
        left: vec![b'm', b'a', b'c', b'o', b'n', b'h', b'a'],
        middle: Vec::new(),
        right: Vec::new(),
    };

    let lines = vec![
        Line {
            content: Vec::new(),
            indent: 0,
        }
    ];

    Buffer {
        file_name: None,
        offset: Offset {
            x: 0,
            y: 0,
        },
        main_cursor_index: 0,
        unique_chars: unique_chars_from_lines(chars_per_row, chars_per_coloumn, &lines, &mode_line),
        lines,
        mode_line,
        cursors: vec![
            Cursor {
                selection: None,
                position: Position {
                    x: 0,
                    y: 0,
                },
            }
        ],
    }
}

pub fn buffer_from_file(chars_per_row: u32, chars_per_coloumn: u32, file_path: &str) -> Option<Buffer> {
    let content = if let Ok(content) = std::fs::read_to_string(file_path) {
        content
    } else {
        return None;
    };

    let mut lines: Vec<Line> = Vec::with_capacity(30);
    for line in content.lines() {

        let mut initial_spacing_count = 0;
        let mut first_non_blank_char_flag = false;

        let mut current_line = Line {
            content: Vec::with_capacity(30),
            indent: 0,
        };

        for c in line.chars() {
            let c = c as u8;

            if !first_non_blank_char_flag {
                if c == b' ' {
                    initial_spacing_count += 1;
                } else {
                    first_non_blank_char_flag = true;
                }
            }

            current_line.content.push(c);
        }

        current_line.indent = initial_spacing_count;
        lines.push(current_line);
    }

    let mode_line = ModeLine {
        left: file_path.chars().map(|c| c as u8).collect(),
        middle: Vec::new(),
        right: get_position_bytes(0, 0),
    };

    Some(Buffer {
        file_name: Some(file_path.chars().map(|c| c as u8).collect()),
        offset: Offset {
            x: 0,
            y: 0,
        },
        main_cursor_index: 0,
        unique_chars: unique_chars_from_lines(chars_per_row, chars_per_coloumn, &lines, &mode_line),
        lines,
        mode_line,
        cursors: vec![
            Cursor {
                selection: None,
                position: Position {
                    x: 0,
                    y: 0,
                }
            }
        ],
    })
}

pub fn execute_command(core: &mut Core) {
    let mut command_content_iter = core.command.iter();
    let command: Vec<u8> = command_content_iter.by_ref().take_while(|c| **c != b' ').map(|c| *c).collect();
    let argument: Vec<u8> = command_content_iter.by_ref().skip_while(|c| **c == b' ').map(|c| *c).collect();

    let command_slice: &[u8] = &command;

    match command_slice {
        &[b'e'] => {
            if let Some(buffer) = buffer_from_file(core.chars_per_row, core.chars_per_coloumn, &argument.into_iter().map(|c| c as char).collect::<String>()[..]) {
                let len = core.buffers.len();
                core.buffers.push(buffer);
                core.main_buffer_index = len as u32;
            }
        },
        &[b'b'] => {
            for (i, buffer) in core.buffers.iter().enumerate() {
                if buffer.file_name.as_ref() == Some(&argument) {
                    core.main_buffer_index = i as u32;
                }
            }
        }
        _ => println!("command not found"),
    }

    core.command.clear();
}

pub fn save_buffer(core: &Core) {
    use std::io::Write;

    let buffer = &core.buffers[core.main_buffer_index as usize];
    let file_name: String = buffer.file_name.as_ref().unwrap().iter().map(|c| *c as char).collect();
    let mut file = std::fs::File::create(file_name).unwrap();
    let mut content = Vec::with_capacity(buffer.lines.len() * 80);

    for line in buffer.lines.iter() {
        content.extend_from_slice(&line.content);
        content.push(b'\n');
    }

    file.write_all(&content).unwrap();
}

pub fn get_this_line_or_max(lines: &[Line], i: u32) -> u32 {
    let len = lines.len() as u32;

    if len < i {
        len
    } else {
        i
    }
}

pub fn check_offset(core: &mut Core) -> bool {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &buffer.cursors[buffer.main_cursor_index as usize].position;
    let mut change_flag = false;

    if position.y >= buffer.offset.y + core.chars_per_coloumn {
        buffer.offset.y = position.y - core.chars_per_coloumn + 1;
        change_flag = true;
    } else if position.y < buffer.offset.y {
        buffer.offset.y = position.y;
        change_flag = true;
    }

    if position.x > buffer.offset.x + core.chars_per_row {
        buffer.offset.x = position.x - core.chars_per_row;
        change_flag = true;
    } else if position.x < buffer.offset.x {
        buffer.offset.x = position.x;
        change_flag = true;
    }

    change_flag
}

pub fn show_completion_box(core: &mut Core, completions: Vec<Vec<u8>>) {
    let cols = 3;
    let mut completion_lines: Vec<Line> = Vec::with_capacity(completions.len() / cols + 1);
    let completion_box_h_space = core.chars_per_row / cols as u32;

    for (i, comp) in completions.into_iter().enumerate() {
        let comp_index = i % 3;
        if comp_index == 0 {
            completion_lines.push(Line {
                content: Vec::from(comp),
                indent: 0,
            });
        } else {
            let last = completion_lines.last_mut().unwrap();
            last.content.resize_with(completion_box_h_space as usize * comp_index, || b' ');
            last.content.extend_from_slice(&comp);
        }
    }

    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let line_max = get_this_line_or_max(&buffer.lines, buffer.offset.y + core.chars_per_coloumn - 3);
    let lines = &buffer.lines[buffer.offset.y as usize..line_max as usize];

    for i in 0..buffer.unique_chars.positions.len() {
        buffer.unique_chars.positions[i].clear();
    }

    for (i, line) in completion_lines.iter().enumerate() {
        if i >= 3 {
            break;
        }

        for (j, u) in line.content.iter().enumerate() {
            let c = *u as usize - 32;
            buffer.unique_chars.positions[c].push([j as u8, i as u8 + core.chars_per_coloumn as u8 - 3]);
        }
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, u) in get_slice(&line.content, buffer.offset.x, core.chars_per_row + buffer.offset.x).iter().enumerate() {
            let c = *u as usize - 32;

            buffer.unique_chars.positions[c].push([j as u8, i as u8]);
        }
    }

    let mode_line_content = command_string(&core.command);

    for (j, u) in mode_line_content.iter().enumerate() {
        let c = *u as usize - 32;
        buffer.unique_chars.positions[c].push([j as u8, core.chars_per_coloumn as u8]);
    }
}

fn get_position_bytes(x: u32, y: u32) -> Vec<u8> {
    let line_number_parse = y.to_string();
    let col_number_parse = x.to_string();

    let mut string = Vec::new();
    string.extend_from_slice(&[b'l', b':']);
    string.extend_from_slice(&line_number_parse.into_bytes());
    string.extend_from_slice(&[b' ', b'c', b':']);
    string.extend_from_slice(&col_number_parse.into_bytes());

    string
}

pub fn update_mode_line_right(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let cursor = &buffer.cursors[buffer.main_cursor_index as usize];
    buffer.mode_line.right = get_position_bytes(cursor.position.x, cursor.position.y);
}

pub fn delete_prev_char(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    if position.x == 0 {
        if position.y > 0 {
            position.y -= 1;
            position.x = buffer.lines[position.y as usize].content.len() as u32;

            let next_line = buffer.lines.remove(position.y as usize + 1);
            buffer.lines[position.y as usize].content.extend_from_slice(&next_line.content);
        }
    } else {
        position.x -= 1;
        buffer.lines[position.y as usize].content.remove(position.x as usize);
    }
}

pub fn start_of_line(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    position.x = 0;
}

pub fn end_of_line(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    position.x = buffer.lines[position.y as usize].content.len() as u32;
}

pub fn next_char(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    let line_len = buffer.lines[position.y as usize].content.len() as u32;

    if position.x > line_len {
        if buffer.lines.len() > position.y as usize + 1 {
            position.x = 0;
            position.y += 1;
        }
    } else {
        position.x += 1;
    }
}

pub fn prev_char(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    if position.x == 0 {
        if position.y > 0 {
            position.y -= 1;
            position.x = buffer.lines[position.y as usize].content.len() as u32;
        }
    } else {
        position.x -= 1;
    }
}

pub fn prev_line(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    if position.y > 0 {
        position.y -= 1;

        let line_len = buffer.lines[position.y as usize].content.len() as u32;
        if position.x > line_len {
            position.x = line_len;
        }
    }
}

pub fn next_line(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;

    if buffer.lines.len() > position.y as usize + 1 {
        position.y += 1;

        let line_len = buffer.lines[position.y as usize].content.len() as u32;
        if position.x > line_len {
            position.x = line_len;
        }
    }
}

pub fn insert_new_line(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    let indent = buffer.lines[position.y as usize].indent;
    let mut vec = vec![b' '; indent as usize];
    vec.extend_from_slice(&buffer.lines[position.y as usize].content[position.x as usize..]);

    let line = Line {
        content: vec,
        indent,
    };

    position.y += 1;
    buffer.lines.insert(position.y as usize, line);
    buffer.lines[position.y as usize - 1].content.truncate(position.x as usize);
    position.x = indent;
}

const TAB_LEN: usize = 4;

pub fn insert_char_at(core: &mut Core, position_index: usize) {
    let c = core.last_inserted_char;
    let chars_inserted = if c == b'\t' {
        let y = core.buffers[core.main_buffer_index as usize].cursors[position_index].position.y;
        let mut identation_count = 0;
        let mut content = vec![b' '; TAB_LEN as usize];
        let iter = core.buffers[core.main_buffer_index as usize].lines[y as usize].content.iter();

        for c in iter {
            if *c != b' ' {
                break;
            }

            identation_count += 1;
            content.push(*c);
        }

        let indent = TAB_LEN as u32 + identation_count;

        core.buffers[core.main_buffer_index as usize].lines[y as usize]= Line {
            content,
            indent,
        };

        TAB_LEN
    } else {
        let x = core.buffers[core.main_buffer_index as usize].cursors[position_index].position.x;
        let y = core.buffers[core.main_buffer_index as usize].cursors[position_index].position.y;
        core.buffers[core.main_buffer_index as usize].lines[y as usize].content.insert(x as usize, c);
        1
    };

    update_chars(core);
    core.buffers[core.main_buffer_index as usize].cursors[position_index].position.x += chars_inserted as u32;
}

pub fn delete_char_at(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    if buffer.lines[position.y as usize].content.len() <= position.x as usize {
        if buffer.lines.len() > position.y as usize + 1 {
            let next_line = buffer.lines.remove(position.y as usize + 1);
            buffer.lines[position.y as usize].content.extend_from_slice(&next_line.content);
        }
    } else {
        buffer.lines[position.y as usize].content.remove(position.x as usize);
    }
}

pub fn delete_to_line_end(core: &mut Core, position_index: usize) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let position = &mut buffer.cursors[position_index].position;
    buffer.lines[position.y as usize].content.drain(position.x as usize..);
}

fn command_string(string: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(string.len() + 1);
    v.push(b':');
    v.extend_from_slice(string);

    v
}

pub fn update_chars(core: &mut Core) {
    let buffer = &mut core.buffers[core.main_buffer_index as usize];
    let line_max = get_this_line_or_max(&buffer.lines, buffer.offset.y + core.chars_per_coloumn);
    let lines = &buffer.lines[buffer.offset.y as usize..line_max as usize];

    for i in 0..buffer.unique_chars.positions.len() {
        buffer.unique_chars.positions[i].clear();
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, u) in get_slice(&line.content, buffer.offset.x, core.chars_per_row + buffer.offset.x).iter().enumerate() {
            let c = *u as usize - 32;

            buffer.unique_chars.positions[c].push([j as u8, i as u8]);
        }
    }

    let mode_line_content = if core.command_mode {
        command_string(&core.command)
    } else {
        mode_line_string(core.chars_per_row, &buffer.mode_line)
    };

    for (j, u) in mode_line_content.iter().enumerate() {
        let c = *u as usize - 32;
        buffer.unique_chars.positions[c].push([j as u8, core.chars_per_coloumn as u8]);
    }
}

fn mode_line_string(chars_per_row: u32, mode_line: &ModeLine) -> Vec<u8> {
    let mut content = Vec::new();
    content.extend_from_slice(&mode_line.left);
    let l = (chars_per_row as usize / 2) - (mode_line.middle.len() / 2);

    if l < content.len() {
        return content;
    }

    let white_space = l - content.len();

    content.extend_from_slice(&vec![b' '; white_space]);
    content.extend_from_slice(&mode_line.middle);

    let white_space = chars_per_row as usize - content.len() - mode_line.right.len();
    content.extend_from_slice(&vec![b' '; white_space]);
    content.extend_from_slice(&mode_line.right);

    content
}

pub fn unique_chars_from_lines(chars_per_row: u32, chars_per_coloumn: u32, lines: &[Line], mode_line: &ModeLine) -> UniqueChars {
    let line_max = get_this_line_or_max(lines, chars_per_coloumn);
    let lines = &lines[0..line_max as usize];

    let mut positions: Vec<Vec<[u8; 2]>> = vec![Vec::with_capacity(10); 95];
    for (i, line) in lines.iter().enumerate() {
        for (j, u) in get_slice(&line.content, 0, chars_per_row).iter().enumerate() {
            let c = *u as usize - 32;
            positions[c].push([j as u8, i as u8]);
        }
    }

    let mode_line_content = mode_line_string(chars_per_row, mode_line);

    for (j, u) in mode_line_content.iter().enumerate() {
        let c = *u as usize - 32;
        positions[c].push([j as u8, chars_per_coloumn as u8]);
    }

    UniqueChars {
        positions
    }
}

fn get_slice(line: &[u8], offset: u32, size: u32) -> &[u8] {
    let len = line.len() as u32;

    if len < offset {
        &[]
    } else if len < size {
        &line[offset as usize..len as usize]
    } else {
        &line[offset as usize..size as usize]
    }
}
