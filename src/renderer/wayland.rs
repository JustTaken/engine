use crate::binding::wayland;

pub struct Core {
    pub display: *mut wayland::wl_display,
    pub surface: *mut wayland::wl_surface,

    pub extensions: [*const i8; 2],
    pub running: bool,
    pub width: u32,
    pub height: u32,

    pub changed: bool,
    pub buffer: buffer::Buffer,

    scale: f32,
    x_ratio: f32,
    window_ratio: f32,

    chars_per_row: u32,
    chars_per_coloumn: u32,

    key_delay: std::time::Duration,
    key_rate: std::time::Duration,

    last_inserted_char: u8,
    last_function: Option<fn(&mut Core)>,
    last_fetch_rate: std::time::Instant,
    last_fetch_delay: std::time::Instant,
    alt_modifier: bool,
    shift_modifier: bool,
    control_modifier: bool,

    registry: *mut wayland::wl_registry,
    compositor: *mut wayland::wl_compositor,
    seat: *mut wayland::wl_seat,
    keyboard: *mut wayland::wl_keyboard,
    xdg_shell: *mut wayland::xdg_wm_base,
    xdg_surface: *mut wayland::xdg_surface,
    xdg_toplevel: *mut wayland::xdg_toplevel,

    seat_listener: wayland::wl_seat_listener,
    keyboard_listener: wayland::wl_keyboard_listener,
    registry_listener: wayland::wl_registry_listener,
    shell_listener: wayland::xdg_wm_base_listener,
    shell_surface_listener: wayland::xdg_surface_listener,
    toplevel_listener: wayland::xdg_toplevel_listener,
}

#[derive(Debug)]
pub enum WaylandError {
    CouldNotAddListener,
    NotAscci,
}

pub mod buffer {
    use super::Core;

    // pub struct Character {
    //     next: Option<Character>,
    //     prev: Option<Character>,

    //     next_same: SameCode,
    //     prev_same: SameCode,

    //     position: Position,
    // }

