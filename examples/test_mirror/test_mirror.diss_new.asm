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
0012: 49           .db $49
0013: 52           .db $52
0014: 52           .db $52
0015: 4F           CLRA
0016: 52           .db $52
0017: 20 54       BRA $006D
0019: 45           .db $45
001A: 53           .db $53
001B: 54           .db $54
001C: 80           .db $80
001D: 00           .db $00
001E: 86 D0       LDA #$D0
0020: 1F 8B       TFR
0022: 7F           .db $7F
0023: C8 0E       EORB #$0E
0025: 86 80       LDA #$80
0027: B7 D0 04    STA $D004
002A: 10 CE CB FF LDS #$CBFF
002E: 7E           .db $7E
002F: 00           .db $00
0030: 31           .db $31
0031: CC 00 7F    LDD #$007F
0034: FD C8 80    STD $C880
0037: B6 C8 81    LDA $C881
003A: BD F2 AB    JSR $F2AB
003D: CC 00 00    LDD #$0000
0040: FD C8 80    STD $C880
0043: BD 00 48    JSR $0048
0046: 20 FB       BRA $0043
0048: BD F1 92    JSR $F192
004B: BD F3 54    JSR $F354
004E: BD F1 AA    JSR $F1AA
0051: BD F1 BA    JSR $F1BA
0054: BD F1 AF    JSR $F1AF
0057: CC 00 00    LDD #$0000
005A: FD C8 80    STD $C880
005D: B6 C8 81    LDA $C881
0060: B7 C8 82    STA $C882
0063: CC 00 00    LDD #$0000
0066: FD C8 80    STD $C880
0069: B6 C8 81    LDA $C881
006C: B7 C8 83    STA $C883
006F: B6 C8 82    LDA $C882
0072: B7 C8 80    STA $C880
0075: B6 C8 83    LDA $C883
0078: B7 C8 82    STA $C882
007B: 7F           .db $7F
007C: C8 84       EORB #$84
007E: 7F           .db $7F
007F: C8 86       EORB #$86
0081: 7F           .db $7F
0082: C8 88       EORB #$88
0084: BD F1 AA    JSR $F1AA
0087: 8E 01 ED    LDX #$01ED
008A: BD 00 B3    JSR $00B3
008D: BD F1 AF    JSR $F1AF
0090: CC 00 00    LDD #$0000
0093: FD C8 80    STD $C880
0096: 39           RTS
0097: 34 16       PSHS (regs)
0099: 34 06       PSHS (regs)
009B: EC           .db $EC
009C: 64           .db $64
009D: 10 A3        (unimpl prefix)
009F: 62           .db $62
00A0: 35 06       PULS (regs)
00A2: 2D 0A       BLT $00AE
00A4: AE 62       LDX (indexed)
00A6: EC           .db $EC
00A7: E4           .db $E4
00A8: 30 8B       LEAX (indexed)
00AA: AF           .db $AF
00AB: 62           .db $62
00AC: 20 EB       BRA $0099
00AE: EC           .db $EC
00AF: 62           .db $62
00B0: 32 64       LEAS (indexed)
00B2: 39           RTS
00B3: B6 C8 88    LDA $C888
00B6: 26 04       BNE $00BC
00B8: A6           .db $A6
00B9: 80           .db $80
00BA: 20 02       BRA $00BE
00BC: 30 01       LEAX (indexed)
00BE: BD F2 AB    JSR $F2AB
00C1: E6           .db $E6
00C2: 80           .db $80
00C3: 7D           .db $7D
00C4: C8 86       EORB #$86
00C6: 27 01       BEQ $00C9
00C8: 50           .db $50
00C9: FB           .db $FB
00CA: C8 82       EORB #$82
00CC: A6           .db $A6
00CD: 80           .db $80
00CE: 7D           .db $7D
00CF: C8 84       EORB #$84
00D1: 27 01       BEQ $00D4
00D3: 40           .db $40
00D4: BB           .db $BB
00D5: C8 80       EORB #$80
00D7: FD C8 86    STD $C886
00DA: 7F           .db $7F
00DB: D0           .db $D0
00DC: 0A           .db $0A
00DD: 86 CC       LDA #$CC
00DF: B7 D0 0C    STA $D00C
00E2: 7F           .db $7F
00E3: D0           .db $D0
00E4: 01           .db $01
00E5: 86 82       LDA #$82
00E7: B7 D0 00    STA $D000
00EA: 12           .db $12
00EB: 12           .db $12
00EC: 12           .db $12
00ED: 12           .db $12
00EE: 12           .db $12
00EF: 86 83       LDA #$83
00F1: B7 D0 00    STA $D000
00F4: FC C8 86    LDD $C886
00F7: F7 D0 01    STB $D001
00FA: 34 02       PSHS (regs)
00FC: 86 CE       LDA #$CE
00FE: B7 D0 0C    STA $D00C
0101: 7F           .db $7F
0102: D0           .db $D0
0103: 00           .db $00
0104: 86 01       LDA #$01
0106: B7 D0 00    STA $D000
0109: 35 02       PULS (regs)
010B: B7 D0 01    STA $D001
010E: 86 7F       LDA #$7F
0110: B7 D0 04    STA $D004
0113: 7F           .db $7F
0114: D0           .db $D0
0115: 05           .db $05
0116: 30 02       LEAX (indexed)
0118: B6 D0 0D    LDA $D00D
011B: 84           .db $84
011C: 40           .db $40
011D: 27 F9       BEQ $0118
011F: A6           .db $A6
0120: 80           .db $80
0121: 81 02       CMPA #$02
0123: 10 27 00 BE LBEQ <rel>
0127: 81 01       CMPA #$01
0129: 10 27 00 37 LBEQ <rel>
012D: E6           .db $E6
012E: 80           .db $80
012F: 7D           .db $7D
0130: C8 86       EORB #$86
0132: 27 01       BEQ $0135
0134: 50           .db $50
0135: A6           .db $A6
0136: 80           .db $80
0137: 7D           .db $7D
0138: C8 84       EORB #$84
013A: 27 01       BEQ $013D
013C: 40           .db $40
013D: 34 02       PSHS (regs)
013F: F7 D0 01    STB $D001
0142: 7F           .db $7F
0143: D0           .db $D0
0144: 00           .db $00
0145: 86 01       LDA #$01
0147: B7 D0 00    STA $D000
014A: 35 02       PULS (regs)
014C: B7 D0 01    STA $D001
014F: 7F           .db $7F
0150: D0           .db $D0
0151: 05           .db $05
0152: 86 FF       LDA #$FF
0154: B7 D0 0A    STA $D00A
0157: B6 D0 0D    LDA $D00D
015A: 84           .db $84
015B: 40           .db $40
015C: 27 F9       BEQ $0157
015E: 7F           .db $7F
015F: D0           .db $D0
0160: 0A           .db $0A
0161: 16           .db $16
0162: FF           .db $FF
0163: BB           .db $BB
0164: 1F 10       TFR
0166: 34 06       PSHS (regs)
0168: B6 C8 88    LDA $C888
016B: 26 04       BNE $0171
016D: A6           .db $A6
016E: 80           .db $80
016F: 20 02       BRA $0173
0171: 30 01       LEAX (indexed)
0173: 34 02       PSHS (regs)
0175: E6           .db $E6
0176: 80           .db $80
0177: 7D           .db $7D
0178: C8 86       EORB #$86
017A: 27 01       BEQ $017D
017C: 50           .db $50
017D: FB           .db $FB
017E: C8 82       EORB #$82
0180: A6           .db $A6
0181: 80           .db $80
0182: 7D           .db $7D
0183: C8 84       EORB #$84
0185: 27 01       BEQ $0188
0187: 40           .db $40
0188: BB           .db $BB
0189: C8 80       EORB #$80
018B: FD C8 86    STD $C886
018E: 35 02       PULS (regs)
0190: BD F2 AB    JSR $F2AB
0193: 35 06       PULS (regs)
0195: C3           .db $C3
0196: 00           .db $00
0197: 03           .db $03
0198: 1F 01       TFR
019A: 7F           .db $7F
019B: D0           .db $D0
019C: 0A           .db $0A
019D: 86 CC       LDA #$CC
019F: B7 D0 0C    STA $D00C
01A2: 7F           .db $7F
01A3: D0           .db $D0
01A4: 01           .db $01
01A5: 86 82       LDA #$82
01A7: B7 D0 00    STA $D000
01AA: 12           .db $12
01AB: 12           .db $12
01AC: 12           .db $12
01AD: 12           .db $12
01AE: 12           .db $12
01AF: 86 83       LDA #$83
01B1: B7 D0 00    STA $D000
01B4: FC C8 86    LDD $C886
01B7: F7 D0 01    STB $D001
01BA: 34 02       PSHS (regs)
01BC: 86 CE       LDA #$CE
01BE: B7 D0 0C    STA $D00C
01C1: 7F           .db $7F
01C2: D0           .db $D0
01C3: 00           .db $00
01C4: 86 01       LDA #$01
01C6: B7 D0 00    STA $D000
01C9: 35 02       PULS (regs)
01CB: B7 D0 01    STA $D001
01CE: 86 7F       LDA #$7F
01D0: B7 D0 04    STA $D004
01D3: 7F           .db $7F
01D4: D0           .db $D0
01D5: 05           .db $05
01D6: 30 02       LEAX (indexed)
01D8: B6 D0 0D    LDA $D00D
01DB: 84           .db $84
01DC: 40           .db $40
01DD: 27 F9       BEQ $01D8
01DF: 7F           .db $7F
01E0: D0           .db $D0
01E1: 0A           .db $0A
01E2: 16           .db $16
01E3: FF           .db $FF
01E4: 3A           .db $3A
01E5: 86 C8       LDA #$C8
01E7: 1F 8B       TFR
01E9: 39           RTS
01EA: 01           .db $01
01EB: 01           .db $01
01EC: ED 7F       STD (indexed)
01EE: F6 F6 00    LDB $F600
01F1: 00           .db $00
01F2: FF           .db $FF
01F3: 14           .db $14
01F4: 0A           .db $0A
01F5: FF           .db $FF
01F6: EC           .db $EC
01F7: 0A           .db $0A
01F8: 02           .db $02
01F9-7FFE: [FF padding - 32262 bytes]
7FFF: FF           .db $FF
