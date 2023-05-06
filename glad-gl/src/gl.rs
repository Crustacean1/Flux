pub use self::enumerations::*;
pub use self::functions::*;
pub use self::types::*;

use std::os::raw::c_void;

#[derive(Copy, Clone)]
struct FnPtr {
    ptr: *const c_void,
    is_loaded: bool,
}

#[allow(dead_code)]
impl FnPtr {
    fn new(ptr: *const c_void) -> FnPtr {
        if !ptr.is_null() {
            FnPtr {
                ptr,
                is_loaded: true,
            }
        } else {
            FnPtr {
                ptr: FnPtr::not_initialized as *const c_void,
                is_loaded: false,
            }
        }
    }

    fn set_ptr(&mut self, ptr: *const c_void) {
        *self = Self::new(ptr);
    }

    fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            *self = *other;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! {
        panic!("gl: function not initialized")
    }
}

unsafe impl Sync for FnPtr {}
unsafe impl Send for FnPtr {}

pub mod types {
    #![allow(dead_code, non_snake_case, non_camel_case_types)]

    use std::os::raw;

    pub type GLvoid = raw::c_void;

    pub type GLbyte = raw::c_char;
    pub type GLubyte = raw::c_uchar;
    pub type GLchar = raw::c_char;
    pub type GLboolean = raw::c_uchar;

    pub type GLshort = raw::c_short;
    pub type GLushort = raw::c_ushort;

    pub type GLint = raw::c_int;
    pub type GLuint = raw::c_uint;
    pub type GLint64 = i64;
    pub type GLuint64 = u64;

    pub type GLintptr = isize;
    pub type GLsizeiptr = isize;
    pub type GLintptrARB = isize;
    pub type GLsizeiptrARB = isize;
    pub type GLint64EXT = i64;
    pub type GLuint64EXT = u64;

    pub type GLsizei = GLint;
    pub type GLclampx = raw::c_int;
    pub type GLfixed = GLint;
    pub type GLhalf = raw::c_ushort;
    pub type GLhalfNV = raw::c_ushort;
    pub type GLhalfARB = raw::c_ushort;

    pub type GLenum = raw::c_uint;
    pub type GLbitfield = raw::c_uint;

    pub type GLfloat = raw::c_float;
    pub type GLdouble = raw::c_double;
    pub type GLclampf = raw::c_float;
    pub type GLclampd = raw::c_double;

    pub type GLcharARB = raw::c_char;

    #[cfg(target_os = "macos")]
    pub type GLhandleARB = *const raw::c_void;
    #[cfg(not(target_os = "macos"))]
    pub type GLhandleARB = raw::c_uint;

    pub enum __GLsync {}

    pub type GLsync = *const __GLsync;

    pub enum _cl_context {}

    pub enum _cl_event {}

    pub type GLvdpauSurfaceNV = GLintptr;
    pub type GLeglClientBufferEXT = *const raw::c_void;
    pub type GLeglImageOES = *const raw::c_void;

