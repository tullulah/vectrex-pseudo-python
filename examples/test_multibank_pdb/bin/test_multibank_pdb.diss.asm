0000: 67           .db $67
0001: 20 47       BRA $004A
0003: 43           .db $43
0004: 45           .db $45
0005: 20 32       BRA $0039
0007: 30 32       LEAX (indexed)
0009: 35 80       PULS (regs)
000B: FD 0D F8    STD $0DF8
000E: 50           .db $50
000F: 20 BB       BRA $FFCC
0011: 4D           .db $4D
0012: 55           .db $55
0013: 4C           .db $4C
0014: 54           .db $54
0015: 49           .db $49
0016: 42           .db $42
0017: 41           .db $41
0018: 4E           .db $4E
0019: 4B           .db $4B
001A: 20 50       BRA $006C
001C: 44           .db $44
001D: 42           .db $42
001E: 20 54       BRA $0074
0020: 45           .db $45
0021: 53           .db $53
0022: 54           .db $54
0023: 80           .db $80
0024: 00           .db $00
0025: 86 D0       LDA #$D0
0027: 1F 8B       TFR
0029: 7F           .db $7F
002A: C8 0E       EORB #$0E
002C: 86 80       LDA #$80
002E: B7 D0 04    STA $D004
0031: 10 CE CB FF LDS #$CBFF
0035: 7E           .db $7E
0036: 00           .db $00
0037: 38           .db $38
0038: BD F1 AF    JSR $F1AF
003B: 7F           .db $7F
003C: C8 23       EORB #$23
003E: 86 01       LDA #$01
0040: B7 C8 1A    STA $C81A
0043: 86 01       LDA #$01
0045: B7 C8 1F    STA $C81F
0048: 86 03       LDA #$03
004A: B7 C8 20    STA $C820
004D: 86 00       LDA #$00
004F: B7 C8 21    STA $C821
0052: B7 C8 22    STA $C822
0055: CC 00 7F    LDD #$007F
0058: FD C8 80    STD $C880
005B: B6 C8 81    LDA $C881
005E: BD F2 AB    JSR $F2AB
0061: CC 00 00    LDD #$0000
0064: FD C8 80    STD $C880
0067: BD 00 6D    JSR $006D
006A: 16           .db $16
006B: FF           .db $FF
006C: FA           .db $FA
006D: BD F1 92    JSR $F192
0070: BD F3 54    JSR $F354
0073: BD F1 AA    JSR $F1AA
0076: BD F1 BA    JSR $F1BA
0079: BD F1 AF    JSR $F1AF
007C: CC FF BA    LDD #$FFBA
007F: FD C8 80    STD $C880
0082: FC C8 80    LDD $C880
0085: FD CF E0    STD $CFE0
0088: CC 00 00    LDD #$0000
008B: FD C8 80    STD $C880
008E: FC C8 80    LDD $C880
0091: FD CF E2    STD $CFE2
0094: 8E 40 3A    LDX #$403A
0097: BF CF E4    STX $CFE4
009A: BD 40 00    JSR $4000
009D: CC 00 00    LDD #$0000
00A0: FD C8 80    STD $C880
00A3: CC 00 00    LDD #$0000
00A6: FD C8 80    STD $C880
00A9: FC C8 80    LDD $C880
00AC: FD CF E0    STD $CFE0
00AF: CC 00 00    LDD #$0000
00B2: FD C8 80    STD $C880
00B5: FC C8 80    LDD $C880
00B8: FD CF E2    STD $CFE2
00BB: 8E 40 40    LDX #$4040
00BE: BF CF E4    STX $CFE4
00C1: BD 40 00    JSR $4000
00C4: CC 00 00    LDD #$0000
00C7: FD C8 80    STD $C880
00CA: 39           RTS
00CB-3FFE: [FF padding - 16180 bytes]
3FFF: FF           .db $FF
4000: 86 98       LDA #$98
4002: B7 D0 0C    STA $D00C
4005: BD F1 AA    JSR $F1AA
4008: FE CF E4    LDU $CFE4
400B: B6 CF E3    LDA $CFE3
400E: F6 CF E1    LDB $CFE1
4011: BD F3 7A    JSR $F37A
4014: BD F3 5B    JSR $F35B
4017: BD F1 AF    JSR $F1AF
401A: 39           RTS
401B: 34 16       PSHS (regs)
401D: 34 06       PSHS (regs)
401F: EC           .db $EC
4020: 64           .db $64
4021: 10 A3        (unimpl prefix)
4023: 62           .db $62
4024: 35 06       PULS (regs)
4026: 10 2D        (unimpl prefix)
4028: C0           .db $C0
4029: 0B           .db $0B
402A: AE 62       LDX (indexed)
402C: EC           .db $EC
402D: E4           .db $E4
402E: 30 8B       LEAX (indexed)
4030: AF           .db $AF
4031: 62           .db $62
4032: 16           .db $16
4033: BF E8 EC    STX $E8EC
4036: 62           .db $62
4037: 32 64       LEAS (indexed)
4039: 39           RTS
403A: 48           .db $48
403B: 45           .db $45
403C: 4C           .db $4C
403D: 4C           .db $4C
403E: 4F           CLRA
403F: 80           .db $80
4040: 57           .db $57
4041: 4F           CLRA
4042: 52           .db $52
4043: 4C           .db $4C
4044: 44           .db $44
4045: 80           .db $80
4046-7FFC: [FF padding - 16311 bytes]
7FFD: FF           .db $FF
7FFE: 00           .db $00
7FFF: 40           .db $40