    // pub enum CharCode {
    //     Same(&Character),
    //     Other(&Character),
    //     None,
    // }

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
        pub cursors: Vec<Position>,
        pub offset: Offset,
        pub lines: Vec<Line>,
        pub file_name: Option<String>,
        pub unique_chars: UniqueChars,
        pub mode_line: ModeLine,
        main_cursor_index: u32,
    }

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
                Position {
                    x: 0,
                    y: 0,
                }
            ],
        }
    }

    pub fn buffer_from_file(chars_per_row: u32, chars_per_coloumn: u32, file_path: &str) -> Buffer {
        let content = std::fs::read_to_string(file_path).unwrap();

        let mut lines: Vec<Line> = Vec::with_capacity(30);
        for line in content.lines() {

            let mut initial_spacing_count = 0;
            let mut first_non_blank_char_flag = false;

            let mut current_line = Line {
                content: Vec::with_capacity(30),
                indent: 0,
            };

            let len = line.len();
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
            right: vec![b'l', b':', b' ', b'0', b' ', b'c', b':', b' ', b'0'],
        };

        Buffer {
            file_name: Some(file_path.to_owned()),
            offset: Offset {
                x: 0,
                y: 0,
            },
            main_cursor_index: 0,
            unique_chars: unique_chars_from_lines(chars_per_row, chars_per_coloumn, &lines, &mode_line),
            lines,
            mode_line,
            cursors: vec![
                Position {
                    x: 0,
                    y: 0,
                }
            ],
        }
    }

    pub fn get_this_line_or_max(lines: &[Line], i: u32) -> u32 {
        let len = lines.len() as u32;

        if len < i {
            len
        } else {
            i
        }
    }

    // fn parse(number: u32) -> String {
    //     let mut n = number;
    //     let mut i = 9;
    //     while n > 0 {
    //         let c = n - i;
    //         n -= n - c - i;
    //     }

    //     let first = number & 0x000000FF;
    //     let second = (number & 0x0000FF00) >> 8;
    //     let third = (number & 0x00FF0000) >> 16;
    //     let fourth = (number & 0xFF000000) >> 24;
    // }

    pub fn check_offset(core: &mut Core) -> bool {
        let buffer = &mut core.buffer;
        let cursor = &buffer.cursors[buffer.main_cursor_index as usize];
        let mut change_flag = false;

        if cursor.y >= buffer.offset.y + core.chars_per_coloumn {
            buffer.offset.y = cursor.y - core.chars_per_coloumn + 1;
            change_flag = true;
        } else if cursor.y < buffer.offset.y {
            buffer.offset.y = cursor.y;
            change_flag = true;
        }

        if cursor.x > buffer.offset.x + core.chars_per_row {
            buffer.offset.x = cursor.x - core.chars_per_row;
            change_flag = true;
        } else if cursor.x < buffer.offset.x {
            buffer.offset.x = cursor.x;
            change_flag = true;
        }

        // let line_number_parse = str::parse(cursor.position.y).unwrap();
        // let col_number_parse = str::parse(cursor.position.x).unwrap();
        // core.buffer.mode_line.right =

        change_flag
    }

    pub fn delete_prev_char(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        if position.x == 0 {
            if position.y > 0 {
                position.y -= 1;
                position.x = core.buffer.lines[position.y as usize].content.len() as u32;

                let next_line = core.buffer.lines.remove(position.y as usize + 1);
                core.buffer.lines[position.y as usize].content.extend_from_slice(&next_line.content);
            }
        } else {
            position.x -= 1;
            core.buffer.lines[position.y as usize].content.remove(position.x as usize);
        }
    }

    pub fn start_of_line(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        position.x = 0;
    }

    pub fn end_of_line(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        position.x = core.buffer.lines[position.y as usize].content.len() as u32;
    }

    pub fn next_char(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        let line_len = core.buffer.lines[position.y as usize].content.len() as u32;

        if position.x > line_len {
            if core.buffer.lines.len() > position.y as usize + 1 {
                position.x = 0;
                position.y = 0;
            }
        } else {
            position.x += 1;
        }
    }

    pub fn prev_char(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        if position.x == 0 {
            if position.y > 0 {
                position.y -= 1;
                position.x = core.buffer.lines[position.y as usize].content.len() as u32;
            }
        } else {
            position.x -= 1;
        }
    }

    pub fn prev_line(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        if position.y > 0 {
            position.y -= 1;

            let line_len = core.buffer.lines[position.y as usize].content.len() as u32;
            if position.x > line_len {
                position.x = line_len;
            }
        }
    }

    pub fn next_line(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];

        if core.buffer.lines.len() > position.y as usize + 1 {
            position.y += 1;

            let line_len = core.buffer.lines[position.y as usize].content.len() as u32;
            if position.x > line_len {
                position.x = line_len;
            }
        }
    }

    pub fn insert_new_line(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        let indent = core.buffer.lines[position.y as usize].indent;
        let mut vec = vec![b' '; indent as usize];
        vec.extend_from_slice(&core.buffer.lines[position.y as usize].content[position.x as usize..]);
        println!("indent: {}", indent);

        let line = Line {
            content: vec,
            indent: indent,
        };

        position.y += 1;
        core.buffer.lines.insert(position.y as usize, line);
        core.buffer.lines[position.y as usize - 1].content.truncate(position.x as usize);
        position.x = indent;
    }

    const TAB_LEN: usize = 4;

    pub fn insert_char_at(core: &mut Core, position_index: usize) {
        let c = core.last_inserted_char;
        let chars_inserted = if c == b'\t' {
            let position = &core.buffer.cursors[position_index];
            let current_line = &mut core.buffer.lines[position.y as usize];
            let indent = TAB_LEN as u32 + current_line.indent;

            let mut content = vec![b' '; TAB_LEN as usize];
            content.extend_from_slice(&current_line.content);

            *current_line = Line {
                content,
                indent,
            };

            TAB_LEN
        } else {
        let position = &core.buffer.cursors[position_index];
            core.buffer.lines[position.y as usize].content.insert(position.x as usize, c);
            1
        };

        update_chars(core);
        core.buffer.cursors[position_index].x += chars_inserted as u32;
    }

    pub fn delete_char_at(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        if core.buffer.lines[position.y as usize].content.len() <= position.x as usize {
            if core.buffer.lines.len() > position.y as usize + 1 {
                let next_line = core.buffer.lines.remove(position.y as usize + 1);
                core.buffer.lines[position.y as usize].content.extend_from_slice(&next_line.content);
            }
        } else {
            core.buffer.lines[position.y as usize].content.remove(position.x as usize);
        }
    }

    pub fn delete_to_line_end(core: &mut Core, position_index: usize) {
        let position = &mut core.buffer.cursors[position_index];
        core.buffer.lines[position.y as usize].content.drain(position.x as usize..);
    }

    // fn insert_unique_tab(core: &mut Core, position_index: usize) {
    //     let pos = &core.buffer.cursors[position_index];
    //     let content = &core.buffer.lines[pos.y as usize].content;
    //     let relative_line = pos.y - core.buffer.offset.y;
    //     let line_offset = relative_line as usize * core.chars_per_row as usize;
    //     let mut max_col = content.len() - core.buffer.offset.x as usize - 1;

    //     if max_col + 4 >= core.chars_per_row as usize {
    //         max_col = core.chars_per_row as usize - 4;
    //     }

    //     for j in 0..max_col {
    //         let coordinates = core.buffer.unique_chars.screen_coordinates[line_offset + max_col - j as usize - 1];
    //         core.buffer.unique_chars.positions[coordinates[0] as usize][coordinates[1] as usize][0] += TAB_LEN as u8;
    //         core.buffer.unique_chars.screen_coordinates[line_offset + max_col - j as usize + TAB_LEN] = core.buffer.unique_chars.screen_coordinates[line_offset + max_col - j as usize - 1];
    //     }
    // }

    // fn insert_unique_char(core: &mut Core, position_index: usize) {
    //     let pos = &core.buffer.cursors[position_index];
    //     let content = &core.buffer.lines[pos.y as usize].content;
    //     let c = content[pos.x as usize] - 32;
    //     let line_len = content.len();

    //     let relative_line = pos.y - core.buffer.offset.y;
    //     let relative_coloumn = pos.x - core.buffer.offset.x;
    //     let line_offset = relative_line as usize * core.chars_per_row as usize;

    //     if pos.x + 1 < line_len as u32 {
    //         let max_col = line_len - core.buffer.offset.x as usize - 1;

    //         for j in 0..max_col as u32 - relative_coloumn {
    //             let coordinates = core.buffer.unique_chars.screen_coordinates[line_offset + max_col - j as usize - 1];

    //             core.buffer.unique_chars.positions[coordinates[0] as usize][coordinates[1] as usize][0] += 1;
    //             core.buffer.unique_chars.screen_coordinates[line_offset + max_col - j as usize] = core.buffer.unique_chars.screen_coordinates[line_offset + max_col - j as usize - 1];
    //         }
    //     }

    //     let index = core.buffer.unique_chars.positions[c as usize].len() as u8;
    //     core.buffer.unique_chars.positions[c as usize].push([relative_coloumn as u8, relative_line as u8]);
    //     core.buffer.unique_chars.screen_coordinates[line_offset + relative_coloumn as usize] = [c, index];
    // }

    // pub fn delete_unique_char(core: &mut Core, position_index: usize) {
    //     let pos = &core.buffer.cursors[position_index];
    //     let content = &core.buffer.lines[pos.y as usize].content;

    //     let line_len = content.len();
    //     let relative_line = pos.y - core.buffer.offset.y;
    //     let relative_coloumn = pos.x - core.buffer.offset.x;
    //     let line_offset = relative_line as usize * core.chars_per_row as usize;
    //     let max_col = line_len - core.buffer.offset.x as usize - 1;

    //     if pos.x + 1 < line_len as u32 {
    //         for j in relative_coloumn + 1..max_col as u32 {
    //             let coordinates = core.unique_chars.screen_coordinates[line_offset + j as usize];

    //             core.unique_chars.positions[coordinates[0] as usize][coordinates[1] as usize][0] -= 1;
    //             core.unique_chars.screen_coordinates[line_offset + j as usize ] = core.unique_chars.screen_coordinates[line_offset + j as usize + 1];
    //         }
    //     }

    //     let coordinate_to_delete = core.unique_chars.screen_coordinates[line_offset + relative_coloumn as usize];
    //     let positions = &core.unique_chars.positions[coordinate_to_delete[0] as usize];

    //     if positions.len() <= coordinate_to_delete[1] as usize {
    //         return;
    //     }

    //     for position in positions[coordinate_to_delete[1] as usize + 1..].iter() {
    //         core.unique_chars.screen_coordinates[(position[1] as u32 * core.chars_per_row + position[0] as u32) as usize][1] -= 1;
    //     }

    //     core.unique_chars.positions[coordinate_to_delete[0] as usize].remove(coordinate_to_delete[1] as usize);
    // }

    pub fn update_chars(core: &mut Core) {
        let line_max = get_this_line_or_max(&core.buffer.lines, core.buffer.offset.y + core.chars_per_coloumn);
        let lines = &core.buffer.lines[core.buffer.offset.y as usize..line_max as usize];

        for i in 0..core.buffer.unique_chars.positions.len() {
            core.buffer.unique_chars.positions[i].clear();
        }

        for (i, line) in lines.iter().enumerate() {
            for (j, u) in get_slice(&line.content, core.buffer.offset.x, core.chars_per_row + core.buffer.offset.x).iter().enumerate() {
                let c = *u as usize - 32;

                core.buffer.unique_chars.positions[c].push([j as u8, i as u8]);
            }
        }

        let mode_line_content = mode_line_string(core.chars_per_row, &core.buffer.mode_line);

        for (j, u) in mode_line_content.chars().enumerate() {
            let c = u as usize - 32;
            core.buffer.unique_chars.positions[c].push([j as u8, core.chars_per_coloumn as u8]);
        }
    }

    fn mode_line_string(chars_per_row: u32, mode_line: &ModeLine) -> String {
        let mut content: String = std::str::from_utf8(&mode_line.left).unwrap().to_owned();
        let l = chars_per_row as usize / 2 - mode_line.middle.len() / 2;
        let white_space = l - content.len();

        content.extend(std::str::from_utf8(&vec![b' '; white_space]));
        content.extend(std::str::from_utf8(&mode_line.middle));

        let white_space = chars_per_row as usize - content.len() - mode_line.right.len();
        content.extend(std::str::from_utf8(&mode_line.right));

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

        for (j, u) in mode_line_content.chars().enumerate() {
            let c = u as usize - 32;
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
}

pub fn set_unchanged(core: &mut Core) {
    core.changed = false;
}

pub fn update(core: &mut Core) {
    unsafe {
        wayland::wl_proxy_marshal_flags(core.surface as *mut wayland::wl_proxy, wayland::WL_SURFACE_COMMIT, std::ptr::null(), wayland::wl_proxy_get_version(core.surface as *mut wayland::wl_proxy), 0);
        wayland::wl_display_roundtrip(core.display);
    };

    if let Some(f) = core.last_function {
        if core.last_fetch_delay.elapsed() >= core.key_delay {
            if core.last_fetch_rate.elapsed() >= core.key_rate {
                f(core);
                core.last_fetch_rate = std::time::Instant::now();
            }
        }
    }
}

fn prev_line(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::prev_line(core, i);
    }

    if buffer::check_offset(core) {
        buffer::update_chars(core);
    }

    core.changed = true;
}

