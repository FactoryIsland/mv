pub const NOOP: u8 = 0;
pub const END: u8 = 1;
pub const STORE: u8 = 2;
pub const JMP: u8 = 10;
pub const PRINT: u8 = 20;
pub const SH: u8 = 21;
pub const GIT_ADD_ALL: u8 = 22;
pub const GIT_ADD: u8 = 23;
pub const GIT_COMMIT_DEFAULT: u8 = 24;
pub const GIT_COMMIT: u8 = 25;
pub const GIT_PUSH_UPSTREAM: u8 = 26;
pub const GIT_PUSH: u8 = 27;

pub const LITERAL: char = '#';
pub const ARGUMENT: char = '%';
pub const VARIABLE: char = '$';