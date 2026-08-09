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

//
// Additional text styles (SGR)
//

/// Rapidly blinking text. Not widely supported.
pub const BLINK_RAPID: &str = "\x1b[6m";

/// Select the primary font.
pub const FONT_PRIMARY: &str = "\x1b[10m";

/// Select an alternative font.
pub const FONT_ALT1: &str = "\x1b[11m";

/// Select an alternative font.
pub const FONT_ALT2: &str = "\x1b[12m";

/// Select an alternative font.
pub const FONT_ALT3: &str = "\x1b[13m";

/// Select an alternative font.
pub const FONT_ALT4: &str = "\x1b[14m";

/// Select an alternative font.
pub const FONT_ALT5: &str = "\x1b[15m";

/// Select an alternative font.
pub const FONT_ALT6: &str = "\x1b[16m";

/// Select an alternative font.
pub const FONT_ALT7: &str = "\x1b[17m";

/// Select an alternative font.
pub const FONT_ALT8: &str = "\x1b[18m";

/// Select an alternative font.
pub const FONT_ALT9: &str = "\x1b[19m";

/// Select Fraktur (Blackletter) font. Rarely supported.
pub const FONT_FRAKTUR: &str = "\x1b[20m";

/// Double underline. Sometimes interpreted as "disable bold."
pub const UNDERLINE_DOUBLE: &str = "\x1b[21m";

/// Remove dim formatting.
pub const NO_DIM: &str = "\x1b[22m";

/// Enable proportional spacing.
pub const PROPORTIONAL_SPACING: &str = "\x1b[26m";

/// Reset foreground color to the default.
pub const FG_DEFAULT: &str = "\x1b[39m";

/// Reset background color to the default.
pub const BG_DEFAULT: &str = "\x1b[49m";

/// Disable proportional spacing.
pub const NO_PROPORTIONAL_SPACING: &str = "\x1b[50m";

/// Frame the text.
pub const FRAMED: &str = "\x1b[51m";

/// Encircle the text.
pub const ENCIRCLED: &str = "\x1b[52m";

/// Overline the text.
pub const OVERLINED: &str = "\x1b[53m";

/// Remove frame and encircle formatting.
pub const NO_FRAME_ENCIRCLE: &str = "\x1b[54m";

/// Remove overline formatting.
pub const NO_OVERLINE: &str = "\x1b[55m";

//
// Control characters
//

/// Bell (audible beep).
pub const BELL: &str = "\x07";

/// Backspace.
pub const BACKSPACE: &str = "\x08";

/// Escape.
pub const ESC: &str = "\x1b";

/// Control Sequence Introducer.
pub const CSI: &str = "\x1b[";

/// Operating System Command.
pub const OSC: &str = "\x1b]";

/// String Terminator.
pub const ST: &str = "\x1b\\";

//
// Cursor movement & editing (CSI)
//

/// Move the cursor up one line.
pub const CURSOR_UP: &str = "\x1b[A";

/// Move the cursor down one line.
pub const CURSOR_DOWN: &str = "\x1b[B";

/// Move the cursor forward one column.
pub const CURSOR_FORWARD: &str = "\x1b[C";

/// Move the cursor back one column.
pub const CURSOR_BACK: &str = "\x1b[D";

/// Move the cursor to the home position (top-left).
pub const CURSOR_HOME: &str = "\x1b[H";

/// Erase from the cursor to the end of the display.
pub const ERASE_DISPLAY: &str = "\x1b[J";

/// Erase the entire display.
pub const ERASE_DISPLAY_ALL: &str = "\x1b[2J";

/// Erase from the cursor to the end of the line.
pub const ERASE_LINE: &str = "\x1b[K";

/// Scroll the display up one line.
pub const SCROLL_UP: &str = "\x1b[S";

/// Scroll the display down one line.
pub const SCROLL_DOWN: &str = "\x1b[T";

//
// Private CSI / DEC sequences
//

/// Save the current cursor position.
pub const SAVE_CURSOR: &str = "\x1b[s";

/// Restore a saved cursor position.
pub const RESTORE_CURSOR: &str = "\x1b[u";

/// Enable auto-wrap mode.
pub const AUTO_WRAP_ON: &str = "\x1b[?7h";

/// Disable auto-wrap mode.
pub const AUTO_WRAP_OFF: &str = "\x1b[?7l";

/// Show the text cursor.
pub const SHOW_CURSOR: &str = "\x1b[?25h";

/// Hide the text cursor.
pub const HIDE_CURSOR: &str = "\x1b[?25l";

/// Enter the alternate screen buffer.
pub const ENTER_ALTERNATE_SCREEN: &str = "\x1b[?1049h";

/// Exit the alternate screen buffer.
pub const EXIT_ALTERNATE_SCREEN: &str = "\x1b[?1049l";

//
// Fp escape sequences (DEC)
//

/// DEC: save the current cursor position.
pub const DEC_SAVE_CURSOR: &str = "\x1b7";

/// DEC: restore a saved cursor position.
pub const DEC_RESTORE_CURSOR: &str = "\x1b8";

/// Move the cursor to the given `row` and `column` (1-based).
#[must_use]
pub fn cursor_position(row: u16, col: u16) -> String {
    format!("\x1b[{row};{col}H")
}

