# 🌌 midnight
Emacs-inspired modern TUI platform and editor. Written in Rust.

Features:

- All-in-one platform for the Terminal; not just an editor.
- Tiling panel managment
- Standard applets written in Rust
- Syntax highlightings

Implemented applets:
- Helium: simple text editor inspired by Emacs
- Thè: text-based terminal emulator
- editbar: polybar-inspired tool bar, customizable with different Widgets
- tabs: contain different Applets insides, switchable using tabs

Coming soon:

- IPC support (`emacsd`-like)
- Full customization
- LSP support
- MoonKnight (File explorer and Project tree)

## Keybindings

Midnight keybindings:
- `M+x`: open Command prompt
- `M+o`: Open file in Helium
- `Ctrl+n`: New file in Helium
- `M+q`: Quit midnight
- `M+e`: kill current Applet
- `Ctrl+r`: Resize current Panel
- `Ctrl+m`: Move current Panel

Helium keybindings:
- Most Emacs keybindings are compatible.
- `Ctrl+s`: Save current file

Thè keybindings:
(No keybindings)

Tabs keybindings:
- `M+w`: Kill current Tab

## Install

### I. Compiling (RACCOMENDED)

- `git clone https://github.com/djtech-dev/midnight.git`
- `git submodule init`
- `cargo rustc --release --features=[...] -- -C target-cpu=native`
You need to insert your selected features. (`[]` for a minimalistic experience)

There are two advantages with this method:
- Every single extra feature in Midnight is a Cargo features, so you can choose exactly what will be in your editor and what will be cut off.
- `-- -C target-cpu=native` uses rustc's features for better optimization for your specific hardware

### II. From releases

With the future release of `v1.0.0-rc1` (first public release), the executables will be avaiable in Github Releases.
