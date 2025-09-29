; High-level pseudo source equivalent of hello.asm
; Should compile to nearly identical ROM header + loop

META TITLE = "HELLO WORLD PROG 1"
META MUSIC = "music1"
META COPYRIGHT = "g GCE 2025"

def main():
    # one frame body (auto loop will add frame loop)
    PRINT_TEXT( -0x70, 0x10, "HELLO WORLD FUNCIONA")