fn next_line(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::next_line(core, i);
    }

    if buffer::check_offset(core) {
        buffer::update_chars(core);
    }

    core.changed = true;
}

fn prev_char(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::prev_char(core, i);
    }

    if buffer::check_offset(core) {
        buffer::update_chars(core);
    }

    core.changed = true;
}

fn next_char(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::next_char(core, i);
    }

    if buffer::check_offset(core) {
        buffer::update_chars(core);
    }

    core.changed = true;
}

fn end_of_line(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::end_of_line(core, i);
    }

    if buffer::check_offset(core) {
        buffer::update_chars(core);
    }

    core.changed = true;
}

fn start_of_line(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::start_of_line(core, i);
    }

    if buffer::check_offset(core) {
        buffer::update_chars(core);
    }

    core.changed = true;
}

fn delete_char_at(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::delete_char_at(core, i);
    }

    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn delete_to_line_end(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::delete_to_line_end(core, i);
    }

    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn insert_char_at_current_position(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::insert_char_at(core, i);
    }

    buffer::check_offset(core);
    buffer::update_chars(core);
    core.changed = true;
}

fn insert_new_line(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::insert_new_line(core, i);
    }

    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

fn delete_prev_char(core: &mut Core) {
    for i in 0..core.buffer.cursors.len() {
        buffer::delete_prev_char(core, i);
    }

    buffer::check_offset(core);
    buffer::update_chars(core);

    core.changed = true;
}