    pub type GLDEBUGPROC = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut raw::c_void,
    );
    pub type GLDEBUGPROCARB = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut raw::c_void,
    );
    pub type GLDEBUGPROCKHR = extern "system" fn(
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLDEBUGPROCAMD = extern "system" fn(
        id: GLuint,
        category: GLenum,
        severity: GLenum,
        length: GLsizei,
        message: *const GLchar,
        userParam: *mut GLvoid,
    );
    pub type GLVULKANPROCNV = extern "system" fn();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use super::types::*;
    use std::os::raw::*;

    pub const ACTIVE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D9;
    pub const ACTIVE_ATTRIBUTES: c_uint = 0x8B89;
    pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: c_uint = 0x8B8A;
    pub const ACTIVE_PROGRAM: c_uint = 0x8259;
    pub const ACTIVE_RESOURCES: c_uint = 0x92F5;
    pub const ACTIVE_SUBROUTINES: c_uint = 0x8DE5;
    pub const ACTIVE_SUBROUTINE_MAX_LENGTH: c_uint = 0x8E48;
    pub const ACTIVE_SUBROUTINE_UNIFORMS: c_uint = 0x8DE6;
    pub const ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8E47;
    pub const ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH: c_uint = 0x8E49;
    pub const ACTIVE_TEXTURE: c_uint = 0x84E0;
    pub const ACTIVE_UNIFORMS: c_uint = 0x8B86;
    pub const ACTIVE_UNIFORM_BLOCKS: c_uint = 0x8A36;
    pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: c_uint = 0x8A35;
    pub const ACTIVE_UNIFORM_MAX_LENGTH: c_uint = 0x8B87;
    pub const ACTIVE_VARIABLES: c_uint = 0x9305;
    pub const ALIASED_LINE_WIDTH_RANGE: c_uint = 0x846E;
    pub const ALL_BARRIER_BITS: c_uint = 0xFFFFFFFF;
    pub const ALL_SHADER_BITS: c_uint = 0xFFFFFFFF;
    pub const ALPHA: c_uint = 0x1906;
    pub const ALREADY_SIGNALED: c_uint = 0x911A;
    pub const ALWAYS: c_uint = 0x0207;
    pub const AND: c_uint = 0x1501;
    pub const AND_INVERTED: c_uint = 0x1504;
    pub const AND_REVERSE: c_uint = 0x1502;
    pub const ANY_SAMPLES_PASSED: c_uint = 0x8C2F;
    pub const ANY_SAMPLES_PASSED_CONSERVATIVE: c_uint = 0x8D6A;
    pub const ARRAY_BUFFER: c_uint = 0x8892;
    pub const ARRAY_BUFFER_BINDING: c_uint = 0x8894;
    pub const ARRAY_SIZE: c_uint = 0x92FB;
    pub const ARRAY_STRIDE: c_uint = 0x92FE;
    pub const ATOMIC_COUNTER_BARRIER_BIT: c_uint = 0x00001000;
    pub const ATOMIC_COUNTER_BUFFER: c_uint = 0x92C0;
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS: c_uint = 0x92C5;
    pub const ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES: c_uint = 0x92C6;
    pub const ATOMIC_COUNTER_BUFFER_BINDING: c_uint = 0x92C1;
    pub const ATOMIC_COUNTER_BUFFER_DATA_SIZE: c_uint = 0x92C4;
    pub const ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x9301;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90ED;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x92CB;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x92CA;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x92C8;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x92C9;
    pub const ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x92C7;
    pub const ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92C3;
    pub const ATOMIC_COUNTER_BUFFER_START: c_uint = 0x92C2;
    pub const ATTACHED_SHADERS: c_uint = 0x8B85;
    pub const AUTO_GENERATE_MIPMAP: c_uint = 0x8295;
    pub const BACK: c_uint = 0x0405;
    pub const BACK_LEFT: c_uint = 0x0402;
    pub const BACK_RIGHT: c_uint = 0x0403;
    pub const BGR: c_uint = 0x80E0;
    pub const BGRA: c_uint = 0x80E1;
    pub const BGRA_INTEGER: c_uint = 0x8D9B;
    pub const BGR_INTEGER: c_uint = 0x8D9A;
    pub const BLEND: c_uint = 0x0BE2;
    pub const BLEND_COLOR: c_uint = 0x8005;
    pub const BLEND_DST: c_uint = 0x0BE0;
    pub const BLEND_DST_ALPHA: c_uint = 0x80CA;
    pub const BLEND_DST_RGB: c_uint = 0x80C8;
    pub const BLEND_EQUATION: c_uint = 0x8009;
    pub const BLEND_EQUATION_ALPHA: c_uint = 0x883D;
    pub const BLEND_EQUATION_RGB: c_uint = 0x8009;
    pub const BLEND_SRC: c_uint = 0x0BE1;
    pub const BLEND_SRC_ALPHA: c_uint = 0x80CB;
    pub const BLEND_SRC_RGB: c_uint = 0x80C9;
    pub const BLOCK_INDEX: c_uint = 0x92FD;
    pub const BLUE: c_uint = 0x1905;
    pub const BLUE_INTEGER: c_uint = 0x8D96;
    pub const BOOL: c_uint = 0x8B56;
    pub const BOOL_VEC2: c_uint = 0x8B57;
    pub const BOOL_VEC3: c_uint = 0x8B58;
    pub const BOOL_VEC4: c_uint = 0x8B59;
    pub const BUFFER: c_uint = 0x82E0;
    pub const BUFFER_ACCESS: c_uint = 0x88BB;
    pub const BUFFER_ACCESS_FLAGS: c_uint = 0x911F;
    pub const BUFFER_BINDING: c_uint = 0x9302;
    pub const BUFFER_DATA_SIZE: c_uint = 0x9303;
    pub const BUFFER_IMMUTABLE_STORAGE: c_uint = 0x821F;
    pub const BUFFER_MAPPED: c_uint = 0x88BC;
    pub const BUFFER_MAP_LENGTH: c_uint = 0x9120;
    pub const BUFFER_MAP_OFFSET: c_uint = 0x9121;
    pub const BUFFER_MAP_POINTER: c_uint = 0x88BD;
    pub const BUFFER_SIZE: c_uint = 0x8764;
    pub const BUFFER_STORAGE_FLAGS: c_uint = 0x8220;
    pub const BUFFER_UPDATE_BARRIER_BIT: c_uint = 0x00000200;
    pub const BUFFER_USAGE: c_uint = 0x8765;
    pub const BUFFER_VARIABLE: c_uint = 0x92E5;
    pub const BYTE: c_uint = 0x1400;
    pub const CAVEAT_SUPPORT: c_uint = 0x82B8;
    pub const CCW: c_uint = 0x0901;
    pub const CLAMP_READ_COLOR: c_uint = 0x891C;
    pub const CLAMP_TO_BORDER: c_uint = 0x812D;
    pub const CLAMP_TO_EDGE: c_uint = 0x812F;
    pub const CLEAR: c_uint = 0x1500;
    pub const CLEAR_BUFFER: c_uint = 0x82B4;
    pub const CLEAR_TEXTURE: c_uint = 0x9365;
    pub const CLIENT_MAPPED_BUFFER_BARRIER_BIT: c_uint = 0x00004000;
    pub const CLIENT_STORAGE_BIT: c_uint = 0x0200;
    pub const CLIPPING_INPUT_PRIMITIVES: c_uint = 0x82F6;
    pub const CLIPPING_OUTPUT_PRIMITIVES: c_uint = 0x82F7;
    pub const CLIP_DEPTH_MODE: c_uint = 0x935D;
    pub const CLIP_DISTANCE0: c_uint = 0x3000;
    pub const CLIP_DISTANCE1: c_uint = 0x3001;
    pub const CLIP_DISTANCE2: c_uint = 0x3002;
    pub const CLIP_DISTANCE3: c_uint = 0x3003;
    pub const CLIP_DISTANCE4: c_uint = 0x3004;
    pub const CLIP_DISTANCE5: c_uint = 0x3005;
    pub const CLIP_DISTANCE6: c_uint = 0x3006;
    pub const CLIP_DISTANCE7: c_uint = 0x3007;
    pub const CLIP_ORIGIN: c_uint = 0x935C;
    pub const COLOR: c_uint = 0x1800;
    pub const COLOR_ATTACHMENT0: c_uint = 0x8CE0;
    pub const COLOR_ATTACHMENT1: c_uint = 0x8CE1;
    pub const COLOR_ATTACHMENT10: c_uint = 0x8CEA;
    pub const COLOR_ATTACHMENT11: c_uint = 0x8CEB;
    pub const COLOR_ATTACHMENT12: c_uint = 0x8CEC;
    pub const COLOR_ATTACHMENT13: c_uint = 0x8CED;
    pub const COLOR_ATTACHMENT14: c_uint = 0x8CEE;
    pub const COLOR_ATTACHMENT15: c_uint = 0x8CEF;
    pub const COLOR_ATTACHMENT16: c_uint = 0x8CF0;
    pub const COLOR_ATTACHMENT17: c_uint = 0x8CF1;
    pub const COLOR_ATTACHMENT18: c_uint = 0x8CF2;
    pub const COLOR_ATTACHMENT19: c_uint = 0x8CF3;
    pub const COLOR_ATTACHMENT2: c_uint = 0x8CE2;
    pub const COLOR_ATTACHMENT20: c_uint = 0x8CF4;
    pub const COLOR_ATTACHMENT21: c_uint = 0x8CF5;
    pub const COLOR_ATTACHMENT22: c_uint = 0x8CF6;
    pub const COLOR_ATTACHMENT23: c_uint = 0x8CF7;
    pub const COLOR_ATTACHMENT24: c_uint = 0x8CF8;
    pub const COLOR_ATTACHMENT25: c_uint = 0x8CF9;
    pub const COLOR_ATTACHMENT26: c_uint = 0x8CFA;
    pub const COLOR_ATTACHMENT27: c_uint = 0x8CFB;
    pub const COLOR_ATTACHMENT28: c_uint = 0x8CFC;
    pub const COLOR_ATTACHMENT29: c_uint = 0x8CFD;
    pub const COLOR_ATTACHMENT3: c_uint = 0x8CE3;
    pub const COLOR_ATTACHMENT30: c_uint = 0x8CFE;
    pub const COLOR_ATTACHMENT31: c_uint = 0x8CFF;
    pub const COLOR_ATTACHMENT4: c_uint = 0x8CE4;
    pub const COLOR_ATTACHMENT5: c_uint = 0x8CE5;
    pub const COLOR_ATTACHMENT6: c_uint = 0x8CE6;
    pub const COLOR_ATTACHMENT7: c_uint = 0x8CE7;
    pub const COLOR_ATTACHMENT8: c_uint = 0x8CE8;
    pub const COLOR_ATTACHMENT9: c_uint = 0x8CE9;
    pub const COLOR_BUFFER_BIT: c_uint = 0x00004000;
    pub const COLOR_CLEAR_VALUE: c_uint = 0x0C22;
    pub const COLOR_COMPONENTS: c_uint = 0x8283;
    pub const COLOR_ENCODING: c_uint = 0x8296;
    pub const COLOR_LOGIC_OP: c_uint = 0x0BF2;
    pub const COLOR_RENDERABLE: c_uint = 0x8286;
    pub const COLOR_WRITEMASK: c_uint = 0x0C23;
    pub const COMMAND_BARRIER_BIT: c_uint = 0x00000040;
    pub const COMPARE_REF_TO_TEXTURE: c_uint = 0x884E;
    pub const COMPATIBLE_SUBROUTINES: c_uint = 0x8E4B;
    pub const COMPILE_STATUS: c_uint = 0x8B81;
    pub const COMPRESSED_R11_EAC: c_uint = 0x9270;
    pub const COMPRESSED_RED: c_uint = 0x8225;
    pub const COMPRESSED_RED_RGTC1: c_uint = 0x8DBB;
    pub const COMPRESSED_RG: c_uint = 0x8226;
    pub const COMPRESSED_RG11_EAC: c_uint = 0x9272;
    pub const COMPRESSED_RGB: c_uint = 0x84ED;
    pub const COMPRESSED_RGB8_ETC2: c_uint = 0x9274;
    pub const COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9276;
    pub const COMPRESSED_RGBA: c_uint = 0x84EE;
    pub const COMPRESSED_RGBA8_ETC2_EAC: c_uint = 0x9278;
    pub const COMPRESSED_RGBA_BPTC_UNORM: c_uint = 0x8E8C;
    pub const COMPRESSED_RGB_BPTC_SIGNED_FLOAT: c_uint = 0x8E8E;
    pub const COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: c_uint = 0x8E8F;
    pub const COMPRESSED_RG_RGTC2: c_uint = 0x8DBD;
    pub const COMPRESSED_SIGNED_R11_EAC: c_uint = 0x9271;
    pub const COMPRESSED_SIGNED_RED_RGTC1: c_uint = 0x8DBC;
    pub const COMPRESSED_SIGNED_RG11_EAC: c_uint = 0x9273;
    pub const COMPRESSED_SIGNED_RG_RGTC2: c_uint = 0x8DBE;
    pub const COMPRESSED_SRGB: c_uint = 0x8C48;
    pub const COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: c_uint = 0x9279;
    pub const COMPRESSED_SRGB8_ETC2: c_uint = 0x9275;
    pub const COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: c_uint = 0x9277;
    pub const COMPRESSED_SRGB_ALPHA: c_uint = 0x8C49;
    pub const COMPRESSED_SRGB_ALPHA_BPTC_UNORM: c_uint = 0x8E8D;
    pub const COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A3;
    pub const COMPUTE_SHADER: c_uint = 0x91B9;
    pub const COMPUTE_SHADER_BIT: c_uint = 0x00000020;
    pub const COMPUTE_SHADER_INVOCATIONS: c_uint = 0x82F5;
    pub const COMPUTE_SUBROUTINE: c_uint = 0x92ED;
    pub const COMPUTE_SUBROUTINE_UNIFORM: c_uint = 0x92F3;
    pub const COMPUTE_TEXTURE: c_uint = 0x82A0;
    pub const COMPUTE_WORK_GROUP_SIZE: c_uint = 0x8267;
    pub const CONDITION_SATISFIED: c_uint = 0x911C;
    pub const CONSTANT_ALPHA: c_uint = 0x8003;
    pub const CONSTANT_COLOR: c_uint = 0x8001;
    pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: c_uint = 0x00000002;
    pub const CONTEXT_CORE_PROFILE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_FLAGS: c_uint = 0x821E;
    pub const CONTEXT_FLAG_DEBUG_BIT: c_uint = 0x00000002;
    pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: c_uint = 0x00000001;
    pub const CONTEXT_FLAG_NO_ERROR_BIT: c_uint = 0x00000008;
    pub const CONTEXT_FLAG_ROBUST_ACCESS_BIT: c_uint = 0x00000004;
    pub const CONTEXT_LOST: c_uint = 0x0507;
    pub const CONTEXT_PROFILE_MASK: c_uint = 0x9126;
    pub const CONTEXT_RELEASE_BEHAVIOR: c_uint = 0x82FB;
    pub const CONTEXT_RELEASE_BEHAVIOR_FLUSH: c_uint = 0x82FC;
    pub const COPY: c_uint = 0x1503;
    pub const COPY_INVERTED: c_uint = 0x150C;
    pub const COPY_READ_BUFFER: c_uint = 0x8F36;
    pub const COPY_READ_BUFFER_BINDING: c_uint = 0x8F36;
    pub const COPY_WRITE_BUFFER: c_uint = 0x8F37;
    pub const COPY_WRITE_BUFFER_BINDING: c_uint = 0x8F37;
    pub const CULL_FACE: c_uint = 0x0B44;
    pub const CULL_FACE_MODE: c_uint = 0x0B45;
    pub const CURRENT_PROGRAM: c_uint = 0x8B8D;
    pub const CURRENT_QUERY: c_uint = 0x8865;
    pub const CURRENT_VERTEX_ATTRIB: c_uint = 0x8626;
    pub const CW: c_uint = 0x0900;
    pub const DEBUG_CALLBACK_FUNCTION: c_uint = 0x8244;
    pub const DEBUG_CALLBACK_USER_PARAM: c_uint = 0x8245;
    pub const DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826D;
    pub const DEBUG_LOGGED_MESSAGES: c_uint = 0x9145;
    pub const DEBUG_NEXT_LOGGED_MESSAGE_LENGTH: c_uint = 0x8243;
    pub const DEBUG_OUTPUT: c_uint = 0x92E0;
    pub const DEBUG_OUTPUT_SYNCHRONOUS: c_uint = 0x8242;
    pub const DEBUG_SEVERITY_HIGH: c_uint = 0x9146;
    pub const DEBUG_SEVERITY_LOW: c_uint = 0x9148;
    pub const DEBUG_SEVERITY_MEDIUM: c_uint = 0x9147;
    pub const DEBUG_SEVERITY_NOTIFICATION: c_uint = 0x826B;
    pub const DEBUG_SOURCE_API: c_uint = 0x8246;
    pub const DEBUG_SOURCE_APPLICATION: c_uint = 0x824A;
    pub const DEBUG_SOURCE_OTHER: c_uint = 0x824B;
    pub const DEBUG_SOURCE_SHADER_COMPILER: c_uint = 0x8248;
    pub const DEBUG_SOURCE_THIRD_PARTY: c_uint = 0x8249;
    pub const DEBUG_SOURCE_WINDOW_SYSTEM: c_uint = 0x8247;
    pub const DEBUG_TYPE_DEPRECATED_BEHAVIOR: c_uint = 0x824D;
    pub const DEBUG_TYPE_ERROR: c_uint = 0x824C;
    pub const DEBUG_TYPE_MARKER: c_uint = 0x8268;
    pub const DEBUG_TYPE_OTHER: c_uint = 0x8251;
    pub const DEBUG_TYPE_PERFORMANCE: c_uint = 0x8250;
    pub const DEBUG_TYPE_POP_GROUP: c_uint = 0x826A;
    pub const DEBUG_TYPE_PORTABILITY: c_uint = 0x824F;
    pub const DEBUG_TYPE_PUSH_GROUP: c_uint = 0x8269;
    pub const DEBUG_TYPE_UNDEFINED_BEHAVIOR: c_uint = 0x824E;
    pub const DECR: c_uint = 0x1E03;
    pub const DECR_WRAP: c_uint = 0x8508;
    pub const DELETE_STATUS: c_uint = 0x8B80;
    pub const DEPTH: c_uint = 0x1801;
    pub const DEPTH24_STENCIL8: c_uint = 0x88F0;
    pub const DEPTH32F_STENCIL8: c_uint = 0x8CAD;
    pub const DEPTH_ATTACHMENT: c_uint = 0x8D00;
    pub const DEPTH_BUFFER_BIT: c_uint = 0x00000100;
    pub const DEPTH_CLAMP: c_uint = 0x864F;
    pub const DEPTH_CLEAR_VALUE: c_uint = 0x0B73;
    pub const DEPTH_COMPONENT: c_uint = 0x1902;
    pub const DEPTH_COMPONENT16: c_uint = 0x81A5;
    pub const DEPTH_COMPONENT24: c_uint = 0x81A6;
    pub const DEPTH_COMPONENT32: c_uint = 0x81A7;
    pub const DEPTH_COMPONENT32F: c_uint = 0x8CAC;
    pub const DEPTH_COMPONENTS: c_uint = 0x8284;
    pub const DEPTH_FUNC: c_uint = 0x0B74;
    pub const DEPTH_RANGE: c_uint = 0x0B70;
    pub const DEPTH_RENDERABLE: c_uint = 0x8287;
    pub const DEPTH_STENCIL: c_uint = 0x84F9;
    pub const DEPTH_STENCIL_ATTACHMENT: c_uint = 0x821A;
    pub const DEPTH_STENCIL_TEXTURE_MODE: c_uint = 0x90EA;
    pub const DEPTH_TEST: c_uint = 0x0B71;
    pub const DEPTH_WRITEMASK: c_uint = 0x0B72;
    pub const DISPATCH_INDIRECT_BUFFER: c_uint = 0x90EE;
    pub const DISPATCH_INDIRECT_BUFFER_BINDING: c_uint = 0x90EF;
    pub const DITHER: c_uint = 0x0BD0;
    pub const DONT_CARE: c_uint = 0x1100;
    pub const DOUBLE: c_uint = 0x140A;
    pub const DOUBLEBUFFER: c_uint = 0x0C32;
    pub const DOUBLE_MAT2: c_uint = 0x8F46;
    pub const DOUBLE_MAT2x3: c_uint = 0x8F49;
    pub const DOUBLE_MAT2x4: c_uint = 0x8F4A;
    pub const DOUBLE_MAT3: c_uint = 0x8F47;
    pub const DOUBLE_MAT3x2: c_uint = 0x8F4B;
    pub const DOUBLE_MAT3x4: c_uint = 0x8F4C;
    pub const DOUBLE_MAT4: c_uint = 0x8F48;
    pub const DOUBLE_MAT4x2: c_uint = 0x8F4D;
    pub const DOUBLE_MAT4x3: c_uint = 0x8F4E;
    pub const DOUBLE_VEC2: c_uint = 0x8FFC;
    pub const DOUBLE_VEC3: c_uint = 0x8FFD;
    pub const DOUBLE_VEC4: c_uint = 0x8FFE;
    pub const DRAW_BUFFER: c_uint = 0x0C01;
    pub const DRAW_BUFFER0: c_uint = 0x8825;
    pub const DRAW_BUFFER1: c_uint = 0x8826;
    pub const DRAW_BUFFER10: c_uint = 0x882F;
    pub const DRAW_BUFFER11: c_uint = 0x8830;
    pub const DRAW_BUFFER12: c_uint = 0x8831;
    pub const DRAW_BUFFER13: c_uint = 0x8832;
    pub const DRAW_BUFFER14: c_uint = 0x8833;
    pub const DRAW_BUFFER15: c_uint = 0x8834;
    pub const DRAW_BUFFER2: c_uint = 0x8827;
    pub const DRAW_BUFFER3: c_uint = 0x8828;
    pub const DRAW_BUFFER4: c_uint = 0x8829;
    pub const DRAW_BUFFER5: c_uint = 0x882A;
    pub const DRAW_BUFFER6: c_uint = 0x882B;
    pub const DRAW_BUFFER7: c_uint = 0x882C;
    pub const DRAW_BUFFER8: c_uint = 0x882D;
    pub const DRAW_BUFFER9: c_uint = 0x882E;
    pub const DRAW_FRAMEBUFFER: c_uint = 0x8CA9;
    pub const DRAW_FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const DRAW_INDIRECT_BUFFER: c_uint = 0x8F3F;
    pub const DRAW_INDIRECT_BUFFER_BINDING: c_uint = 0x8F43;
    pub const DST_ALPHA: c_uint = 0x0304;
    pub const DST_COLOR: c_uint = 0x0306;
    pub const DYNAMIC_COPY: c_uint = 0x88EA;
    pub const DYNAMIC_DRAW: c_uint = 0x88E8;
    pub const DYNAMIC_READ: c_uint = 0x88E9;
    pub const DYNAMIC_STORAGE_BIT: c_uint = 0x0100;
    pub const ELEMENT_ARRAY_BARRIER_BIT: c_uint = 0x00000002;
    pub const ELEMENT_ARRAY_BUFFER: c_uint = 0x8893;
    pub const ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895;
    pub const EQUAL: c_uint = 0x0202;
    pub const EQUIV: c_uint = 0x1509;
    pub const EXTENSIONS: c_uint = 0x1F03;
    pub const FALSE: c_uchar = 0;
    pub const FASTEST: c_uint = 0x1101;
    pub const FILL: c_uint = 0x1B02;
    pub const FILTER: c_uint = 0x829A;
    pub const FIRST_VERTEX_CONVENTION: c_uint = 0x8E4D;
    pub const FIXED: c_uint = 0x140C;
    pub const FIXED_ONLY: c_uint = 0x891D;
    pub const FLOAT: c_uint = 0x1406;
    pub const FLOAT_32_UNSIGNED_INT_24_8_REV: c_uint = 0x8DAD;
    pub const FLOAT_MAT2: c_uint = 0x8B5A;
    pub const FLOAT_MAT2x3: c_uint = 0x8B65;
    pub const FLOAT_MAT2x4: c_uint = 0x8B66;
    pub const FLOAT_MAT3: c_uint = 0x8B5B;
    pub const FLOAT_MAT3x2: c_uint = 0x8B67;
    pub const FLOAT_MAT3x4: c_uint = 0x8B68;
    pub const FLOAT_MAT4: c_uint = 0x8B5C;
    pub const FLOAT_MAT4x2: c_uint = 0x8B69;
    pub const FLOAT_MAT4x3: c_uint = 0x8B6A;
    pub const FLOAT_VEC2: c_uint = 0x8B50;
    pub const FLOAT_VEC3: c_uint = 0x8B51;
    pub const FLOAT_VEC4: c_uint = 0x8B52;
    pub const FRACTIONAL_EVEN: c_uint = 0x8E7C;
    pub const FRACTIONAL_ODD: c_uint = 0x8E7B;
    pub const FRAGMENT_INTERPOLATION_OFFSET_BITS: c_uint = 0x8E5D;
    pub const FRAGMENT_SHADER: c_uint = 0x8B30;
    pub const FRAGMENT_SHADER_BIT: c_uint = 0x00000002;
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT: c_uint = 0x8B8B;
    pub const FRAGMENT_SHADER_INVOCATIONS: c_uint = 0x82F4;
    pub const FRAGMENT_SUBROUTINE: c_uint = 0x92EC;
    pub const FRAGMENT_SUBROUTINE_UNIFORM: c_uint = 0x92F2;
    pub const FRAGMENT_TEXTURE: c_uint = 0x829F;
    pub const FRAMEBUFFER: c_uint = 0x8D40;
    pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: c_uint = 0x8215;
    pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: c_uint = 0x8214;
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: c_uint = 0x8210;
    pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: c_uint = 0x8211;
    pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: c_uint = 0x8216;
    pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: c_uint = 0x8213;
    pub const FRAMEBUFFER_ATTACHMENT_LAYERED: c_uint = 0x8DA7;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: c_uint = 0x8CD1;
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: c_uint = 0x8CD0;
    pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: c_uint = 0x8212;
    pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: c_uint = 0x8217;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: c_uint = 0x8CD3;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: c_uint = 0x8CD4;
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: c_uint = 0x8CD2;
    pub const FRAMEBUFFER_BARRIER_BIT: c_uint = 0x00000400;
    pub const FRAMEBUFFER_BINDING: c_uint = 0x8CA6;
    pub const FRAMEBUFFER_BLEND: c_uint = 0x828B;
    pub const FRAMEBUFFER_COMPLETE: c_uint = 0x8CD5;
    pub const FRAMEBUFFER_DEFAULT: c_uint = 0x8218;
    pub const FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9314;
    pub const FRAMEBUFFER_DEFAULT_HEIGHT: c_uint = 0x9311;
    pub const FRAMEBUFFER_DEFAULT_LAYERS: c_uint = 0x9312;
    pub const FRAMEBUFFER_DEFAULT_SAMPLES: c_uint = 0x9313;
    pub const FRAMEBUFFER_DEFAULT_WIDTH: c_uint = 0x9310;
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: c_uint = 0x8CD6;
    pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: c_uint = 0x8CDB;
    pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: c_uint = 0x8DA8;
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: c_uint = 0x8CD7;
    pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: c_uint = 0x8D56;
    pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: c_uint = 0x8CDC;
    pub const FRAMEBUFFER_RENDERABLE: c_uint = 0x8289;
    pub const FRAMEBUFFER_RENDERABLE_LAYERED: c_uint = 0x828A;
    pub const FRAMEBUFFER_SRGB: c_uint = 0x8DB9;
    pub const FRAMEBUFFER_UNDEFINED: c_uint = 0x8219;
    pub const FRAMEBUFFER_UNSUPPORTED: c_uint = 0x8CDD;
    pub const FRONT: c_uint = 0x0404;
    pub const FRONT_AND_BACK: c_uint = 0x0408;
    pub const FRONT_FACE: c_uint = 0x0B46;
    pub const FRONT_LEFT: c_uint = 0x0400;
    pub const FRONT_RIGHT: c_uint = 0x0401;
    pub const FULL_SUPPORT: c_uint = 0x82B7;
    pub const FUNC_ADD: c_uint = 0x8006;
    pub const FUNC_REVERSE_SUBTRACT: c_uint = 0x800B;
    pub const FUNC_SUBTRACT: c_uint = 0x800A;
    pub const GEOMETRY_INPUT_TYPE: c_uint = 0x8917;
    pub const GEOMETRY_OUTPUT_TYPE: c_uint = 0x8918;
    pub const GEOMETRY_SHADER: c_uint = 0x8DD9;
    pub const GEOMETRY_SHADER_BIT: c_uint = 0x00000004;
    pub const GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x887F;
    pub const GEOMETRY_SHADER_PRIMITIVES_EMITTED: c_uint = 0x82F3;
    pub const GEOMETRY_SUBROUTINE: c_uint = 0x92EB;
    pub const GEOMETRY_SUBROUTINE_UNIFORM: c_uint = 0x92F1;
    pub const GEOMETRY_TEXTURE: c_uint = 0x829E;
    pub const GEOMETRY_VERTICES_OUT: c_uint = 0x8916;
    pub const GEQUAL: c_uint = 0x0206;
    pub const GET_TEXTURE_IMAGE_FORMAT: c_uint = 0x8291;
    pub const GET_TEXTURE_IMAGE_TYPE: c_uint = 0x8292;
    pub const GREATER: c_uint = 0x0204;
    pub const GREEN: c_uint = 0x1904;
    pub const GREEN_INTEGER: c_uint = 0x8D95;
    pub const GUILTY_CONTEXT_RESET: c_uint = 0x8253;
    pub const HALF_FLOAT: c_uint = 0x140B;
    pub const HIGH_FLOAT: c_uint = 0x8DF2;
    pub const HIGH_INT: c_uint = 0x8DF5;
    pub const IMAGE_1D: c_uint = 0x904C;
    pub const IMAGE_1D_ARRAY: c_uint = 0x9052;
    pub const IMAGE_2D: c_uint = 0x904D;
    pub const IMAGE_2D_ARRAY: c_uint = 0x9053;
    pub const IMAGE_2D_MULTISAMPLE: c_uint = 0x9055;
    pub const IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9056;
    pub const IMAGE_2D_RECT: c_uint = 0x904F;
    pub const IMAGE_3D: c_uint = 0x904E;
    pub const IMAGE_BINDING_ACCESS: c_uint = 0x8F3E;
    pub const IMAGE_BINDING_FORMAT: c_uint = 0x906E;
    pub const IMAGE_BINDING_LAYER: c_uint = 0x8F3D;
    pub const IMAGE_BINDING_LAYERED: c_uint = 0x8F3C;
    pub const IMAGE_BINDING_LEVEL: c_uint = 0x8F3B;
    pub const IMAGE_BINDING_NAME: c_uint = 0x8F3A;
    pub const IMAGE_BUFFER: c_uint = 0x9051;
    pub const IMAGE_CLASS_10_10_10_2: c_uint = 0x82C3;
    pub const IMAGE_CLASS_11_11_10: c_uint = 0x82C2;
    pub const IMAGE_CLASS_1_X_16: c_uint = 0x82BE;
    pub const IMAGE_CLASS_1_X_32: c_uint = 0x82BB;
    pub const IMAGE_CLASS_1_X_8: c_uint = 0x82C1;
    pub const IMAGE_CLASS_2_X_16: c_uint = 0x82BD;
    pub const IMAGE_CLASS_2_X_32: c_uint = 0x82BA;
    pub const IMAGE_CLASS_2_X_8: c_uint = 0x82C0;
    pub const IMAGE_CLASS_4_X_16: c_uint = 0x82BC;
    pub const IMAGE_CLASS_4_X_32: c_uint = 0x82B9;
    pub const IMAGE_CLASS_4_X_8: c_uint = 0x82BF;
    pub const IMAGE_COMPATIBILITY_CLASS: c_uint = 0x82A8;
    pub const IMAGE_CUBE: c_uint = 0x9050;
    pub const IMAGE_CUBE_MAP_ARRAY: c_uint = 0x9054;
    pub const IMAGE_FORMAT_COMPATIBILITY_BY_CLASS: c_uint = 0x90C9;
    pub const IMAGE_FORMAT_COMPATIBILITY_BY_SIZE: c_uint = 0x90C8;
    pub const IMAGE_FORMAT_COMPATIBILITY_TYPE: c_uint = 0x90C7;
    pub const IMAGE_PIXEL_FORMAT: c_uint = 0x82A9;
    pub const IMAGE_PIXEL_TYPE: c_uint = 0x82AA;
    pub const IMAGE_TEXEL_SIZE: c_uint = 0x82A7;
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: c_uint = 0x8B9B;
    pub const IMPLEMENTATION_COLOR_READ_TYPE: c_uint = 0x8B9A;
    pub const INCR: c_uint = 0x1E02;
    pub const INCR_WRAP: c_uint = 0x8507;
    pub const INFO_LOG_LENGTH: c_uint = 0x8B84;
    pub const INNOCENT_CONTEXT_RESET: c_uint = 0x8254;
    pub const INT: c_uint = 0x1404;
    pub const INTERLEAVED_ATTRIBS: c_uint = 0x8C8C;
    pub const INTERNALFORMAT_ALPHA_SIZE: c_uint = 0x8274;
    pub const INTERNALFORMAT_ALPHA_TYPE: c_uint = 0x827B;
    pub const INTERNALFORMAT_BLUE_SIZE: c_uint = 0x8273;
    pub const INTERNALFORMAT_BLUE_TYPE: c_uint = 0x827A;
    pub const INTERNALFORMAT_DEPTH_SIZE: c_uint = 0x8275;
    pub const INTERNALFORMAT_DEPTH_TYPE: c_uint = 0x827C;
    pub const INTERNALFORMAT_GREEN_SIZE: c_uint = 0x8272;
    pub const INTERNALFORMAT_GREEN_TYPE: c_uint = 0x8279;
    pub const INTERNALFORMAT_PREFERRED: c_uint = 0x8270;
    pub const INTERNALFORMAT_RED_SIZE: c_uint = 0x8271;
    pub const INTERNALFORMAT_RED_TYPE: c_uint = 0x8278;
    pub const INTERNALFORMAT_SHARED_SIZE: c_uint = 0x8277;
    pub const INTERNALFORMAT_STENCIL_SIZE: c_uint = 0x8276;
    pub const INTERNALFORMAT_STENCIL_TYPE: c_uint = 0x827D;
    pub const INTERNALFORMAT_SUPPORTED: c_uint = 0x826F;
    pub const INT_2_10_10_10_REV: c_uint = 0x8D9F;
    pub const INT_IMAGE_1D: c_uint = 0x9057;
    pub const INT_IMAGE_1D_ARRAY: c_uint = 0x905D;
    pub const INT_IMAGE_2D: c_uint = 0x9058;
    pub const INT_IMAGE_2D_ARRAY: c_uint = 0x905E;
    pub const INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x9060;
    pub const INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9061;
    pub const INT_IMAGE_2D_RECT: c_uint = 0x905A;
    pub const INT_IMAGE_3D: c_uint = 0x9059;
    pub const INT_IMAGE_BUFFER: c_uint = 0x905C;
    pub const INT_IMAGE_CUBE: c_uint = 0x905B;
    pub const INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x905F;
    pub const INT_SAMPLER_1D: c_uint = 0x8DC9;
    pub const INT_SAMPLER_1D_ARRAY: c_uint = 0x8DCE;
    pub const INT_SAMPLER_2D: c_uint = 0x8DCA;
    pub const INT_SAMPLER_2D_ARRAY: c_uint = 0x8DCF;
    pub const INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x9109;
    pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910C;
    pub const INT_SAMPLER_2D_RECT: c_uint = 0x8DCD;
    pub const INT_SAMPLER_3D: c_uint = 0x8DCB;
    pub const INT_SAMPLER_BUFFER: c_uint = 0x8DD0;
    pub const INT_SAMPLER_CUBE: c_uint = 0x8DCC;
    pub const INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900E;
    pub const INT_VEC2: c_uint = 0x8B53;
    pub const INT_VEC3: c_uint = 0x8B54;
    pub const INT_VEC4: c_uint = 0x8B55;
    pub const INVALID_ENUM: c_uint = 0x0500;
    pub const INVALID_FRAMEBUFFER_OPERATION: c_uint = 0x0506;
    pub const INVALID_INDEX: c_uint = 0xFFFFFFFF;
    pub const INVALID_OPERATION: c_uint = 0x0502;
    pub const INVALID_VALUE: c_uint = 0x0501;
    pub const INVERT: c_uint = 0x150A;
    pub const ISOLINES: c_uint = 0x8E7A;
    pub const IS_PER_PATCH: c_uint = 0x92E7;
    pub const IS_ROW_MAJOR: c_uint = 0x9300;
    pub const KEEP: c_uint = 0x1E00;
    pub const LAST_VERTEX_CONVENTION: c_uint = 0x8E4E;
    pub const LAYER_PROVOKING_VERTEX: c_uint = 0x825E;
    pub const LEFT: c_uint = 0x0406;
    pub const LEQUAL: c_uint = 0x0203;
    pub const LESS: c_uint = 0x0201;
    pub const LINE: c_uint = 0x1B01;
    pub const LINEAR: c_uint = 0x2601;
    pub const LINEAR_MIPMAP_LINEAR: c_uint = 0x2703;
    pub const LINEAR_MIPMAP_NEAREST: c_uint = 0x2701;
    pub const LINES: c_uint = 0x0001;
    pub const LINES_ADJACENCY: c_uint = 0x000A;
    pub const LINE_LOOP: c_uint = 0x0002;
    pub const LINE_SMOOTH: c_uint = 0x0B20;
    pub const LINE_SMOOTH_HINT: c_uint = 0x0C52;
    pub const LINE_STRIP: c_uint = 0x0003;
    pub const LINE_STRIP_ADJACENCY: c_uint = 0x000B;
    pub const LINE_WIDTH: c_uint = 0x0B21;
    pub const LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const LINK_STATUS: c_uint = 0x8B82;
    pub const LOCATION: c_uint = 0x930E;
    pub const LOCATION_COMPONENT: c_uint = 0x934A;
    pub const LOCATION_INDEX: c_uint = 0x930F;
    pub const LOGIC_OP_MODE: c_uint = 0x0BF0;
    pub const LOSE_CONTEXT_ON_RESET: c_uint = 0x8252;
    pub const LOWER_LEFT: c_uint = 0x8CA1;
    pub const LOW_FLOAT: c_uint = 0x8DF0;
    pub const LOW_INT: c_uint = 0x8DF3;
    pub const MAJOR_VERSION: c_uint = 0x821B;
    pub const MANUAL_GENERATE_MIPMAP: c_uint = 0x8294;
    pub const MAP_COHERENT_BIT: c_uint = 0x0080;
    pub const MAP_FLUSH_EXPLICIT_BIT: c_uint = 0x0010;
    pub const MAP_INVALIDATE_BUFFER_BIT: c_uint = 0x0008;
    pub const MAP_INVALIDATE_RANGE_BIT: c_uint = 0x0004;
    pub const MAP_PERSISTENT_BIT: c_uint = 0x0040;
    pub const MAP_READ_BIT: c_uint = 0x0001;
    pub const MAP_UNSYNCHRONIZED_BIT: c_uint = 0x0020;
    pub const MAP_WRITE_BIT: c_uint = 0x0002;
    pub const MATRIX_STRIDE: c_uint = 0x92FF;
    pub const MAX: c_uint = 0x8008;
    pub const MAX_3D_TEXTURE_SIZE: c_uint = 0x8073;
    pub const MAX_ARRAY_TEXTURE_LAYERS: c_uint = 0x88FF;
    pub const MAX_ATOMIC_COUNTER_BUFFER_BINDINGS: c_uint = 0x92DC;
    pub const MAX_ATOMIC_COUNTER_BUFFER_SIZE: c_uint = 0x92D8;
    pub const MAX_CLIP_DISTANCES: c_uint = 0x0D32;
    pub const MAX_COLOR_ATTACHMENTS: c_uint = 0x8CDF;
    pub const MAX_COLOR_TEXTURE_SAMPLES: c_uint = 0x910E;
    pub const MAX_COMBINED_ATOMIC_COUNTERS: c_uint = 0x92D7;
    pub const MAX_COMBINED_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D1;
    pub const MAX_COMBINED_CLIP_AND_CULL_DISTANCES: c_uint = 0x82FA;
    pub const MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8266;
    pub const MAX_COMBINED_DIMENSIONS: c_uint = 0x8282;
    pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8A33;
    pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8A32;
    pub const MAX_COMBINED_IMAGE_UNIFORMS: c_uint = 0x90CF;
    pub const MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS: c_uint = 0x8F39;
    pub const MAX_COMBINED_SHADER_OUTPUT_RESOURCES: c_uint = 0x8F39;
    pub const MAX_COMBINED_SHADER_STORAGE_BLOCKS: c_uint = 0x90DC;
    pub const MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E1E;
    pub const MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E1F;
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D;
    pub const MAX_COMBINED_UNIFORM_BLOCKS: c_uint = 0x8A2E;
    pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8A31;
    pub const MAX_COMPUTE_ATOMIC_COUNTERS: c_uint = 0x8265;
    pub const MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS: c_uint = 0x8264;
    pub const MAX_COMPUTE_IMAGE_UNIFORMS: c_uint = 0x91BD;
    pub const MAX_COMPUTE_SHADER_STORAGE_BLOCKS: c_uint = 0x90DB;
    pub const MAX_COMPUTE_SHARED_MEMORY_SIZE: c_uint = 0x8262;
    pub const MAX_COMPUTE_TEXTURE_IMAGE_UNITS: c_uint = 0x91BC;
    pub const MAX_COMPUTE_UNIFORM_BLOCKS: c_uint = 0x91BB;
    pub const MAX_COMPUTE_UNIFORM_COMPONENTS: c_uint = 0x8263;
    pub const MAX_COMPUTE_WORK_GROUP_COUNT: c_uint = 0x91BE;
    pub const MAX_COMPUTE_WORK_GROUP_INVOCATIONS: c_uint = 0x90EB;
    pub const MAX_COMPUTE_WORK_GROUP_SIZE: c_uint = 0x91BF;
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: c_uint = 0x851C;
    pub const MAX_CULL_DISTANCES: c_uint = 0x82F9;
    pub const MAX_DEBUG_GROUP_STACK_DEPTH: c_uint = 0x826C;
    pub const MAX_DEBUG_LOGGED_MESSAGES: c_uint = 0x9144;
    pub const MAX_DEBUG_MESSAGE_LENGTH: c_uint = 0x9143;
    pub const MAX_DEPTH: c_uint = 0x8280;
    pub const MAX_DEPTH_TEXTURE_SAMPLES: c_uint = 0x910F;
    pub const MAX_DRAW_BUFFERS: c_uint = 0x8824;
    pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: c_uint = 0x88FC;
    pub const MAX_ELEMENTS_INDICES: c_uint = 0x80E9;
    pub const MAX_ELEMENTS_VERTICES: c_uint = 0x80E8;
    pub const MAX_ELEMENT_INDEX: c_uint = 0x8D6B;
    pub const MAX_FRAGMENT_ATOMIC_COUNTERS: c_uint = 0x92D6;
    pub const MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92D0;
    pub const MAX_FRAGMENT_IMAGE_UNIFORMS: c_uint = 0x90CE;
    pub const MAX_FRAGMENT_INPUT_COMPONENTS: c_uint = 0x9125;
    pub const MAX_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5C;
    pub const MAX_FRAGMENT_SHADER_STORAGE_BLOCKS: c_uint = 0x90DA;
    pub const MAX_FRAGMENT_UNIFORM_BLOCKS: c_uint = 0x8A2D;
    pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: c_uint = 0x8B49;
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: c_uint = 0x8DFD;
    pub const MAX_FRAMEBUFFER_HEIGHT: c_uint = 0x9316;
    pub const MAX_FRAMEBUFFER_LAYERS: c_uint = 0x9317;
    pub const MAX_FRAMEBUFFER_SAMPLES: c_uint = 0x9318;
    pub const MAX_FRAMEBUFFER_WIDTH: c_uint = 0x9315;
    pub const MAX_GEOMETRY_ATOMIC_COUNTERS: c_uint = 0x92D5;
    pub const MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CF;
    pub const MAX_GEOMETRY_IMAGE_UNIFORMS: c_uint = 0x90CD;
    pub const MAX_GEOMETRY_INPUT_COMPONENTS: c_uint = 0x9123;
    pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: c_uint = 0x9124;
    pub const MAX_GEOMETRY_OUTPUT_VERTICES: c_uint = 0x8DE0;
    pub const MAX_GEOMETRY_SHADER_INVOCATIONS: c_uint = 0x8E5A;
    pub const MAX_GEOMETRY_SHADER_STORAGE_BLOCKS: c_uint = 0x90D7;
    pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: c_uint = 0x8C29;
    pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8DE1;
    pub const MAX_GEOMETRY_UNIFORM_BLOCKS: c_uint = 0x8A2C;
    pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: c_uint = 0x8DDF;
    pub const MAX_HEIGHT: c_uint = 0x827F;
    pub const MAX_IMAGE_SAMPLES: c_uint = 0x906D;
    pub const MAX_IMAGE_UNITS: c_uint = 0x8F38;
    pub const MAX_INTEGER_SAMPLES: c_uint = 0x9110;
    pub const MAX_LABEL_LENGTH: c_uint = 0x82E8;
    pub const MAX_LAYERS: c_uint = 0x8281;
    pub const MAX_NAME_LENGTH: c_uint = 0x92F6;
    pub const MAX_NUM_ACTIVE_VARIABLES: c_uint = 0x92F7;
    pub const MAX_NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x92F8;
    pub const MAX_PATCH_VERTICES: c_uint = 0x8E7D;
    pub const MAX_PROGRAM_TEXEL_OFFSET: c_uint = 0x8905;
    pub const MAX_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5F;
    pub const MAX_RECTANGLE_TEXTURE_SIZE: c_uint = 0x84F8;
    pub const MAX_RENDERBUFFER_SIZE: c_uint = 0x84E8;
    pub const MAX_SAMPLES: c_uint = 0x8D57;
    pub const MAX_SAMPLE_MASK_WORDS: c_uint = 0x8E59;
    pub const MAX_SERVER_WAIT_TIMEOUT: c_uint = 0x9111;
    pub const MAX_SHADER_STORAGE_BLOCK_SIZE: c_uint = 0x90DE;
    pub const MAX_SHADER_STORAGE_BUFFER_BINDINGS: c_uint = 0x90DD;
    pub const MAX_SUBROUTINES: c_uint = 0x8DE7;
    pub const MAX_SUBROUTINE_UNIFORM_LOCATIONS: c_uint = 0x8DE8;
    pub const MAX_TESS_CONTROL_ATOMIC_COUNTERS: c_uint = 0x92D3;
    pub const MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CD;
    pub const MAX_TESS_CONTROL_IMAGE_UNIFORMS: c_uint = 0x90CB;
    pub const MAX_TESS_CONTROL_INPUT_COMPONENTS: c_uint = 0x886C;
    pub const MAX_TESS_CONTROL_OUTPUT_COMPONENTS: c_uint = 0x8E83;
    pub const MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS: c_uint = 0x90D8;
    pub const MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS: c_uint = 0x8E81;
    pub const MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS: c_uint = 0x8E85;
    pub const MAX_TESS_CONTROL_UNIFORM_BLOCKS: c_uint = 0x8E89;
    pub const MAX_TESS_CONTROL_UNIFORM_COMPONENTS: c_uint = 0x8E7F;
    pub const MAX_TESS_EVALUATION_ATOMIC_COUNTERS: c_uint = 0x92D4;
    pub const MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CE;
    pub const MAX_TESS_EVALUATION_IMAGE_UNIFORMS: c_uint = 0x90CC;
    pub const MAX_TESS_EVALUATION_INPUT_COMPONENTS: c_uint = 0x886D;
    pub const MAX_TESS_EVALUATION_OUTPUT_COMPONENTS: c_uint = 0x8E86;
    pub const MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS: c_uint = 0x90D9;
    pub const MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS: c_uint = 0x8E82;
    pub const MAX_TESS_EVALUATION_UNIFORM_BLOCKS: c_uint = 0x8E8A;
    pub const MAX_TESS_EVALUATION_UNIFORM_COMPONENTS: c_uint = 0x8E80;
    pub const MAX_TESS_GEN_LEVEL: c_uint = 0x8E7E;
    pub const MAX_TESS_PATCH_COMPONENTS: c_uint = 0x8E84;
    pub const MAX_TEXTURE_BUFFER_SIZE: c_uint = 0x8C2B;
    pub const MAX_TEXTURE_IMAGE_UNITS: c_uint = 0x8872;
    pub const MAX_TEXTURE_LOD_BIAS: c_uint = 0x84FD;
    pub const MAX_TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FF;
    pub const MAX_TEXTURE_SIZE: c_uint = 0x0D33;
    pub const MAX_TRANSFORM_FEEDBACK_BUFFERS: c_uint = 0x8E70;
    pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: c_uint = 0x8C8A;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: c_uint = 0x8C8B;
    pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: c_uint = 0x8C80;
    pub const MAX_UNIFORM_BLOCK_SIZE: c_uint = 0x8A30;
    pub const MAX_UNIFORM_BUFFER_BINDINGS: c_uint = 0x8A2F;
    pub const MAX_UNIFORM_LOCATIONS: c_uint = 0x826E;
    pub const MAX_VARYING_COMPONENTS: c_uint = 0x8B4B;
    pub const MAX_VARYING_FLOATS: c_uint = 0x8B4B;
    pub const MAX_VARYING_VECTORS: c_uint = 0x8DFC;
    pub const MAX_VERTEX_ATOMIC_COUNTERS: c_uint = 0x92D2;
    pub const MAX_VERTEX_ATOMIC_COUNTER_BUFFERS: c_uint = 0x92CC;
    pub const MAX_VERTEX_ATTRIBS: c_uint = 0x8869;
    pub const MAX_VERTEX_ATTRIB_BINDINGS: c_uint = 0x82DA;
    pub const MAX_VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D9;
    pub const MAX_VERTEX_ATTRIB_STRIDE: c_uint = 0x82E5;
    pub const MAX_VERTEX_IMAGE_UNIFORMS: c_uint = 0x90CA;
    pub const MAX_VERTEX_OUTPUT_COMPONENTS: c_uint = 0x9122;
    pub const MAX_VERTEX_SHADER_STORAGE_BLOCKS: c_uint = 0x90D6;
    pub const MAX_VERTEX_STREAMS: c_uint = 0x8E71;
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4C;
    pub const MAX_VERTEX_UNIFORM_BLOCKS: c_uint = 0x8A2B;
    pub const MAX_VERTEX_UNIFORM_COMPONENTS: c_uint = 0x8B4A;
    pub const MAX_VERTEX_UNIFORM_VECTORS: c_uint = 0x8DFB;
    pub const MAX_VIEWPORTS: c_uint = 0x825B;
    pub const MAX_VIEWPORT_DIMS: c_uint = 0x0D3A;
    pub const MAX_WIDTH: c_uint = 0x827E;
    pub const MEDIUM_FLOAT: c_uint = 0x8DF1;
    pub const MEDIUM_INT: c_uint = 0x8DF4;
    pub const MIN: c_uint = 0x8007;
    pub const MINOR_VERSION: c_uint = 0x821C;
    pub const MIN_FRAGMENT_INTERPOLATION_OFFSET: c_uint = 0x8E5B;
    pub const MIN_MAP_BUFFER_ALIGNMENT: c_uint = 0x90BC;
    pub const MIN_PROGRAM_TEXEL_OFFSET: c_uint = 0x8904;
    pub const MIN_PROGRAM_TEXTURE_GATHER_OFFSET: c_uint = 0x8E5E;
    pub const MIN_SAMPLE_SHADING_VALUE: c_uint = 0x8C37;
    pub const MIPMAP: c_uint = 0x8293;
    pub const MIRRORED_REPEAT: c_uint = 0x8370;
    pub const MIRROR_CLAMP_TO_EDGE: c_uint = 0x8743;
    pub const MULTISAMPLE: c_uint = 0x809D;
    pub const NAME_LENGTH: c_uint = 0x92F9;
    pub const NAND: c_uint = 0x150E;
    pub const NEAREST: c_uint = 0x2600;
    pub const NEAREST_MIPMAP_LINEAR: c_uint = 0x2702;
    pub const NEAREST_MIPMAP_NEAREST: c_uint = 0x2700;
    pub const NEGATIVE_ONE_TO_ONE: c_uint = 0x935E;
    pub const NEVER: c_uint = 0x0200;
    pub const NICEST: c_uint = 0x1102;
    pub const NONE: c_uint = 0;
    pub const NOOP: c_uint = 0x1505;
    pub const NOR: c_uint = 0x1508;
    pub const NOTEQUAL: c_uint = 0x0205;
    pub const NO_ERROR: c_uint = 0;
    pub const NO_RESET_NOTIFICATION: c_uint = 0x8261;
    pub const NUM_ACTIVE_VARIABLES: c_uint = 0x9304;
    pub const NUM_COMPATIBLE_SUBROUTINES: c_uint = 0x8E4A;
    pub const NUM_COMPRESSED_TEXTURE_FORMATS: c_uint = 0x86A2;
    pub const NUM_EXTENSIONS: c_uint = 0x821D;
    pub const NUM_PROGRAM_BINARY_FORMATS: c_uint = 0x87FE;
    pub const NUM_SAMPLE_COUNTS: c_uint = 0x9380;
    pub const NUM_SHADER_BINARY_FORMATS: c_uint = 0x8DF9;
    pub const NUM_SHADING_LANGUAGE_VERSIONS: c_uint = 0x82E9;
    pub const NUM_SPIR_V_EXTENSIONS: c_uint = 0x9554;
    pub const OBJECT_TYPE: c_uint = 0x9112;
    pub const OFFSET: c_uint = 0x92FC;
    pub const ONE: c_uint = 1;
    pub const ONE_MINUS_CONSTANT_ALPHA: c_uint = 0x8004;
    pub const ONE_MINUS_CONSTANT_COLOR: c_uint = 0x8002;
    pub const ONE_MINUS_DST_ALPHA: c_uint = 0x0305;
    pub const ONE_MINUS_DST_COLOR: c_uint = 0x0307;
    pub const ONE_MINUS_SRC1_ALPHA: c_uint = 0x88FB;
    pub const ONE_MINUS_SRC1_COLOR: c_uint = 0x88FA;
    pub const ONE_MINUS_SRC_ALPHA: c_uint = 0x0303;
    pub const ONE_MINUS_SRC_COLOR: c_uint = 0x0301;
    pub const OR: c_uint = 0x1507;
    pub const OR_INVERTED: c_uint = 0x150D;
    pub const OR_REVERSE: c_uint = 0x150B;
    pub const OUT_OF_MEMORY: c_uint = 0x0505;
    pub const PACK_ALIGNMENT: c_uint = 0x0D05;
    pub const PACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x912D;
    pub const PACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x912C;
    pub const PACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912E;
    pub const PACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x912B;
    pub const PACK_IMAGE_HEIGHT: c_uint = 0x806C;
    pub const PACK_LSB_FIRST: c_uint = 0x0D01;
    pub const PACK_ROW_LENGTH: c_uint = 0x0D02;
    pub const PACK_SKIP_IMAGES: c_uint = 0x806B;
    pub const PACK_SKIP_PIXELS: c_uint = 0x0D04;
    pub const PACK_SKIP_ROWS: c_uint = 0x0D03;
    pub const PACK_SWAP_BYTES: c_uint = 0x0D00;
    pub const PARAMETER_BUFFER: c_uint = 0x80EE;
    pub const PARAMETER_BUFFER_BINDING: c_uint = 0x80EF;
    pub const PATCHES: c_uint = 0x000E;
    pub const PATCH_DEFAULT_INNER_LEVEL: c_uint = 0x8E73;
    pub const PATCH_DEFAULT_OUTER_LEVEL: c_uint = 0x8E74;
    pub const PATCH_VERTICES: c_uint = 0x8E72;
    pub const PIXEL_BUFFER_BARRIER_BIT: c_uint = 0x00000080;
    pub const PIXEL_PACK_BUFFER: c_uint = 0x88EB;
    pub const PIXEL_PACK_BUFFER_BINDING: c_uint = 0x88ED;
    pub const PIXEL_UNPACK_BUFFER: c_uint = 0x88EC;
    pub const PIXEL_UNPACK_BUFFER_BINDING: c_uint = 0x88EF;
    pub const POINT: c_uint = 0x1B00;
    pub const POINTS: c_uint = 0x0000;
    pub const POINT_FADE_THRESHOLD_SIZE: c_uint = 0x8128;
    pub const POINT_SIZE: c_uint = 0x0B11;
    pub const POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const POINT_SPRITE_COORD_ORIGIN: c_uint = 0x8CA0;
    pub const POLYGON_MODE: c_uint = 0x0B40;
    pub const POLYGON_OFFSET_CLAMP: c_uint = 0x8E1B;
    pub const POLYGON_OFFSET_FACTOR: c_uint = 0x8038;
    pub const POLYGON_OFFSET_FILL: c_uint = 0x8037;
    pub const POLYGON_OFFSET_LINE: c_uint = 0x2A02;
    pub const POLYGON_OFFSET_POINT: c_uint = 0x2A01;
    pub const POLYGON_OFFSET_UNITS: c_uint = 0x2A00;
    pub const POLYGON_SMOOTH: c_uint = 0x0B41;
    pub const POLYGON_SMOOTH_HINT: c_uint = 0x0C53;
    pub const PRIMITIVES_GENERATED: c_uint = 0x8C87;
    pub const PRIMITIVES_SUBMITTED: c_uint = 0x82EF;
    pub const PRIMITIVE_RESTART: c_uint = 0x8F9D;
    pub const PRIMITIVE_RESTART_FIXED_INDEX: c_uint = 0x8D69;
    pub const PRIMITIVE_RESTART_FOR_PATCHES_SUPPORTED: c_uint = 0x8221;
    pub const PRIMITIVE_RESTART_INDEX: c_uint = 0x8F9E;
    pub const PROGRAM: c_uint = 0x82E2;
    pub const PROGRAM_BINARY_FORMATS: c_uint = 0x87FF;
    pub const PROGRAM_BINARY_LENGTH: c_uint = 0x8741;
    pub const PROGRAM_BINARY_RETRIEVABLE_HINT: c_uint = 0x8257;
    pub const PROGRAM_INPUT: c_uint = 0x92E3;
    pub const PROGRAM_OUTPUT: c_uint = 0x92E4;
    pub const PROGRAM_PIPELINE: c_uint = 0x82E4;
    pub const PROGRAM_PIPELINE_BINDING: c_uint = 0x825A;
    pub const PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const PROGRAM_SEPARABLE: c_uint = 0x8258;
    pub const PROVOKING_VERTEX: c_uint = 0x8E4F;
    pub const PROXY_TEXTURE_1D: c_uint = 0x8063;
    pub const PROXY_TEXTURE_1D_ARRAY: c_uint = 0x8C19;
    pub const PROXY_TEXTURE_2D: c_uint = 0x8064;
    pub const PROXY_TEXTURE_2D_ARRAY: c_uint = 0x8C1B;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE: c_uint = 0x9101;
    pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9103;
    pub const PROXY_TEXTURE_3D: c_uint = 0x8070;
    pub const PROXY_TEXTURE_CUBE_MAP: c_uint = 0x851B;
    pub const PROXY_TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x900B;
    pub const PROXY_TEXTURE_RECTANGLE: c_uint = 0x84F7;
    pub const QUADS: c_uint = 0x0007;
    pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: c_uint = 0x8E4C;
    pub const QUERY: c_uint = 0x82E3;
    pub const QUERY_BUFFER: c_uint = 0x9192;
    pub const QUERY_BUFFER_BARRIER_BIT: c_uint = 0x00008000;
    pub const QUERY_BUFFER_BINDING: c_uint = 0x9193;
    pub const QUERY_BY_REGION_NO_WAIT: c_uint = 0x8E16;
    pub const QUERY_BY_REGION_NO_WAIT_INVERTED: c_uint = 0x8E1A;
    pub const QUERY_BY_REGION_WAIT: c_uint = 0x8E15;
    pub const QUERY_BY_REGION_WAIT_INVERTED: c_uint = 0x8E19;
    pub const QUERY_COUNTER_BITS: c_uint = 0x8864;
    pub const QUERY_NO_WAIT: c_uint = 0x8E14;
    pub const QUERY_NO_WAIT_INVERTED: c_uint = 0x8E18;
    pub const QUERY_RESULT: c_uint = 0x8866;
    pub const QUERY_RESULT_AVAILABLE: c_uint = 0x8867;
    pub const QUERY_RESULT_NO_WAIT: c_uint = 0x9194;
    pub const QUERY_TARGET: c_uint = 0x82EA;
    pub const QUERY_WAIT: c_uint = 0x8E13;
    pub const QUERY_WAIT_INVERTED: c_uint = 0x8E17;
    pub const R11F_G11F_B10F: c_uint = 0x8C3A;
    pub const R16: c_uint = 0x822A;
    pub const R16F: c_uint = 0x822D;
    pub const R16I: c_uint = 0x8233;
    pub const R16UI: c_uint = 0x8234;
    pub const R16_SNORM: c_uint = 0x8F98;
    pub const R32F: c_uint = 0x822E;
    pub const R32I: c_uint = 0x8235;
    pub const R32UI: c_uint = 0x8236;
    pub const R3_G3_B2: c_uint = 0x2A10;
    pub const R8: c_uint = 0x8229;
    pub const R8I: c_uint = 0x8231;
    pub const R8UI: c_uint = 0x8232;
    pub const R8_SNORM: c_uint = 0x8F94;
    pub const RASTERIZER_DISCARD: c_uint = 0x8C89;
    pub const READ_BUFFER: c_uint = 0x0C02;
    pub const READ_FRAMEBUFFER: c_uint = 0x8CA8;
    pub const READ_FRAMEBUFFER_BINDING: c_uint = 0x8CAA;
    pub const READ_ONLY: c_uint = 0x88B8;
    pub const READ_PIXELS: c_uint = 0x828C;
    pub const READ_PIXELS_FORMAT: c_uint = 0x828D;
    pub const READ_PIXELS_TYPE: c_uint = 0x828E;
    pub const READ_WRITE: c_uint = 0x88BA;
    pub const RED: c_uint = 0x1903;
    pub const RED_INTEGER: c_uint = 0x8D94;
    pub const REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x930B;
    pub const REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x930A;
    pub const REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x9309;
    pub const REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x9307;
    pub const REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x9308;
    pub const REFERENCED_BY_VERTEX_SHADER: c_uint = 0x9306;
    pub const RENDERBUFFER: c_uint = 0x8D41;
    pub const RENDERBUFFER_ALPHA_SIZE: c_uint = 0x8D53;
    pub const RENDERBUFFER_BINDING: c_uint = 0x8CA7;
    pub const RENDERBUFFER_BLUE_SIZE: c_uint = 0x8D52;
    pub const RENDERBUFFER_DEPTH_SIZE: c_uint = 0x8D54;
    pub const RENDERBUFFER_GREEN_SIZE: c_uint = 0x8D51;
    pub const RENDERBUFFER_HEIGHT: c_uint = 0x8D43;
    pub const RENDERBUFFER_INTERNAL_FORMAT: c_uint = 0x8D44;
    pub const RENDERBUFFER_RED_SIZE: c_uint = 0x8D50;
    pub const RENDERBUFFER_SAMPLES: c_uint = 0x8CAB;
    pub const RENDERBUFFER_STENCIL_SIZE: c_uint = 0x8D55;
    pub const RENDERBUFFER_WIDTH: c_uint = 0x8D42;
    pub const RENDERER: c_uint = 0x1F01;
    pub const REPEAT: c_uint = 0x2901;
    pub const REPLACE: c_uint = 0x1E01;
    pub const RESET_NOTIFICATION_STRATEGY: c_uint = 0x8256;
    pub const RG: c_uint = 0x8227;
    pub const RG16: c_uint = 0x822C;
    pub const RG16F: c_uint = 0x822F;
    pub const RG16I: c_uint = 0x8239;
    pub const RG16UI: c_uint = 0x823A;
    pub const RG16_SNORM: c_uint = 0x8F99;
    pub const RG32F: c_uint = 0x8230;
    pub const RG32I: c_uint = 0x823B;
    pub const RG32UI: c_uint = 0x823C;
    pub const RG8: c_uint = 0x822B;
    pub const RG8I: c_uint = 0x8237;
    pub const RG8UI: c_uint = 0x8238;
    pub const RG8_SNORM: c_uint = 0x8F95;
    pub const RGB: c_uint = 0x1907;
    pub const RGB10: c_uint = 0x8052;
    pub const RGB10_A2: c_uint = 0x8059;
    pub const RGB10_A2UI: c_uint = 0x906F;
    pub const RGB12: c_uint = 0x8053;
    pub const RGB16: c_uint = 0x8054;
    pub const RGB16F: c_uint = 0x881B;
    pub const RGB16I: c_uint = 0x8D89;
    pub const RGB16UI: c_uint = 0x8D77;
    pub const RGB16_SNORM: c_uint = 0x8F9A;
    pub const RGB32F: c_uint = 0x8815;
    pub const RGB32I: c_uint = 0x8D83;
    pub const RGB32UI: c_uint = 0x8D71;
    pub const RGB4: c_uint = 0x804F;
    pub const RGB5: c_uint = 0x8050;
    pub const RGB565: c_uint = 0x8D62;
    pub const RGB5_A1: c_uint = 0x8057;
    pub const RGB8: c_uint = 0x8051;
    pub const RGB8I: c_uint = 0x8D8F;
    pub const RGB8UI: c_uint = 0x8D7D;
    pub const RGB8_SNORM: c_uint = 0x8F96;
    pub const RGB9_E5: c_uint = 0x8C3D;
    pub const RGBA: c_uint = 0x1908;
    pub const RGBA12: c_uint = 0x805A;
    pub const RGBA16: c_uint = 0x805B;
    pub const RGBA16F: c_uint = 0x881A;
    pub const RGBA16I: c_uint = 0x8D88;
    pub const RGBA16UI: c_uint = 0x8D76;
    pub const RGBA16_SNORM: c_uint = 0x8F9B;
    pub const RGBA2: c_uint = 0x8055;
    pub const RGBA32F: c_uint = 0x8814;
    pub const RGBA32I: c_uint = 0x8D82;
    pub const RGBA32UI: c_uint = 0x8D70;
    pub const RGBA4: c_uint = 0x8056;
    pub const RGBA8: c_uint = 0x8058;
    pub const RGBA8I: c_uint = 0x8D8E;
    pub const RGBA8UI: c_uint = 0x8D7C;
    pub const RGBA8_SNORM: c_uint = 0x8F97;
    pub const RGBA_INTEGER: c_uint = 0x8D99;
    pub const RGB_INTEGER: c_uint = 0x8D98;
    pub const RG_INTEGER: c_uint = 0x8228;
    pub const RIGHT: c_uint = 0x0407;
    pub const SAMPLER: c_uint = 0x82E6;
    pub const SAMPLER_1D: c_uint = 0x8B5D;
    pub const SAMPLER_1D_ARRAY: c_uint = 0x8DC0;
    pub const SAMPLER_1D_ARRAY_SHADOW: c_uint = 0x8DC3;
    pub const SAMPLER_1D_SHADOW: c_uint = 0x8B61;
    pub const SAMPLER_2D: c_uint = 0x8B5E;
    pub const SAMPLER_2D_ARRAY: c_uint = 0x8DC1;
    pub const SAMPLER_2D_ARRAY_SHADOW: c_uint = 0x8DC4;
    pub const SAMPLER_2D_MULTISAMPLE: c_uint = 0x9108;
    pub const SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910B;
    pub const SAMPLER_2D_RECT: c_uint = 0x8B63;
    pub const SAMPLER_2D_RECT_SHADOW: c_uint = 0x8B64;
    pub const SAMPLER_2D_SHADOW: c_uint = 0x8B62;
    pub const SAMPLER_3D: c_uint = 0x8B5F;
    pub const SAMPLER_BINDING: c_uint = 0x8919;
    pub const SAMPLER_BUFFER: c_uint = 0x8DC2;
    pub const SAMPLER_CUBE: c_uint = 0x8B60;
    pub const SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900C;
    pub const SAMPLER_CUBE_MAP_ARRAY_SHADOW: c_uint = 0x900D;
    pub const SAMPLER_CUBE_SHADOW: c_uint = 0x8DC5;
    pub const SAMPLES: c_uint = 0x80A9;
    pub const SAMPLES_PASSED: c_uint = 0x8914;
    pub const SAMPLE_ALPHA_TO_COVERAGE: c_uint = 0x809E;
    pub const SAMPLE_ALPHA_TO_ONE: c_uint = 0x809F;
    pub const SAMPLE_BUFFERS: c_uint = 0x80A8;
    pub const SAMPLE_COVERAGE: c_uint = 0x80A0;
    pub const SAMPLE_COVERAGE_INVERT: c_uint = 0x80AB;
    pub const SAMPLE_COVERAGE_VALUE: c_uint = 0x80AA;
    pub const SAMPLE_MASK: c_uint = 0x8E51;
    pub const SAMPLE_MASK_VALUE: c_uint = 0x8E52;
    pub const SAMPLE_POSITION: c_uint = 0x8E50;
    pub const SAMPLE_SHADING: c_uint = 0x8C36;
    pub const SCISSOR_BOX: c_uint = 0x0C10;
    pub const SCISSOR_TEST: c_uint = 0x0C11;
    pub const SEPARATE_ATTRIBS: c_uint = 0x8C8D;
    pub const SET: c_uint = 0x150F;
    pub const SHADER: c_uint = 0x82E1;
    pub const SHADER_BINARY_FORMATS: c_uint = 0x8DF8;
    pub const SHADER_BINARY_FORMAT_SPIR_V: c_uint = 0x9551;
    pub const SHADER_COMPILER: c_uint = 0x8DFA;
    pub const SHADER_IMAGE_ACCESS_BARRIER_BIT: c_uint = 0x00000020;
    pub const SHADER_IMAGE_ATOMIC: c_uint = 0x82A6;
    pub const SHADER_IMAGE_LOAD: c_uint = 0x82A4;
    pub const SHADER_IMAGE_STORE: c_uint = 0x82A5;
    pub const SHADER_SOURCE_LENGTH: c_uint = 0x8B88;
    pub const SHADER_STORAGE_BARRIER_BIT: c_uint = 0x00002000;
    pub const SHADER_STORAGE_BLOCK: c_uint = 0x92E6;
    pub const SHADER_STORAGE_BUFFER: c_uint = 0x90D2;
    pub const SHADER_STORAGE_BUFFER_BINDING: c_uint = 0x90D3;
    pub const SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x90DF;
    pub const SHADER_STORAGE_BUFFER_SIZE: c_uint = 0x90D5;
    pub const SHADER_STORAGE_BUFFER_START: c_uint = 0x90D4;
    pub const SHADER_TYPE: c_uint = 0x8B4F;
    pub const SHADING_LANGUAGE_VERSION: c_uint = 0x8B8C;
    pub const SHORT: c_uint = 0x1402;
    pub const SIGNALED: c_uint = 0x9119;
    pub const SIGNED_NORMALIZED: c_uint = 0x8F9C;
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST: c_uint = 0x82AC;
    pub const SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE: c_uint = 0x82AE;
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST: c_uint = 0x82AD;
    pub const SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE: c_uint = 0x82AF;
    pub const SMOOTH_LINE_WIDTH_GRANULARITY: c_uint = 0x0B23;
    pub const SMOOTH_LINE_WIDTH_RANGE: c_uint = 0x0B22;
    pub const SMOOTH_POINT_SIZE_GRANULARITY: c_uint = 0x0B13;
    pub const SMOOTH_POINT_SIZE_RANGE: c_uint = 0x0B12;
    pub const SPIR_V_BINARY: c_uint = 0x9552;
    pub const SPIR_V_EXTENSIONS: c_uint = 0x9553;
    pub const SRC1_ALPHA: c_uint = 0x8589;
    pub const SRC1_COLOR: c_uint = 0x88F9;
    pub const SRC_ALPHA: c_uint = 0x0302;
    pub const SRC_ALPHA_SATURATE: c_uint = 0x0308;
    pub const SRC_COLOR: c_uint = 0x0300;
    pub const SRGB: c_uint = 0x8C40;
    pub const SRGB8: c_uint = 0x8C41;
    pub const SRGB8_ALPHA8: c_uint = 0x8C43;
    pub const SRGB_ALPHA: c_uint = 0x8C42;
    pub const SRGB_READ: c_uint = 0x8297;
    pub const SRGB_WRITE: c_uint = 0x8298;
    pub const STACK_OVERFLOW: c_uint = 0x0503;
    pub const STACK_UNDERFLOW: c_uint = 0x0504;
    pub const STATIC_COPY: c_uint = 0x88E6;
    pub const STATIC_DRAW: c_uint = 0x88E4;
    pub const STATIC_READ: c_uint = 0x88E5;
    pub const STENCIL: c_uint = 0x1802;
    pub const STENCIL_ATTACHMENT: c_uint = 0x8D20;
    pub const STENCIL_BACK_FAIL: c_uint = 0x8801;
    pub const STENCIL_BACK_FUNC: c_uint = 0x8800;
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: c_uint = 0x8802;
    pub const STENCIL_BACK_PASS_DEPTH_PASS: c_uint = 0x8803;
    pub const STENCIL_BACK_REF: c_uint = 0x8CA3;
    pub const STENCIL_BACK_VALUE_MASK: c_uint = 0x8CA4;
    pub const STENCIL_BACK_WRITEMASK: c_uint = 0x8CA5;
    pub const STENCIL_BUFFER_BIT: c_uint = 0x00000400;
    pub const STENCIL_CLEAR_VALUE: c_uint = 0x0B91;
    pub const STENCIL_COMPONENTS: c_uint = 0x8285;
    pub const STENCIL_FAIL: c_uint = 0x0B94;
    pub const STENCIL_FUNC: c_uint = 0x0B92;
    pub const STENCIL_INDEX: c_uint = 0x1901;
    pub const STENCIL_INDEX1: c_uint = 0x8D46;
    pub const STENCIL_INDEX16: c_uint = 0x8D49;
    pub const STENCIL_INDEX4: c_uint = 0x8D47;
    pub const STENCIL_INDEX8: c_uint = 0x8D48;
    pub const STENCIL_PASS_DEPTH_FAIL: c_uint = 0x0B95;
    pub const STENCIL_PASS_DEPTH_PASS: c_uint = 0x0B96;
    pub const STENCIL_REF: c_uint = 0x0B97;
    pub const STENCIL_RENDERABLE: c_uint = 0x8288;
    pub const STENCIL_TEST: c_uint = 0x0B90;
    pub const STENCIL_VALUE_MASK: c_uint = 0x0B93;
    pub const STENCIL_WRITEMASK: c_uint = 0x0B98;
    pub const STEREO: c_uint = 0x0C33;
    pub const STREAM_COPY: c_uint = 0x88E2;
    pub const STREAM_DRAW: c_uint = 0x88E0;
    pub const STREAM_READ: c_uint = 0x88E1;
    pub const SUBPIXEL_BITS: c_uint = 0x0D50;
    pub const SYNC_CONDITION: c_uint = 0x9113;
    pub const SYNC_FENCE: c_uint = 0x9116;
    pub const SYNC_FLAGS: c_uint = 0x9115;
    pub const SYNC_FLUSH_COMMANDS_BIT: c_uint = 0x00000001;
    pub const SYNC_GPU_COMMANDS_COMPLETE: c_uint = 0x9117;
    pub const SYNC_STATUS: c_uint = 0x9114;
    pub const TESS_CONTROL_OUTPUT_VERTICES: c_uint = 0x8E75;
    pub const TESS_CONTROL_SHADER: c_uint = 0x8E88;
    pub const TESS_CONTROL_SHADER_BIT: c_uint = 0x00000008;
    pub const TESS_CONTROL_SHADER_PATCHES: c_uint = 0x82F1;
    pub const TESS_CONTROL_SUBROUTINE: c_uint = 0x92E9;
    pub const TESS_CONTROL_SUBROUTINE_UNIFORM: c_uint = 0x92EF;
    pub const TESS_CONTROL_TEXTURE: c_uint = 0x829C;
    pub const TESS_EVALUATION_SHADER: c_uint = 0x8E87;
    pub const TESS_EVALUATION_SHADER_BIT: c_uint = 0x00000010;
    pub const TESS_EVALUATION_SHADER_INVOCATIONS: c_uint = 0x82F2;
    pub const TESS_EVALUATION_SUBROUTINE: c_uint = 0x92EA;
    pub const TESS_EVALUATION_SUBROUTINE_UNIFORM: c_uint = 0x92F0;
    pub const TESS_EVALUATION_TEXTURE: c_uint = 0x829D;
    pub const TESS_GEN_MODE: c_uint = 0x8E76;
    pub const TESS_GEN_POINT_MODE: c_uint = 0x8E79;
    pub const TESS_GEN_SPACING: c_uint = 0x8E77;
    pub const TESS_GEN_VERTEX_ORDER: c_uint = 0x8E78;
    pub const TEXTURE: c_uint = 0x1702;
    pub const TEXTURE0: c_uint = 0x84C0;
    pub const TEXTURE1: c_uint = 0x84C1;
    pub const TEXTURE10: c_uint = 0x84CA;
    pub const TEXTURE11: c_uint = 0x84CB;
    pub const TEXTURE12: c_uint = 0x84CC;
    pub const TEXTURE13: c_uint = 0x84CD;
    pub const TEXTURE14: c_uint = 0x84CE;
    pub const TEXTURE15: c_uint = 0x84CF;
    pub const TEXTURE16: c_uint = 0x84D0;
    pub const TEXTURE17: c_uint = 0x84D1;
    pub const TEXTURE18: c_uint = 0x84D2;
    pub const TEXTURE19: c_uint = 0x84D3;
    pub const TEXTURE2: c_uint = 0x84C2;
    pub const TEXTURE20: c_uint = 0x84D4;
    pub const TEXTURE21: c_uint = 0x84D5;
    pub const TEXTURE22: c_uint = 0x84D6;
    pub const TEXTURE23: c_uint = 0x84D7;
    pub const TEXTURE24: c_uint = 0x84D8;
    pub const TEXTURE25: c_uint = 0x84D9;
    pub const TEXTURE26: c_uint = 0x84DA;
    pub const TEXTURE27: c_uint = 0x84DB;
    pub const TEXTURE28: c_uint = 0x84DC;
    pub const TEXTURE29: c_uint = 0x84DD;
    pub const TEXTURE3: c_uint = 0x84C3;
    pub const TEXTURE30: c_uint = 0x84DE;
    pub const TEXTURE31: c_uint = 0x84DF;
    pub const TEXTURE4: c_uint = 0x84C4;
    pub const TEXTURE5: c_uint = 0x84C5;
    pub const TEXTURE6: c_uint = 0x84C6;
    pub const TEXTURE7: c_uint = 0x84C7;
    pub const TEXTURE8: c_uint = 0x84C8;
    pub const TEXTURE9: c_uint = 0x84C9;
    pub const TEXTURE_1D: c_uint = 0x0DE0;
    pub const TEXTURE_1D_ARRAY: c_uint = 0x8C18;
    pub const TEXTURE_2D: c_uint = 0x0DE1;
    pub const TEXTURE_2D_ARRAY: c_uint = 0x8C1A;
    pub const TEXTURE_2D_MULTISAMPLE: c_uint = 0x9100;
    pub const TEXTURE_2D_MULTISAMPLE_ARRAY: c_uint = 0x9102;
    pub const TEXTURE_3D: c_uint = 0x806F;
    pub const TEXTURE_ALPHA_SIZE: c_uint = 0x805F;
    pub const TEXTURE_ALPHA_TYPE: c_uint = 0x8C13;
    pub const TEXTURE_BASE_LEVEL: c_uint = 0x813C;
    pub const TEXTURE_BINDING_1D: c_uint = 0x8068;
    pub const TEXTURE_BINDING_1D_ARRAY: c_uint = 0x8C1C;
    pub const TEXTURE_BINDING_2D: c_uint = 0x8069;
    pub const TEXTURE_BINDING_2D_ARRAY: c_uint = 0x8C1D;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE: c_uint = 0x9104;
    pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: c_uint = 0x9105;
    pub const TEXTURE_BINDING_3D: c_uint = 0x806A;
    pub const TEXTURE_BINDING_BUFFER: c_uint = 0x8C2C;
    pub const TEXTURE_BINDING_CUBE_MAP: c_uint = 0x8514;
    pub const TEXTURE_BINDING_CUBE_MAP_ARRAY: c_uint = 0x900A;
    pub const TEXTURE_BINDING_RECTANGLE: c_uint = 0x84F6;
    pub const TEXTURE_BLUE_SIZE: c_uint = 0x805E;
    pub const TEXTURE_BLUE_TYPE: c_uint = 0x8C12;
    pub const TEXTURE_BORDER_COLOR: c_uint = 0x1004;
    pub const TEXTURE_BUFFER: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_BINDING: c_uint = 0x8C2A;
    pub const TEXTURE_BUFFER_DATA_STORE_BINDING: c_uint = 0x8C2D;
    pub const TEXTURE_BUFFER_OFFSET: c_uint = 0x919D;
    pub const TEXTURE_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x919F;
    pub const TEXTURE_BUFFER_SIZE: c_uint = 0x919E;
    pub const TEXTURE_COMPARE_FUNC: c_uint = 0x884D;
    pub const TEXTURE_COMPARE_MODE: c_uint = 0x884C;
    pub const TEXTURE_COMPRESSED: c_uint = 0x86A1;
    pub const TEXTURE_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x82B2;
    pub const TEXTURE_COMPRESSED_BLOCK_SIZE: c_uint = 0x82B3;
    pub const TEXTURE_COMPRESSED_BLOCK_WIDTH: c_uint = 0x82B1;
    pub const TEXTURE_COMPRESSED_IMAGE_SIZE: c_uint = 0x86A0;
    pub const TEXTURE_COMPRESSION_HINT: c_uint = 0x84EF;
    pub const TEXTURE_CUBE_MAP: c_uint = 0x8513;
    pub const TEXTURE_CUBE_MAP_ARRAY: c_uint = 0x9009;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: c_uint = 0x8516;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: c_uint = 0x8518;
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: c_uint = 0x851A;
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: c_uint = 0x8515;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: c_uint = 0x8517;
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: c_uint = 0x8519;
    pub const TEXTURE_CUBE_MAP_SEAMLESS: c_uint = 0x884F;
    pub const TEXTURE_DEPTH: c_uint = 0x8071;
    pub const TEXTURE_DEPTH_SIZE: c_uint = 0x884A;
    pub const TEXTURE_DEPTH_TYPE: c_uint = 0x8C16;
    pub const TEXTURE_FETCH_BARRIER_BIT: c_uint = 0x00000008;
    pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: c_uint = 0x9107;
    pub const TEXTURE_GATHER: c_uint = 0x82A2;
    pub const TEXTURE_GATHER_SHADOW: c_uint = 0x82A3;
    pub const TEXTURE_GREEN_SIZE: c_uint = 0x805D;
    pub const TEXTURE_GREEN_TYPE: c_uint = 0x8C11;
    pub const TEXTURE_HEIGHT: c_uint = 0x1001;
    pub const TEXTURE_IMAGE_FORMAT: c_uint = 0x828F;
    pub const TEXTURE_IMAGE_TYPE: c_uint = 0x8290;
    pub const TEXTURE_IMMUTABLE_FORMAT: c_uint = 0x912F;
    pub const TEXTURE_IMMUTABLE_LEVELS: c_uint = 0x82DF;
    pub const TEXTURE_INTERNAL_FORMAT: c_uint = 0x1003;
    pub const TEXTURE_LOD_BIAS: c_uint = 0x8501;
    pub const TEXTURE_MAG_FILTER: c_uint = 0x2800;
    pub const TEXTURE_MAX_ANISOTROPY: c_uint = 0x84FE;
    pub const TEXTURE_MAX_LEVEL: c_uint = 0x813D;
    pub const TEXTURE_MAX_LOD: c_uint = 0x813B;
    pub const TEXTURE_MIN_FILTER: c_uint = 0x2801;
    pub const TEXTURE_MIN_LOD: c_uint = 0x813A;
    pub const TEXTURE_RECTANGLE: c_uint = 0x84F5;
    pub const TEXTURE_RED_SIZE: c_uint = 0x805C;
    pub const TEXTURE_RED_TYPE: c_uint = 0x8C10;
    pub const TEXTURE_SAMPLES: c_uint = 0x9106;
    pub const TEXTURE_SHADOW: c_uint = 0x82A1;
    pub const TEXTURE_SHARED_SIZE: c_uint = 0x8C3F;
    pub const TEXTURE_STENCIL_SIZE: c_uint = 0x88F1;
    pub const TEXTURE_SWIZZLE_A: c_uint = 0x8E45;
    pub const TEXTURE_SWIZZLE_B: c_uint = 0x8E44;
    pub const TEXTURE_SWIZZLE_G: c_uint = 0x8E43;
    pub const TEXTURE_SWIZZLE_R: c_uint = 0x8E42;
    pub const TEXTURE_SWIZZLE_RGBA: c_uint = 0x8E46;
    pub const TEXTURE_TARGET: c_uint = 0x1006;
    pub const TEXTURE_UPDATE_BARRIER_BIT: c_uint = 0x00000100;
    pub const TEXTURE_VIEW: c_uint = 0x82B5;
    pub const TEXTURE_VIEW_MIN_LAYER: c_uint = 0x82DD;
    pub const TEXTURE_VIEW_MIN_LEVEL: c_uint = 0x82DB;
    pub const TEXTURE_VIEW_NUM_LAYERS: c_uint = 0x82DE;
    pub const TEXTURE_VIEW_NUM_LEVELS: c_uint = 0x82DC;
    pub const TEXTURE_WIDTH: c_uint = 0x1000;
    pub const TEXTURE_WRAP_R: c_uint = 0x8072;
    pub const TEXTURE_WRAP_S: c_uint = 0x2802;
    pub const TEXTURE_WRAP_T: c_uint = 0x2803;
    pub const TIMEOUT_EXPIRED: c_uint = 0x911B;
    pub const TIMEOUT_IGNORED: u64 = 0xFFFFFFFFFFFFFFFF;
    pub const TIMESTAMP: c_uint = 0x8E28;
    pub const TIME_ELAPSED: c_uint = 0x88BF;
    pub const TOP_LEVEL_ARRAY_SIZE: c_uint = 0x930C;
    pub const TOP_LEVEL_ARRAY_STRIDE: c_uint = 0x930D;
    pub const TRANSFORM_FEEDBACK: c_uint = 0x8E22;
    pub const TRANSFORM_FEEDBACK_ACTIVE: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_BARRIER_BIT: c_uint = 0x00000800;
    pub const TRANSFORM_FEEDBACK_BINDING: c_uint = 0x8E25;
    pub const TRANSFORM_FEEDBACK_BUFFER: c_uint = 0x8C8E;
    pub const TRANSFORM_FEEDBACK_BUFFER_ACTIVE: c_uint = 0x8E24;
    pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: c_uint = 0x8C8F;
    pub const TRANSFORM_FEEDBACK_BUFFER_INDEX: c_uint = 0x934B;
    pub const TRANSFORM_FEEDBACK_BUFFER_MODE: c_uint = 0x8C7F;
    pub const TRANSFORM_FEEDBACK_BUFFER_PAUSED: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: c_uint = 0x8C85;
    pub const TRANSFORM_FEEDBACK_BUFFER_START: c_uint = 0x8C84;
    pub const TRANSFORM_FEEDBACK_BUFFER_STRIDE: c_uint = 0x934C;
    pub const TRANSFORM_FEEDBACK_OVERFLOW: c_uint = 0x82EC;
    pub const TRANSFORM_FEEDBACK_PAUSED: c_uint = 0x8E23;
    pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: c_uint = 0x8C88;
    pub const TRANSFORM_FEEDBACK_STREAM_OVERFLOW: c_uint = 0x82ED;
    pub const TRANSFORM_FEEDBACK_VARYING: c_uint = 0x92F4;
    pub const TRANSFORM_FEEDBACK_VARYINGS: c_uint = 0x8C83;
    pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: c_uint = 0x8C76;
    pub const TRIANGLES: c_uint = 0x0004;
    pub const TRIANGLES_ADJACENCY: c_uint = 0x000C;
    pub const TRIANGLE_FAN: c_uint = 0x0006;
    pub const TRIANGLE_STRIP: c_uint = 0x0005;
    pub const TRIANGLE_STRIP_ADJACENCY: c_uint = 0x000D;
    pub const TRUE: c_uchar = 1;
    pub const TYPE: c_uint = 0x92FA;
    pub const UNDEFINED_VERTEX: c_uint = 0x8260;
    pub const UNIFORM: c_uint = 0x92E1;
    pub const UNIFORM_ARRAY_STRIDE: c_uint = 0x8A3C;
    pub const UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX: c_uint = 0x92DA;
    pub const UNIFORM_BARRIER_BIT: c_uint = 0x00000004;
    pub const UNIFORM_BLOCK: c_uint = 0x92E2;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: c_uint = 0x8A42;
    pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: c_uint = 0x8A43;
    pub const UNIFORM_BLOCK_BINDING: c_uint = 0x8A3F;
    pub const UNIFORM_BLOCK_DATA_SIZE: c_uint = 0x8A40;
    pub const UNIFORM_BLOCK_INDEX: c_uint = 0x8A3A;
    pub const UNIFORM_BLOCK_NAME_LENGTH: c_uint = 0x8A41;
    pub const UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER: c_uint = 0x90EC;
    pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: c_uint = 0x8A46;
    pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: c_uint = 0x8A45;
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER: c_uint = 0x84F0;
    pub const UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER: c_uint = 0x84F1;
    pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: c_uint = 0x8A44;
    pub const UNIFORM_BUFFER: c_uint = 0x8A11;
    pub const UNIFORM_BUFFER_BINDING: c_uint = 0x8A28;
    pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: c_uint = 0x8A34;
    pub const UNIFORM_BUFFER_SIZE: c_uint = 0x8A2A;
    pub const UNIFORM_BUFFER_START: c_uint = 0x8A29;
    pub const UNIFORM_IS_ROW_MAJOR: c_uint = 0x8A3E;
    pub const UNIFORM_MATRIX_STRIDE: c_uint = 0x8A3D;
    pub const UNIFORM_NAME_LENGTH: c_uint = 0x8A39;
    pub const UNIFORM_OFFSET: c_uint = 0x8A3B;
    pub const UNIFORM_SIZE: c_uint = 0x8A38;
    pub const UNIFORM_TYPE: c_uint = 0x8A37;
    pub const UNKNOWN_CONTEXT_RESET: c_uint = 0x8255;
    pub const UNPACK_ALIGNMENT: c_uint = 0x0CF5;
    pub const UNPACK_COMPRESSED_BLOCK_DEPTH: c_uint = 0x9129;
    pub const UNPACK_COMPRESSED_BLOCK_HEIGHT: c_uint = 0x9128;
    pub const UNPACK_COMPRESSED_BLOCK_SIZE: c_uint = 0x912A;
    pub const UNPACK_COMPRESSED_BLOCK_WIDTH: c_uint = 0x9127;
    pub const UNPACK_IMAGE_HEIGHT: c_uint = 0x806E;
    pub const UNPACK_LSB_FIRST: c_uint = 0x0CF1;
    pub const UNPACK_ROW_LENGTH: c_uint = 0x0CF2;
    pub const UNPACK_SKIP_IMAGES: c_uint = 0x806D;
    pub const UNPACK_SKIP_PIXELS: c_uint = 0x0CF4;
    pub const UNPACK_SKIP_ROWS: c_uint = 0x0CF3;
    pub const UNPACK_SWAP_BYTES: c_uint = 0x0CF0;
    pub const UNSIGNALED: c_uint = 0x9118;
    pub const UNSIGNED_BYTE: c_uint = 0x1401;
    pub const UNSIGNED_BYTE_2_3_3_REV: c_uint = 0x8362;
    pub const UNSIGNED_BYTE_3_3_2: c_uint = 0x8032;
    pub const UNSIGNED_INT: c_uint = 0x1405;
    pub const UNSIGNED_INT_10F_11F_11F_REV: c_uint = 0x8C3B;
    pub const UNSIGNED_INT_10_10_10_2: c_uint = 0x8036;
    pub const UNSIGNED_INT_24_8: c_uint = 0x84FA;
    pub const UNSIGNED_INT_2_10_10_10_REV: c_uint = 0x8368;
    pub const UNSIGNED_INT_5_9_9_9_REV: c_uint = 0x8C3E;
    pub const UNSIGNED_INT_8_8_8_8: c_uint = 0x8035;
    pub const UNSIGNED_INT_8_8_8_8_REV: c_uint = 0x8367;
    pub const UNSIGNED_INT_ATOMIC_COUNTER: c_uint = 0x92DB;
    pub const UNSIGNED_INT_IMAGE_1D: c_uint = 0x9062;
    pub const UNSIGNED_INT_IMAGE_1D_ARRAY: c_uint = 0x9068;
    pub const UNSIGNED_INT_IMAGE_2D: c_uint = 0x9063;
    pub const UNSIGNED_INT_IMAGE_2D_ARRAY: c_uint = 0x9069;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE: c_uint = 0x906B;
    pub const UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY: c_uint = 0x906C;
    pub const UNSIGNED_INT_IMAGE_2D_RECT: c_uint = 0x9065;
    pub const UNSIGNED_INT_IMAGE_3D: c_uint = 0x9064;
    pub const UNSIGNED_INT_IMAGE_BUFFER: c_uint = 0x9067;
    pub const UNSIGNED_INT_IMAGE_CUBE: c_uint = 0x9066;
    pub const UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY: c_uint = 0x906A;
    pub const UNSIGNED_INT_SAMPLER_1D: c_uint = 0x8DD1;
    pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: c_uint = 0x8DD6;
    pub const UNSIGNED_INT_SAMPLER_2D: c_uint = 0x8DD2;
    pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: c_uint = 0x8DD7;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: c_uint = 0x910A;
    pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: c_uint = 0x910D;
    pub const UNSIGNED_INT_SAMPLER_2D_RECT: c_uint = 0x8DD5;
    pub const UNSIGNED_INT_SAMPLER_3D: c_uint = 0x8DD3;
    pub const UNSIGNED_INT_SAMPLER_BUFFER: c_uint = 0x8DD8;
    pub const UNSIGNED_INT_SAMPLER_CUBE: c_uint = 0x8DD4;
    pub const UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY: c_uint = 0x900F;
    pub const UNSIGNED_INT_VEC2: c_uint = 0x8DC6;
    pub const UNSIGNED_INT_VEC3: c_uint = 0x8DC7;
    pub const UNSIGNED_INT_VEC4: c_uint = 0x8DC8;
    pub const UNSIGNED_NORMALIZED: c_uint = 0x8C17;
    pub const UNSIGNED_SHORT: c_uint = 0x1403;
    pub const UNSIGNED_SHORT_1_5_5_5_REV: c_uint = 0x8366;
    pub const UNSIGNED_SHORT_4_4_4_4: c_uint = 0x8033;
    pub const UNSIGNED_SHORT_4_4_4_4_REV: c_uint = 0x8365;
    pub const UNSIGNED_SHORT_5_5_5_1: c_uint = 0x8034;
    pub const UNSIGNED_SHORT_5_6_5: c_uint = 0x8363;
    pub const UNSIGNED_SHORT_5_6_5_REV: c_uint = 0x8364;
    pub const UPPER_LEFT: c_uint = 0x8CA2;
    pub const VALIDATE_STATUS: c_uint = 0x8B83;
    pub const VENDOR: c_uint = 0x1F00;
    pub const VERSION: c_uint = 0x1F02;
    pub const VERTEX_ARRAY: c_uint = 0x8074;
    pub const VERTEX_ARRAY_BINDING: c_uint = 0x85B5;
    pub const VERTEX_ATTRIB_ARRAY_BARRIER_BIT: c_uint = 0x00000001;
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: c_uint = 0x889F;
    pub const VERTEX_ATTRIB_ARRAY_DIVISOR: c_uint = 0x88FE;
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: c_uint = 0x8622;
    pub const VERTEX_ATTRIB_ARRAY_INTEGER: c_uint = 0x88FD;
    pub const VERTEX_ATTRIB_ARRAY_LONG: c_uint = 0x874E;
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: c_uint = 0x886A;
    pub const VERTEX_ATTRIB_ARRAY_POINTER: c_uint = 0x8645;
    pub const VERTEX_ATTRIB_ARRAY_SIZE: c_uint = 0x8623;
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: c_uint = 0x8624;
    pub const VERTEX_ATTRIB_ARRAY_TYPE: c_uint = 0x8625;
    pub const VERTEX_ATTRIB_BINDING: c_uint = 0x82D4;
    pub const VERTEX_ATTRIB_RELATIVE_OFFSET: c_uint = 0x82D5;
    pub const VERTEX_BINDING_BUFFER: c_uint = 0x8F4F;
    pub const VERTEX_BINDING_DIVISOR: c_uint = 0x82D6;
    pub const VERTEX_BINDING_OFFSET: c_uint = 0x82D7;
    pub const VERTEX_BINDING_STRIDE: c_uint = 0x82D8;
    pub const VERTEX_PROGRAM_POINT_SIZE: c_uint = 0x8642;
    pub const VERTEX_SHADER: c_uint = 0x8B31;
    pub const VERTEX_SHADER_BIT: c_uint = 0x00000001;
    pub const VERTEX_SHADER_INVOCATIONS: c_uint = 0x82F0;
    pub const VERTEX_SUBROUTINE: c_uint = 0x92E8;
    pub const VERTEX_SUBROUTINE_UNIFORM: c_uint = 0x92EE;
    pub const VERTEX_TEXTURE: c_uint = 0x829B;
    pub const VERTICES_SUBMITTED: c_uint = 0x82EE;
    pub const VIEWPORT: c_uint = 0x0BA2;
    pub const VIEWPORT_BOUNDS_RANGE: c_uint = 0x825D;
    pub const VIEWPORT_INDEX_PROVOKING_VERTEX: c_uint = 0x825F;
    pub const VIEWPORT_SUBPIXEL_BITS: c_uint = 0x825C;
    pub const VIEW_CLASS_128_BITS: c_uint = 0x82C4;
    pub const VIEW_CLASS_16_BITS: c_uint = 0x82CA;
    pub const VIEW_CLASS_24_BITS: c_uint = 0x82C9;
    pub const VIEW_CLASS_32_BITS: c_uint = 0x82C8;
    pub const VIEW_CLASS_48_BITS: c_uint = 0x82C7;
    pub const VIEW_CLASS_64_BITS: c_uint = 0x82C6;
    pub const VIEW_CLASS_8_BITS: c_uint = 0x82CB;
    pub const VIEW_CLASS_96_BITS: c_uint = 0x82C5;
    pub const VIEW_CLASS_BPTC_FLOAT: c_uint = 0x82D3;
    pub const VIEW_CLASS_BPTC_UNORM: c_uint = 0x82D2;
    pub const VIEW_CLASS_RGTC1_RED: c_uint = 0x82D0;
    pub const VIEW_CLASS_RGTC2_RG: c_uint = 0x82D1;
    pub const VIEW_CLASS_S3TC_DXT1_RGB: c_uint = 0x82CC;
    pub const VIEW_CLASS_S3TC_DXT1_RGBA: c_uint = 0x82CD;
    pub const VIEW_CLASS_S3TC_DXT3_RGBA: c_uint = 0x82CE;
    pub const VIEW_CLASS_S3TC_DXT5_RGBA: c_uint = 0x82CF;
    pub const VIEW_COMPATIBILITY_CLASS: c_uint = 0x82B6;
    pub const WAIT_FAILED: c_uint = 0x911D;
    pub const WRITE_ONLY: c_uint = 0x88B9;
    pub const XOR: c_uint = 0x1506;
    pub const ZERO: c_uint = 0;
    pub const ZERO_TO_ONE: c_uint = 0x935F;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code, unused_imports)]

    use super::types::*;
    use super::*;
    use std::mem::transmute;
    use std::os::raw::*;

    macro_rules! func {
        ($fun:ident, $ret:ty, $($name:ident: $typ:ty),*) => {
            #[inline] pub unsafe fn $fun($($name: $typ),*) -> $ret {
                transmute::<_, extern "system" fn($($typ),*) -> $ret>(storage::$fun.ptr)($($name),*)
            }
        }
    }

    func!(ActiveShaderProgram, (), pipeline: GLuint, program: GLuint);
    func!(ActiveTexture, (), texture: GLenum);
    func!(AttachShader, (), program: GLuint, shader: GLuint);
    func!(BeginConditionalRender, (), id: GLuint, mode: GLenum);
    func!(BeginQuery, (), target: GLenum, id: GLuint);
    func!(
        BeginQueryIndexed,
        (),
        target: GLenum,
        index: GLuint,
        id: GLuint
    );
    func!(BeginTransformFeedback, (), primitiveMode: GLenum);
    func!(
        BindAttribLocation,
        (),
        program: GLuint,
        index: GLuint,
        name: *const GLchar
    );
    func!(BindBuffer, (), target: GLenum, buffer: GLuint);
    func!(
        BindBufferBase,
        (),
        target: GLenum,
        index: GLuint,
        buffer: GLuint
    );
    func!(
        BindBufferRange,
        (),
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        BindBuffersBase,
        (),
        target: GLenum,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint
    );
    func!(
        BindBuffersRange,
        (),
        target: GLenum,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        sizes: *const GLsizeiptr
    );
    func!(
        BindFragDataLocation,
        (),
        program: GLuint,
        color: GLuint,
        name: *const GLchar
    );
    func!(
        BindFragDataLocationIndexed,
        (),
        program: GLuint,
        colorNumber: GLuint,
        index: GLuint,
        name: *const GLchar
    );
    func!(BindFramebuffer, (), target: GLenum, framebuffer: GLuint);
    func!(
        BindImageTexture,
        (),
        unit: GLuint,
        texture: GLuint,
        level: GLint,
        layered: GLboolean,
        layer: GLint,
        access: GLenum,
        format: GLenum
    );
    func!(
        BindImageTextures,
        (),
        first: GLuint,
        count: GLsizei,
        textures: *const GLuint
    );
    func!(BindProgramPipeline, (), pipeline: GLuint);
    func!(BindRenderbuffer, (), target: GLenum, renderbuffer: GLuint);
    func!(BindSampler, (), unit: GLuint, sampler: GLuint);
    func!(
        BindSamplers,
        (),
        first: GLuint,
        count: GLsizei,
        samplers: *const GLuint
    );
    func!(BindTexture, (), target: GLenum, texture: GLuint);
    func!(BindTextureUnit, (), unit: GLuint, texture: GLuint);
    func!(
        BindTextures,
        (),
        first: GLuint,
        count: GLsizei,
        textures: *const GLuint
    );
    func!(BindTransformFeedback, (), target: GLenum, id: GLuint);
    func!(BindVertexArray, (), array: GLuint);
    func!(
        BindVertexBuffer,
        (),
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei
    );
    func!(
        BindVertexBuffers,
        (),
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei
    );
    func!(
        BlendColor,
        (),
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat
    );
    func!(BlendEquation, (), mode: GLenum);
    func!(
        BlendEquationSeparate,
        (),
        modeRGB: GLenum,
        modeAlpha: GLenum
    );
    func!(
        BlendEquationSeparatei,
        (),
        buf: GLuint,
        modeRGB: GLenum,
        modeAlpha: GLenum
    );
    func!(BlendEquationi, (), buf: GLuint, mode: GLenum);
    func!(BlendFunc, (), sfactor: GLenum, dfactor: GLenum);
    func!(
        BlendFuncSeparate,
        (),
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum
    );
    func!(
        BlendFuncSeparatei,
        (),
        buf: GLuint,
        srcRGB: GLenum,
        dstRGB: GLenum,
        srcAlpha: GLenum,
        dstAlpha: GLenum
    );
    func!(BlendFunci, (), buf: GLuint, src: GLenum, dst: GLenum);
    func!(
        BlitFramebuffer,
        (),
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum
    );
    func!(
        BlitNamedFramebuffer,
        (),
        readFramebuffer: GLuint,
        drawFramebuffer: GLuint,
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum
    );
    func!(
        BufferData,
        (),
        target: GLenum,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum
    );
    func!(
        BufferStorage,
        (),
        target: GLenum,
        size: GLsizeiptr,
        data: *const c_void,
        flags: GLbitfield
    );
    func!(
        BufferSubData,
        (),
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const c_void
    );
    func!(CheckFramebufferStatus, GLenum, target: GLenum);
    func!(
        CheckNamedFramebufferStatus,
        GLenum,
        framebuffer: GLuint,
        target: GLenum
    );
    func!(ClampColor, (), target: GLenum, clamp: GLenum);
    func!(Clear, (), mask: GLbitfield);
    func!(
        ClearBufferData,
        (),
        target: GLenum,
        internalformat: GLenum,
        format: GLenum,
        type_: GLenum,
        data: *const c_void
    );
    func!(
        ClearBufferSubData,
        (),
        target: GLenum,
        internalformat: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        format: GLenum,
        type_: GLenum,
        data: *const c_void
    );
    func!(
        ClearBufferfi,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint
    );
    func!(
        ClearBufferfv,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLfloat
    );
    func!(
        ClearBufferiv,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLint
    );
    func!(
        ClearBufferuiv,
        (),
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLuint
    );
    func!(
        ClearColor,
        (),
        red: GLfloat,
        green: GLfloat,
        blue: GLfloat,
        alpha: GLfloat
    );
    func!(ClearDepth, (), depth: GLdouble);
    func!(ClearDepthf, (), d: GLfloat);
    func!(
        ClearNamedBufferData,
        (),
        buffer: GLuint,
        internalformat: GLenum,
        format: GLenum,
        type_: GLenum,
        data: *const c_void
    );
    func!(
        ClearNamedBufferSubData,
        (),
        buffer: GLuint,
        internalformat: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        format: GLenum,
        type_: GLenum,
        data: *const c_void
    );
    func!(
        ClearNamedFramebufferfi,
        (),
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        depth: GLfloat,
        stencil: GLint
    );
    func!(
        ClearNamedFramebufferfv,
        (),
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLfloat
    );
    func!(
        ClearNamedFramebufferiv,
        (),
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLint
    );
    func!(
        ClearNamedFramebufferuiv,
        (),
        framebuffer: GLuint,
        buffer: GLenum,
        drawbuffer: GLint,
        value: *const GLuint
    );
    func!(ClearStencil, (), s: GLint);
    func!(
        ClearTexImage,
        (),
        texture: GLuint,
        level: GLint,
        format: GLenum,
        type_: GLenum,
        data: *const c_void
    );
    func!(
        ClearTexSubImage,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        data: *const c_void
    );
    func!(
        ClientWaitSync,
        GLenum,
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64
    );
    func!(ClipControl, (), origin: GLenum, depth: GLenum);
    func!(
        ColorMask,
        (),
        red: GLboolean,
        green: GLboolean,
        blue: GLboolean,
        alpha: GLboolean
    );
    func!(
        ColorMaski,
        (),
        index: GLuint,
        r: GLboolean,
        g: GLboolean,
        b: GLboolean,
        a: GLboolean
    );
    func!(CompileShader, (), shader: GLuint);
    func!(
        CompressedTexImage1D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexImage2D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexImage3D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexSubImage1D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexSubImage2D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTexSubImage3D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTextureSubImage1D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTextureSubImage2D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CompressedTextureSubImage3D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const c_void
    );
    func!(
        CopyBufferSubData,
        (),
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        CopyImageSubData,
        (),
        srcName: GLuint,
        srcTarget: GLenum,
        srcLevel: GLint,
        srcX: GLint,
        srcY: GLint,
        srcZ: GLint,
        dstName: GLuint,
        dstTarget: GLenum,
        dstLevel: GLint,
        dstX: GLint,
        dstY: GLint,
        dstZ: GLint,
        srcWidth: GLsizei,
        srcHeight: GLsizei,
        srcDepth: GLsizei
    );
    func!(
        CopyNamedBufferSubData,
        (),
        readBuffer: GLuint,
        writeBuffer: GLuint,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        CopyTexImage1D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        border: GLint
    );
    func!(
        CopyTexImage2D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint
    );
    func!(
        CopyTexSubImage1D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei
    );
    func!(
        CopyTexSubImage2D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        CopyTexSubImage3D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        CopyTextureSubImage1D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei
    );
    func!(
        CopyTextureSubImage2D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        CopyTextureSubImage3D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(CreateBuffers, (), n: GLsizei, buffers: *mut GLuint);
    func!(
        CreateFramebuffers,
        (),
        n: GLsizei,
        framebuffers: *mut GLuint
    );
    func!(CreateProgram, GLuint,);
    func!(
        CreateProgramPipelines,
        (),
        n: GLsizei,
        pipelines: *mut GLuint
    );
    func!(
        CreateQueries,
        (),
        target: GLenum,
        n: GLsizei,
        ids: *mut GLuint
    );
    func!(
        CreateRenderbuffers,
        (),
        n: GLsizei,
        renderbuffers: *mut GLuint
    );
    func!(CreateSamplers, (), n: GLsizei, samplers: *mut GLuint);
    func!(CreateShader, GLuint, type_: GLenum);
    func!(
        CreateShaderProgramv,
        GLuint,
        type_: GLenum,
        count: GLsizei,
        strings: *const *const GLchar
    );
    func!(
        CreateTextures,
        (),
        target: GLenum,
        n: GLsizei,
        textures: *mut GLuint
    );
    func!(CreateTransformFeedbacks, (), n: GLsizei, ids: *mut GLuint);
    func!(CreateVertexArrays, (), n: GLsizei, arrays: *mut GLuint);
    func!(CullFace, (), mode: GLenum);
    func!(
        DebugMessageCallback,
        (),
        callback: GLDEBUGPROC,
        userParam: *const c_void
    );
    func!(
        DebugMessageControl,
        (),
        source: GLenum,
        type_: GLenum,
        severity: GLenum,
        count: GLsizei,
        ids: *const GLuint,
        enabled: GLboolean
    );
    func!(
        DebugMessageInsert,
        (),
        source: GLenum,
        type_: GLenum,
        id: GLuint,
        severity: GLenum,
        length: GLsizei,
        buf: *const GLchar
    );
    func!(DeleteBuffers, (), n: GLsizei, buffers: *const GLuint);
    func!(
        DeleteFramebuffers,
        (),
        n: GLsizei,
        framebuffers: *const GLuint
    );
    func!(DeleteProgram, (), program: GLuint);
    func!(
        DeleteProgramPipelines,
        (),
        n: GLsizei,
        pipelines: *const GLuint
    );
    func!(DeleteQueries, (), n: GLsizei, ids: *const GLuint);
    func!(
        DeleteRenderbuffers,
        (),
        n: GLsizei,
        renderbuffers: *const GLuint
    );
    func!(DeleteSamplers, (), count: GLsizei, samplers: *const GLuint);
    func!(DeleteShader, (), shader: GLuint);
    func!(DeleteSync, (), sync: GLsync);
    func!(DeleteTextures, (), n: GLsizei, textures: *const GLuint);
    func!(DeleteTransformFeedbacks, (), n: GLsizei, ids: *const GLuint);
    func!(DeleteVertexArrays, (), n: GLsizei, arrays: *const GLuint);
    func!(DepthFunc, (), func: GLenum);
    func!(DepthMask, (), flag: GLboolean);
    func!(DepthRange, (), n: GLdouble, f: GLdouble);
    func!(
        DepthRangeArrayv,
        (),
        first: GLuint,
        count: GLsizei,
        v: *const GLdouble
    );
    func!(
        DepthRangeIndexed,
        (),
        index: GLuint,
        n: GLdouble,
        f: GLdouble
    );
    func!(DepthRangef, (), n: GLfloat, f: GLfloat);
    func!(DetachShader, (), program: GLuint, shader: GLuint);
    func!(Disable, (), cap: GLenum);
    func!(DisableVertexArrayAttrib, (), vaobj: GLuint, index: GLuint);
    func!(DisableVertexAttribArray, (), index: GLuint);
    func!(Disablei, (), target: GLenum, index: GLuint);
    func!(
        DispatchCompute,
        (),
        num_groups_x: GLuint,
        num_groups_y: GLuint,
        num_groups_z: GLuint
    );
    func!(DispatchComputeIndirect, (), indirect: GLintptr);
    func!(DrawArrays, (), mode: GLenum, first: GLint, count: GLsizei);
    func!(
        DrawArraysIndirect,
        (),
        mode: GLenum,
        indirect: *const c_void
    );
    func!(
        DrawArraysInstanced,
        (),
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei
    );
    func!(
        DrawArraysInstancedBaseInstance,
        (),
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
        baseinstance: GLuint
    );
    func!(DrawBuffer, (), buf: GLenum);
    func!(DrawBuffers, (), n: GLsizei, bufs: *const GLenum);
    func!(
        DrawElements,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void
    );
    func!(
        DrawElementsBaseVertex,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        basevertex: GLint
    );
    func!(
        DrawElementsIndirect,
        (),
        mode: GLenum,
        type_: GLenum,
        indirect: *const c_void
    );
    func!(
        DrawElementsInstanced,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei
    );
    func!(
        DrawElementsInstancedBaseInstance,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        baseinstance: GLuint
    );
    func!(
        DrawElementsInstancedBaseVertex,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        basevertex: GLint
    );
    func!(
        DrawElementsInstancedBaseVertexBaseInstance,
        (),
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        instancecount: GLsizei,
        basevertex: GLint,
        baseinstance: GLuint
    );
    func!(
        DrawRangeElements,
        (),
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void
    );
    func!(
        DrawRangeElementsBaseVertex,
        (),
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const c_void,
        basevertex: GLint
    );
    func!(DrawTransformFeedback, (), mode: GLenum, id: GLuint);
    func!(
        DrawTransformFeedbackInstanced,
        (),
        mode: GLenum,
        id: GLuint,
        instancecount: GLsizei
    );
    func!(
        DrawTransformFeedbackStream,
        (),
        mode: GLenum,
        id: GLuint,
        stream: GLuint
    );
    func!(
        DrawTransformFeedbackStreamInstanced,
        (),
        mode: GLenum,
        id: GLuint,
        stream: GLuint,
        instancecount: GLsizei
    );
    func!(Enable, (), cap: GLenum);
    func!(EnableVertexArrayAttrib, (), vaobj: GLuint, index: GLuint);
    func!(EnableVertexAttribArray, (), index: GLuint);
    func!(Enablei, (), target: GLenum, index: GLuint);
    func!(EndConditionalRender, (),);
    func!(EndQuery, (), target: GLenum);
    func!(EndQueryIndexed, (), target: GLenum, index: GLuint);
    func!(EndTransformFeedback, (),);
    func!(FenceSync, GLsync, condition: GLenum, flags: GLbitfield);
    func!(Finish, (),);
    func!(Flush, (),);
    func!(
        FlushMappedBufferRange,
        (),
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr
    );
    func!(
        FlushMappedNamedBufferRange,
        (),
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr
    );
    func!(
        FramebufferParameteri,
        (),
        target: GLenum,
        pname: GLenum,
        param: GLint
    );
    func!(
        FramebufferRenderbuffer,
        (),
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint
    );
    func!(
        FramebufferTexture,
        (),
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint
    );
    func!(
        FramebufferTexture1D,
        (),
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint
    );
    func!(
        FramebufferTexture2D,
        (),
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint
    );
    func!(
        FramebufferTexture3D,
        (),
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
        zoffset: GLint
    );
    func!(
        FramebufferTextureLayer,
        (),
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint
    );
    func!(FrontFace, (), mode: GLenum);
    func!(GenBuffers, (), n: GLsizei, buffers: *mut GLuint);
    func!(GenFramebuffers, (), n: GLsizei, framebuffers: *mut GLuint);
    func!(GenProgramPipelines, (), n: GLsizei, pipelines: *mut GLuint);
    func!(GenQueries, (), n: GLsizei, ids: *mut GLuint);
    func!(GenRenderbuffers, (), n: GLsizei, renderbuffers: *mut GLuint);
    func!(GenSamplers, (), count: GLsizei, samplers: *mut GLuint);
    func!(GenTextures, (), n: GLsizei, textures: *mut GLuint);
    func!(GenTransformFeedbacks, (), n: GLsizei, ids: *mut GLuint);
    func!(GenVertexArrays, (), n: GLsizei, arrays: *mut GLuint);
    func!(GenerateMipmap, (), target: GLenum);
    func!(GenerateTextureMipmap, (), texture: GLuint);
    func!(
        GetActiveAtomicCounterBufferiv,
        (),
        program: GLuint,
        bufferIndex: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetActiveAttrib,
        (),
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar
    );
    func!(
        GetActiveSubroutineName,
        (),
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar
    );
    func!(
        GetActiveSubroutineUniformName,
        (),
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar
    );
    func!(
        GetActiveSubroutineUniformiv,
        (),
        program: GLuint,
        shadertype: GLenum,
        index: GLuint,
        pname: GLenum,
        values: *mut GLint
    );
    func!(
        GetActiveUniform,
        (),
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar
    );
    func!(
        GetActiveUniformBlockName,
        (),
        program: GLuint,
        uniformBlockIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformBlockName: *mut GLchar
    );
    func!(
        GetActiveUniformBlockiv,
        (),
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetActiveUniformName,
        (),
        program: GLuint,
        uniformIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformName: *mut GLchar
    );
    func!(
        GetActiveUniformsiv,
        (),
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetAttachedShaders,
        (),
        program: GLuint,
        maxCount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint
    );
    func!(
        GetAttribLocation,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetBooleani_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLboolean
    );
    func!(GetBooleanv, (), pname: GLenum, data: *mut GLboolean);
    func!(
        GetBufferParameteri64v,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint64
    );
    func!(
        GetBufferParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetBufferPointerv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut *mut c_void
    );
    func!(
        GetBufferSubData,
        (),
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut c_void
    );
    func!(
        GetCompressedTexImage,
        (),
        target: GLenum,
        level: GLint,
        img: *mut c_void
    );
    func!(
        GetCompressedTextureImage,
        (),
        texture: GLuint,
        level: GLint,
        bufSize: GLsizei,
        pixels: *mut c_void
    );
    func!(
        GetCompressedTextureSubImage,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        bufSize: GLsizei,
        pixels: *mut c_void
    );
    func!(
        GetDebugMessageLog,
        GLuint,
        count: GLuint,
        bufSize: GLsizei,
        sources: *mut GLenum,
        types: *mut GLenum,
        ids: *mut GLuint,
        severities: *mut GLenum,
        lengths: *mut GLsizei,
        messageLog: *mut GLchar
    );
    func!(
        GetDoublei_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLdouble
    );
    func!(GetDoublev, (), pname: GLenum, data: *mut GLdouble);
    func!(GetError, GLenum,);
    func!(
        GetFloati_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLfloat
    );
    func!(GetFloatv, (), pname: GLenum, data: *mut GLfloat);
    func!(
        GetFragDataIndex,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetFragDataLocation,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetFramebufferAttachmentParameteriv,
        (),
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetFramebufferParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(GetGraphicsResetStatus, GLenum,);
    func!(
        GetInteger64i_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLint64
    );
    func!(GetInteger64v, (), pname: GLenum, data: *mut GLint64);
    func!(
        GetIntegeri_v,
        (),
        target: GLenum,
        index: GLuint,
        data: *mut GLint
    );
    func!(GetIntegerv, (), pname: GLenum, data: *mut GLint);
    func!(
        GetInternalformati64v,
        (),
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        count: GLsizei,
        params: *mut GLint64
    );
    func!(
        GetInternalformativ,
        (),
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        count: GLsizei,
        params: *mut GLint
    );
    func!(
        GetMultisamplefv,
        (),
        pname: GLenum,
        index: GLuint,
        val: *mut GLfloat
    );
    func!(
        GetNamedBufferParameteri64v,
        (),
        buffer: GLuint,
        pname: GLenum,
        params: *mut GLint64
    );
    func!(
        GetNamedBufferParameteriv,
        (),
        buffer: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetNamedBufferPointerv,
        (),
        buffer: GLuint,
        pname: GLenum,
        params: *mut *mut c_void
    );
    func!(
        GetNamedBufferSubData,
        (),
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *mut c_void
    );
    func!(
        GetNamedFramebufferAttachmentParameteriv,
        (),
        framebuffer: GLuint,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetNamedFramebufferParameteriv,
        (),
        framebuffer: GLuint,
        pname: GLenum,
        param: *mut GLint
    );
    func!(
        GetNamedRenderbufferParameteriv,
        (),
        renderbuffer: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetObjectLabel,
        (),
        identifier: GLenum,
        name: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar
    );
    func!(
        GetObjectPtrLabel,
        (),
        ptr: *const c_void,
        bufSize: GLsizei,
        length: *mut GLsizei,
        label: *mut GLchar
    );
    func!(GetPointerv, (), pname: GLenum, params: *mut *mut c_void);
    func!(
        GetProgramBinary,
        (),
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        binaryFormat: *mut GLenum,
        binary: *mut c_void
    );
    func!(
        GetProgramInfoLog,
        (),
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    );
    func!(
        GetProgramInterfaceiv,
        (),
        program: GLuint,
        programInterface: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetProgramPipelineInfoLog,
        (),
        pipeline: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    );
    func!(
        GetProgramPipelineiv,
        (),
        pipeline: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetProgramResourceIndex,
        GLuint,
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar
    );
    func!(
        GetProgramResourceLocation,
        GLint,
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar
    );
    func!(
        GetProgramResourceLocationIndex,
        GLint,
        program: GLuint,
        programInterface: GLenum,
        name: *const GLchar
    );
    func!(
        GetProgramResourceName,
        (),
        program: GLuint,
        programInterface: GLenum,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar
    );
    func!(
        GetProgramResourceiv,
        (),
        program: GLuint,
        programInterface: GLenum,
        index: GLuint,
        propCount: GLsizei,
        props: *const GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint
    );
    func!(
        GetProgramStageiv,
        (),
        program: GLuint,
        shadertype: GLenum,
        pname: GLenum,
        values: *mut GLint
    );
    func!(
        GetProgramiv,
        (),
        program: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetQueryBufferObjecti64v,
        (),
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr
    );
    func!(
        GetQueryBufferObjectiv,
        (),
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr
    );
    func!(
        GetQueryBufferObjectui64v,
        (),
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr
    );
    func!(
        GetQueryBufferObjectuiv,
        (),
        id: GLuint,
        buffer: GLuint,
        pname: GLenum,
        offset: GLintptr
    );
    func!(
        GetQueryIndexediv,
        (),
        target: GLenum,
        index: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetQueryObjecti64v,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLint64
    );
    func!(
        GetQueryObjectiv,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetQueryObjectui64v,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLuint64
    );
    func!(
        GetQueryObjectuiv,
        (),
        id: GLuint,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetQueryiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetRenderbufferParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetSamplerParameterIiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetSamplerParameterIuiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetSamplerParameterfv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetSamplerParameteriv,
        (),
        sampler: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetShaderInfoLog,
        (),
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar
    );
    func!(
        GetShaderPrecisionFormat,
        (),
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint
    );
    func!(
        GetShaderSource,
        (),
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar
    );
    func!(
        GetShaderiv,
        (),
        shader: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(GetString, *const GLubyte, name: GLenum);
    func!(GetStringi, *const GLubyte, name: GLenum, index: GLuint);
    func!(
        GetSubroutineIndex,
        GLuint,
        program: GLuint,
        shadertype: GLenum,
        name: *const GLchar
    );
    func!(
        GetSubroutineUniformLocation,
        GLint,
        program: GLuint,
        shadertype: GLenum,
        name: *const GLchar
    );
    func!(
        GetSynciv,
        (),
        sync: GLsync,
        pname: GLenum,
        count: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint
    );
    func!(
        GetTexImage,
        (),
        target: GLenum,
        level: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *mut c_void
    );
    func!(
        GetTexLevelParameterfv,
        (),
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetTexLevelParameteriv,
        (),
        target: GLenum,
        level: GLint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTexParameterIiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTexParameterIuiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetTexParameterfv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetTexParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTextureImage,
        (),
        texture: GLuint,
        level: GLint,
        format: GLenum,
        type_: GLenum,
        bufSize: GLsizei,
        pixels: *mut c_void
    );
    func!(
        GetTextureLevelParameterfv,
        (),
        texture: GLuint,
        level: GLint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetTextureLevelParameteriv,
        (),
        texture: GLuint,
        level: GLint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTextureParameterIiv,
        (),
        texture: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTextureParameterIuiv,
        (),
        texture: GLuint,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetTextureParameterfv,
        (),
        texture: GLuint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetTextureParameteriv,
        (),
        texture: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetTextureSubImage,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        bufSize: GLsizei,
        pixels: *mut c_void
    );
    func!(
        GetTransformFeedbackVarying,
        (),
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar
    );
    func!(
        GetTransformFeedbacki64_v,
        (),
        xfb: GLuint,
        pname: GLenum,
        index: GLuint,
        param: *mut GLint64
    );
    func!(
        GetTransformFeedbacki_v,
        (),
        xfb: GLuint,
        pname: GLenum,
        index: GLuint,
        param: *mut GLint
    );
    func!(
        GetTransformFeedbackiv,
        (),
        xfb: GLuint,
        pname: GLenum,
        param: *mut GLint
    );
    func!(
        GetUniformBlockIndex,
        GLuint,
        program: GLuint,
        uniformBlockName: *const GLchar
    );
    func!(
        GetUniformIndices,
        (),
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint
    );
    func!(
        GetUniformLocation,
        GLint,
        program: GLuint,
        name: *const GLchar
    );
    func!(
        GetUniformSubroutineuiv,
        (),
        shadertype: GLenum,
        location: GLint,
        params: *mut GLuint
    );
    func!(
        GetUniformdv,
        (),
        program: GLuint,
        location: GLint,
        params: *mut GLdouble
    );
    func!(
        GetUniformfv,
        (),
        program: GLuint,
        location: GLint,
        params: *mut GLfloat
    );
    func!(
        GetUniformiv,
        (),
        program: GLuint,
        location: GLint,
        params: *mut GLint
    );
    func!(
        GetUniformuiv,
        (),
        program: GLuint,
        location: GLint,
        params: *mut GLuint
    );
    func!(
        GetVertexArrayIndexed64iv,
        (),
        vaobj: GLuint,
        index: GLuint,
        pname: GLenum,
        param: *mut GLint64
    );
    func!(
        GetVertexArrayIndexediv,
        (),
        vaobj: GLuint,
        index: GLuint,
        pname: GLenum,
        param: *mut GLint
    );
    func!(
        GetVertexArrayiv,
        (),
        vaobj: GLuint,
        pname: GLenum,
        param: *mut GLint
    );
    func!(
        GetVertexAttribIiv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetVertexAttribIuiv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLuint
    );
    func!(
        GetVertexAttribLdv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLdouble
    );
    func!(
        GetVertexAttribPointerv,
        (),
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut c_void
    );
    func!(
        GetVertexAttribdv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLdouble
    );
    func!(
        GetVertexAttribfv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLfloat
    );
    func!(
        GetVertexAttribiv,
        (),
        index: GLuint,
        pname: GLenum,
        params: *mut GLint
    );
    func!(
        GetnCompressedTexImage,
        (),
        target: GLenum,
        lod: GLint,
        bufSize: GLsizei,
        pixels: *mut c_void
    );
    func!(
        GetnTexImage,
        (),
        target: GLenum,
        level: GLint,
        format: GLenum,
        type_: GLenum,
        bufSize: GLsizei,
        pixels: *mut c_void
    );
    func!(
        GetnUniformdv,
        (),
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLdouble
    );
    func!(
        GetnUniformfv,
        (),
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLfloat
    );
    func!(
        GetnUniformiv,
        (),
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLint
    );
    func!(
        GetnUniformuiv,
        (),
        program: GLuint,
        location: GLint,
        bufSize: GLsizei,
        params: *mut GLuint
    );
    func!(Hint, (), target: GLenum, mode: GLenum);
    func!(InvalidateBufferData, (), buffer: GLuint);
    func!(
        InvalidateBufferSubData,
        (),
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr
    );
    func!(
        InvalidateFramebuffer,
        (),
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum
    );
    func!(
        InvalidateNamedFramebufferData,
        (),
        framebuffer: GLuint,
        numAttachments: GLsizei,
        attachments: *const GLenum
    );
    func!(
        InvalidateNamedFramebufferSubData,
        (),
        framebuffer: GLuint,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        InvalidateSubFramebuffer,
        (),
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(InvalidateTexImage, (), texture: GLuint, level: GLint);
    func!(
        InvalidateTexSubImage,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei
    );
    func!(IsBuffer, GLboolean, buffer: GLuint);
    func!(IsEnabled, GLboolean, cap: GLenum);
    func!(IsEnabledi, GLboolean, target: GLenum, index: GLuint);
    func!(IsFramebuffer, GLboolean, framebuffer: GLuint);
    func!(IsProgram, GLboolean, program: GLuint);
    func!(IsProgramPipeline, GLboolean, pipeline: GLuint);
    func!(IsQuery, GLboolean, id: GLuint);
    func!(IsRenderbuffer, GLboolean, renderbuffer: GLuint);
    func!(IsSampler, GLboolean, sampler: GLuint);
    func!(IsShader, GLboolean, shader: GLuint);
    func!(IsSync, GLboolean, sync: GLsync);
    func!(IsTexture, GLboolean, texture: GLuint);
    func!(IsTransformFeedback, GLboolean, id: GLuint);
    func!(IsVertexArray, GLboolean, array: GLuint);
    func!(LineWidth, (), width: GLfloat);
    func!(LinkProgram, (), program: GLuint);
    func!(LogicOp, (), opcode: GLenum);
    func!(MapBuffer, *mut c_void, target: GLenum, access: GLenum);
    func!(
        MapBufferRange,
        *mut c_void,
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield
    );
    func!(MapNamedBuffer, *mut c_void, buffer: GLuint, access: GLenum);
    func!(
        MapNamedBufferRange,
        *mut c_void,
        buffer: GLuint,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield
    );
    func!(MemoryBarrier, (), barriers: GLbitfield);
    func!(MemoryBarrierByRegion, (), barriers: GLbitfield);
    func!(MinSampleShading, (), value: GLfloat);
    func!(
        MultiDrawArrays,
        (),
        mode: GLenum,
        first: *const GLint,
        count: *const GLsizei,
        drawcount: GLsizei
    );
    func!(
        MultiDrawArraysIndirect,
        (),
        mode: GLenum,
        indirect: *const c_void,
        drawcount: GLsizei,
        stride: GLsizei
    );
    func!(
        MultiDrawArraysIndirectCount,
        (),
        mode: GLenum,
        indirect: *const c_void,
        drawcount: GLintptr,
        maxdrawcount: GLsizei,
        stride: GLsizei
    );
    func!(
        MultiDrawElements,
        (),
        mode: GLenum,
        count: *const GLsizei,
        type_: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei
    );
    func!(
        MultiDrawElementsBaseVertex,
        (),
        mode: GLenum,
        count: *const GLsizei,
        type_: GLenum,
        indices: *const *const c_void,
        drawcount: GLsizei,
        basevertex: *const GLint
    );
    func!(
        MultiDrawElementsIndirect,
        (),
        mode: GLenum,
        type_: GLenum,
        indirect: *const c_void,
        drawcount: GLsizei,
        stride: GLsizei
    );
    func!(
        MultiDrawElementsIndirectCount,
        (),
        mode: GLenum,
        type_: GLenum,
        indirect: *const c_void,
        drawcount: GLintptr,
        maxdrawcount: GLsizei,
        stride: GLsizei
    );
    func!(
        NamedBufferData,
        (),
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const c_void,
        usage: GLenum
    );
    func!(
        NamedBufferStorage,
        (),
        buffer: GLuint,
        size: GLsizeiptr,
        data: *const c_void,
        flags: GLbitfield
    );
    func!(
        NamedBufferSubData,
        (),
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const c_void
    );
    func!(
        NamedFramebufferDrawBuffer,
        (),
        framebuffer: GLuint,
        buf: GLenum
    );
    func!(
        NamedFramebufferDrawBuffers,
        (),
        framebuffer: GLuint,
        n: GLsizei,
        bufs: *const GLenum
    );
    func!(
        NamedFramebufferParameteri,
        (),
        framebuffer: GLuint,
        pname: GLenum,
        param: GLint
    );
    func!(
        NamedFramebufferReadBuffer,
        (),
        framebuffer: GLuint,
        src: GLenum
    );
    func!(
        NamedFramebufferRenderbuffer,
        (),
        framebuffer: GLuint,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint
    );
    func!(
        NamedFramebufferTexture,
        (),
        framebuffer: GLuint,
        attachment: GLenum,
        texture: GLuint,
        level: GLint
    );
    func!(
        NamedFramebufferTextureLayer,
        (),
        framebuffer: GLuint,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint
    );
    func!(
        NamedRenderbufferStorage,
        (),
        renderbuffer: GLuint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        NamedRenderbufferStorageMultisample,
        (),
        renderbuffer: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        ObjectLabel,
        (),
        identifier: GLenum,
        name: GLuint,
        length: GLsizei,
        label: *const GLchar
    );
    func!(
        ObjectPtrLabel,
        (),
        ptr: *const c_void,
        length: GLsizei,
        label: *const GLchar
    );
    func!(PatchParameterfv, (), pname: GLenum, values: *const GLfloat);
    func!(PatchParameteri, (), pname: GLenum, value: GLint);
    func!(PauseTransformFeedback, (),);
    func!(PixelStoref, (), pname: GLenum, param: GLfloat);
    func!(PixelStorei, (), pname: GLenum, param: GLint);
    func!(PointParameterf, (), pname: GLenum, param: GLfloat);
    func!(PointParameterfv, (), pname: GLenum, params: *const GLfloat);
    func!(PointParameteri, (), pname: GLenum, param: GLint);
    func!(PointParameteriv, (), pname: GLenum, params: *const GLint);
    func!(PointSize, (), size: GLfloat);
    func!(PolygonMode, (), face: GLenum, mode: GLenum);
    func!(PolygonOffset, (), factor: GLfloat, units: GLfloat);
    func!(
        PolygonOffsetClamp,
        (),
        factor: GLfloat,
        units: GLfloat,
        clamp: GLfloat
    );
    func!(PopDebugGroup, (),);
    func!(PrimitiveRestartIndex, (), index: GLuint);
    func!(
        ProgramBinary,
        (),
        program: GLuint,
        binaryFormat: GLenum,
        binary: *const c_void,
        length: GLsizei
    );
    func!(
        ProgramParameteri,
        (),
        program: GLuint,
        pname: GLenum,
        value: GLint
    );
    func!(
        ProgramUniform1d,
        (),
        program: GLuint,
        location: GLint,
        v0: GLdouble
    );
    func!(
        ProgramUniform1dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(
        ProgramUniform1f,
        (),
        program: GLuint,
        location: GLint,
        v0: GLfloat
    );
    func!(
        ProgramUniform1fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        ProgramUniform1i,
        (),
        program: GLuint,
        location: GLint,
        v0: GLint
    );
    func!(
        ProgramUniform1iv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        ProgramUniform1ui,
        (),
        program: GLuint,
        location: GLint,
        v0: GLuint
    );
    func!(
        ProgramUniform1uiv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        ProgramUniform2d,
        (),
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble
    );
    func!(
        ProgramUniform2dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(
        ProgramUniform2f,
        (),
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat
    );
    func!(
        ProgramUniform2fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        ProgramUniform2i,
        (),
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint
    );
    func!(
        ProgramUniform2iv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        ProgramUniform2ui,
        (),
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint
    );
    func!(
        ProgramUniform2uiv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        ProgramUniform3d,
        (),
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
        v2: GLdouble
    );
    func!(
        ProgramUniform3dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(
        ProgramUniform3f,
        (),
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat
    );
    func!(
        ProgramUniform3fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        ProgramUniform3i,
        (),
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint
    );
    func!(
        ProgramUniform3iv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        ProgramUniform3ui,
        (),
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint
    );
    func!(
        ProgramUniform3uiv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        ProgramUniform4d,
        (),
        program: GLuint,
        location: GLint,
        v0: GLdouble,
        v1: GLdouble,
        v2: GLdouble,
        v3: GLdouble
    );
    func!(
        ProgramUniform4dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(
        ProgramUniform4f,
        (),
        program: GLuint,
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat
    );
    func!(
        ProgramUniform4fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        ProgramUniform4i,
        (),
        program: GLuint,
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint
    );
    func!(
        ProgramUniform4iv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        ProgramUniform4ui,
        (),
        program: GLuint,
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint
    );
    func!(
        ProgramUniform4uiv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        ProgramUniformMatrix2dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix2fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix2x3dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix2x3fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix2x4dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix2x4fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix3dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix3fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix3x2dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix3x2fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix3x4dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix3x4fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix4dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix4fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix4x2dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix4x2fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        ProgramUniformMatrix4x3dv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        ProgramUniformMatrix4x3fv,
        (),
        program: GLuint,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(ProvokingVertex, (), mode: GLenum);
    func!(
        PushDebugGroup,
        (),
        source: GLenum,
        id: GLuint,
        length: GLsizei,
        message: *const GLchar
    );
    func!(QueryCounter, (), id: GLuint, target: GLenum);
    func!(ReadBuffer, (), src: GLenum);
    func!(
        ReadPixels,
        (),
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut c_void
    );
    func!(
        ReadnPixels,
        (),
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        bufSize: GLsizei,
        data: *mut c_void
    );
    func!(ReleaseShaderCompiler, (),);
    func!(
        RenderbufferStorage,
        (),
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        RenderbufferStorageMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(ResumeTransformFeedback, (),);
    func!(SampleCoverage, (), value: GLfloat, invert: GLboolean);
    func!(SampleMaski, (), maskNumber: GLuint, mask: GLbitfield);
    func!(
        SamplerParameterIiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint
    );
    func!(
        SamplerParameterIuiv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLuint
    );
    func!(
        SamplerParameterf,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: GLfloat
    );
    func!(
        SamplerParameterfv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLfloat
    );
    func!(
        SamplerParameteri,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: GLint
    );
    func!(
        SamplerParameteriv,
        (),
        sampler: GLuint,
        pname: GLenum,
        param: *const GLint
    );
    func!(
        Scissor,
        (),
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        ScissorArrayv,
        (),
        first: GLuint,
        count: GLsizei,
        v: *const GLint
    );
    func!(
        ScissorIndexed,
        (),
        index: GLuint,
        left: GLint,
        bottom: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(ScissorIndexedv, (), index: GLuint, v: *const GLint);
    func!(
        ShaderBinary,
        (),
        count: GLsizei,
        shaders: *const GLuint,
        binaryFormat: GLenum,
        binary: *const c_void,
        length: GLsizei
    );
    func!(
        ShaderSource,
        (),
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint
    );
    func!(
        ShaderStorageBlockBinding,
        (),
        program: GLuint,
        storageBlockIndex: GLuint,
        storageBlockBinding: GLuint
    );
    func!(
        SpecializeShader,
        (),
        shader: GLuint,
        pEntryPoint: *const GLchar,
        numSpecializationConstants: GLuint,
        pConstantIndex: *const GLuint,
        pConstantValue: *const GLuint
    );
    func!(StencilFunc, (), func: GLenum, ref_: GLint, mask: GLuint);
    func!(
        StencilFuncSeparate,
        (),
        face: GLenum,
        func: GLenum,
        ref_: GLint,
        mask: GLuint
    );
    func!(StencilMask, (), mask: GLuint);
    func!(StencilMaskSeparate, (), face: GLenum, mask: GLuint);
    func!(StencilOp, (), fail: GLenum, zfail: GLenum, zpass: GLenum);
    func!(
        StencilOpSeparate,
        (),
        face: GLenum,
        sfail: GLenum,
        dpfail: GLenum,
        dppass: GLenum
    );
    func!(
        TexBuffer,
        (),
        target: GLenum,
        internalformat: GLenum,
        buffer: GLuint
    );
    func!(
        TexBufferRange,
        (),
        target: GLenum,
        internalformat: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        TexImage1D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexImage2D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexImage2DMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TexImage3D,
        (),
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexImage3DMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TexParameterIiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLint
    );
    func!(
        TexParameterIuiv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLuint
    );
    func!(
        TexParameterf,
        (),
        target: GLenum,
        pname: GLenum,
        param: GLfloat
    );
    func!(
        TexParameterfv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLfloat
    );
    func!(
        TexParameteri,
        (),
        target: GLenum,
        pname: GLenum,
        param: GLint
    );
    func!(
        TexParameteriv,
        (),
        target: GLenum,
        pname: GLenum,
        params: *const GLint
    );
    func!(
        TexStorage1D,
        (),
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei
    );
    func!(
        TexStorage2D,
        (),
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        TexStorage2DMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TexStorage3D,
        (),
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei
    );
    func!(
        TexStorage3DMultisample,
        (),
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TexSubImage1D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexSubImage2D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TexSubImage3D,
        (),
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(TextureBarrier, (),);
    func!(
        TextureBuffer,
        (),
        texture: GLuint,
        internalformat: GLenum,
        buffer: GLuint
    );
    func!(
        TextureBufferRange,
        (),
        texture: GLuint,
        internalformat: GLenum,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        TextureParameterIiv,
        (),
        texture: GLuint,
        pname: GLenum,
        params: *const GLint
    );
    func!(
        TextureParameterIuiv,
        (),
        texture: GLuint,
        pname: GLenum,
        params: *const GLuint
    );
    func!(
        TextureParameterf,
        (),
        texture: GLuint,
        pname: GLenum,
        param: GLfloat
    );
    func!(
        TextureParameterfv,
        (),
        texture: GLuint,
        pname: GLenum,
        param: *const GLfloat
    );
    func!(
        TextureParameteri,
        (),
        texture: GLuint,
        pname: GLenum,
        param: GLint
    );
    func!(
        TextureParameteriv,
        (),
        texture: GLuint,
        pname: GLenum,
        param: *const GLint
    );
    func!(
        TextureStorage1D,
        (),
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei
    );
    func!(
        TextureStorage2D,
        (),
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        TextureStorage2DMultisample,
        (),
        texture: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TextureStorage3D,
        (),
        texture: GLuint,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei
    );
    func!(
        TextureStorage3DMultisample,
        (),
        texture: GLuint,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        fixedsamplelocations: GLboolean
    );
    func!(
        TextureSubImage1D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        width: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TextureSubImage2D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TextureSubImage3D,
        (),
        texture: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const c_void
    );
    func!(
        TextureView,
        (),
        texture: GLuint,
        target: GLenum,
        origtexture: GLuint,
        internalformat: GLenum,
        minlevel: GLuint,
        numlevels: GLuint,
        minlayer: GLuint,
        numlayers: GLuint
    );
    func!(
        TransformFeedbackBufferBase,
        (),
        xfb: GLuint,
        index: GLuint,
        buffer: GLuint
    );
    func!(
        TransformFeedbackBufferRange,
        (),
        xfb: GLuint,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr
    );
    func!(
        TransformFeedbackVaryings,
        (),
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum
    );
    func!(Uniform1d, (), location: GLint, x: GLdouble);
    func!(
        Uniform1dv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(Uniform1f, (), location: GLint, v0: GLfloat);
    func!(
        Uniform1fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(Uniform1i, (), location: GLint, v0: GLint);
    func!(
        Uniform1iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(Uniform1ui, (), location: GLint, v0: GLuint);
    func!(
        Uniform1uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(Uniform2d, (), location: GLint, x: GLdouble, y: GLdouble);
    func!(
        Uniform2dv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(Uniform2f, (), location: GLint, v0: GLfloat, v1: GLfloat);
    func!(
        Uniform2fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(Uniform2i, (), location: GLint, v0: GLint, v1: GLint);
    func!(
        Uniform2iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(Uniform2ui, (), location: GLint, v0: GLuint, v1: GLuint);
    func!(
        Uniform2uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        Uniform3d,
        (),
        location: GLint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble
    );
    func!(
        Uniform3dv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(
        Uniform3f,
        (),
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat
    );
    func!(
        Uniform3fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        Uniform3i,
        (),
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint
    );
    func!(
        Uniform3iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        Uniform3ui,
        (),
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint
    );
    func!(
        Uniform3uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        Uniform4d,
        (),
        location: GLint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble
    );
    func!(
        Uniform4dv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLdouble
    );
    func!(
        Uniform4f,
        (),
        location: GLint,
        v0: GLfloat,
        v1: GLfloat,
        v2: GLfloat,
        v3: GLfloat
    );
    func!(
        Uniform4fv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLfloat
    );
    func!(
        Uniform4i,
        (),
        location: GLint,
        v0: GLint,
        v1: GLint,
        v2: GLint,
        v3: GLint
    );
    func!(
        Uniform4iv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLint
    );
    func!(
        Uniform4ui,
        (),
        location: GLint,
        v0: GLuint,
        v1: GLuint,
        v2: GLuint,
        v3: GLuint
    );
    func!(
        Uniform4uiv,
        (),
        location: GLint,
        count: GLsizei,
        value: *const GLuint
    );
    func!(
        UniformBlockBinding,
        (),
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint
    );
    func!(
        UniformMatrix2dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix2fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix2x3dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix2x3fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix2x4dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix2x4fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix3dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix3fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix3x2dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix3x2fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix3x4dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix3x4fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix4dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix4fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix4x2dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix4x2fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformMatrix4x3dv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLdouble
    );
    func!(
        UniformMatrix4x3fv,
        (),
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat
    );
    func!(
        UniformSubroutinesuiv,
        (),
        shadertype: GLenum,
        count: GLsizei,
        indices: *const GLuint
    );
    func!(UnmapBuffer, GLboolean, target: GLenum);
    func!(UnmapNamedBuffer, GLboolean, buffer: GLuint);
    func!(UseProgram, (), program: GLuint);
    func!(
        UseProgramStages,
        (),
        pipeline: GLuint,
        stages: GLbitfield,
        program: GLuint
    );
    func!(ValidateProgram, (), program: GLuint);
    func!(ValidateProgramPipeline, (), pipeline: GLuint);
    func!(
        VertexArrayAttribBinding,
        (),
        vaobj: GLuint,
        attribindex: GLuint,
        bindingindex: GLuint
    );
    func!(
        VertexArrayAttribFormat,
        (),
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint
    );
    func!(
        VertexArrayAttribIFormat,
        (),
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        relativeoffset: GLuint
    );
    func!(
        VertexArrayAttribLFormat,
        (),
        vaobj: GLuint,
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        relativeoffset: GLuint
    );
    func!(
        VertexArrayBindingDivisor,
        (),
        vaobj: GLuint,
        bindingindex: GLuint,
        divisor: GLuint
    );
    func!(VertexArrayElementBuffer, (), vaobj: GLuint, buffer: GLuint);
    func!(
        VertexArrayVertexBuffer,
        (),
        vaobj: GLuint,
        bindingindex: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        stride: GLsizei
    );
    func!(
        VertexArrayVertexBuffers,
        (),
        vaobj: GLuint,
        first: GLuint,
        count: GLsizei,
        buffers: *const GLuint,
        offsets: *const GLintptr,
        strides: *const GLsizei
    );
    func!(VertexAttrib1d, (), index: GLuint, x: GLdouble);
    func!(VertexAttrib1dv, (), index: GLuint, v: *const GLdouble);
    func!(VertexAttrib1f, (), index: GLuint, x: GLfloat);
    func!(VertexAttrib1fv, (), index: GLuint, v: *const GLfloat);
    func!(VertexAttrib1s, (), index: GLuint, x: GLshort);
    func!(VertexAttrib1sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttrib2d, (), index: GLuint, x: GLdouble, y: GLdouble);
    func!(VertexAttrib2dv, (), index: GLuint, v: *const GLdouble);
    func!(VertexAttrib2f, (), index: GLuint, x: GLfloat, y: GLfloat);
    func!(VertexAttrib2fv, (), index: GLuint, v: *const GLfloat);
    func!(VertexAttrib2s, (), index: GLuint, x: GLshort, y: GLshort);
    func!(VertexAttrib2sv, (), index: GLuint, v: *const GLshort);
    func!(
        VertexAttrib3d,
        (),
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble
    );
    func!(VertexAttrib3dv, (), index: GLuint, v: *const GLdouble);
    func!(
        VertexAttrib3f,
        (),
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat
    );
    func!(VertexAttrib3fv, (), index: GLuint, v: *const GLfloat);
    func!(
        VertexAttrib3s,
        (),
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort
    );
    func!(VertexAttrib3sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttrib4Nbv, (), index: GLuint, v: *const GLbyte);
    func!(VertexAttrib4Niv, (), index: GLuint, v: *const GLint);
    func!(VertexAttrib4Nsv, (), index: GLuint, v: *const GLshort);
    func!(
        VertexAttrib4Nub,
        (),
        index: GLuint,
        x: GLubyte,
        y: GLubyte,
        z: GLubyte,
        w: GLubyte
    );
    func!(VertexAttrib4Nubv, (), index: GLuint, v: *const GLubyte);
    func!(VertexAttrib4Nuiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttrib4Nusv, (), index: GLuint, v: *const GLushort);
    func!(VertexAttrib4bv, (), index: GLuint, v: *const GLbyte);
    func!(
        VertexAttrib4d,
        (),
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble
    );
    func!(VertexAttrib4dv, (), index: GLuint, v: *const GLdouble);
    func!(
        VertexAttrib4f,
        (),
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        z: GLfloat,
        w: GLfloat
    );
    func!(VertexAttrib4fv, (), index: GLuint, v: *const GLfloat);
    func!(VertexAttrib4iv, (), index: GLuint, v: *const GLint);
    func!(
        VertexAttrib4s,
        (),
        index: GLuint,
        x: GLshort,
        y: GLshort,
        z: GLshort,
        w: GLshort
    );
    func!(VertexAttrib4sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttrib4ubv, (), index: GLuint, v: *const GLubyte);
    func!(VertexAttrib4uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttrib4usv, (), index: GLuint, v: *const GLushort);
    func!(
        VertexAttribBinding,
        (),
        attribindex: GLuint,
        bindingindex: GLuint
    );
    func!(VertexAttribDivisor, (), index: GLuint, divisor: GLuint);
    func!(
        VertexAttribFormat,
        (),
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        relativeoffset: GLuint
    );
    func!(VertexAttribI1i, (), index: GLuint, x: GLint);
    func!(VertexAttribI1iv, (), index: GLuint, v: *const GLint);
    func!(VertexAttribI1ui, (), index: GLuint, x: GLuint);
    func!(VertexAttribI1uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttribI2i, (), index: GLuint, x: GLint, y: GLint);
    func!(VertexAttribI2iv, (), index: GLuint, v: *const GLint);
    func!(VertexAttribI2ui, (), index: GLuint, x: GLuint, y: GLuint);
    func!(VertexAttribI2uiv, (), index: GLuint, v: *const GLuint);
    func!(
        VertexAttribI3i,
        (),
        index: GLuint,
        x: GLint,
        y: GLint,
        z: GLint
    );
    func!(VertexAttribI3iv, (), index: GLuint, v: *const GLint);
    func!(
        VertexAttribI3ui,
        (),
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint
    );
    func!(VertexAttribI3uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttribI4bv, (), index: GLuint, v: *const GLbyte);
    func!(
        VertexAttribI4i,
        (),
        index: GLuint,
        x: GLint,
        y: GLint,
        z: GLint,
        w: GLint
    );
    func!(VertexAttribI4iv, (), index: GLuint, v: *const GLint);
    func!(VertexAttribI4sv, (), index: GLuint, v: *const GLshort);
    func!(VertexAttribI4ubv, (), index: GLuint, v: *const GLubyte);
    func!(
        VertexAttribI4ui,
        (),
        index: GLuint,
        x: GLuint,
        y: GLuint,
        z: GLuint,
        w: GLuint
    );
    func!(VertexAttribI4uiv, (), index: GLuint, v: *const GLuint);
    func!(VertexAttribI4usv, (), index: GLuint, v: *const GLushort);
    func!(
        VertexAttribIFormat,
        (),
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        relativeoffset: GLuint
    );
    func!(
        VertexAttribIPointer,
        (),
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const c_void
    );
    func!(VertexAttribL1d, (), index: GLuint, x: GLdouble);
    func!(VertexAttribL1dv, (), index: GLuint, v: *const GLdouble);
    func!(VertexAttribL2d, (), index: GLuint, x: GLdouble, y: GLdouble);
    func!(VertexAttribL2dv, (), index: GLuint, v: *const GLdouble);
    func!(
        VertexAttribL3d,
        (),
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble
    );
    func!(VertexAttribL3dv, (), index: GLuint, v: *const GLdouble);
    func!(
        VertexAttribL4d,
        (),
        index: GLuint,
        x: GLdouble,
        y: GLdouble,
        z: GLdouble,
        w: GLdouble
    );
    func!(VertexAttribL4dv, (), index: GLuint, v: *const GLdouble);
    func!(
        VertexAttribLFormat,
        (),
        attribindex: GLuint,
        size: GLint,
        type_: GLenum,
        relativeoffset: GLuint
    );
    func!(
        VertexAttribLPointer,
        (),
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const c_void
    );
    func!(
        VertexAttribP1ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP1uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribP2ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP2uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribP3ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP3uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribP4ui,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: GLuint
    );
    func!(
        VertexAttribP4uiv,
        (),
        index: GLuint,
        type_: GLenum,
        normalized: GLboolean,
        value: *const GLuint
    );
    func!(
        VertexAttribPointer,
        (),
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void
    );
    func!(
        VertexBindingDivisor,
        (),
        bindingindex: GLuint,
        divisor: GLuint
    );
    func!(
        Viewport,
        (),
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei
    );
    func!(
        ViewportArrayv,
        (),
        first: GLuint,
        count: GLsizei,
        v: *const GLfloat
    );
    func!(
        ViewportIndexedf,
        (),
        index: GLuint,
        x: GLfloat,
        y: GLfloat,
        w: GLfloat,
        h: GLfloat
    );
    func!(ViewportIndexedfv, (), index: GLuint, v: *const GLfloat);
    func!(
        WaitSync,
        (),
        sync: GLsync,
        flags: GLbitfield,
        timeout: GLuint64
    );
}

mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw::*;

    macro_rules! store {
        ($name:ident) => {
            pub(super) static mut $name: FnPtr = FnPtr {
                ptr: FnPtr::not_initialized as *const c_void,
                is_loaded: false,
            };
        };
    }

    store!(ActiveShaderProgram);
    store!(ActiveTexture);
    store!(AttachShader);
    store!(BeginConditionalRender);
    store!(BeginQuery);
    store!(BeginQueryIndexed);
    store!(BeginTransformFeedback);
    store!(BindAttribLocation);
    store!(BindBuffer);
    store!(BindBufferBase);
    store!(BindBufferRange);
    store!(BindBuffersBase);
    store!(BindBuffersRange);
    store!(BindFragDataLocation);
    store!(BindFragDataLocationIndexed);
    store!(BindFramebuffer);
    store!(BindImageTexture);
    store!(BindImageTextures);
    store!(BindProgramPipeline);
    store!(BindRenderbuffer);
    store!(BindSampler);
    store!(BindSamplers);
    store!(BindTexture);
    store!(BindTextureUnit);
    store!(BindTextures);
    store!(BindTransformFeedback);
    store!(BindVertexArray);
    store!(BindVertexBuffer);
    store!(BindVertexBuffers);
    store!(BlendColor);
    store!(BlendEquation);
    store!(BlendEquationSeparate);
    store!(BlendEquationSeparatei);
    store!(BlendEquationi);
    store!(BlendFunc);
    store!(BlendFuncSeparate);
    store!(BlendFuncSeparatei);
    store!(BlendFunci);
    store!(BlitFramebuffer);
    store!(BlitNamedFramebuffer);
    store!(BufferData);
    store!(BufferStorage);
    store!(BufferSubData);
    store!(CheckFramebufferStatus);
    store!(CheckNamedFramebufferStatus);
    store!(ClampColor);
    store!(Clear);
    store!(ClearBufferData);
    store!(ClearBufferSubData);
    store!(ClearBufferfi);
    store!(ClearBufferfv);
    store!(ClearBufferiv);
    store!(ClearBufferuiv);
    store!(ClearColor);
    store!(ClearDepth);
    store!(ClearDepthf);
    store!(ClearNamedBufferData);
    store!(ClearNamedBufferSubData);
    store!(ClearNamedFramebufferfi);
    store!(ClearNamedFramebufferfv);
    store!(ClearNamedFramebufferiv);
    store!(ClearNamedFramebufferuiv);
    store!(ClearStencil);
    store!(ClearTexImage);
    store!(ClearTexSubImage);
    store!(ClientWaitSync);
    store!(ClipControl);
    store!(ColorMask);
    store!(ColorMaski);
    store!(CompileShader);
    store!(CompressedTexImage1D);
    store!(CompressedTexImage2D);
    store!(CompressedTexImage3D);
    store!(CompressedTexSubImage1D);
    store!(CompressedTexSubImage2D);
    store!(CompressedTexSubImage3D);
    store!(CompressedTextureSubImage1D);
    store!(CompressedTextureSubImage2D);
    store!(CompressedTextureSubImage3D);
    store!(CopyBufferSubData);
    store!(CopyImageSubData);
    store!(CopyNamedBufferSubData);
    store!(CopyTexImage1D);
    store!(CopyTexImage2D);
    store!(CopyTexSubImage1D);
    store!(CopyTexSubImage2D);
    store!(CopyTexSubImage3D);
    store!(CopyTextureSubImage1D);
    store!(CopyTextureSubImage2D);
    store!(CopyTextureSubImage3D);
    store!(CreateBuffers);
    store!(CreateFramebuffers);
    store!(CreateProgram);
    store!(CreateProgramPipelines);
    store!(CreateQueries);
    store!(CreateRenderbuffers);
    store!(CreateSamplers);
    store!(CreateShader);
    store!(CreateShaderProgramv);
    store!(CreateTextures);
    store!(CreateTransformFeedbacks);
    store!(CreateVertexArrays);
    store!(CullFace);
    store!(DebugMessageCallback);
    store!(DebugMessageControl);
    store!(DebugMessageInsert);
    store!(DeleteBuffers);
    store!(DeleteFramebuffers);
    store!(DeleteProgram);
    store!(DeleteProgramPipelines);
    store!(DeleteQueries);
    store!(DeleteRenderbuffers);
    store!(DeleteSamplers);
    store!(DeleteShader);
    store!(DeleteSync);
    store!(DeleteTextures);
    store!(DeleteTransformFeedbacks);
    store!(DeleteVertexArrays);
    store!(DepthFunc);
    store!(DepthMask);
    store!(DepthRange);
    store!(DepthRangeArrayv);
    store!(DepthRangeIndexed);
    store!(DepthRangef);
    store!(DetachShader);
    store!(Disable);
    store!(DisableVertexArrayAttrib);
    store!(DisableVertexAttribArray);
    store!(Disablei);
    store!(DispatchCompute);
    store!(DispatchComputeIndirect);
    store!(DrawArrays);
    store!(DrawArraysIndirect);
    store!(DrawArraysInstanced);
    store!(DrawArraysInstancedBaseInstance);
    store!(DrawBuffer);
    store!(DrawBuffers);
    store!(DrawElements);
    store!(DrawElementsBaseVertex);
    store!(DrawElementsIndirect);
    store!(DrawElementsInstanced);
    store!(DrawElementsInstancedBaseInstance);
    store!(DrawElementsInstancedBaseVertex);
    store!(DrawElementsInstancedBaseVertexBaseInstance);
    store!(DrawRangeElements);
    store!(DrawRangeElementsBaseVertex);
    store!(DrawTransformFeedback);
    store!(DrawTransformFeedbackInstanced);
    store!(DrawTransformFeedbackStream);
    store!(DrawTransformFeedbackStreamInstanced);
    store!(Enable);
    store!(EnableVertexArrayAttrib);
    store!(EnableVertexAttribArray);
    store!(Enablei);
    store!(EndConditionalRender);
    store!(EndQuery);
    store!(EndQueryIndexed);
    store!(EndTransformFeedback);
    store!(FenceSync);
    store!(Finish);
    store!(Flush);
    store!(FlushMappedBufferRange);
    store!(FlushMappedNamedBufferRange);
    store!(FramebufferParameteri);
    store!(FramebufferRenderbuffer);
    store!(FramebufferTexture);
    store!(FramebufferTexture1D);
    store!(FramebufferTexture2D);
    store!(FramebufferTexture3D);
    store!(FramebufferTextureLayer);
    store!(FrontFace);
    store!(GenBuffers);
    store!(GenFramebuffers);
    store!(GenProgramPipelines);
    store!(GenQueries);
    store!(GenRenderbuffers);
    store!(GenSamplers);
    store!(GenTextures);
    store!(GenTransformFeedbacks);
    store!(GenVertexArrays);
    store!(GenerateMipmap);
    store!(GenerateTextureMipmap);
    store!(GetActiveAtomicCounterBufferiv);
    store!(GetActiveAttrib);
    store!(GetActiveSubroutineName);
    store!(GetActiveSubroutineUniformName);
    store!(GetActiveSubroutineUniformiv);
    store!(GetActiveUniform);
    store!(GetActiveUniformBlockName);
    store!(GetActiveUniformBlockiv);
    store!(GetActiveUniformName);
    store!(GetActiveUniformsiv);
    store!(GetAttachedShaders);
    store!(GetAttribLocation);
    store!(GetBooleani_v);
    store!(GetBooleanv);
    store!(GetBufferParameteri64v);
    store!(GetBufferParameteriv);
    store!(GetBufferPointerv);
    store!(GetBufferSubData);
    store!(GetCompressedTexImage);
    store!(GetCompressedTextureImage);
    store!(GetCompressedTextureSubImage);
    store!(GetDebugMessageLog);
    store!(GetDoublei_v);
    store!(GetDoublev);
    store!(GetError);
    store!(GetFloati_v);
    store!(GetFloatv);
    store!(GetFragDataIndex);
    store!(GetFragDataLocation);
    store!(GetFramebufferAttachmentParameteriv);
    store!(GetFramebufferParameteriv);
    store!(GetGraphicsResetStatus);
    store!(GetInteger64i_v);
    store!(GetInteger64v);
    store!(GetIntegeri_v);
    store!(GetIntegerv);
    store!(GetInternalformati64v);
    store!(GetInternalformativ);
    store!(GetMultisamplefv);
    store!(GetNamedBufferParameteri64v);
    store!(GetNamedBufferParameteriv);
    store!(GetNamedBufferPointerv);
    store!(GetNamedBufferSubData);
    store!(GetNamedFramebufferAttachmentParameteriv);
    store!(GetNamedFramebufferParameteriv);
    store!(GetNamedRenderbufferParameteriv);
    store!(GetObjectLabel);
    store!(GetObjectPtrLabel);
    store!(GetPointerv);
    store!(GetProgramBinary);
    store!(GetProgramInfoLog);
    store!(GetProgramInterfaceiv);
    store!(GetProgramPipelineInfoLog);
    store!(GetProgramPipelineiv);
    store!(GetProgramResourceIndex);
    store!(GetProgramResourceLocation);
    store!(GetProgramResourceLocationIndex);
    store!(GetProgramResourceName);
    store!(GetProgramResourceiv);
    store!(GetProgramStageiv);
    store!(GetProgramiv);
    store!(GetQueryBufferObjecti64v);
    store!(GetQueryBufferObjectiv);
    store!(GetQueryBufferObjectui64v);
    store!(GetQueryBufferObjectuiv);
    store!(GetQueryIndexediv);
    store!(GetQueryObjecti64v);
    store!(GetQueryObjectiv);
    store!(GetQueryObjectui64v);
    store!(GetQueryObjectuiv);
    store!(GetQueryiv);
    store!(GetRenderbufferParameteriv);
    store!(GetSamplerParameterIiv);
    store!(GetSamplerParameterIuiv);
    store!(GetSamplerParameterfv);
    store!(GetSamplerParameteriv);
    store!(GetShaderInfoLog);
    store!(GetShaderPrecisionFormat);
    store!(GetShaderSource);
    store!(GetShaderiv);
    store!(GetString);
    store!(GetStringi);
    store!(GetSubroutineIndex);
    store!(GetSubroutineUniformLocation);
    store!(GetSynciv);
    store!(GetTexImage);
    store!(GetTexLevelParameterfv);
    store!(GetTexLevelParameteriv);
    store!(GetTexParameterIiv);
    store!(GetTexParameterIuiv);
    store!(GetTexParameterfv);
    store!(GetTexParameteriv);
    store!(GetTextureImage);
    store!(GetTextureLevelParameterfv);
    store!(GetTextureLevelParameteriv);
    store!(GetTextureParameterIiv);
    store!(GetTextureParameterIuiv);
    store!(GetTextureParameterfv);
    store!(GetTextureParameteriv);
    store!(GetTextureSubImage);
    store!(GetTransformFeedbackVarying);
    store!(GetTransformFeedbacki64_v);
    store!(GetTransformFeedbacki_v);
    store!(GetTransformFeedbackiv);
    store!(GetUniformBlockIndex);
    store!(GetUniformIndices);
    store!(GetUniformLocation);
    store!(GetUniformSubroutineuiv);
    store!(GetUniformdv);
    store!(GetUniformfv);
    store!(GetUniformiv);
    store!(GetUniformuiv);
    store!(GetVertexArrayIndexed64iv);
    store!(GetVertexArrayIndexediv);
    store!(GetVertexArrayiv);
    store!(GetVertexAttribIiv);
    store!(GetVertexAttribIuiv);
    store!(GetVertexAttribLdv);
    store!(GetVertexAttribPointerv);
    store!(GetVertexAttribdv);
    store!(GetVertexAttribfv);
    store!(GetVertexAttribiv);
    store!(GetnCompressedTexImage);
    store!(GetnTexImage);
    store!(GetnUniformdv);
    store!(GetnUniformfv);
    store!(GetnUniformiv);
    store!(GetnUniformuiv);
    store!(Hint);
    store!(InvalidateBufferData);
    store!(InvalidateBufferSubData);
    store!(InvalidateFramebuffer);
    store!(InvalidateNamedFramebufferData);
    store!(InvalidateNamedFramebufferSubData);
    store!(InvalidateSubFramebuffer);
    store!(InvalidateTexImage);
    store!(InvalidateTexSubImage);
    store!(IsBuffer);
    store!(IsEnabled);
    store!(IsEnabledi);
    store!(IsFramebuffer);
    store!(IsProgram);
    store!(IsProgramPipeline);
    store!(IsQuery);
    store!(IsRenderbuffer);
    store!(IsSampler);
    store!(IsShader);
    store!(IsSync);
    store!(IsTexture);
    store!(IsTransformFeedback);
    store!(IsVertexArray);
    store!(LineWidth);
    store!(LinkProgram);
    store!(LogicOp);
    store!(MapBuffer);
    store!(MapBufferRange);
    store!(MapNamedBuffer);
    store!(MapNamedBufferRange);
    store!(MemoryBarrier);
    store!(MemoryBarrierByRegion);
    store!(MinSampleShading);
    store!(MultiDrawArrays);
    store!(MultiDrawArraysIndirect);
    store!(MultiDrawArraysIndirectCount);
    store!(MultiDrawElements);
    store!(MultiDrawElementsBaseVertex);
    store!(MultiDrawElementsIndirect);
    store!(MultiDrawElementsIndirectCount);
    store!(NamedBufferData);
    store!(NamedBufferStorage);
    store!(NamedBufferSubData);
    store!(NamedFramebufferDrawBuffer);
    store!(NamedFramebufferDrawBuffers);
    store!(NamedFramebufferParameteri);
    store!(NamedFramebufferReadBuffer);
    store!(NamedFramebufferRenderbuffer);
    store!(NamedFramebufferTexture);
    store!(NamedFramebufferTextureLayer);
    store!(NamedRenderbufferStorage);
    store!(NamedRenderbufferStorageMultisample);
    store!(ObjectLabel);
    store!(ObjectPtrLabel);
    store!(PatchParameterfv);
    store!(PatchParameteri);
    store!(PauseTransformFeedback);
    store!(PixelStoref);
    store!(PixelStorei);
    store!(PointParameterf);
    store!(PointParameterfv);
    store!(PointParameteri);
    store!(PointParameteriv);
    store!(PointSize);
    store!(PolygonMode);
    store!(PolygonOffset);
    store!(PolygonOffsetClamp);
    store!(PopDebugGroup);
    store!(PrimitiveRestartIndex);
    store!(ProgramBinary);
    store!(ProgramParameteri);
    store!(ProgramUniform1d);
    store!(ProgramUniform1dv);
    store!(ProgramUniform1f);
    store!(ProgramUniform1fv);
    store!(ProgramUniform1i);
    store!(ProgramUniform1iv);
    store!(ProgramUniform1ui);
    store!(ProgramUniform1uiv);
    store!(ProgramUniform2d);
    store!(ProgramUniform2dv);
    store!(ProgramUniform2f);
    store!(ProgramUniform2fv);
    store!(ProgramUniform2i);
    store!(ProgramUniform2iv);
    store!(ProgramUniform2ui);
    store!(ProgramUniform2uiv);
    store!(ProgramUniform3d);
    store!(ProgramUniform3dv);
    store!(ProgramUniform3f);
    store!(ProgramUniform3fv);
    store!(ProgramUniform3i);
    store!(ProgramUniform3iv);
    store!(ProgramUniform3ui);
    store!(ProgramUniform3uiv);
    store!(ProgramUniform4d);
    store!(ProgramUniform4dv);
    store!(ProgramUniform4f);
    store!(ProgramUniform4fv);
    store!(ProgramUniform4i);
    store!(ProgramUniform4iv);
    store!(ProgramUniform4ui);
    store!(ProgramUniform4uiv);
    store!(ProgramUniformMatrix2dv);
    store!(ProgramUniformMatrix2fv);
    store!(ProgramUniformMatrix2x3dv);
    store!(ProgramUniformMatrix2x3fv);
    store!(ProgramUniformMatrix2x4dv);
    store!(ProgramUniformMatrix2x4fv);
    store!(ProgramUniformMatrix3dv);
    store!(ProgramUniformMatrix3fv);
    store!(ProgramUniformMatrix3x2dv);
    store!(ProgramUniformMatrix3x2fv);
    store!(ProgramUniformMatrix3x4dv);
    store!(ProgramUniformMatrix3x4fv);
    store!(ProgramUniformMatrix4dv);
    store!(ProgramUniformMatrix4fv);
    store!(ProgramUniformMatrix4x2dv);
    store!(ProgramUniformMatrix4x2fv);
    store!(ProgramUniformMatrix4x3dv);
    store!(ProgramUniformMatrix4x3fv);
    store!(ProvokingVertex);
    store!(PushDebugGroup);
    store!(QueryCounter);
    store!(ReadBuffer);
    store!(ReadPixels);
    store!(ReadnPixels);
    store!(ReleaseShaderCompiler);
    store!(RenderbufferStorage);
    store!(RenderbufferStorageMultisample);
    store!(ResumeTransformFeedback);
    store!(SampleCoverage);
    store!(SampleMaski);
    store!(SamplerParameterIiv);
    store!(SamplerParameterIuiv);
    store!(SamplerParameterf);
    store!(SamplerParameterfv);
    store!(SamplerParameteri);
    store!(SamplerParameteriv);
    store!(Scissor);
    store!(ScissorArrayv);
    store!(ScissorIndexed);
    store!(ScissorIndexedv);
    store!(ShaderBinary);
    store!(ShaderSource);
    store!(ShaderStorageBlockBinding);
    store!(SpecializeShader);
    store!(StencilFunc);
    store!(StencilFuncSeparate);
    store!(StencilMask);
    store!(StencilMaskSeparate);
    store!(StencilOp);
    store!(StencilOpSeparate);
    store!(TexBuffer);
    store!(TexBufferRange);
    store!(TexImage1D);
    store!(TexImage2D);
    store!(TexImage2DMultisample);
    store!(TexImage3D);
    store!(TexImage3DMultisample);
    store!(TexParameterIiv);
    store!(TexParameterIuiv);
    store!(TexParameterf);
    store!(TexParameterfv);
    store!(TexParameteri);
    store!(TexParameteriv);
    store!(TexStorage1D);
    store!(TexStorage2D);
    store!(TexStorage2DMultisample);
    store!(TexStorage3D);
    store!(TexStorage3DMultisample);
    store!(TexSubImage1D);
    store!(TexSubImage2D);
    store!(TexSubImage3D);
    store!(TextureBarrier);
    store!(TextureBuffer);
    store!(TextureBufferRange);
    store!(TextureParameterIiv);
    store!(TextureParameterIuiv);
    store!(TextureParameterf);
    store!(TextureParameterfv);
    store!(TextureParameteri);
    store!(TextureParameteriv);
    store!(TextureStorage1D);
    store!(TextureStorage2D);
    store!(TextureStorage2DMultisample);
    store!(TextureStorage3D);
    store!(TextureStorage3DMultisample);
    store!(TextureSubImage1D);
    store!(TextureSubImage2D);
    store!(TextureSubImage3D);
    store!(TextureView);
    store!(TransformFeedbackBufferBase);
    store!(TransformFeedbackBufferRange);
    store!(TransformFeedbackVaryings);
    store!(Uniform1d);
    store!(Uniform1dv);
    store!(Uniform1f);
    store!(Uniform1fv);
    store!(Uniform1i);
    store!(Uniform1iv);
    store!(Uniform1ui);
    store!(Uniform1uiv);
    store!(Uniform2d);
    store!(Uniform2dv);
    store!(Uniform2f);
    store!(Uniform2fv);
    store!(Uniform2i);
    store!(Uniform2iv);
    store!(Uniform2ui);
    store!(Uniform2uiv);
    store!(Uniform3d);
    store!(Uniform3dv);
    store!(Uniform3f);
    store!(Uniform3fv);
    store!(Uniform3i);
    store!(Uniform3iv);
    store!(Uniform3ui);
    store!(Uniform3uiv);
    store!(Uniform4d);
    store!(Uniform4dv);
    store!(Uniform4f);
    store!(Uniform4fv);
    store!(Uniform4i);
    store!(Uniform4iv);
    store!(Uniform4ui);
    store!(Uniform4uiv);
    store!(UniformBlockBinding);
    store!(UniformMatrix2dv);
    store!(UniformMatrix2fv);
    store!(UniformMatrix2x3dv);
    store!(UniformMatrix2x3fv);
    store!(UniformMatrix2x4dv);
    store!(UniformMatrix2x4fv);
    store!(UniformMatrix3dv);
    store!(UniformMatrix3fv);
    store!(UniformMatrix3x2dv);
    store!(UniformMatrix3x2fv);
    store!(UniformMatrix3x4dv);
    store!(UniformMatrix3x4fv);
    store!(UniformMatrix4dv);
    store!(UniformMatrix4fv);
    store!(UniformMatrix4x2dv);
    store!(UniformMatrix4x2fv);
    store!(UniformMatrix4x3dv);
    store!(UniformMatrix4x3fv);
    store!(UniformSubroutinesuiv);
    store!(UnmapBuffer);
    store!(UnmapNamedBuffer);
    store!(UseProgram);
    store!(UseProgramStages);
    store!(ValidateProgram);
    store!(ValidateProgramPipeline);
    store!(VertexArrayAttribBinding);
    store!(VertexArrayAttribFormat);
    store!(VertexArrayAttribIFormat);
    store!(VertexArrayAttribLFormat);
    store!(VertexArrayBindingDivisor);
    store!(VertexArrayElementBuffer);
    store!(VertexArrayVertexBuffer);
    store!(VertexArrayVertexBuffers);
    store!(VertexAttrib1d);
    store!(VertexAttrib1dv);
    store!(VertexAttrib1f);
    store!(VertexAttrib1fv);
    store!(VertexAttrib1s);
    store!(VertexAttrib1sv);
    store!(VertexAttrib2d);
    store!(VertexAttrib2dv);
    store!(VertexAttrib2f);
    store!(VertexAttrib2fv);
    store!(VertexAttrib2s);
    store!(VertexAttrib2sv);
    store!(VertexAttrib3d);
    store!(VertexAttrib3dv);
    store!(VertexAttrib3f);
    store!(VertexAttrib3fv);
    store!(VertexAttrib3s);
    store!(VertexAttrib3sv);
    store!(VertexAttrib4Nbv);
    store!(VertexAttrib4Niv);
    store!(VertexAttrib4Nsv);
    store!(VertexAttrib4Nub);
    store!(VertexAttrib4Nubv);
    store!(VertexAttrib4Nuiv);
    store!(VertexAttrib4Nusv);
    store!(VertexAttrib4bv);
    store!(VertexAttrib4d);
    store!(VertexAttrib4dv);
    store!(VertexAttrib4f);
    store!(VertexAttrib4fv);
    store!(VertexAttrib4iv);
    store!(VertexAttrib4s);
    store!(VertexAttrib4sv);
    store!(VertexAttrib4ubv);
    store!(VertexAttrib4uiv);
    store!(VertexAttrib4usv);
    store!(VertexAttribBinding);
    store!(VertexAttribDivisor);
    store!(VertexAttribFormat);
    store!(VertexAttribI1i);
    store!(VertexAttribI1iv);
    store!(VertexAttribI1ui);
    store!(VertexAttribI1uiv);
    store!(VertexAttribI2i);
    store!(VertexAttribI2iv);
    store!(VertexAttribI2ui);
    store!(VertexAttribI2uiv);
    store!(VertexAttribI3i);
    store!(VertexAttribI3iv);
    store!(VertexAttribI3ui);
    store!(VertexAttribI3uiv);
    store!(VertexAttribI4bv);
    store!(VertexAttribI4i);
    store!(VertexAttribI4iv);
    store!(VertexAttribI4sv);
    store!(VertexAttribI4ubv);
    store!(VertexAttribI4ui);
    store!(VertexAttribI4uiv);
    store!(VertexAttribI4usv);
    store!(VertexAttribIFormat);
    store!(VertexAttribIPointer);
    store!(VertexAttribL1d);
    store!(VertexAttribL1dv);
    store!(VertexAttribL2d);
    store!(VertexAttribL2dv);
    store!(VertexAttribL3d);
    store!(VertexAttribL3dv);
    store!(VertexAttribL4d);
    store!(VertexAttribL4dv);
    store!(VertexAttribLFormat);
    store!(VertexAttribLPointer);
    store!(VertexAttribP1ui);
    store!(VertexAttribP1uiv);
    store!(VertexAttribP2ui);
    store!(VertexAttribP2uiv);
    store!(VertexAttribP3ui);
    store!(VertexAttribP3uiv);
    store!(VertexAttribP4ui);
    store!(VertexAttribP4uiv);
    store!(VertexAttribPointer);
    store!(VertexBindingDivisor);
    store!(Viewport);
    store!(ViewportArrayv);
    store!(ViewportIndexedf);
    store!(ViewportIndexedfv);
    store!(WaitSync);
}

pub fn load<F>(mut loadfn: F)
where
    F: FnMut(&'static str) -> *const c_void,
{
    unsafe {
        storage::ActiveShaderProgram.set_ptr(loadfn("glActiveShaderProgram"));
        storage::ActiveTexture.set_ptr(loadfn("glActiveTexture"));
        storage::AttachShader.set_ptr(loadfn("glAttachShader"));
        storage::BeginConditionalRender.set_ptr(loadfn("glBeginConditionalRender"));
        storage::BeginQuery.set_ptr(loadfn("glBeginQuery"));
        storage::BeginQueryIndexed.set_ptr(loadfn("glBeginQueryIndexed"));
        storage::BeginTransformFeedback.set_ptr(loadfn("glBeginTransformFeedback"));
        storage::BindAttribLocation.set_ptr(loadfn("glBindAttribLocation"));
        storage::BindBuffer.set_ptr(loadfn("glBindBuffer"));
        storage::BindBufferBase.set_ptr(loadfn("glBindBufferBase"));
        storage::BindBufferRange.set_ptr(loadfn("glBindBufferRange"));
        storage::BindBuffersBase.set_ptr(loadfn("glBindBuffersBase"));
        storage::BindBuffersRange.set_ptr(loadfn("glBindBuffersRange"));
        storage::BindFragDataLocation.set_ptr(loadfn("glBindFragDataLocation"));
        storage::BindFragDataLocationIndexed.set_ptr(loadfn("glBindFragDataLocationIndexed"));
        storage::BindFramebuffer.set_ptr(loadfn("glBindFramebuffer"));
        storage::BindImageTexture.set_ptr(loadfn("glBindImageTexture"));
        storage::BindImageTextures.set_ptr(loadfn("glBindImageTextures"));
        storage::BindProgramPipeline.set_ptr(loadfn("glBindProgramPipeline"));
        storage::BindRenderbuffer.set_ptr(loadfn("glBindRenderbuffer"));
        storage::BindSampler.set_ptr(loadfn("glBindSampler"));
        storage::BindSamplers.set_ptr(loadfn("glBindSamplers"));
        storage::BindTexture.set_ptr(loadfn("glBindTexture"));
        storage::BindTextureUnit.set_ptr(loadfn("glBindTextureUnit"));
        storage::BindTextures.set_ptr(loadfn("glBindTextures"));
        storage::BindTransformFeedback.set_ptr(loadfn("glBindTransformFeedback"));
        storage::BindVertexArray.set_ptr(loadfn("glBindVertexArray"));
        storage::BindVertexBuffer.set_ptr(loadfn("glBindVertexBuffer"));
        storage::BindVertexBuffers.set_ptr(loadfn("glBindVertexBuffers"));
        storage::BlendColor.set_ptr(loadfn("glBlendColor"));
        storage::BlendEquation.set_ptr(loadfn("glBlendEquation"));
        storage::BlendEquationSeparate.set_ptr(loadfn("glBlendEquationSeparate"));
        storage::BlendEquationSeparatei.set_ptr(loadfn("glBlendEquationSeparatei"));
        storage::BlendEquationi.set_ptr(loadfn("glBlendEquationi"));
        storage::BlendFunc.set_ptr(loadfn("glBlendFunc"));
        storage::BlendFuncSeparate.set_ptr(loadfn("glBlendFuncSeparate"));
        storage::BlendFuncSeparatei.set_ptr(loadfn("glBlendFuncSeparatei"));
        storage::BlendFunci.set_ptr(loadfn("glBlendFunci"));
        storage::BlitFramebuffer.set_ptr(loadfn("glBlitFramebuffer"));
        storage::BlitNamedFramebuffer.set_ptr(loadfn("glBlitNamedFramebuffer"));
        storage::BufferData.set_ptr(loadfn("glBufferData"));
        storage::BufferStorage.set_ptr(loadfn("glBufferStorage"));
        storage::BufferSubData.set_ptr(loadfn("glBufferSubData"));
        storage::CheckFramebufferStatus.set_ptr(loadfn("glCheckFramebufferStatus"));
        storage::CheckNamedFramebufferStatus.set_ptr(loadfn("glCheckNamedFramebufferStatus"));
        storage::ClampColor.set_ptr(loadfn("glClampColor"));
        storage::Clear.set_ptr(loadfn("glClear"));
        storage::ClearBufferData.set_ptr(loadfn("glClearBufferData"));
        storage::ClearBufferSubData.set_ptr(loadfn("glClearBufferSubData"));
        storage::ClearBufferfi.set_ptr(loadfn("glClearBufferfi"));
        storage::ClearBufferfv.set_ptr(loadfn("glClearBufferfv"));
        storage::ClearBufferiv.set_ptr(loadfn("glClearBufferiv"));
        storage::ClearBufferuiv.set_ptr(loadfn("glClearBufferuiv"));
        storage::ClearColor.set_ptr(loadfn("glClearColor"));
        storage::ClearDepth.set_ptr(loadfn("glClearDepth"));
        storage::ClearDepthf.set_ptr(loadfn("glClearDepthf"));
        storage::ClearNamedBufferData.set_ptr(loadfn("glClearNamedBufferData"));
        storage::ClearNamedBufferSubData.set_ptr(loadfn("glClearNamedBufferSubData"));
        storage::ClearNamedFramebufferfi.set_ptr(loadfn("glClearNamedFramebufferfi"));
        storage::ClearNamedFramebufferfv.set_ptr(loadfn("glClearNamedFramebufferfv"));
        storage::ClearNamedFramebufferiv.set_ptr(loadfn("glClearNamedFramebufferiv"));
        storage::ClearNamedFramebufferuiv.set_ptr(loadfn("glClearNamedFramebufferuiv"));
        storage::ClearStencil.set_ptr(loadfn("glClearStencil"));
        storage::ClearTexImage.set_ptr(loadfn("glClearTexImage"));
        storage::ClearTexSubImage.set_ptr(loadfn("glClearTexSubImage"));
        storage::ClientWaitSync.set_ptr(loadfn("glClientWaitSync"));
        storage::ClipControl.set_ptr(loadfn("glClipControl"));
        storage::ColorMask.set_ptr(loadfn("glColorMask"));
        storage::ColorMaski.set_ptr(loadfn("glColorMaski"));
        storage::CompileShader.set_ptr(loadfn("glCompileShader"));
        storage::CompressedTexImage1D.set_ptr(loadfn("glCompressedTexImage1D"));
        storage::CompressedTexImage2D.set_ptr(loadfn("glCompressedTexImage2D"));
        storage::CompressedTexImage3D.set_ptr(loadfn("glCompressedTexImage3D"));
        storage::CompressedTexSubImage1D.set_ptr(loadfn("glCompressedTexSubImage1D"));
        storage::CompressedTexSubImage2D.set_ptr(loadfn("glCompressedTexSubImage2D"));
        storage::CompressedTexSubImage3D.set_ptr(loadfn("glCompressedTexSubImage3D"));
        storage::CompressedTextureSubImage1D.set_ptr(loadfn("glCompressedTextureSubImage1D"));
        storage::CompressedTextureSubImage2D.set_ptr(loadfn("glCompressedTextureSubImage2D"));
        storage::CompressedTextureSubImage3D.set_ptr(loadfn("glCompressedTextureSubImage3D"));
        storage::CopyBufferSubData.set_ptr(loadfn("glCopyBufferSubData"));
        storage::CopyImageSubData.set_ptr(loadfn("glCopyImageSubData"));
        storage::CopyNamedBufferSubData.set_ptr(loadfn("glCopyNamedBufferSubData"));
        storage::CopyTexImage1D.set_ptr(loadfn("glCopyTexImage1D"));
        storage::CopyTexImage2D.set_ptr(loadfn("glCopyTexImage2D"));
        storage::CopyTexSubImage1D.set_ptr(loadfn("glCopyTexSubImage1D"));
        storage::CopyTexSubImage2D.set_ptr(loadfn("glCopyTexSubImage2D"));
        storage::CopyTexSubImage3D.set_ptr(loadfn("glCopyTexSubImage3D"));
        storage::CopyTextureSubImage1D.set_ptr(loadfn("glCopyTextureSubImage1D"));
        storage::CopyTextureSubImage2D.set_ptr(loadfn("glCopyTextureSubImage2D"));
        storage::CopyTextureSubImage3D.set_ptr(loadfn("glCopyTextureSubImage3D"));
        storage::CreateBuffers.set_ptr(loadfn("glCreateBuffers"));
        storage::CreateFramebuffers.set_ptr(loadfn("glCreateFramebuffers"));
        storage::CreateProgram.set_ptr(loadfn("glCreateProgram"));
        storage::CreateProgramPipelines.set_ptr(loadfn("glCreateProgramPipelines"));
        storage::CreateQueries.set_ptr(loadfn("glCreateQueries"));
        storage::CreateRenderbuffers.set_ptr(loadfn("glCreateRenderbuffers"));
        storage::CreateSamplers.set_ptr(loadfn("glCreateSamplers"));
        storage::CreateShader.set_ptr(loadfn("glCreateShader"));
        storage::CreateShaderProgramv.set_ptr(loadfn("glCreateShaderProgramv"));
        storage::CreateTextures.set_ptr(loadfn("glCreateTextures"));
        storage::CreateTransformFeedbacks.set_ptr(loadfn("glCreateTransformFeedbacks"));
        storage::CreateVertexArrays.set_ptr(loadfn("glCreateVertexArrays"));
        storage::CullFace.set_ptr(loadfn("glCullFace"));
        storage::DebugMessageCallback.set_ptr(loadfn("glDebugMessageCallback"));
        storage::DebugMessageControl.set_ptr(loadfn("glDebugMessageControl"));
        storage::DebugMessageInsert.set_ptr(loadfn("glDebugMessageInsert"));
        storage::DeleteBuffers.set_ptr(loadfn("glDeleteBuffers"));
        storage::DeleteFramebuffers.set_ptr(loadfn("glDeleteFramebuffers"));
        storage::DeleteProgram.set_ptr(loadfn("glDeleteProgram"));
        storage::DeleteProgramPipelines.set_ptr(loadfn("glDeleteProgramPipelines"));
        storage::DeleteQueries.set_ptr(loadfn("glDeleteQueries"));
        storage::DeleteRenderbuffers.set_ptr(loadfn("glDeleteRenderbuffers"));
        storage::DeleteSamplers.set_ptr(loadfn("glDeleteSamplers"));
        storage::DeleteShader.set_ptr(loadfn("glDeleteShader"));
        storage::DeleteSync.set_ptr(loadfn("glDeleteSync"));
        storage::DeleteTextures.set_ptr(loadfn("glDeleteTextures"));
        storage::DeleteTransformFeedbacks.set_ptr(loadfn("glDeleteTransformFeedbacks"));
        storage::DeleteVertexArrays.set_ptr(loadfn("glDeleteVertexArrays"));
        storage::DepthFunc.set_ptr(loadfn("glDepthFunc"));
        storage::DepthMask.set_ptr(loadfn("glDepthMask"));
        storage::DepthRange.set_ptr(loadfn("glDepthRange"));
        storage::DepthRangeArrayv.set_ptr(loadfn("glDepthRangeArrayv"));
        storage::DepthRangeIndexed.set_ptr(loadfn("glDepthRangeIndexed"));
        storage::DepthRangef.set_ptr(loadfn("glDepthRangef"));
        storage::DetachShader.set_ptr(loadfn("glDetachShader"));
        storage::Disable.set_ptr(loadfn("glDisable"));
        storage::DisableVertexArrayAttrib.set_ptr(loadfn("glDisableVertexArrayAttrib"));
        storage::DisableVertexAttribArray.set_ptr(loadfn("glDisableVertexAttribArray"));
        storage::Disablei.set_ptr(loadfn("glDisablei"));
        storage::DispatchCompute.set_ptr(loadfn("glDispatchCompute"));
        storage::DispatchComputeIndirect.set_ptr(loadfn("glDispatchComputeIndirect"));
        storage::DrawArrays.set_ptr(loadfn("glDrawArrays"));
        storage::DrawArraysIndirect.set_ptr(loadfn("glDrawArraysIndirect"));
        storage::DrawArraysInstanced.set_ptr(loadfn("glDrawArraysInstanced"));
        storage::DrawArraysInstancedBaseInstance
            .set_ptr(loadfn("glDrawArraysInstancedBaseInstance"));
        storage::DrawBuffer.set_ptr(loadfn("glDrawBuffer"));
        storage::DrawBuffers.set_ptr(loadfn("glDrawBuffers"));
        storage::DrawElements.set_ptr(loadfn("glDrawElements"));
        storage::DrawElementsBaseVertex.set_ptr(loadfn("glDrawElementsBaseVertex"));
        storage::DrawElementsIndirect.set_ptr(loadfn("glDrawElementsIndirect"));
        storage::DrawElementsInstanced.set_ptr(loadfn("glDrawElementsInstanced"));
        storage::DrawElementsInstancedBaseInstance
            .set_ptr(loadfn("glDrawElementsInstancedBaseInstance"));
        storage::DrawElementsInstancedBaseVertex
            .set_ptr(loadfn("glDrawElementsInstancedBaseVertex"));
        storage::DrawElementsInstancedBaseVertexBaseInstance
            .set_ptr(loadfn("glDrawElementsInstancedBaseVertexBaseInstance"));
        storage::DrawRangeElements.set_ptr(loadfn("glDrawRangeElements"));
        storage::DrawRangeElementsBaseVertex.set_ptr(loadfn("glDrawRangeElementsBaseVertex"));
        storage::DrawTransformFeedback.set_ptr(loadfn("glDrawTransformFeedback"));
        storage::DrawTransformFeedbackInstanced.set_ptr(loadfn("glDrawTransformFeedbackInstanced"));
        storage::DrawTransformFeedbackStream.set_ptr(loadfn("glDrawTransformFeedbackStream"));
        storage::DrawTransformFeedbackStreamInstanced
            .set_ptr(loadfn("glDrawTransformFeedbackStreamInstanced"));
        storage::Enable.set_ptr(loadfn("glEnable"));
        storage::EnableVertexArrayAttrib.set_ptr(loadfn("glEnableVertexArrayAttrib"));
        storage::EnableVertexAttribArray.set_ptr(loadfn("glEnableVertexAttribArray"));
        storage::Enablei.set_ptr(loadfn("glEnablei"));
        storage::EndConditionalRender.set_ptr(loadfn("glEndConditionalRender"));
        storage::EndQuery.set_ptr(loadfn("glEndQuery"));
        storage::EndQueryIndexed.set_ptr(loadfn("glEndQueryIndexed"));
        storage::EndTransformFeedback.set_ptr(loadfn("glEndTransformFeedback"));
        storage::FenceSync.set_ptr(loadfn("glFenceSync"));
        storage::Finish.set_ptr(loadfn("glFinish"));
        storage::Flush.set_ptr(loadfn("glFlush"));
        storage::FlushMappedBufferRange.set_ptr(loadfn("glFlushMappedBufferRange"));
        storage::FlushMappedNamedBufferRange.set_ptr(loadfn("glFlushMappedNamedBufferRange"));
        storage::FramebufferParameteri.set_ptr(loadfn("glFramebufferParameteri"));
        storage::FramebufferRenderbuffer.set_ptr(loadfn("glFramebufferRenderbuffer"));
        storage::FramebufferTexture.set_ptr(loadfn("glFramebufferTexture"));
        storage::FramebufferTexture1D.set_ptr(loadfn("glFramebufferTexture1D"));
        storage::FramebufferTexture2D.set_ptr(loadfn("glFramebufferTexture2D"));
        storage::FramebufferTexture3D.set_ptr(loadfn("glFramebufferTexture3D"));
        storage::FramebufferTextureLayer.set_ptr(loadfn("glFramebufferTextureLayer"));
        storage::FrontFace.set_ptr(loadfn("glFrontFace"));
        storage::GenBuffers.set_ptr(loadfn("glGenBuffers"));
        storage::GenFramebuffers.set_ptr(loadfn("glGenFramebuffers"));
        storage::GenProgramPipelines.set_ptr(loadfn("glGenProgramPipelines"));
        storage::GenQueries.set_ptr(loadfn("glGenQueries"));
        storage::GenRenderbuffers.set_ptr(loadfn("glGenRenderbuffers"));
        storage::GenSamplers.set_ptr(loadfn("glGenSamplers"));
        storage::GenTextures.set_ptr(loadfn("glGenTextures"));
        storage::GenTransformFeedbacks.set_ptr(loadfn("glGenTransformFeedbacks"));
        storage::GenVertexArrays.set_ptr(loadfn("glGenVertexArrays"));
        storage::GenerateMipmap.set_ptr(loadfn("glGenerateMipmap"));
        storage::GenerateTextureMipmap.set_ptr(loadfn("glGenerateTextureMipmap"));
        storage::GetActiveAtomicCounterBufferiv.set_ptr(loadfn("glGetActiveAtomicCounterBufferiv"));
        storage::GetActiveAttrib.set_ptr(loadfn("glGetActiveAttrib"));
        storage::GetActiveSubroutineName.set_ptr(loadfn("glGetActiveSubroutineName"));
        storage::GetActiveSubroutineUniformName.set_ptr(loadfn("glGetActiveSubroutineUniformName"));
        storage::GetActiveSubroutineUniformiv.set_ptr(loadfn("glGetActiveSubroutineUniformiv"));
        storage::GetActiveUniform.set_ptr(loadfn("glGetActiveUniform"));
        storage::GetActiveUniformBlockName.set_ptr(loadfn("glGetActiveUniformBlockName"));
        storage::GetActiveUniformBlockiv.set_ptr(loadfn("glGetActiveUniformBlockiv"));
        storage::GetActiveUniformName.set_ptr(loadfn("glGetActiveUniformName"));
        storage::GetActiveUniformsiv.set_ptr(loadfn("glGetActiveUniformsiv"));
        storage::GetAttachedShaders.set_ptr(loadfn("glGetAttachedShaders"));
        storage::GetAttribLocation.set_ptr(loadfn("glGetAttribLocation"));
        storage::GetBooleani_v.set_ptr(loadfn("glGetBooleani_v"));
        storage::GetBooleanv.set_ptr(loadfn("glGetBooleanv"));
        storage::GetBufferParameteri64v.set_ptr(loadfn("glGetBufferParameteri64v"));
        storage::GetBufferParameteriv.set_ptr(loadfn("glGetBufferParameteriv"));
        storage::GetBufferPointerv.set_ptr(loadfn("glGetBufferPointerv"));
        storage::GetBufferSubData.set_ptr(loadfn("glGetBufferSubData"));
        storage::GetCompressedTexImage.set_ptr(loadfn("glGetCompressedTexImage"));
        storage::GetCompressedTextureImage.set_ptr(loadfn("glGetCompressedTextureImage"));
        storage::GetCompressedTextureSubImage.set_ptr(loadfn("glGetCompressedTextureSubImage"));
        storage::GetDebugMessageLog.set_ptr(loadfn("glGetDebugMessageLog"));
        storage::GetDoublei_v.set_ptr(loadfn("glGetDoublei_v"));
        storage::GetDoublev.set_ptr(loadfn("glGetDoublev"));
        storage::GetError.set_ptr(loadfn("glGetError"));
        storage::GetFloati_v.set_ptr(loadfn("glGetFloati_v"));
        storage::GetFloatv.set_ptr(loadfn("glGetFloatv"));
        storage::GetFragDataIndex.set_ptr(loadfn("glGetFragDataIndex"));
        storage::GetFragDataLocation.set_ptr(loadfn("glGetFragDataLocation"));
        storage::GetFramebufferAttachmentParameteriv
            .set_ptr(loadfn("glGetFramebufferAttachmentParameteriv"));
        storage::GetFramebufferParameteriv.set_ptr(loadfn("glGetFramebufferParameteriv"));
        storage::GetGraphicsResetStatus.set_ptr(loadfn("glGetGraphicsResetStatus"));
        storage::GetInteger64i_v.set_ptr(loadfn("glGetInteger64i_v"));
        storage::GetInteger64v.set_ptr(loadfn("glGetInteger64v"));
        storage::GetIntegeri_v.set_ptr(loadfn("glGetIntegeri_v"));
        storage::GetIntegerv.set_ptr(loadfn("glGetIntegerv"));
        storage::GetInternalformati64v.set_ptr(loadfn("glGetInternalformati64v"));
        storage::GetInternalformativ.set_ptr(loadfn("glGetInternalformativ"));
        storage::GetMultisamplefv.set_ptr(loadfn("glGetMultisamplefv"));
        storage::GetNamedBufferParameteri64v.set_ptr(loadfn("glGetNamedBufferParameteri64v"));
        storage::GetNamedBufferParameteriv.set_ptr(loadfn("glGetNamedBufferParameteriv"));
        storage::GetNamedBufferPointerv.set_ptr(loadfn("glGetNamedBufferPointerv"));
        storage::GetNamedBufferSubData.set_ptr(loadfn("glGetNamedBufferSubData"));
        storage::GetNamedFramebufferAttachmentParameteriv
            .set_ptr(loadfn("glGetNamedFramebufferAttachmentParameteriv"));
        storage::GetNamedFramebufferParameteriv.set_ptr(loadfn("glGetNamedFramebufferParameteriv"));
        storage::GetNamedRenderbufferParameteriv
            .set_ptr(loadfn("glGetNamedRenderbufferParameteriv"));
        storage::GetObjectLabel.set_ptr(loadfn("glGetObjectLabel"));
        storage::GetObjectPtrLabel.set_ptr(loadfn("glGetObjectPtrLabel"));
        storage::GetPointerv.set_ptr(loadfn("glGetPointerv"));
        storage::GetProgramBinary.set_ptr(loadfn("glGetProgramBinary"));
        storage::GetProgramInfoLog.set_ptr(loadfn("glGetProgramInfoLog"));
        storage::GetProgramInterfaceiv.set_ptr(loadfn("glGetProgramInterfaceiv"));
        storage::GetProgramPipelineInfoLog.set_ptr(loadfn("glGetProgramPipelineInfoLog"));
        storage::GetProgramPipelineiv.set_ptr(loadfn("glGetProgramPipelineiv"));
        storage::GetProgramResourceIndex.set_ptr(loadfn("glGetProgramResourceIndex"));
        storage::GetProgramResourceLocation.set_ptr(loadfn("glGetProgramResourceLocation"));
        storage::GetProgramResourceLocationIndex
            .set_ptr(loadfn("glGetProgramResourceLocationIndex"));
        storage::GetProgramResourceName.set_ptr(loadfn("glGetProgramResourceName"));
        storage::GetProgramResourceiv.set_ptr(loadfn("glGetProgramResourceiv"));
        storage::GetProgramStageiv.set_ptr(loadfn("glGetProgramStageiv"));
        storage::GetProgramiv.set_ptr(loadfn("glGetProgramiv"));
        storage::GetQueryBufferObjecti64v.set_ptr(loadfn("glGetQueryBufferObjecti64v"));
        storage::GetQueryBufferObjectiv.set_ptr(loadfn("glGetQueryBufferObjectiv"));
        storage::GetQueryBufferObjectui64v.set_ptr(loadfn("glGetQueryBufferObjectui64v"));
        storage::GetQueryBufferObjectuiv.set_ptr(loadfn("glGetQueryBufferObjectuiv"));
        storage::GetQueryIndexediv.set_ptr(loadfn("glGetQueryIndexediv"));
        storage::GetQueryObjecti64v.set_ptr(loadfn("glGetQueryObjecti64v"));
        storage::GetQueryObjectiv.set_ptr(loadfn("glGetQueryObjectiv"));
        storage::GetQueryObjectui64v.set_ptr(loadfn("glGetQueryObjectui64v"));
        storage::GetQueryObjectuiv.set_ptr(loadfn("glGetQueryObjectuiv"));
        storage::GetQueryiv.set_ptr(loadfn("glGetQueryiv"));
        storage::GetRenderbufferParameteriv.set_ptr(loadfn("glGetRenderbufferParameteriv"));
        storage::GetSamplerParameterIiv.set_ptr(loadfn("glGetSamplerParameterIiv"));
        storage::GetSamplerParameterIuiv.set_ptr(loadfn("glGetSamplerParameterIuiv"));
        storage::GetSamplerParameterfv.set_ptr(loadfn("glGetSamplerParameterfv"));
        storage::GetSamplerParameteriv.set_ptr(loadfn("glGetSamplerParameteriv"));
        storage::GetShaderInfoLog.set_ptr(loadfn("glGetShaderInfoLog"));
        storage::GetShaderPrecisionFormat.set_ptr(loadfn("glGetShaderPrecisionFormat"));
        storage::GetShaderSource.set_ptr(loadfn("glGetShaderSource"));
        storage::GetShaderiv.set_ptr(loadfn("glGetShaderiv"));
        storage::GetString.set_ptr(loadfn("glGetString"));
        storage::GetStringi.set_ptr(loadfn("glGetStringi"));
        storage::GetSubroutineIndex.set_ptr(loadfn("glGetSubroutineIndex"));
        storage::GetSubroutineUniformLocation.set_ptr(loadfn("glGetSubroutineUniformLocation"));
        storage::GetSynciv.set_ptr(loadfn("glGetSynciv"));
        storage::GetTexImage.set_ptr(loadfn("glGetTexImage"));
        storage::GetTexLevelParameterfv.set_ptr(loadfn("glGetTexLevelParameterfv"));
        storage::GetTexLevelParameteriv.set_ptr(loadfn("glGetTexLevelParameteriv"));
        storage::GetTexParameterIiv.set_ptr(loadfn("glGetTexParameterIiv"));
        storage::GetTexParameterIuiv.set_ptr(loadfn("glGetTexParameterIuiv"));
        storage::GetTexParameterfv.set_ptr(loadfn("glGetTexParameterfv"));
        storage::GetTexParameteriv.set_ptr(loadfn("glGetTexParameteriv"));
        storage::GetTextureImage.set_ptr(loadfn("glGetTextureImage"));
        storage::GetTextureLevelParameterfv.set_ptr(loadfn("glGetTextureLevelParameterfv"));
        storage::GetTextureLevelParameteriv.set_ptr(loadfn("glGetTextureLevelParameteriv"));
        storage::GetTextureParameterIiv.set_ptr(loadfn("glGetTextureParameterIiv"));
        storage::GetTextureParameterIuiv.set_ptr(loadfn("glGetTextureParameterIuiv"));
        storage::GetTextureParameterfv.set_ptr(loadfn("glGetTextureParameterfv"));
        storage::GetTextureParameteriv.set_ptr(loadfn("glGetTextureParameteriv"));
        storage::GetTextureSubImage.set_ptr(loadfn("glGetTextureSubImage"));
        storage::GetTransformFeedbackVarying.set_ptr(loadfn("glGetTransformFeedbackVarying"));
        storage::GetTransformFeedbacki64_v.set_ptr(loadfn("glGetTransformFeedbacki64_v"));
        storage::GetTransformFeedbacki_v.set_ptr(loadfn("glGetTransformFeedbacki_v"));
        storage::GetTransformFeedbackiv.set_ptr(loadfn("glGetTransformFeedbackiv"));
        storage::GetUniformBlockIndex.set_ptr(loadfn("glGetUniformBlockIndex"));
        storage::GetUniformIndices.set_ptr(loadfn("glGetUniformIndices"));
        storage::GetUniformLocation.set_ptr(loadfn("glGetUniformLocation"));
        storage::GetUniformSubroutineuiv.set_ptr(loadfn("glGetUniformSubroutineuiv"));
        storage::GetUniformdv.set_ptr(loadfn("glGetUniformdv"));
        storage::GetUniformfv.set_ptr(loadfn("glGetUniformfv"));
        storage::GetUniformiv.set_ptr(loadfn("glGetUniformiv"));
        storage::GetUniformuiv.set_ptr(loadfn("glGetUniformuiv"));
        storage::GetVertexArrayIndexed64iv.set_ptr(loadfn("glGetVertexArrayIndexed64iv"));
        storage::GetVertexArrayIndexediv.set_ptr(loadfn("glGetVertexArrayIndexediv"));
        storage::GetVertexArrayiv.set_ptr(loadfn("glGetVertexArrayiv"));
        storage::GetVertexAttribIiv.set_ptr(loadfn("glGetVertexAttribIiv"));
        storage::GetVertexAttribIuiv.set_ptr(loadfn("glGetVertexAttribIuiv"));
        storage::GetVertexAttribLdv.set_ptr(loadfn("glGetVertexAttribLdv"));
        storage::GetVertexAttribPointerv.set_ptr(loadfn("glGetVertexAttribPointerv"));
        storage::GetVertexAttribdv.set_ptr(loadfn("glGetVertexAttribdv"));
        storage::GetVertexAttribfv.set_ptr(loadfn("glGetVertexAttribfv"));
        storage::GetVertexAttribiv.set_ptr(loadfn("glGetVertexAttribiv"));
        storage::GetnCompressedTexImage.set_ptr(loadfn("glGetnCompressedTexImage"));
        storage::GetnTexImage.set_ptr(loadfn("glGetnTexImage"));
        storage::GetnUniformdv.set_ptr(loadfn("glGetnUniformdv"));
        storage::GetnUniformfv.set_ptr(loadfn("glGetnUniformfv"));
        storage::GetnUniformiv.set_ptr(loadfn("glGetnUniformiv"));
        storage::GetnUniformuiv.set_ptr(loadfn("glGetnUniformuiv"));
        storage::Hint.set_ptr(loadfn("glHint"));
        storage::InvalidateBufferData.set_ptr(loadfn("glInvalidateBufferData"));
        storage::InvalidateBufferSubData.set_ptr(loadfn("glInvalidateBufferSubData"));
        storage::InvalidateFramebuffer.set_ptr(loadfn("glInvalidateFramebuffer"));
        storage::InvalidateNamedFramebufferData.set_ptr(loadfn("glInvalidateNamedFramebufferData"));
        storage::InvalidateNamedFramebufferSubData
            .set_ptr(loadfn("glInvalidateNamedFramebufferSubData"));
        storage::InvalidateSubFramebuffer.set_ptr(loadfn("glInvalidateSubFramebuffer"));
        storage::InvalidateTexImage.set_ptr(loadfn("glInvalidateTexImage"));
        storage::InvalidateTexSubImage.set_ptr(loadfn("glInvalidateTexSubImage"));
        storage::IsBuffer.set_ptr(loadfn("glIsBuffer"));
        storage::IsEnabled.set_ptr(loadfn("glIsEnabled"));
        storage::IsEnabledi.set_ptr(loadfn("glIsEnabledi"));
        storage::IsFramebuffer.set_ptr(loadfn("glIsFramebuffer"));
        storage::IsProgram.set_ptr(loadfn("glIsProgram"));
        storage::IsProgramPipeline.set_ptr(loadfn("glIsProgramPipeline"));
        storage::IsQuery.set_ptr(loadfn("glIsQuery"));
        storage::IsRenderbuffer.set_ptr(loadfn("glIsRenderbuffer"));
        storage::IsSampler.set_ptr(loadfn("glIsSampler"));
        storage::IsShader.set_ptr(loadfn("glIsShader"));
        storage::IsSync.set_ptr(loadfn("glIsSync"));
        storage::IsTexture.set_ptr(loadfn("glIsTexture"));
        storage::IsTransformFeedback.set_ptr(loadfn("glIsTransformFeedback"));
        storage::IsVertexArray.set_ptr(loadfn("glIsVertexArray"));
        storage::LineWidth.set_ptr(loadfn("glLineWidth"));
        storage::LinkProgram.set_ptr(loadfn("glLinkProgram"));
        storage::LogicOp.set_ptr(loadfn("glLogicOp"));
        storage::MapBuffer.set_ptr(loadfn("glMapBuffer"));
        storage::MapBufferRange.set_ptr(loadfn("glMapBufferRange"));
        storage::MapNamedBuffer.set_ptr(loadfn("glMapNamedBuffer"));
        storage::MapNamedBufferRange.set_ptr(loadfn("glMapNamedBufferRange"));
        storage::MemoryBarrier.set_ptr(loadfn("glMemoryBarrier"));
        storage::MemoryBarrierByRegion.set_ptr(loadfn("glMemoryBarrierByRegion"));
        storage::MinSampleShading.set_ptr(loadfn("glMinSampleShading"));
        storage::MultiDrawArrays.set_ptr(loadfn("glMultiDrawArrays"));
        storage::MultiDrawArraysIndirect.set_ptr(loadfn("glMultiDrawArraysIndirect"));
        storage::MultiDrawArraysIndirectCount.set_ptr(loadfn("glMultiDrawArraysIndirectCount"));
        storage::MultiDrawElements.set_ptr(loadfn("glMultiDrawElements"));
        storage::MultiDrawElementsBaseVertex.set_ptr(loadfn("glMultiDrawElementsBaseVertex"));
        storage::MultiDrawElementsIndirect.set_ptr(loadfn("glMultiDrawElementsIndirect"));
        storage::MultiDrawElementsIndirectCount.set_ptr(loadfn("glMultiDrawElementsIndirectCount"));
        storage::NamedBufferData.set_ptr(loadfn("glNamedBufferData"));
        storage::NamedBufferStorage.set_ptr(loadfn("glNamedBufferStorage"));
        storage::NamedBufferSubData.set_ptr(loadfn("glNamedBufferSubData"));
        storage::NamedFramebufferDrawBuffer.set_ptr(loadfn("glNamedFramebufferDrawBuffer"));
        storage::NamedFramebufferDrawBuffers.set_ptr(loadfn("glNamedFramebufferDrawBuffers"));
        storage::NamedFramebufferParameteri.set_ptr(loadfn("glNamedFramebufferParameteri"));
        storage::NamedFramebufferReadBuffer.set_ptr(loadfn("glNamedFramebufferReadBuffer"));
        storage::NamedFramebufferRenderbuffer.set_ptr(loadfn("glNamedFramebufferRenderbuffer"));
        storage::NamedFramebufferTexture.set_ptr(loadfn("glNamedFramebufferTexture"));
        storage::NamedFramebufferTextureLayer.set_ptr(loadfn("glNamedFramebufferTextureLayer"));
        storage::NamedRenderbufferStorage.set_ptr(loadfn("glNamedRenderbufferStorage"));
        storage::NamedRenderbufferStorageMultisample
            .set_ptr(loadfn("glNamedRenderbufferStorageMultisample"));
        storage::ObjectLabel.set_ptr(loadfn("glObjectLabel"));
        storage::ObjectPtrLabel.set_ptr(loadfn("glObjectPtrLabel"));
        storage::PatchParameterfv.set_ptr(loadfn("glPatchParameterfv"));
        storage::PatchParameteri.set_ptr(loadfn("glPatchParameteri"));
        storage::PauseTransformFeedback.set_ptr(loadfn("glPauseTransformFeedback"));
        storage::PixelStoref.set_ptr(loadfn("glPixelStoref"));
        storage::PixelStorei.set_ptr(loadfn("glPixelStorei"));
        storage::PointParameterf.set_ptr(loadfn("glPointParameterf"));
        storage::PointParameterfv.set_ptr(loadfn("glPointParameterfv"));
        storage::PointParameteri.set_ptr(loadfn("glPointParameteri"));
        storage::PointParameteriv.set_ptr(loadfn("glPointParameteriv"));
        storage::PointSize.set_ptr(loadfn("glPointSize"));
        storage::PolygonMode.set_ptr(loadfn("glPolygonMode"));
        storage::PolygonOffset.set_ptr(loadfn("glPolygonOffset"));
        storage::PolygonOffsetClamp.set_ptr(loadfn("glPolygonOffsetClamp"));
        storage::PopDebugGroup.set_ptr(loadfn("glPopDebugGroup"));
        storage::PrimitiveRestartIndex.set_ptr(loadfn("glPrimitiveRestartIndex"));
        storage::ProgramBinary.set_ptr(loadfn("glProgramBinary"));
        storage::ProgramParameteri.set_ptr(loadfn("glProgramParameteri"));
        storage::ProgramUniform1d.set_ptr(loadfn("glProgramUniform1d"));
        storage::ProgramUniform1dv.set_ptr(loadfn("glProgramUniform1dv"));
        storage::ProgramUniform1f.set_ptr(loadfn("glProgramUniform1f"));
        storage::ProgramUniform1fv.set_ptr(loadfn("glProgramUniform1fv"));
        storage::ProgramUniform1i.set_ptr(loadfn("glProgramUniform1i"));
        storage::ProgramUniform1iv.set_ptr(loadfn("glProgramUniform1iv"));
        storage::ProgramUniform1ui.set_ptr(loadfn("glProgramUniform1ui"));
        storage::ProgramUniform1uiv.set_ptr(loadfn("glProgramUniform1uiv"));
        storage::ProgramUniform2d.set_ptr(loadfn("glProgramUniform2d"));
        storage::ProgramUniform2dv.set_ptr(loadfn("glProgramUniform2dv"));
        storage::ProgramUniform2f.set_ptr(loadfn("glProgramUniform2f"));
        storage::ProgramUniform2fv.set_ptr(loadfn("glProgramUniform2fv"));
        storage::ProgramUniform2i.set_ptr(loadfn("glProgramUniform2i"));
        storage::ProgramUniform2iv.set_ptr(loadfn("glProgramUniform2iv"));
        storage::ProgramUniform2ui.set_ptr(loadfn("glProgramUniform2ui"));
        storage::ProgramUniform2uiv.set_ptr(loadfn("glProgramUniform2uiv"));
        storage::ProgramUniform3d.set_ptr(loadfn("glProgramUniform3d"));
        storage::ProgramUniform3dv.set_ptr(loadfn("glProgramUniform3dv"));
        storage::ProgramUniform3f.set_ptr(loadfn("glProgramUniform3f"));
        storage::ProgramUniform3fv.set_ptr(loadfn("glProgramUniform3fv"));
        storage::ProgramUniform3i.set_ptr(loadfn("glProgramUniform3i"));
        storage::ProgramUniform3iv.set_ptr(loadfn("glProgramUniform3iv"));
        storage::ProgramUniform3ui.set_ptr(loadfn("glProgramUniform3ui"));
        storage::ProgramUniform3uiv.set_ptr(loadfn("glProgramUniform3uiv"));
        storage::ProgramUniform4d.set_ptr(loadfn("glProgramUniform4d"));
        storage::ProgramUniform4dv.set_ptr(loadfn("glProgramUniform4dv"));
        storage::ProgramUniform4f.set_ptr(loadfn("glProgramUniform4f"));
        storage::ProgramUniform4fv.set_ptr(loadfn("glProgramUniform4fv"));
        storage::ProgramUniform4i.set_ptr(loadfn("glProgramUniform4i"));
        storage::ProgramUniform4iv.set_ptr(loadfn("glProgramUniform4iv"));
        storage::ProgramUniform4ui.set_ptr(loadfn("glProgramUniform4ui"));
        storage::ProgramUniform4uiv.set_ptr(loadfn("glProgramUniform4uiv"));
        storage::ProgramUniformMatrix2dv.set_ptr(loadfn("glProgramUniformMatrix2dv"));
        storage::ProgramUniformMatrix2fv.set_ptr(loadfn("glProgramUniformMatrix2fv"));
        storage::ProgramUniformMatrix2x3dv.set_ptr(loadfn("glProgramUniformMatrix2x3dv"));
        storage::ProgramUniformMatrix2x3fv.set_ptr(loadfn("glProgramUniformMatrix2x3fv"));
        storage::ProgramUniformMatrix2x4dv.set_ptr(loadfn("glProgramUniformMatrix2x4dv"));
        storage::ProgramUniformMatrix2x4fv.set_ptr(loadfn("glProgramUniformMatrix2x4fv"));
        storage::ProgramUniformMatrix3dv.set_ptr(loadfn("glProgramUniformMatrix3dv"));
        storage::ProgramUniformMatrix3fv.set_ptr(loadfn("glProgramUniformMatrix3fv"));
        storage::ProgramUniformMatrix3x2dv.set_ptr(loadfn("glProgramUniformMatrix3x2dv"));
        storage::ProgramUniformMatrix3x2fv.set_ptr(loadfn("glProgramUniformMatrix3x2fv"));
        storage::ProgramUniformMatrix3x4dv.set_ptr(loadfn("glProgramUniformMatrix3x4dv"));
        storage::ProgramUniformMatrix3x4fv.set_ptr(loadfn("glProgramUniformMatrix3x4fv"));
        storage::ProgramUniformMatrix4dv.set_ptr(loadfn("glProgramUniformMatrix4dv"));
        storage::ProgramUniformMatrix4fv.set_ptr(loadfn("glProgramUniformMatrix4fv"));
        storage::ProgramUniformMatrix4x2dv.set_ptr(loadfn("glProgramUniformMatrix4x2dv"));
        storage::ProgramUniformMatrix4x2fv.set_ptr(loadfn("glProgramUniformMatrix4x2fv"));
        storage::ProgramUniformMatrix4x3dv.set_ptr(loadfn("glProgramUniformMatrix4x3dv"));
        storage::ProgramUniformMatrix4x3fv.set_ptr(loadfn("glProgramUniformMatrix4x3fv"));
        storage::ProvokingVertex.set_ptr(loadfn("glProvokingVertex"));
        storage::PushDebugGroup.set_ptr(loadfn("glPushDebugGroup"));
        storage::QueryCounter.set_ptr(loadfn("glQueryCounter"));
        storage::ReadBuffer.set_ptr(loadfn("glReadBuffer"));
        storage::ReadPixels.set_ptr(loadfn("glReadPixels"));
        storage::ReadnPixels.set_ptr(loadfn("glReadnPixels"));
        storage::ReleaseShaderCompiler.set_ptr(loadfn("glReleaseShaderCompiler"));
        storage::RenderbufferStorage.set_ptr(loadfn("glRenderbufferStorage"));
        storage::RenderbufferStorageMultisample.set_ptr(loadfn("glRenderbufferStorageMultisample"));
        storage::ResumeTransformFeedback.set_ptr(loadfn("glResumeTransformFeedback"));
        storage::SampleCoverage.set_ptr(loadfn("glSampleCoverage"));
        storage::SampleMaski.set_ptr(loadfn("glSampleMaski"));
        storage::SamplerParameterIiv.set_ptr(loadfn("glSamplerParameterIiv"));
        storage::SamplerParameterIuiv.set_ptr(loadfn("glSamplerParameterIuiv"));
        storage::SamplerParameterf.set_ptr(loadfn("glSamplerParameterf"));
        storage::SamplerParameterfv.set_ptr(loadfn("glSamplerParameterfv"));
        storage::SamplerParameteri.set_ptr(loadfn("glSamplerParameteri"));
        storage::SamplerParameteriv.set_ptr(loadfn("glSamplerParameteriv"));
        storage::Scissor.set_ptr(loadfn("glScissor"));
        storage::ScissorArrayv.set_ptr(loadfn("glScissorArrayv"));
        storage::ScissorIndexed.set_ptr(loadfn("glScissorIndexed"));
        storage::ScissorIndexedv.set_ptr(loadfn("glScissorIndexedv"));
        storage::ShaderBinary.set_ptr(loadfn("glShaderBinary"));
        storage::ShaderSource.set_ptr(loadfn("glShaderSource"));
        storage::ShaderStorageBlockBinding.set_ptr(loadfn("glShaderStorageBlockBinding"));
        storage::SpecializeShader.set_ptr(loadfn("glSpecializeShader"));
        storage::StencilFunc.set_ptr(loadfn("glStencilFunc"));
        storage::StencilFuncSeparate.set_ptr(loadfn("glStencilFuncSeparate"));
        storage::StencilMask.set_ptr(loadfn("glStencilMask"));
        storage::StencilMaskSeparate.set_ptr(loadfn("glStencilMaskSeparate"));
        storage::StencilOp.set_ptr(loadfn("glStencilOp"));
        storage::StencilOpSeparate.set_ptr(loadfn("glStencilOpSeparate"));
        storage::TexBuffer.set_ptr(loadfn("glTexBuffer"));
        storage::TexBufferRange.set_ptr(loadfn("glTexBufferRange"));
        storage::TexImage1D.set_ptr(loadfn("glTexImage1D"));
        storage::TexImage2D.set_ptr(loadfn("glTexImage2D"));
        storage::TexImage2DMultisample.set_ptr(loadfn("glTexImage2DMultisample"));
        storage::TexImage3D.set_ptr(loadfn("glTexImage3D"));
        storage::TexImage3DMultisample.set_ptr(loadfn("glTexImage3DMultisample"));
        storage::TexParameterIiv.set_ptr(loadfn("glTexParameterIiv"));
        storage::TexParameterIuiv.set_ptr(loadfn("glTexParameterIuiv"));
        storage::TexParameterf.set_ptr(loadfn("glTexParameterf"));
        storage::TexParameterfv.set_ptr(loadfn("glTexParameterfv"));
        storage::TexParameteri.set_ptr(loadfn("glTexParameteri"));
        storage::TexParameteriv.set_ptr(loadfn("glTexParameteriv"));
        storage::TexStorage1D.set_ptr(loadfn("glTexStorage1D"));
        storage::TexStorage2D.set_ptr(loadfn("glTexStorage2D"));
        storage::TexStorage2DMultisample.set_ptr(loadfn("glTexStorage2DMultisample"));
        storage::TexStorage3D.set_ptr(loadfn("glTexStorage3D"));
        storage::TexStorage3DMultisample.set_ptr(loadfn("glTexStorage3DMultisample"));
        storage::TexSubImage1D.set_ptr(loadfn("glTexSubImage1D"));
        storage::TexSubImage2D.set_ptr(loadfn("glTexSubImage2D"));
        storage::TexSubImage3D.set_ptr(loadfn("glTexSubImage3D"));
        storage::TextureBarrier.set_ptr(loadfn("glTextureBarrier"));
        storage::TextureBuffer.set_ptr(loadfn("glTextureBuffer"));
        storage::TextureBufferRange.set_ptr(loadfn("glTextureBufferRange"));
        storage::TextureParameterIiv.set_ptr(loadfn("glTextureParameterIiv"));
        storage::TextureParameterIuiv.set_ptr(loadfn("glTextureParameterIuiv"));
        storage::TextureParameterf.set_ptr(loadfn("glTextureParameterf"));
        storage::TextureParameterfv.set_ptr(loadfn("glTextureParameterfv"));
        storage::TextureParameteri.set_ptr(loadfn("glTextureParameteri"));
        storage::TextureParameteriv.set_ptr(loadfn("glTextureParameteriv"));
        storage::TextureStorage1D.set_ptr(loadfn("glTextureStorage1D"));
        storage::TextureStorage2D.set_ptr(loadfn("glTextureStorage2D"));
        storage::TextureStorage2DMultisample.set_ptr(loadfn("glTextureStorage2DMultisample"));
        storage::TextureStorage3D.set_ptr(loadfn("glTextureStorage3D"));
        storage::TextureStorage3DMultisample.set_ptr(loadfn("glTextureStorage3DMultisample"));
        storage::TextureSubImage1D.set_ptr(loadfn("glTextureSubImage1D"));
        storage::TextureSubImage2D.set_ptr(loadfn("glTextureSubImage2D"));
        storage::TextureSubImage3D.set_ptr(loadfn("glTextureSubImage3D"));
        storage::TextureView.set_ptr(loadfn("glTextureView"));
        storage::TransformFeedbackBufferBase.set_ptr(loadfn("glTransformFeedbackBufferBase"));
        storage::TransformFeedbackBufferRange.set_ptr(loadfn("glTransformFeedbackBufferRange"));
        storage::TransformFeedbackVaryings.set_ptr(loadfn("glTransformFeedbackVaryings"));
        storage::Uniform1d.set_ptr(loadfn("glUniform1d"));
        storage::Uniform1dv.set_ptr(loadfn("glUniform1dv"));
        storage::Uniform1f.set_ptr(loadfn("glUniform1f"));
        storage::Uniform1fv.set_ptr(loadfn("glUniform1fv"));
        storage::Uniform1i.set_ptr(loadfn("glUniform1i"));
        storage::Uniform1iv.set_ptr(loadfn("glUniform1iv"));
        storage::Uniform1ui.set_ptr(loadfn("glUniform1ui"));
        storage::Uniform1uiv.set_ptr(loadfn("glUniform1uiv"));
        storage::Uniform2d.set_ptr(loadfn("glUniform2d"));
        storage::Uniform2dv.set_ptr(loadfn("glUniform2dv"));
        storage::Uniform2f.set_ptr(loadfn("glUniform2f"));
        storage::Uniform2fv.set_ptr(loadfn("glUniform2fv"));
        storage::Uniform2i.set_ptr(loadfn("glUniform2i"));
        storage::Uniform2iv.set_ptr(loadfn("glUniform2iv"));
        storage::Uniform2ui.set_ptr(loadfn("glUniform2ui"));
        storage::Uniform2uiv.set_ptr(loadfn("glUniform2uiv"));
        storage::Uniform3d.set_ptr(loadfn("glUniform3d"));
        storage::Uniform3dv.set_ptr(loadfn("glUniform3dv"));
        storage::Uniform3f.set_ptr(loadfn("glUniform3f"));
        storage::Uniform3fv.set_ptr(loadfn("glUniform3fv"));
        storage::Uniform3i.set_ptr(loadfn("glUniform3i"));
        storage::Uniform3iv.set_ptr(loadfn("glUniform3iv"));
        storage::Uniform3ui.set_ptr(loadfn("glUniform3ui"));
        storage::Uniform3uiv.set_ptr(loadfn("glUniform3uiv"));
        storage::Uniform4d.set_ptr(loadfn("glUniform4d"));
        storage::Uniform4dv.set_ptr(loadfn("glUniform4dv"));
        storage::Uniform4f.set_ptr(loadfn("glUniform4f"));
        storage::Uniform4fv.set_ptr(loadfn("glUniform4fv"));
        storage::Uniform4i.set_ptr(loadfn("glUniform4i"));
        storage::Uniform4iv.set_ptr(loadfn("glUniform4iv"));
        storage::Uniform4ui.set_ptr(loadfn("glUniform4ui"));
        storage::Uniform4uiv.set_ptr(loadfn("glUniform4uiv"));
        storage::UniformBlockBinding.set_ptr(loadfn("glUniformBlockBinding"));
        storage::UniformMatrix2dv.set_ptr(loadfn("glUniformMatrix2dv"));
        storage::UniformMatrix2fv.set_ptr(loadfn("glUniformMatrix2fv"));
        storage::UniformMatrix2x3dv.set_ptr(loadfn("glUniformMatrix2x3dv"));
        storage::UniformMatrix2x3fv.set_ptr(loadfn("glUniformMatrix2x3fv"));
        storage::UniformMatrix2x4dv.set_ptr(loadfn("glUniformMatrix2x4dv"));
        storage::UniformMatrix2x4fv.set_ptr(loadfn("glUniformMatrix2x4fv"));
        storage::UniformMatrix3dv.set_ptr(loadfn("glUniformMatrix3dv"));
        storage::UniformMatrix3fv.set_ptr(loadfn("glUniformMatrix3fv"));
        storage::UniformMatrix3x2dv.set_ptr(loadfn("glUniformMatrix3x2dv"));
        storage::UniformMatrix3x2fv.set_ptr(loadfn("glUniformMatrix3x2fv"));
        storage::UniformMatrix3x4dv.set_ptr(loadfn("glUniformMatrix3x4dv"));
        storage::UniformMatrix3x4fv.set_ptr(loadfn("glUniformMatrix3x4fv"));
        storage::UniformMatrix4dv.set_ptr(loadfn("glUniformMatrix4dv"));
        storage::UniformMatrix4fv.set_ptr(loadfn("glUniformMatrix4fv"));
        storage::UniformMatrix4x2dv.set_ptr(loadfn("glUniformMatrix4x2dv"));
        storage::UniformMatrix4x2fv.set_ptr(loadfn("glUniformMatrix4x2fv"));
        storage::UniformMatrix4x3dv.set_ptr(loadfn("glUniformMatrix4x3dv"));
        storage::UniformMatrix4x3fv.set_ptr(loadfn("glUniformMatrix4x3fv"));
        storage::UniformSubroutinesuiv.set_ptr(loadfn("glUniformSubroutinesuiv"));
        storage::UnmapBuffer.set_ptr(loadfn("glUnmapBuffer"));
        storage::UnmapNamedBuffer.set_ptr(loadfn("glUnmapNamedBuffer"));
        storage::UseProgram.set_ptr(loadfn("glUseProgram"));
        storage::UseProgramStages.set_ptr(loadfn("glUseProgramStages"));
        storage::ValidateProgram.set_ptr(loadfn("glValidateProgram"));
        storage::ValidateProgramPipeline.set_ptr(loadfn("glValidateProgramPipeline"));
        storage::VertexArrayAttribBinding.set_ptr(loadfn("glVertexArrayAttribBinding"));
        storage::VertexArrayAttribFormat.set_ptr(loadfn("glVertexArrayAttribFormat"));
        storage::VertexArrayAttribIFormat.set_ptr(loadfn("glVertexArrayAttribIFormat"));
        storage::VertexArrayAttribLFormat.set_ptr(loadfn("glVertexArrayAttribLFormat"));
        storage::VertexArrayBindingDivisor.set_ptr(loadfn("glVertexArrayBindingDivisor"));
        storage::VertexArrayElementBuffer.set_ptr(loadfn("glVertexArrayElementBuffer"));
        storage::VertexArrayVertexBuffer.set_ptr(loadfn("glVertexArrayVertexBuffer"));
        storage::VertexArrayVertexBuffers.set_ptr(loadfn("glVertexArrayVertexBuffers"));
        storage::VertexAttrib1d.set_ptr(loadfn("glVertexAttrib1d"));
        storage::VertexAttrib1dv.set_ptr(loadfn("glVertexAttrib1dv"));
        storage::VertexAttrib1f.set_ptr(loadfn("glVertexAttrib1f"));
        storage::VertexAttrib1fv.set_ptr(loadfn("glVertexAttrib1fv"));
        storage::VertexAttrib1s.set_ptr(loadfn("glVertexAttrib1s"));
        storage::VertexAttrib1sv.set_ptr(loadfn("glVertexAttrib1sv"));
        storage::VertexAttrib2d.set_ptr(loadfn("glVertexAttrib2d"));
        storage::VertexAttrib2dv.set_ptr(loadfn("glVertexAttrib2dv"));
        storage::VertexAttrib2f.set_ptr(loadfn("glVertexAttrib2f"));
        storage::VertexAttrib2fv.set_ptr(loadfn("glVertexAttrib2fv"));
        storage::VertexAttrib2s.set_ptr(loadfn("glVertexAttrib2s"));
        storage::VertexAttrib2sv.set_ptr(loadfn("glVertexAttrib2sv"));
        storage::VertexAttrib3d.set_ptr(loadfn("glVertexAttrib3d"));
        storage::VertexAttrib3dv.set_ptr(loadfn("glVertexAttrib3dv"));
        storage::VertexAttrib3f.set_ptr(loadfn("glVertexAttrib3f"));
        storage::VertexAttrib3fv.set_ptr(loadfn("glVertexAttrib3fv"));
        storage::VertexAttrib3s.set_ptr(loadfn("glVertexAttrib3s"));
        storage::VertexAttrib3sv.set_ptr(loadfn("glVertexAttrib3sv"));
        storage::VertexAttrib4Nbv.set_ptr(loadfn("glVertexAttrib4Nbv"));
        storage::VertexAttrib4Niv.set_ptr(loadfn("glVertexAttrib4Niv"));
        storage::VertexAttrib4Nsv.set_ptr(loadfn("glVertexAttrib4Nsv"));
        storage::VertexAttrib4Nub.set_ptr(loadfn("glVertexAttrib4Nub"));
        storage::VertexAttrib4Nubv.set_ptr(loadfn("glVertexAttrib4Nubv"));
        storage::VertexAttrib4Nuiv.set_ptr(loadfn("glVertexAttrib4Nuiv"));
        storage::VertexAttrib4Nusv.set_ptr(loadfn("glVertexAttrib4Nusv"));
        storage::VertexAttrib4bv.set_ptr(loadfn("glVertexAttrib4bv"));
        storage::VertexAttrib4d.set_ptr(loadfn("glVertexAttrib4d"));
        storage::VertexAttrib4dv.set_ptr(loadfn("glVertexAttrib4dv"));
        storage::VertexAttrib4f.set_ptr(loadfn("glVertexAttrib4f"));
        storage::VertexAttrib4fv.set_ptr(loadfn("glVertexAttrib4fv"));
        storage::VertexAttrib4iv.set_ptr(loadfn("glVertexAttrib4iv"));
        storage::VertexAttrib4s.set_ptr(loadfn("glVertexAttrib4s"));
        storage::VertexAttrib4sv.set_ptr(loadfn("glVertexAttrib4sv"));
        storage::VertexAttrib4ubv.set_ptr(loadfn("glVertexAttrib4ubv"));
        storage::VertexAttrib4uiv.set_ptr(loadfn("glVertexAttrib4uiv"));
        storage::VertexAttrib4usv.set_ptr(loadfn("glVertexAttrib4usv"));
        storage::VertexAttribBinding.set_ptr(loadfn("glVertexAttribBinding"));
        storage::VertexAttribDivisor.set_ptr(loadfn("glVertexAttribDivisor"));
        storage::VertexAttribFormat.set_ptr(loadfn("glVertexAttribFormat"));
        storage::VertexAttribI1i.set_ptr(loadfn("glVertexAttribI1i"));
        storage::VertexAttribI1iv.set_ptr(loadfn("glVertexAttribI1iv"));
        storage::VertexAttribI1ui.set_ptr(loadfn("glVertexAttribI1ui"));
        storage::VertexAttribI1uiv.set_ptr(loadfn("glVertexAttribI1uiv"));
        storage::VertexAttribI2i.set_ptr(loadfn("glVertexAttribI2i"));
        storage::VertexAttribI2iv.set_ptr(loadfn("glVertexAttribI2iv"));
        storage::VertexAttribI2ui.set_ptr(loadfn("glVertexAttribI2ui"));
        storage::VertexAttribI2uiv.set_ptr(loadfn("glVertexAttribI2uiv"));
        storage::VertexAttribI3i.set_ptr(loadfn("glVertexAttribI3i"));
        storage::VertexAttribI3iv.set_ptr(loadfn("glVertexAttribI3iv"));
        storage::VertexAttribI3ui.set_ptr(loadfn("glVertexAttribI3ui"));
        storage::VertexAttribI3uiv.set_ptr(loadfn("glVertexAttribI3uiv"));
        storage::VertexAttribI4bv.set_ptr(loadfn("glVertexAttribI4bv"));
        storage::VertexAttribI4i.set_ptr(loadfn("glVertexAttribI4i"));
        storage::VertexAttribI4iv.set_ptr(loadfn("glVertexAttribI4iv"));
        storage::VertexAttribI4sv.set_ptr(loadfn("glVertexAttribI4sv"));
        storage::VertexAttribI4ubv.set_ptr(loadfn("glVertexAttribI4ubv"));
        storage::VertexAttribI4ui.set_ptr(loadfn("glVertexAttribI4ui"));
        storage::VertexAttribI4uiv.set_ptr(loadfn("glVertexAttribI4uiv"));
        storage::VertexAttribI4usv.set_ptr(loadfn("glVertexAttribI4usv"));
        storage::VertexAttribIFormat.set_ptr(loadfn("glVertexAttribIFormat"));
        storage::VertexAttribIPointer.set_ptr(loadfn("glVertexAttribIPointer"));
        storage::VertexAttribL1d.set_ptr(loadfn("glVertexAttribL1d"));
        storage::VertexAttribL1dv.set_ptr(loadfn("glVertexAttribL1dv"));
        storage::VertexAttribL2d.set_ptr(loadfn("glVertexAttribL2d"));
        storage::VertexAttribL2dv.set_ptr(loadfn("glVertexAttribL2dv"));
        storage::VertexAttribL3d.set_ptr(loadfn("glVertexAttribL3d"));
        storage::VertexAttribL3dv.set_ptr(loadfn("glVertexAttribL3dv"));
        storage::VertexAttribL4d.set_ptr(loadfn("glVertexAttribL4d"));
        storage::VertexAttribL4dv.set_ptr(loadfn("glVertexAttribL4dv"));
        storage::VertexAttribLFormat.set_ptr(loadfn("glVertexAttribLFormat"));
        storage::VertexAttribLPointer.set_ptr(loadfn("glVertexAttribLPointer"));
        storage::VertexAttribP1ui.set_ptr(loadfn("glVertexAttribP1ui"));
        storage::VertexAttribP1uiv.set_ptr(loadfn("glVertexAttribP1uiv"));
        storage::VertexAttribP2ui.set_ptr(loadfn("glVertexAttribP2ui"));
        storage::VertexAttribP2uiv.set_ptr(loadfn("glVertexAttribP2uiv"));
        storage::VertexAttribP3ui.set_ptr(loadfn("glVertexAttribP3ui"));
        storage::VertexAttribP3uiv.set_ptr(loadfn("glVertexAttribP3uiv"));
        storage::VertexAttribP4ui.set_ptr(loadfn("glVertexAttribP4ui"));
        storage::VertexAttribP4uiv.set_ptr(loadfn("glVertexAttribP4uiv"));
        storage::VertexAttribPointer.set_ptr(loadfn("glVertexAttribPointer"));
        storage::VertexBindingDivisor.set_ptr(loadfn("glVertexBindingDivisor"));
        storage::Viewport.set_ptr(loadfn("glViewport"));
        storage::ViewportArrayv.set_ptr(loadfn("glViewportArrayv"));
        storage::ViewportIndexedf.set_ptr(loadfn("glViewportIndexedf"));
        storage::ViewportIndexedfv.set_ptr(loadfn("glViewportIndexedfv"));
        storage::WaitSync.set_ptr(loadfn("glWaitSync"));
    }
}
