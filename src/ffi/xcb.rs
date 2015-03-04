#![allow(dead_code)]

use libc::{c_int, c_char, c_uchar, c_ushort, c_uint};
use std::ptr;
use std::mem;
use std::default::Default;

// CONSTANTS

const XCB_WINDOW_CLASS_COPY_FROM_PARENT: c_ushort = 0;
const XCB_WINDOW_CLASS_INPUT_OUTPUT: c_ushort = 1;
const XCB_WINDOW_CLASS_INPUT_ONLY: c_ushort = 2;

const XCB_GC_FUNCTION: c_uint = 1;
const XCB_GC_PLANE_MASK: c_uint = 2;
const XCB_GC_FOREGROUND: c_uint = 4;
const XCB_GC_BACKGROUND: c_uint = 8;
const XCB_GC_LINE_WIDTH: c_uint = 16;
const XCB_GC_LINE_STYLE: c_uint = 32;
const XCB_GC_CAP_STYLE: c_uint = 64;
const XCB_GC_JOIN_STYLE: c_uint = 128;
const XCB_GC_FILL_STYLE: c_uint = 256;
const XCB_GC_FILL_RULE: c_uint = 512;
const XCB_GC_TILE: c_uint = 1024;
const XCB_GC_STIPPLE: c_uint = 2048;
const XCB_GC_TILE_STIPPLE_ORIGIN_X: c_uint = 4096;
const XCB_GC_TILE_STIPPLE_ORIGIN_Y: c_uint = 8192;
const XCB_GC_FONT: c_uint = 16384;
const XCB_GC_SUBWINDOW_MODE: c_uint = 32768;
const XCB_GC_GRAPHICS_EXPOSURES: c_uint = 65536;
const XCB_GC_CLIP_ORIGIN_X: c_uint = 131072;
const XCB_GC_CLIP_ORIGIN_Y: c_uint = 262144;
const XCB_GC_CLIP_MASK: c_uint = 524288;
const XCB_GC_DASH_OFFSET: c_uint = 1048576;
const XCB_GC_DASH_LIST: c_uint = 2097152;
const XCB_GC_ARC_MODE: c_uint = 4194304;

const XCB_CW_BACK_PIXMAP: c_uint = 1;
const XCB_CW_BACK_PIXEL: c_uint = 2;
const XCB_CW_BORDER_PIXMAP: c_uint = 4;
const XCB_CW_BORDER_PIXEL: c_uint = 8;
const XCB_CW_BIT_GRAVITY: c_uint = 16;
const XCB_CW_WIN_GRAVITY: c_uint = 32;
const XCB_CW_BACKING_STORE: c_uint = 64;
const XCB_CW_BACKING_PLANES: c_uint = 128;
const XCB_CW_BACKING_PIXEL: c_uint = 256;
const XCB_CW_OVERRIDE_REDIRECT: c_uint = 512;
const XCB_CW_SAVE_UNDER: c_uint = 1024;
const XCB_CW_EVENT_MASK: c_uint = 2048;
const XCB_CW_DONT_PROPAGATE: c_uint = 4096;
const XCB_CW_COLORMAP: c_uint = 8192;
const XCB_CW_CURSOR: c_uint = 16384;