const SHIFT_BIT: u8 = 0x01;
const CAPSLOCK_BIT: u8 = 0x02;
const CONTROL_BIT: u8 = 0x04;
const ALT_BIT: u8 = 0x08;

unsafe extern "C" fn keymap(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: i32, _: u32) {}
unsafe extern "C" fn enter(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: *mut wayland::wl_surface, _: *mut wayland::wl_array) {}
unsafe extern "C" fn leave(_: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: *mut wayland::wl_surface,) {}
unsafe extern "C" fn modifiers(data: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, depressed: u32, _: u32, locked: u32, _: u32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    let pressed = depressed as u8 | locked as u8;

    core.control_modifier = pressed & CONTROL_BIT > 0;
    core.shift_modifier = pressed & (SHIFT_BIT | CAPSLOCK_BIT) > 0;
    core.alt_modifier = pressed & ALT_BIT > 0;
}

unsafe extern "C" fn repeat_info(data: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, rate: i32, delay: i32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    core.key_delay = std::time::Duration::from_millis(delay as u64);
    core.key_rate = std::time::Duration::from_millis(rate as u64);
}

unsafe extern "C" fn key(data: *mut std::ffi::c_void, _: *mut wayland::wl_keyboard, _: u32, _: u32, id: u32, state: u32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    let code = id as u8;

    core.last_fetch_delay = std::time::Instant::now();
    core.last_function = None;

    if state == 1 {
        if let Ok(b) = try_ascci(code) {
            let c = if core.shift_modifier {
                b[1]
            } else {
                b[0]
            };

            if core.control_modifier {
                match c {
                    b'p' => core.last_function = Some(prev_line),
                    b'n' => core.last_function = Some(next_line),
                    b'b' => core.last_function = Some(prev_char),
                    b'f' => core.last_function = Some(next_char),
                    b'e' => core.last_function = Some(end_of_line),
                    b'a' => core.last_function = Some(start_of_line),
                    b'd' => core.last_function = Some(delete_char_at),
                    b'k' => core.last_function = Some(delete_to_line_end),
                    _ => {},
                }
            } else if core.alt_modifier {
                match c {
                    // b'f' => core.last_function = Some(next_word),
                    _ => {},
                }
            } else {
                core.last_function = Some(insert_char_at_current_position);
                core.last_inserted_char = c;
            }
        } else if code == ENTER {
            core.last_function = Some(insert_new_line);
        } else if code == BACKSPACE {
            core.last_function = Some(delete_prev_char);
        }

        if let Some(f) = core.last_function {
            f(core)
        }
    }
}

