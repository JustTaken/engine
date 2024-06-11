#[repr(C)]
pub struct stbtt__buf {
    pub data: *mut u8,
    pub cursor: i32,
    pub size: i32,
}

#[repr(C)]
pub struct stbtt_fontinfo {
    pub userdata: *mut std::ffi::c_void,
    pub data: *mut u8,
    pub fontstart: i32,
    pub numGlyphs: i32,
    pub loca: i32,
    pub head: i32,
    pub glyf: i32,
    pub hhea: i32,
    pub hmtx: i32,
    pub kern: i32,
    pub gpos: i32,
    pub svg: i32,
    pub index_map: i32,
    pub indexToLocFormat: i32,
    pub cff: stbtt__buf,
    pub charstrings: stbtt__buf,
    pub gsubrs: stbtt__buf,
    pub subrs: stbtt__buf,
    pub fontdicts: stbtt__buf,
    pub fdselect: stbtt__buf,
}

extern "C" {
    pub fn stbtt_InitFont( info: *mut stbtt_fontinfo, data: *const ::std::os::raw::c_uchar, offset: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn stbtt_ScaleForPixelHeight(info: *const stbtt_fontinfo, pixels: f32) -> f32;
    pub fn stbtt_GetFontVMetrics(info: *const stbtt_fontinfo, ascent: *mut ::std::os::raw::c_int, descent: *mut ::std::os::raw::c_int, lineGap: *mut ::std::os::raw::c_int);
    pub fn stbtt_GetCodepointHMetrics(info: *const stbtt_fontinfo, codepoint: ::std::os::raw::c_int, advanceWidth: *mut ::std::os::raw::c_int, leftSideBearing: *mut ::std::os::raw::c_int);
    pub fn stbtt_GetCodepointBitmapBoxSubpixel(font: *const stbtt_fontinfo, codepoint: ::std::os::raw::c_int, scale_x: f32, scale_y: f32, shift_x: f32, shift_y: f32, ix0: *mut ::std::os::raw::c_int, iy0: *mut ::std::os::raw::c_int, ix1: *mut ::std::os::raw::c_int, iy1: *mut ::std::os::raw::c_int);
    pub fn stbtt_MakeCodepointBitmapSubpixel(info: *const stbtt_fontinfo, output: *mut ::std::os::raw::c_uchar, out_w: ::std::os::raw::c_int, out_h: ::std::os::raw::c_int, out_stride: ::std::os::raw::c_int, scale_x: f32, scale_y: f32, shift_x: f32, shift_y: f32, codepoint: ::std::os::raw::c_int);
}
