//! # ansicodes
//!
//! A tiny, dependency-free crate providing ANSI escape sequences as string
//! constants for terminal colors and text styles.
//!
//! ## Example
//!
//! ```rust
//! use ansicodes::*;
//!
//! println!("{BRIGHT_CYAN}Hello, world!{RESET}");
//! println!("{BOLD}{RED}Error:{RESET} Something went wrong.");
//! println!("{UNDERLINE}{GREEN}Success!{RESET}");
//! ```
//!
//! ## 24-bit RGB Colors
//!
//! For colors outside the predefined palette, use [`rgb`] and [`bg_rgb`].
//!
//! ```rust
//! use ansicodes::*;
//!
//! println!("{}Orange text{}", rgb(255, 165, 0), RESET);
//! println!("{}White on purple{}", bg_rgb(128, 0, 255), RESET);
//! ```

#![forbid(unsafe_code)]

/// Reset all attributes.
pub const RESET: &str = "\x1b[0m";

/// Bold text.
pub const BOLD: &str = "\x1b[1m";

/// Dim text.
pub const DIM: &str = "\x1b[2m";

/// Italic text.
pub const ITALIC: &str = "\x1b[3m";

/// Underlined text.
pub const UNDERLINE: &str = "\x1b[4m";

/// Slow blinking text.
pub const BLINK: &str = "\x1b[5m";

/// Reverse foreground and background colors.
pub const REVERSE: &str = "\x1b[7m";

/// Hidden text.
pub const HIDDEN: &str = "\x1b[8m";

/// Strikethrough text.
pub const STRIKETHROUGH: &str = "\x1b[9m";

/// Remove bold formatting.
pub const NO_BOLD: &str = "\x1b[22m";

/// Remove italic formatting.
pub const NO_ITALIC: &str = "\x1b[23m";

/// Remove underline formatting.
pub const NO_UNDERLINE: &str = "\x1b[24m";

/// Remove blinking formatting.
pub const NO_BLINK: &str = "\x1b[25m";

/// Remove reverse formatting.
pub const NO_REVERSE: &str = "\x1b[27m";

/// Remove hidden formatting.
pub const NO_HIDDEN: &str = "\x1b[28m";

/// Remove strikethrough formatting.
pub const NO_STRIKETHROUGH: &str = "\x1b[29m";

//
// Standard foreground colors
//

pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

//
// Bright foreground colors
//

pub const BRIGHT_BLACK: &str = "\x1b[90m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

//
// Standard background colors
//

pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE: &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";

//
// Bright background colors
//

pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
pub const BG_BRIGHT_RED: &str = "\x1b[101m";
pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";

/// Create a 24-bit RGB foreground color escape sequence.
///
/// # Example
///
/// ```
/// use ansicodes::*;
///
/// println!("{}Hello{}", rgb(255, 128, 0), RESET);
/// ```
#[must_use]
pub fn rgb(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m")
}

/// Create a 24-bit RGB background color escape sequence.
///
/// # Example
///
/// ```
/// use ansicodes::*;
///
/// println!("{}Hello{}", bg_rgb(255, 128, 0), RESET);
/// ```
#[must_use]
pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{r};{g};{b}m")
}

/// Create an ANSI 256-color foreground escape sequence.
///
/// Valid values are `0..=255`.
#[must_use]
pub fn color256(index: u8) -> String {
    format!("\x1b[38;5;{index}m")
}

/// Create an ANSI 256-color background escape sequence.
///
/// Valid values are `0..=255`.
#[must_use]
pub fn bg_color256(index: u8) -> String {
    format!("\x1b[48;5;{index}m")
}