unsafe extern "C" fn seat_name(_: *mut std::ffi::c_void, _: *mut wayland::wl_seat, _: *const i8) {}
unsafe extern "C" fn capabilities(data: *mut std::ffi::c_void, seat: *mut wayland::wl_seat, capability: u32) {
    let mut core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);

    if capability != 0 && wayland::WL_SEAT_CAPABILITY_KEYBOARD != 0 {
        core.keyboard = wayland::wl_proxy_marshal_flags(seat as *mut wayland::wl_proxy, wayland::WL_SEAT_GET_KEYBOARD, &wayland::wl_keyboard_interface, wayland::wl_proxy_get_version(seat as *mut wayland::wl_proxy), 0, std::ptr::null::<std::ffi::c_void>()) as *mut wayland::wl_keyboard;
        wayland::wl_proxy_add_listener(core.keyboard as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::wl_keyboard_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.keyboard_listener), std::mem::transmute::<&mut Core, *mut std::ffi::c_void>(&mut core));
    }
}

unsafe extern "C" fn shell_ping(_: *mut std::ffi::c_void, s: *mut wayland::xdg_wm_base, serial: u32) {
    wayland::wl_proxy_marshal_flags(s as *mut wayland::wl_proxy, wayland::XDG_WM_BASE_PONG, std::ptr::null(), wayland::wl_proxy_get_version(s as *mut wayland::wl_proxy), 0, serial);
}

unsafe extern "C" fn shell_surface_configure(_: *mut std::ffi::c_void, shell_surface: *mut wayland::xdg_surface, serial: u32) {
    wayland::wl_proxy_marshal_flags(shell_surface as *mut wayland::wl_proxy, wayland::XDG_SURFACE_ACK_CONFIGURE, std::ptr::null(), wayland::wl_proxy_get_version(shell_surface as *mut wayland::wl_proxy), 0, serial);
}

unsafe extern "C" fn toplevel_close(data: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    core.running = false;
}

