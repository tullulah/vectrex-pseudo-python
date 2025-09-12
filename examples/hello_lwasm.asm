; lwasm-compatible version of hello.asm (converted DB/DW -> FCB/FDB)
; Should assemble with: powershell -File tools/lwasm.ps1 examples/hello_lwasm.asm build/hello_lwasm.bin

Intensity_5F    EQU     $F2A5       ; BIOS Intensity routine
Print_Str_d     EQU     $F37A       ; BIOS print routine
Wait_Recal      EQU     $F192       ; BIOS recalibration
music1          EQU     $FD0D       ; BIOS ROM music

        ORG     0
;---------------- HEADER ----------------
        FCC     "g GCE 1998"
        FCB     $80              ; copyright + terminator
        FDB     music1           ; music pointer
        FCB     $F8,$50,$20,$AA  ; height,width,rel y, rel x (-$56)
        FCC     "HELLO WORLD PROG 1"
        FCB     $80              ; title terminator
        FCB     0                ; reserved
;---------------- CODE ------------------
main:
        JSR     Wait_Recal
        JSR     Intensity_5F
        LDU     #hello_world_string
        LDA     #$10
        LDB     #-$50
        JSR     Print_Str_d
        BRA     main
;---------------- DATA ------------------
hello_world_string:
        FCC     "HELLO WORLD"
        FCB     $80
        END     main
