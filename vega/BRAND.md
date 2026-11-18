# VEGA brand

**Name:** VEGA, after one of the brightest stars in the night sky (α Lyrae).
**Tagline:** Code Beyond Limits.
**One-liner:** An independent, open-source, AI-powered code editor.

## Palette

| Token | Hex | Use |
|---|---|---|
| Cosmic Purple | `#7C3AED` | Primary brand, primary actions |
| Electric Blue | `#00D4FF` | Secondary highlights, focus accents |
| Nebula | `#A855F7` | Gradient partner for purple |
| Deep Space | `#0B0B1A` | Main dark background (site, splash, icon) |
| Editor BG | `#101018` | Default editor background |
| Surface | `#151526` | Panels, popovers, elevated surfaces |
| Starlight | `#E6E9FF` | Main text |
| Muted | `#9294AA` | Secondary text |

Brand gradient: `#7C3AED → #00D4FF` (135°). Use it for the icon, website hero and splash only — never across editor chrome.

Keep the editor theme independent from the brand: the default "VEGA Dark" theme may borrow Deep Space / Surface / Starlight, but syntax colors are chosen for long-session readability, and users can pick any theme.

## Icon

Source: `vega/brand/vega-icon.svg` (1024×1024, macOS squircle-style rounded square).
Motif: a geometric **V** with an orbital arc and a four-point star at its upper right.

Export every app icon (macOS PNGs, `Document.icns`, Windows `.ico`) with:

```sh
brew install librsvg   # once, for rsvg-convert
vega/brand/export-icons.sh
```

Channel variants reuse the shape and swap the gradient end color: stable `#00D4FF`, preview `#F472B6`, nightly `#10B981`, dev `#F59E0B`.
