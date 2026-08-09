
# ansicodes

A tiny, dependency-free Rust crate providing ANSI escape sequences as string constants for terminal colors and text styles.

No macros, no formatting wrappers, no builder APIs. Just constants.

## Features

* Standard & bright ANSI foreground/background colors
* Text styles: bold, italic, underline, strike-through, frames, overline, double underline, and more
* ANSI 256-color and 24-bit truecolor RGB support
* Cursor movement, screen clearing, and scrolling (CSI sequences)
* Hide/show cursor and alternate screen buffer (DEC sequences)
* OSC sequences: window title and hyperlinks
* Zero dependencies
* No `unsafe` code

## Installation

```toml
[dependencies]
ansicodes = "0.2"
```

## Example

```rust
use ansicodes::*;

fn main() {
    println!("{BRIGHT_CYAN}Hello, world!{RESET}");

    println!(
        "{BOLD}{RED}Error:{RESET} Something went wrong."
    );

    println!(
        "{UNDERLINE}{GREEN}Success!{RESET}"
    );
}
```

Output:

```text
Hello, world!      // bright cyan
Error:             // bold red
Success!           // underlined green
```

## Available Styles

| Constant        | Description                       |
| --------------- | --------------------------------- |
| `RESET`         | Reset all styles and colors       |
| `BOLD`          | Bold text                         |
| `DIM`           | Dim text                          |
| `ITALIC`        | Italic text                       |
| `UNDERLINE`     | Underlined text                   |
| `BLINK`         | Blinking text                     |
| `REVERSE`       | Reverse foreground and background |
| `HIDDEN`        | Hidden text                       |
| `STRIKETHROUGH` | Strikethrough text                |

## Available Colors

### Foreground Colors

```rust
BLACK
RED
GREEN
YELLOW
BLUE
MAGENTA
CYAN
WHITE
```

### Bright Foreground Colors

```rust
BRIGHT_BLACK
BRIGHT_RED
BRIGHT_GREEN
BRIGHT_YELLOW
BRIGHT_BLUE
BRIGHT_MAGENTA
BRIGHT_CYAN
BRIGHT_WHITE
```

### Background Colors

```rust
BG_BLACK
BG_RED
BG_GREEN
BG_YELLOW
BG_BLUE
BG_MAGENTA
BG_CYAN
BG_WHITE
```

### Bright Background Colors

```rust
BG_BRIGHT_BLACK
BG_BRIGHT_RED
BG_BRIGHT_GREEN
BG_BRIGHT_YELLOW
BG_BRIGHT_BLUE
BG_BRIGHT_MAGENTA
BG_BRIGHT_CYAN
BG_BRIGHT_WHITE
```

## ANSI 256 Colors

```rust
use ansicodes::*;

println!("{}Orange{}", color256(208), RESET);
println!("{}Blue Background{}", bg_color256(27), RESET);
```

## RGB Colors

```rust
use ansicodes::*;

println!("{}Orange{}", rgb(255, 165, 0), RESET);
println!("{}Purple Background{}", bg_rgb(128, 0, 255), RESET);
```

## Sequences

Beyond colors and styles, the crate exposes escape sequences for cursor control,
screen clearing, and the alternate screen buffer:

```rust
use ansicodes::*;

print!("{HIDE_CURSOR}{ENTER_ALTERNATE_SCREEN}");
print!("{CURSOR_HOME}{ERASE_DISPLAY_ALL}");
println!("Hello in a clean screen!");
print!("{EXIT_ALTERNATE_SCREEN}{SHOW_CURSOR}");
```

It also provides small helpers for OSC sequences:

```rust
use ansicodes::*;

println!("{}{}ansicodes{}", set_window_title("ansicodes"), hyperlink("https://github.com/rivethorn/ansicodes-rs"), close_hyperlink());
```

## Why?

Most ANSI crates provide formatting APIs, builder patterns, traits, or macros.

`ansicodes` takes a simpler approach:

```rust
println!("{RED}Error:{RESET} File not found.");
```

Sometimes all you need is the escape sequence itself.

## License

MIT — see `LICENSE`.
