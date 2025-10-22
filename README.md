
# Hotcorn

Hotcorn is a lightweight tool for **hot corners** and **edge triggers** in **Hyprland**.
It allows you to assign actions to screen corners, edges, and arbitrary rectangular zones.

---

## Features

- Hot corners with configurable radius
- Edge triggers with width and height (x/y calculated automatically)
- Arbitrary rectangular zones with custom coordinates and size
- Unique corners and edges — duplicates are automatically ignored
- Supports actions (`workspace`, `exec`, `global`)

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/hotcorn.git
cd hotcorn
cargo build --release
```

2. Create a configuration file:

```bash
cp config.example.toml $HOME/.config/hotcorn/config.toml
```

3. Edit `config.toml` to suit your needs (see example below).

---

## Configuration

Example `config.toml`:

```toml
monitor_name = "eDP-1"
timeout_ms = 100       # ms between checks
sticky_ms = 300        # minimal interval between triggers for the same area


[[triggers]]
# Top-right corner
type = "Corner"
position = "TopRight"
radius = 20
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Top-right corner" }


[[triggers]]
# Bottom-right corner
type = "Corner"
position = "BottomRight"
radius = 20
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Bottom-right corner" }


[[triggers]]
# Top-left corner
type = "Corner"
position = "TopLeft"
radius = 20
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Top-left corner" }


[[triggers]]
# Bottom-left corner
type = "Corner"
position = "BottomLeft"
radius = 20
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Bottom-left corner" }


[[triggers]]
# Top edge
type = "Edge"
position = "Top"
width = 200      # horizontal size (centered automatically)
height = 50      # thickness
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Top edge" }


[[triggers]]
# Right edge
type = "Edge"
position = "Right"
width = 50       # thickness
height = 200     # vertical size (centered automatically)
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Right edge" }


[[triggers]]
# Bottom edge
type = "Edge"
position = "Bottom"
width = 200       # horizontal size (centered automatically)
height = 50     # vertical size (centered automatically)
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Bottom edge" }



[[triggers]]
# Left edge
type = "Edge"
position = "Left"
width = 50       # thickness
height = 200     # vertical size (centered automatically)
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Left edge" }



[[triggers]]
# Arbitrary rectangle
type = "Rect"
x = 300
y = 0
width = 100
height = 40
action = { dispatcher = "exec", args = "hyprctl notify 5 2000 0 Arbitrary rectangle" }

```

### Notes

- **Corners**: configured via `radius`.
- **Edges**: configured via `width` and `height`; x/y are automatically calculated.
- **Rect**: fully configurable coordinates and size.
- Duplicate corners and edges are ignored.

---

## Usage

Simply run `hotcorn` after starting Hyprland:

```bash
~/.cargo/bin/hotcorn
```

You can add it to autostart via `.config/hypr/hyprland.conf`:

```ini
exec-once = ~/.cargo/bin/hotcorn
```
