# CSVLIVE - A minimal csv Editor

A fast, terminal-based CSV editor built with **Rust** and **ratatui**.
Designed for efficiency, it provides a modal editing experience with **Normal** and **Insert** modes—similar to Vim.

## Features

* Fast and lightweight (powered by Rust)
* Terminal UI using `ratatui` and `crossterm`
* Modal editing (Normal / Insert modes)
* Navigate and edit CSV files easily
* Inline cell editing
* Automatically save changes back to file before leaving the application

## Installation

### One line install for linux
```
curl -sSL https://raw.githubusercontent.com/imck037/csvlive/main/install.sh | bash
```

### Prerequisites

* Rust (latest stable recommended)

Install Rust via:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build from source

```bash
git clone https://github.com/imck037/csvlive.git
cd csvlive
cargo build --release
```

Run the application:

```bash
cargo run -- path/to/file.csv
```

---

## Usage

```bash
csvlive <file.csv>
```

Example:

```bash
csvlive data.csv
```

---

## Keybindings

### Normal Mode

| Key       | Action            |
| --------- | ----------------- |
| `h` / `←` | Move left         |
| `l` / `→` | Move right        |
| `k` / `↑` | Move up           |
| `j` / `↓` | Move down         |
| `i`       | Enter Insert mode |
| `q`       | Quit              |

### Insert Mode

| Key         | Action                |
| ----------- | --------------------- |
| `Esc`       | Return to Normal mode |
| Typing      | Edit current cell     |
| `Backspace` | Delete character      |

## Modes Explained

### Normal Mode

* Default mode on startup
* Used for navigation and commands
* No direct editing

### Insert Mode

* Activated with `i`
* Allows editing of the selected cell
* Press `Esc` to cancel edit and return to Normal mode

## Project Structure

```
src/
├── main.rs        # Entry point
├── app.rs         # App state and logic
├── ui.rs          # Rendering with ratatui
└── events.rs      # Event handling
```

## Built With

* [Rust](https://www.rust-lang.org/)
* [ratatui](https://github.com/ratatui-org/ratatui)
* [crossterm](https://github.com/crossterm-rs/crossterm)

## Contributing

Contributions are welcome!

1. Fork the repo
2. Create your feature branch
3. Commit your changes
4. Open a pull request

---

## License

This project is licensed under the MIT License.


## Acknowledgements

Inspired by terminal editors like **Vim** and modern TUI libraries.