unsafe extern "C" fn toplevel_configure(data: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel, width: i32, height: i32, _: *mut wayland::wl_array) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);

    if width > 0 && height > 0 {
        core.width = width as u32;
        core.height = height as u32;
        core.changed = true;

        core.window_ratio = core.height as f32 / core.width as f32;
        core.chars_per_coloumn = (1.0 / core.scale) as u32 - 1;
        core.chars_per_row = (1.0 / (core.scale * core.x_ratio * core.window_ratio)) as u32;
    }
}

unsafe extern "C" fn toplevel_configure_bounds(_: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel, _: i32, _: i32) {}
unsafe extern "C" fn toplevel_wm_capabilities(_: *mut std::ffi::c_void, _: *mut wayland::xdg_toplevel, _: *mut wayland::wl_array) {}
unsafe extern "C" fn remove_listener(_: *mut std::ffi::c_void, _: *mut wayland::wl_registry, _: u32) {}

unsafe extern "C" fn global_listener(data: *mut std::ffi::c_void, wl_registry: *mut wayland::wl_registry, name: u32, interface: *const std::ffi::c_char, _: u32) {
    let core = std::mem::transmute::<*mut std::ffi::c_void, &mut Core>(data);
    let interface_name = std::ffi::CStr::from_ptr(interface);

    if interface_name == std::ffi::CStr::from_ptr(wayland::wl_compositor_interface.name) {
        core.compositor = wayland::wl_proxy_marshal_flags(
            wl_registry as *mut wayland::wl_proxy,
            wayland::WL_REGISTRY_BIND,
            &wayland::wl_compositor_interface,
            4,
            0,
            name,
            wayland::wl_compositor_interface.name,
            4,
            std::ptr::null::<std::ffi::c_void>(),
        ) as *mut wayland::wl_compositor;
    } else if interface_name == std::ffi::CStr::from_ptr(wayland::xdg_wm_base_interface.name) {
        core.xdg_shell = wayland::wl_proxy_marshal_flags(
            wl_registry as *mut wayland::wl_proxy,
            wayland::WL_REGISTRY_BIND,
            &wayland::xdg_wm_base_interface,
            1,
            0,
            name,
            wayland::xdg_wm_base_interface.name,
            1,
            std::ptr::null::<std::ffi::c_void>(),
        ) as *mut wayland::xdg_wm_base;
    } else if interface_name == std::ffi::CStr::from_ptr(wayland::wl_seat_interface.name) {
        core.seat = wayland::wl_proxy_marshal_flags(
            wl_registry as *mut wayland::wl_proxy,
            wayland::WL_REGISTRY_BIND,
            &wayland::wl_seat_interface,
            1,
            0,
            name,
            wayland::wl_seat_interface.name,
            4,
            std::ptr::null::<std::ffi::c_void>(),
        ) as *mut wayland::wl_seat;
    }
}