/// Move the cursor up by `lines` lines.
#[must_use]
pub fn cursor_up(lines: u16) -> String {
    format!("{CSI}{lines}A")
}

/// Move the cursor down by `lines` lines.
#[must_use]
pub fn cursor_down(lines: u16) -> String {
    format!("{CSI}{lines}B")
}

/// Move the cursor forward by `cols` columns.
#[must_use]
pub fn cursor_forward(cols: u16) -> String {
    format!("{CSI}{cols}C")
}

/// Move the cursor back by `cols` columns.
#[must_use]
pub fn cursor_back(cols: u16) -> String {
    format!("{CSI}{cols}D")
}

//
// OSC sequences
//

/// Set the terminal window title, terminated with a Bell.
#[must_use]
pub fn set_window_title(title: &str) -> String {
    format!("\x1b]0;{title}\x07")
}

/// Emit an OSC hyperlink sequence opening `uri`. The following text is shown
/// as a link until the link is closed with [`close_hyperlink`].
#[must_use]
pub fn hyperlink(uri: &str) -> String {
    format!("\x1b]8;;{uri}\x07")
}

/// Close an OSC hyperlink opened with [`hyperlink`].
#[must_use]
pub fn close_hyperlink() -> String {
    "\x1b]8;;\x07".to_owned()
}

/// Set the terminal clipboard to the given `base64`-encoded string.
///
/// Valid values are `c` (clipboard) or `p` (primary selection). Providing an
/// empty `base64` requests the terminal to report the current contents.
#[must_use]
pub fn set_clipboard(base64: &str) -> String {
    format!("\x1b]52;c;{base64}\x07")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn styles() {
        assert_eq!(RESET, "\x1b[0m");
        assert_eq!(BOLD, "\x1b[1m");
        assert_eq!(DIM, "\x1b[2m");
        assert_eq!(ITALIC, "\x1b[3m");
        assert_eq!(UNDERLINE, "\x1b[4m");
        assert_eq!(BLINK, "\x1b[5m");
        assert_eq!(REVERSE, "\x1b[7m");
        assert_eq!(HIDDEN, "\x1b[8m");
        assert_eq!(STRIKETHROUGH, "\x1b[9m");
        assert_eq!(UNDERLINE_DOUBLE, "\x1b[21m");
        assert_eq!(FRAMED, "\x1b[51m");
        assert_eq!(ENCIRCLED, "\x1b[52m");
        assert_eq!(OVERLINED, "\x1b[53m");
        assert_eq!(FONT_PRIMARY, "\x1b[10m");
        assert_eq!(FONT_FRAKTUR, "\x1b[20m");
    }

    #[test]
    fn colors() {
        assert_eq!(RED, "\x1b[31m");
        assert_eq!(BRIGHT_WHITE, "\x1b[97m");
        assert_eq!(BG_BLUE, "\x1b[44m");
        assert_eq!(BG_BRIGHT_GREEN, "\x1b[102m");
        assert_eq!(FG_DEFAULT, "\x1b[39m");
        assert_eq!(BG_DEFAULT, "\x1b[49m");
    }

    #[test]
    fn control_and_cursor() {
        assert_eq!(ESC, "\x1b");
        assert_eq!(CSI, "\x1b[");
        assert_eq!(OSC, "\x1b]");
        assert_eq!(ST, "\x1b\\");
        assert_eq!(BELL, "\x07");
        assert_eq!(CURSOR_UP, "\x1b[A");
        assert_eq!(CURSOR_HOME, "\x1b[H");
        assert_eq!(ERASE_DISPLAY_ALL, "\x1b[2J");
        assert_eq!(HIDE_CURSOR, "\x1b[?25l");
        assert_eq!(SHOW_CURSOR, "\x1b[?25h");
        assert_eq!(ENTER_ALTERNATE_SCREEN, "\x1b[?1049h");
        assert_eq!(EXIT_ALTERNATE_SCREEN, "\x1b[?1049l");
        assert_eq!(DEC_SAVE_CURSOR, "\x1b7");
        assert_eq!(DEC_RESTORE_CURSOR, "\x1b8");
    }

    #[test]
    fn helpers() {
        assert_eq!(rgb(255, 128, 0), "\x1b[38;2;255;128;0m");
        assert_eq!(bg_rgb(0, 0, 0), "\x1b[48;2;0;0;0m");
        assert_eq!(color256(208), "\x1b[38;5;208m");
        assert_eq!(bg_color256(27), "\x1b[48;5;27m");
        assert_eq!(cursor_position(2, 5), "\x1b[2;5H");
        assert_eq!(cursor_up(3), "\x1b[3A");
        assert_eq!(cursor_down(1), "\x1b[1B");
        assert_eq!(cursor_forward(2), "\x1b[2C");
        assert_eq!(cursor_back(4), "\x1b[4D");
        assert_eq!(set_window_title("hi"), "\x1b]0;hi\x07");
        assert_eq!(
            hyperlink("https://example.com"),
            "\x1b]8;;https://example.com\x07"
        );
        assert_eq!(close_hyperlink(), "\x1b]8;;\x07");
        assert_eq!(set_clipboard("QUJD"), "\x1b]52;c;QUJD\x07");
    }
}