const XCB_EVENT_MASK_NO_EVENT: c_uint = 0;
const XCB_EVENT_MASK_KEY_PRESS: c_uint = 1;
const XCB_EVENT_MASK_KEY_RELEASE: c_uint = 2;
const XCB_EVENT_MASK_BUTTON_PRESS: c_uint = 4;
const XCB_EVENT_MASK_BUTTON_RELEASE: c_uint = 8;
const XCB_EVENT_MASK_ENTER_WINDOW: c_uint = 16;
const XCB_EVENT_MASK_LEAVE_WINDOW: c_uint = 32;
const XCB_EVENT_MASK_POINTER_MOTION: c_uint = 64;
const XCB_EVENT_MASK_POINTER_MOTION_HINT: c_uint = 128;
const XCB_EVENT_MASK_BUTTON_1_MOTION: c_uint = 256;
const XCB_EVENT_MASK_BUTTON_2_MOTION: c_uint = 512;
const XCB_EVENT_MASK_BUTTON_3_MOTION: c_uint = 1024;
const XCB_EVENT_MASK_BUTTON_4_MOTION: c_uint = 2048;
const XCB_EVENT_MASK_BUTTON_5_MOTION: c_uint = 4096;
const XCB_EVENT_MASK_BUTTON_MOTION: c_uint = 8192;
const XCB_EVENT_MASK_KEYMAP_STATE: c_uint = 16384;
const XCB_EVENT_MASK_EXPOSURE: c_uint = 32768;
const XCB_EVENT_MASK_VISIBILITY_CHANGE: c_uint = 65536;
const XCB_EVENT_MASK_STRUCTURE_NOTIFY: c_uint = 131072;
const XCB_EVENT_MASK_RESIZE_REDIRECT: c_uint = 262144;
const XCB_EVENT_MASK_SUBSTRUCTURE_NOTIFY: c_uint = 524288;
const XCB_EVENT_MASK_SUBSTRUCTURE_REDIRECT: c_uint = 1048576;
const XCB_EVENT_MASK_FOCUS_CHANGE: c_uint = 2097152;
const XCB_EVENT_MASK_PROPERTY_CHANGE: c_uint = 4194304;
const XCB_EVENT_MASK_COLOR_MAP_CHANGE: c_uint = 8388608;
const XCB_EVENT_MASK_OWNER_GRAB_BUTTON: c_uint = 16777216;


// TYPES & STRUCTURES

type XCBKeycodeFFI = c_uchar;

type XCBWindowFFI = c_uint;

type XCBColormapFFI = c_uint;

type XCBVisualidFFI = c_uint;

type XCBGcontextFFI = c_uint;

type XCBDrawableFFI = c_uint;

#[repr(C)]
struct XCBConnectionFFI;

#[repr(C)]
#[derive(Copy)]
struct XCBSetupFFI {
    status: c_uchar,
    pad0: c_uchar,
    protocol_major_version: c_ushort,
    protocol_minor_version: c_ushort,
    length: c_ushort,
    release_number: c_uint,
    resource_id_base: c_uint,
    resource_id_mask: c_uint,
    motion_buffer_size: c_uint,
    vendor_len: c_ushort,
    maximum_request_length: c_ushort,
    roots_len: c_uchar,
    pixmap_formats_len: c_uchar,
    image_byte_order: c_uchar,
    bitmap_format_bit_order: c_uchar,
    bitmap_format_scanline_unit: c_uchar,
    bitmap_format_scanline_pad: c_uchar,
    min_keycode: XCBKeycodeFFI,
    max_keycode: XCBKeycodeFFI,
    pad1: [c_uchar; 4usize],
}

#[repr(C)]
#[derive(Copy)]
struct XCBScreenFFI {
    root: XCBWindowFFI,
    default_colormap: XCBColormapFFI,
    white_pixel: c_uint,
    black_pixel: c_uint,
    current_input_masks: c_uint,
    width_in_pixels: c_ushort,
    height_in_pixels: c_ushort,
    width_in_millimeters: c_ushort,
    height_in_millimeters: c_ushort,
    min_installed_maps: c_ushort,
    max_installed_maps: c_ushort,
    root_visual: XCBVisualidFFI,
    backing_stores: c_uchar,
    save_unders: c_uchar,
    root_depth: c_uchar,
    allowed_depths_len: c_uchar,
}

#[repr(C)]
#[derive(Copy)]
struct XCBScreenIteratorFFI {
    data: *mut XCBScreenFFI,
    rem: c_int,
    index: c_int,
}

#[repr(C)]
#[derive(Copy)]
struct XCBVoidCookieFFI {
    sequence: c_uint,
}

impl Default for XCBVoidCookieFFI {
    fn default() -> XCBVoidCookieFFI { unsafe { mem::zeroed() } }
}


#[repr(C)]
#[derive(Copy)]
pub struct XCBGenericEventFFI {
    pub response_type: c_uchar,
    pub pad0: c_uchar,
    pub sequence: c_ushort,
    pub pad: [c_uint; 7usize],
    pub full_sequence: c_uint,
}

impl Default for XCBGenericEventFFI {
    fn default() -> XCBGenericEventFFI { unsafe { mem::zeroed() } }
}


// FUNCTIONS