pub fn init(
    name: &str,
    width: u32,
    height: u32,
    scale: f32,
    x_ratio: f32,
) -> Result<Box<Core>, WaylandError> {
    let window_ratio = height as f32 / width as f32;
    let chars_per_coloumn = (1.0 / scale) as u32 - 1;
    let chars_per_row = (1.0 / (scale * x_ratio * window_ratio)) as u32;

    let mut core = Box::new(Core {
        display: std::ptr::null_mut(),
        registry: std::ptr::null_mut(),
        surface: std::ptr::null_mut(),
        compositor: std::ptr::null_mut(),
        seat: std::ptr::null_mut(),
        keyboard: std::ptr::null_mut(),
        xdg_shell: std::ptr::null_mut(),
        xdg_surface: std::ptr::null_mut(),
        xdg_toplevel: std::ptr::null_mut(),
        extensions: [
            std::ffi::CString::new("VK_KHR_surface").unwrap().into_raw(),
            std::ffi::CString::new("VK_KHR_wayland_surface").unwrap().into_raw(),
        ],
        width,
        height,
        scale,
        x_ratio,
        window_ratio,
        running: true,
        changed: false,
        buffer: buffer::buffer_from_file(chars_per_row, chars_per_coloumn, "src/main.rs"),
        chars_per_row,
        chars_per_coloumn,
        key_rate: std::time::Duration::from_millis(20),
        key_delay: std::time::Duration::from_millis(200),
        last_function: None,
        last_inserted_char: b' ',
        last_fetch_delay: std::time::Instant::now(),
        last_fetch_rate: std::time::Instant::now(),
        alt_modifier: false,
        shift_modifier: false,
        control_modifier: false,
        registry_listener: wayland::wl_registry_listener {
            global: Some(global_listener),
            global_remove: Some(remove_listener),
        },
        seat_listener: wayland::wl_seat_listener {
            name: Some(seat_name),
            capabilities: Some(capabilities),
        },
        keyboard_listener: wayland::wl_keyboard_listener {
            keymap: Some(keymap),
            enter: Some(enter),
            leave: Some(leave),
            key: Some(key),
            modifiers: Some(modifiers),
            repeat_info: Some(repeat_info),
        },
        shell_listener: wayland::xdg_wm_base_listener {
            ping: Some(shell_ping)
        },
        shell_surface_listener: wayland::xdg_surface_listener {
            configure: Some(shell_surface_configure)
        },
        toplevel_listener: wayland::xdg_toplevel_listener {
            configure: Some(toplevel_configure),
            configure_bounds: Some(toplevel_configure_bounds),
            close: Some(toplevel_close),
            wm_capabilities: Some(toplevel_wm_capabilities),
        },
    });

    core.display = unsafe { wayland::wl_display_connect(std::ptr::null()) } as *mut wayland::wl_display;
    core.registry = unsafe { wayland::wl_proxy_marshal_flags(core.display as *mut wayland::wl_proxy, wayland::WL_DISPLAY_GET_REGISTRY, &wayland::wl_registry_interface, wayland::wl_proxy_get_version(core.display as *mut wayland::wl_proxy), 0) } as *mut wayland::wl_registry;

    let data = unsafe { std::mem::transmute::<&mut Core, *mut std::ffi::c_void>(&mut core) };
    let listener = unsafe { std::mem::transmute::<*mut wayland::wl_registry_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.registry_listener) };
    if 0 != unsafe { wayland::wl_proxy_add_listener(core.registry as *mut wayland::wl_proxy, listener, data) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    unsafe { wayland::wl_display_roundtrip(core.display) };

    core.surface = unsafe { wayland::wl_proxy_marshal_flags(core.compositor as *mut wayland::wl_proxy, wayland::WL_COMPOSITOR_CREATE_SURFACE, &wayland::wl_surface_interface, wayland::wl_proxy_get_version(core.compositor as *mut wayland::wl_proxy), 0, std::ptr::null::<std::ffi::c_void>()) } as *mut wayland::wl_surface;

    if 0 != unsafe { wayland::wl_proxy_add_listener(core.xdg_shell as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::xdg_wm_base_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.shell_listener), data) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    core.xdg_surface = unsafe { wayland::wl_proxy_marshal_flags(core.xdg_shell as *mut wayland::wl_proxy, wayland::XDG_WM_BASE_GET_XDG_SURFACE, &wayland::xdg_surface_interface, wayland::wl_proxy_get_version(core.xdg_shell as *mut wayland::wl_proxy), 0, data, core.surface) } as *mut wayland::xdg_surface;
    if 0 != unsafe { wayland::wl_proxy_add_listener(core.xdg_surface as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::xdg_surface_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.shell_surface_listener), data) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    core.xdg_toplevel = unsafe { wayland::wl_proxy_marshal_flags(core.xdg_surface as *mut wayland::wl_proxy, wayland::XDG_SURFACE_GET_TOPLEVEL, &wayland::xdg_toplevel_interface, wayland::wl_proxy_get_version(core.xdg_surface as *mut wayland::wl_proxy), 0, std::ptr::null_mut::<std::ffi::c_void>()) } as *mut wayland::xdg_toplevel;
    if 0 != unsafe { wayland::wl_proxy_add_listener(core.xdg_toplevel as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::xdg_toplevel_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.toplevel_listener), std::mem::transmute::<*mut Core, *mut std::ffi::c_void>(&mut *core)) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    if 0 != unsafe { wayland::wl_proxy_add_listener(core.seat as *mut wayland::wl_proxy, std::mem::transmute::<*mut wayland::wl_seat_listener, *mut Option<unsafe extern "C" fn()>>(&mut core.seat_listener), std::mem::transmute::<*mut Core, *mut std::ffi::c_void>(&mut *core)) } {
        return Err(WaylandError::CouldNotAddListener);
    }

    let title = std::ffi::CString::new(name).unwrap();
    unsafe { wayland::wl_proxy_marshal_flags(core.xdg_toplevel as *mut wayland::wl_proxy, wayland::XDG_TOPLEVEL_SET_TITLE, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_toplevel as *mut wayland::wl_proxy), 0, title.as_ptr()) };
    unsafe { wayland::wl_proxy_marshal_flags(core.xdg_toplevel as *mut wayland::wl_proxy, wayland::XDG_TOPLEVEL_SET_APP_ID, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_toplevel as *mut wayland::wl_proxy), 0, title.as_ptr()) };
    unsafe { wayland::wl_proxy_marshal_flags(core.surface as *mut wayland::wl_proxy, wayland::WL_SURFACE_COMMIT, std::ptr::null(), wayland::wl_proxy_get_version(core.surface as *mut wayland::wl_proxy), 0) };
    unsafe { wayland::wl_display_roundtrip(core.display) };

    Ok(core)
}

