0000: 67        .db $67
0001: 20 47    BRA $004A
0003: 43        .db $43
0004: 45        .db $45
0005: 20 32    BRA $0039
0007: 30        .db $30
0008: 32        .db $32
0009: 35 80    PULS (regs)
000B: FD 0D    STD <$DP0D>
000D: F8        .db $F8
000E: 50        .db $50
000F: 20 BB    BRA $FFCC
0011: 4D        .db $4D
0012: 55        .db $55
0013: 4C        .db $4C
0014: 54        .db $54
0015: 49        .db $49
0016: 42        .db $42
0017: 41        .db $41
0018: 4E        .db $4E
0019: 4B        .db $4B
001A: 20 50    BRA $006C
001C: 44        .db $44
001D: 42        .db $42
001E: 20 54    BRA $0074
0020: 45        .db $45
0021: 53        .db $53
0022: 54        .db $54
0023: 80        .db $80
0024: 00        .db $00
0025: 86 D0    LDA #$D0
0027: 1F        .db $1F
0028: 8B        .db $8B
0029: 7F        .db $7F
002A: C8 0E    IMM8
002C: 86 80    LDA #$80
002E: B7        .db $B7
002F: D0        .db $D0
0030: 04        .db $04
0031: 10 CE     LDS #$CBFF
0035: 7E        .db $7E
0036: 00        .db $00
0037: 38        .db $38
0038: CC 00 7F LDD #$007F
003B: FD CF    STD <$DPCF>
003D: 00        .db $00
003E: B6 CF 01 LDA $CF01
0041: BD F2 AB JSR $F2AB
0044: CC 00 00 LDD #$0000
0047: FD CF    STD <$DPCF>
0049: 00        .db $00
004A: BD 00 4F JSR $004F
004D: 20 FB    BRA $004A
004F: BD F1 92 JSR $F192
0052: CC FF BA LDD #$FFBA
0055: FD CF    STD <$DPCF>
0057: 00        .db $00
0058: FC        .db $FC
0059: CF        .db $CF
005A: 00        .db $00
005B: FD CF    STD <$DPCF>
005D: E0        .db $E0
005E: CC 00 00 LDD #$0000
0061: FD CF    STD <$DPCF>
0063: 00        .db $00
0064: FC        .db $FC
0065: CF        .db $CF
0066: 00        .db $00
0067: FD CF    STD <$DPCF>
0069: E2        .db $E2
006A: 8E 00 A1 LDX #$00A1
006D: BF        .db $BF
006E: CF        .db $CF
006F: E4        .db $E4
0070: BD 40 00 JSR $4000
0073: CC 00 00 LDD #$0000
0076: FD CF    STD <$DPCF>
0078: 00        .db $00
0079: CC 00 00 LDD #$0000
007C: FD CF    STD <$DPCF>
007E: 00        .db $00
007F: FC        .db $FC
0080: CF        .db $CF
0081: 00        .db $00
0082: FD CF    STD <$DPCF>
0084: E0        .db $E0
0085: CC 00 00 LDD #$0000
0088: FD CF    STD <$DPCF>
008A: 00        .db $00
008B: FC        .db $FC
008C: CF        .db $CF
008D: 00        .db $00
008E: FD CF    STD <$DPCF>
0090: E2        .db $E2
0091: 8E 00 A7 LDX #$00A7
0094: BF        .db $BF
0095: CF        .db $CF
0096: E4        .db $E4
0097: BD 40 00 JSR $4000
009A: CC 00 00 LDD #$0000
009D: FD CF    STD <$DPCF>
009F: 00        .db $00
00A0: 39        RTS
00A1: 48        .db $48
00A2: 45        .db $45
00A3: 4C        .db $4C
00A4: 4C        .db $4C
00A5: 4F        CLRA
00A6: 80        .db $80
00A7: 57        .db $57
00A8: 4F        CLRA
00A9: 52        .db $52
00AA: 4C        .db $4C
00AB: 44        .db $44
00AC: 80        .db $80
00AD-3FFE: [FF padding - 16210 bytes]
3FFF: FF        .db $FF
4000: BD F1 AA JSR $F1AA
4003: FE        .db $FE
4004: CF        .db $CF
4005: E4        .db $E4
4006: B6 CF E3 LDA $CFE3
4009: F6 CF E1 LDB $CFE1
400C: BD F3 7A JSR $F37A
400F: BD F1 AF JSR $F1AF
4012: 39        RTS
4013: 34 16    PSHS (regs)
4015: CC 00 00 LDD #$0000
4018: AE        .db $AE
4019: 62        .db $62
401A: 27 06    BEQ $4022
401C: E3        .db $E3
401D: E4        .db $E4
401E: 30        .db $30
401F: 1F        .db $1F
4020: 20 F8    BRA $401A
4022: 32        .db $32
4023: 64        .db $64
4024: 39        RTS
4025: 34 16    PSHS (regs)
4027: CC 00 00 LDD #$0000
402A: 34 06    PSHS (regs)
402C: EC        .db $EC
402D: 64        .db $64
402E: 10 A3     (unimpl prefix)
4030: 62        .db $62
4031: 35 06    PULS (regs)
4033: 2D        .db $2D
4034: 11 C3     (prefix2)
4036: 00        .db $00
4037: 01        .db $01
4038: AE        .db $AE
4039: 62        .db $62
403A: 34 06    PSHS (regs)
403C: EC        .db $EC
403D: 62        .db $62
403E: 30        .db $30
403F: 8B        .db $8B
4040: AF        .db $AF
4041: 64        .db $64
4042: 35 06    PULS (regs)
4044: 20 E4    BRA $402A
4046: 32        .db $32
4047: 64        .db $64
4048: 39        RTS
4049: 34 16    PSHS (regs)
404B: 34 06    PSHS (regs)
404D: EC        .db $EC
404E: 64        .db $64
404F: 10 A3     (unimpl prefix)
4051: 62        .db $62
4052: 35 06    PULS (regs)
4054: 2D        .db $2D
4055: 0A        .db $0A
4056: AE        .db $AE
4057: 62        .db $62
4058: EC        .db $EC
4059: E4        .db $E4
405A: 30        .db $30
405B: 8B        .db $8B
405C: AF        .db $AF
405D: 62        .db $62
405E: 20 EB    BRA $404B
4060: EC        .db $EC
4061: 62        .db $62
4062: 32        .db $32
4063: 64        .db $64
4064: 39        RTS
4065-7FFE: [FF padding - 16282 bytes]
7FFF: FF        .db $FF