#[link(name = "xcb")]
extern {
    fn xcb_connect(displayname: *const c_char, screenp:*mut c_int) -> *mut XCBConnectionFFI;

    fn xcb_connection_has_error(c: *mut XCBConnectionFFI) -> c_int;

    fn xcb_disconnect(c: *mut XCBConnectionFFI);

    fn xcb_get_setup(c: *mut XCBConnectionFFI) -> *const XCBSetupFFI;

    fn xcb_setup_roots_iterator(R: *const XCBSetupFFI) -> XCBScreenIteratorFFI;

    fn xcb_generate_id(c: *mut XCBConnectionFFI) -> c_uint;

    fn xcb_create_gc(
        c: *mut XCBConnectionFFI,
        cid: XCBGcontextFFI,
        drawable: XCBDrawableFFI,
        value_mask: c_uint,
        value_list: *const c_uint
    ) -> XCBVoidCookieFFI;

    fn xcb_create_window(
        c: *mut XCBConnectionFFI,
        depth: c_uchar,
        wid: XCBWindowFFI,
        parent: XCBWindowFFI,
        x: c_ushort,
        y: c_ushort,
        width: c_ushort,
        height: c_ushort,
        border_width: c_ushort,
        _class: c_ushort,
        visual: XCBVisualidFFI,
        value_mask: c_uint,
        value_list: *const c_uint
    ) -> XCBVoidCookieFFI;

    fn xcb_map_window(c: *mut XCBConnectionFFI, window: XCBWindowFFI) -> XCBVoidCookieFFI;

    fn xcb_flush(c: *mut XCBConnectionFFI) -> c_int;

    fn xcb_wait_for_event(c: *mut XCBConnectionFFI) -> *mut XCBGenericEventFFI;
}


// PUB API

pub struct XCB {
    connection: *mut XCBConnectionFFI,

    screen: Option<*mut XCBScreenFFI>,

    window: Option<XCBWindowFFI>,
}


impl XCB {
    #[inline]
    pub fn new() -> Self {
        unsafe {
            let mut screen: c_int = 0;
            let connection = xcb_connect(ptr::null(), &mut screen);

            if xcb_connection_has_error(connection) > 0 {
                panic!("Fatal error during establishing a XCB connection.")
            }

            XCB {
                connection: connection,
                screen: None,
                window: None,
            }
        }
    }

    #[inline]
    pub fn disconnect(&self) {
        unsafe {
            xcb_disconnect(self.connection);
        }
    }

    pub fn create_window(&mut self) {
        self.init_screen();

        unsafe {
            let window = xcb_generate_id(self.connection);
            self.window = Some(window);

            let screen = *self.screen.unwrap();

            let mask = XCB_CW_BACK_PIXEL | XCB_CW_EVENT_MASK;
            let value_list: [u32; 2] = [
                screen.black_pixel,
                XCB_EVENT_MASK_EXPOSURE | XCB_EVENT_MASK_KEY_PRESS
            ];

            xcb_create_window(
                self.connection,
                0, // Depth - copy from parent
                window,
                screen.root,
                0, 0,
                screen.width_in_pixels, screen.height_in_pixels,
                0,
                XCB_WINDOW_CLASS_INPUT_OUTPUT,
                screen.root_visual,
                mask, value_list.as_ptr(),
            );

            xcb_map_window(self.connection, window);

            xcb_flush(self.connection);
        }
    }

    pub fn exec(&self) {
        unsafe {
            loop {
                let event = xcb_wait_for_event(self.connection);
                let event_type = (*event).response_type & !0x80;

                if event_type == 12 { // XCB_EXPOSE
                    xcb_flush(self.connection);
                } else if event_type ==  2 { // XCB_KEY_PRESS
                    break;
                }
            }
        }
    }

    fn init_screen(&mut self) {
        match self.screen {
            None => unsafe {
                let iterator = xcb_setup_roots_iterator(
                    xcb_get_setup(self.connection)
                );

                self.screen = Some(iterator.data);
            },
            Some(_) => ()
        }
    }

    pub fn print_screen_dimensions(&self) {
        match self.screen {
            Some(screenp) => unsafe {
                let screen = *screenp;
                println!(
                    "Screen w: {}, h: {}",
                    screen.width_in_pixels,
                    screen.height_in_pixels
                );
            },
            None => ()
        }
    }
}



