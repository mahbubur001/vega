"""Packs PNG files into a Windows .ico (PNG-compressed entries, Vista+).

Usage: python3 png_to_ico.py out.ico in-16.png in-32.png ...
"""

import struct
import sys


def png_size(data: bytes) -> tuple[int, int]:
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise ValueError("not a PNG file")
    return struct.unpack(">II", data[16:24])


def main() -> None:
    out_path, *png_paths = sys.argv[1:]
    images = []
    for path in png_paths:
        with open(path, "rb") as f:
            data = f.read()
        images.append((png_size(data), data))

    header = struct.pack("<HHH", 0, 1, len(images))
    offset = len(header) + 16 * len(images)
    entries = b""
    for (width, height), data in images:
        # 256 px is stored as 0 in the one-byte width/height fields
        entries += struct.pack(
            "<BBBBHHII", width % 256, height % 256, 0, 0, 1, 32, len(data), offset
        )
        offset += len(data)

    with open(out_path, "wb") as f:
        f.write(header + entries + b"".join(data for _, data in images))


if __name__ == "__main__":
    main()
