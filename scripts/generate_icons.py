"""Generates all required Tauri icon files (PNG + ICO) for the build."""
import struct
import zlib
import os


def make_rgba_png(w: int, h: int, r: int = 69, g: int = 137, b: int = 210, a: int = 255) -> bytes:
    """Create a solid-colour RGBA PNG in pure Python."""
    def chunk(name: bytes, data: bytes) -> bytes:
        crc = zlib.crc32(name + data) & 0xFFFFFFFF
        return struct.pack('>I', len(data)) + name + data + struct.pack('>I', crc)

    ihdr = struct.pack('>IIBBBBB', w, h, 8, 6, 0, 0, 0)
    raw = b''.join(b'\x00' + bytes([r, g, b, a] * w) for _ in range(h))
    idat = zlib.compress(raw)
    return b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', ihdr) + chunk(b'IDAT', idat) + chunk(b'IEND', b'')


def make_ico(sizes: list[int]) -> bytes:
    """Bundle multiple PNGs into a single .ico file."""
    png_list = [(s, make_rgba_png(s, s)) for s in sizes]
    num = len(png_list)
    data_offset = 6 + 16 * num
    header = struct.pack('<HHH', 0, 1, num)
    dirs = b''
    imgs = b''
    off = data_offset
    for s, data in png_list:
        w = 0 if s == 256 else s
        h = 0 if s == 256 else s
        dirs += struct.pack('<BBBBHHII', w, h, 0, 0, 1, 32, len(data), off)
        imgs += data
        off += len(data)
    return header + dirs + imgs


if __name__ == '__main__':
    icons_dir = os.path.join(os.path.dirname(__file__), '..', 'src-tauri', 'icons')
    os.makedirs(icons_dir, exist_ok=True)

    sizes = [
        (32, 32, '32x32.png'),
        (128, 128, '128x128.png'),
        (256, 256, '128x128@2x.png'),
        (512, 512, 'icon.png'),
    ]
    for w, h, name in sizes:
        path = os.path.join(icons_dir, name)
        with open(path, 'wb') as f:
            f.write(make_rgba_png(w, h))
        print(f'  ✔  {name}')

    ico_path = os.path.join(icons_dir, 'icon.ico')
    with open(ico_path, 'wb') as f:
        f.write(make_ico([16, 32, 48, 256]))
    print('  ✔  icon.ico')

    # ICNS for macOS (referenced by tauri.conf.json bundle.icon).
    # A minimal 8-byte valid ICNS container (magic + file-length header, no image
    # atoms) is sufficient here because the build targets are Windows-only
    # (NSIS/MSI) and Tauri does not process .icns on non-macOS hosts.
    icns_path = os.path.join(icons_dir, 'icon.icns')
    with open(icns_path, 'wb') as f:
        f.write(b'icns' + struct.pack('>I', 8))
    print('  ✔  icon.icns')

    print('\nAll icons generated successfully.')