pub fn shutdown(core: &Core) {
    unsafe {
        wayland::wl_proxy_marshal_flags(core.seat as *mut wayland::wl_proxy, wayland::WL_SEAT_RELEASE, std::ptr::null(), wayland::wl_proxy_get_version(core.seat as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_marshal_flags(core.xdg_toplevel as *mut wayland::wl_proxy, wayland::XDG_TOPLEVEL_DESTROY, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_toplevel as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_marshal_flags(core.xdg_surface as *mut wayland::wl_proxy, wayland::XDG_SURFACE_DESTROY, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_surface as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_marshal_flags(core.xdg_shell  as *mut wayland::wl_proxy, wayland::XDG_WM_BASE_DESTROY, std::ptr::null(), wayland::wl_proxy_get_version(core.xdg_shell as *mut wayland::wl_proxy), wayland::WL_MARSHAL_FLAG_DESTROY);
        wayland::wl_proxy_destroy(core.surface as *mut wayland::wl_proxy);
        wayland::wl_proxy_destroy(core.compositor as *mut wayland::wl_proxy);
        wayland::wl_proxy_destroy(core.registry as *mut wayland::wl_proxy);
        wayland::wl_display_disconnect(core.display);
    };
}

const ENTER: u8 = 28;
const BACKSPACE: u8 = 14;

fn try_ascci(u: u8) -> Result<[u8; 2], WaylandError> {
    match u {
        2 => Ok([b'1', b'!']),
        3 => Ok([b'2', b'@']),
        4 => Ok([b'3', b'#']),
        5 => Ok([b'4', b'$']),
        6 => Ok([b'5', b'%']),
        7 => Ok([b'6', b'^']),
        8 => Ok([b'7', b'&']),
        9 => Ok([b'8', b'*']),
        10 => Ok([b'9', b'(']),
        11 => Ok([b'0', b')']),
        12 => Ok([b'-', b'_']),
        13 => Ok([b'=', b'+']),
        15 => Ok([b'\t', b'\t']),

        16 => Ok([b'q', b'Q']),
        17 => Ok([b'w', b'W']),
        18 => Ok([b'e', b'E']),
        19 => Ok([b'r', b'R']),
        20 => Ok([b't', b'T']),
        21 => Ok([b'y', b'Y']),
        22 => Ok([b'u', b'U']),
        23 => Ok([b'i', b'I']),
        24 => Ok([b'o', b'O']),
        25 => Ok([b'p', b'P']),

        26 => Ok([b'[', b'{']),
        27 => Ok([b']', b'}']),

        30 => Ok([b'a', b'A']),
        31 => Ok([b's', b'S']),
        32 => Ok([b'd', b'D']),
        33 => Ok([b'f', b'F']),
        34 => Ok([b'g', b'G']),
        35 => Ok([b'h', b'H']),
        36 => Ok([b'j', b'J']),
        37 => Ok([b'k', b'K']),
        38 => Ok([b'l', b'L']),

        39 => Ok([b';', b':']),
        40 => Ok([b'\'', b'"']),

        43 => Ok([b'\\', b'|']),

        44 => Ok([b'z', b'Z']),
        45 => Ok([b'x', b'X']),
        46 => Ok([b'c', b'C']),
        47 => Ok([b'v', b'V']),
        48 => Ok([b'b', b'B']),
        49 => Ok([b'n', b'N']),
        50 => Ok([b'm', b'M']),

        51 => Ok([b',', b'<']),
        52 => Ok([b'.', b'>']),
        53 => Ok([b'/', b'?']),

        57 => Ok([b' ', b' ']),

        _ => Err(WaylandError::NotAscci),
    }
}
