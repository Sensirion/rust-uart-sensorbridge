# -*- coding: utf-8 -*-


def make_resp(addr: int, cmd: int, state: int, data: bytes) -> bytes:
    content = bytearray([addr, cmd, state, len(data)]) + bytearray(data)
    checksum = (~sum(content)) & 0xFF
    content.append(checksum)
    stuffed = bytearray()
    for b in content:
        if b in [0x7E, 0x7D, 0x11, 0x13]:
            stuffed.append(0x7D)
            stuffed.append(b ^ 0x20)
        else:
            stuffed.append(b)
    return bytes(bytearray([0x7E]) + stuffed + bytearray([0x7E]))
