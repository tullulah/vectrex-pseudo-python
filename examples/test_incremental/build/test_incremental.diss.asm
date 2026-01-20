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
0011: 54           .db $54
0012: 45           .db $45
0013: 53           .db $53
0014: 54           .db $54
0015: 20 49       BRA $0060
0017: 4E           .db $4E
0018: 43           .db $43
0019: 52           .db $52
001A: 45           .db $45
001B: 4D           .db $4D
001C: 45           .db $45
001D: 4E           .db $4E
001E: 54           .db $54
001F: 41           .db $41
0020: 4C           .db $4C
0021: 80           .db $80
0022: 00           .db $00
0023: 86 D0       LDA #$D0
0025: 1F 8B       TFR
0027: 7F           .db $7F
0028: C8 0E       EORB #$0E
002A: 86 80       LDA #$80
002C: B7 D0 04    STA $D004
002F: 10 CE CB FF LDS #$CBFF
0033: 7F           .db $7F
0034: C8 A5       EORB #$A5
0036: CC 00 00    LDD #$0000
0039: FD C8 A3    STD $C8A3
003C: 7E           .db $7E
003D: 00           .db $00
003E: 3F           SWI
003F: CC 00 7F    LDD #$007F
0042: FD C8 A6    STD $C8A6
0045: BD F1 AF    JSR $F1AF
0048: 7F           .db $7F
0049: C8 23       EORB #$23
004B: 86 01       LDA #$01
004D: B7 C8 1A    STA $C81A
0050: 86 01       LDA #$01
0052: B7 C8 1F    STA $C81F
0055: 86 03       LDA #$03
0057: B7 C8 20    STA $C820
005A: 86 00       LDA #$00
005C: B7 C8 21    STA $C821
005F: B7 C8 22    STA $C822
0062: CC 00 7F    LDD #$007F
0065: FD C8 80    STD $C880
0068: B6 C8 81    LDA $C881
006B: BD F2 AB    JSR $F2AB
006E: CC 00 00    LDD #$0000
0071: FD C8 80    STD $C880
0074: 8E 00 00    LDX #$0000
0077: BD 40 06    JSR $4006
007A: CC 00 00    LDD #$0000
007D: FD C8 80    STD $C880
0080: BD 00 86    JSR $0086
0083: 16           .db $16
0084: FF           .db $FF
0085: FA           .db $FA
0086: BD F1 92    JSR $F192
0089: BD F3 54    JSR $F354
008C: BD F1 AA    JSR $F1AA
008F: BD F1 BA    JSR $F1BA
0092: BD F1 AF    JSR $F1AF
0095: FC C8 A6    LDD $C8A6
0098: FD C8 80    STD $C880
009B: B6 C8 81    LDA $C881
009E: BD F2 AB    JSR $F2AB
00A1: CC 00 00    LDD #$0000
00A4: FD C8 80    STD $C880
00A7: BD 40 E1    JSR $40E1
00AA: 39           RTS
00AB-3FFE: [FF padding - 16212 bytes]
3FFF: FF           .db $FF
4000: 01           .db $01
4001: 00           .db $00
4002: 00           .db $00
4003: 01           .db $01
4004: 00           .db $00
4005: 00           .db $00
4006: 1F 13       TFR
4008: B6 CF EA    LDA $CFEA
400B: 34 02       PSHS (regs)
400D: 1F 30       TFR
400F: 8E 40 00    LDX #$4000
4012: A6           .db $A6
4013: 8B           .db $8B
4014: B7 CF EA    STA $CFEA
4017: B7 DF 00    STA $DF00
401A: 1F 30       TFR
401C: 58           .db $58
401D: 49           .db $49
401E: 8E 40 01    LDX #$4001
4021: 30 8B       LEAX (indexed)
4023: AE 84       LDX (indexed)
4025: BD 40 50    JSR $4050
4028: 35 02       PULS (regs)
402A: B7 CF EA    STA $CFEA
402D: B7 DF 00    STA $DF00
4030: 39           RTS
4031: 34 16       PSHS (regs)
4033: 34 06       PSHS (regs)
4035: EC           .db $EC
4036: 64           .db $64
4037: 10 A3        (unimpl prefix)
4039: 62           .db $62
403A: 35 06       PULS (regs)
403C: 10 2D        (unimpl prefix)
403E: 00           .db $00
403F: 0B           .db $0B
4040: AE 62       LDX (indexed)
4042: EC           .db $EC
4043: E4           .db $E4
4044: 30 8B       LEAX (indexed)
4046: AF           .db $AF
4047: 62           .db $62
4048: 16           .db $16
4049: FF           .db $FF
404A: E8           .db $E8
404B: EC           .db $EC
404C: 62           .db $62
404D: 32 64       LEAS (indexed)
404F: 39           RTS
4050: BC           .db $BC
4051: C8 9E       EORB #$9E
4053: 10 26 00 07 LBNE <rel>
4057: B6 C8 A1    LDA $C8A1
405A: 10 26 00 0E LBNE <rel>
405E: BF C8 9C    STX $C89C
4061: BF C8 9E    STX $C89E
4064: 7F           .db $7F
4065: C8 A2       EORB #$A2
4067: 86 01       LDA #$01
4069: B7 C8 A1    STA $C8A1
406C: 39           RTS
406D: 86 01       LDA #$01
406F: B7 C8 A0    STA $C8A0
4072: B6 C8 A1    LDA $C8A1
4075: 10 27 00 5A LBEQ <rel>
4079: BE           .db $BE
407A: C8 9C       EORB #$9C
407C: 10 27 00 53 LBEQ <rel>
4080: E6           .db $E6
4081: 80           .db $80
4082: 10 27 00 3F LBEQ <rel>
4086: C1 FF       CMPB #$FF
4088: 10 27 00 3F LBEQ <rel>
408C: 34 04       PSHS (regs)
408E: A6           .db $A6
408F: 80           .db $80
4090: E6           .db $E6
4091: 80           .db $80
4092: 34 10       PSHS (regs)
4094: B7 D0 01    STA $D001
4097: 86 19       LDA #$19
4099: B7 D0 00    STA $D000
409C: 86 01       LDA #$01
409E: B7 D0 00    STA $D000
40A1: B6 D0 01    LDA $D001
40A4: F7 D0 01    STB $D001
40A7: C6 11       LDB #$11
40A9: F7 D0 00    STB $D000
40AC: C6 01       LDB #$01
40AE: F7 D0 00    STB $D000
40B1: 35 10       PULS (regs)
40B3: 35 04       PULS (regs)
40B5: 5A           .db $5A
40B6: 10 27 00 05 LBEQ <rel>
40BA: 34 04       PSHS (regs)
40BC: 16           .db $16
40BD: FF           .db $FF
40BE: CF           .db $CF
40BF: BF C8 9C    STX $C89C
40C2: 16           .db $16
40C3: 00           .db $00
40C4: 0E           .db $0E
40C5: 7F           .db $7F
40C6: C8 A1       EORB #$A1
40C8: 16           .db $16
40C9: 00           .db $00
40CA: 08           .db $08
40CB: EC           .db $EC
40CC: 84           .db $84
40CD: FD C8 9C    STD $C89C
40D0: 16           .db $16
40D1: 00           .db $00
40D2: 00           .db $00
40D3: 7F           .db $7F
40D4: C8 A0       EORB #$A0
40D6: 39           RTS
40D7: 7F           .db $7F
40D8: C8 A1       EORB #$A1
40DA: 7F           .db $7F
40DB: C8 9C       EORB #$9C
40DD: 7F           .db $7F
40DE: C8 9D       EORB #$9D
40E0: 39           RTS
40E1: 34 08       PSHS (regs)
40E3: 86 D0       LDA #$D0
40E5: 1F 8B       TFR
40E7: B6 C8 A1    LDA $C8A1
40EA: 10 27 00 79 LBEQ <rel>
40EE: B6 C8 A2    LDA $C8A2
40F1: 10 27 00 14 LBEQ <rel>
40F5: 4A           .db $4A
40F6: B7 C8 A2    STA $C8A2
40F9: 81 00       CMPA #$00
40FB: 10 26 00 6B LBNE <rel>
40FF: BE           .db $BE
4100: C8 9C       EORB #$9C
4102: 10 27 00 61 LBEQ <rel>
4106: 16           .db $16
4107: 00           .db $00
4108: 15           .db $15
4109: BE           .db $BE
410A: C8 9C       EORB #$9C
410C: 10 27 00 57 LBEQ <rel>
4110: E6           .db $E6
4111: 80           .db $80
4112: C1 FF       CMPB #$FF
4114: 10 27 00 44 LBEQ <rel>
4118: C1 00       CMPB #$00
411A: 10 26 00 0F LBNE <rel>
411E: E6           .db $E6
411F: 80           .db $80
4120: 10 27 00 32 LBEQ <rel>
4124: C1 FF       CMPB #$FF
4126: 10 27 00 32 LBEQ <rel>
412A: 16           .db $16
412B: 00           .db $00
412C: 0A           .db $0A
412D: 5A           .db $5A
412E: F7 C8 A2    STB $C8A2
4131: BF C8 9C    STX $C89C
4134: 16           .db $16
4135: 00           .db $00
4136: 33 34       LEAU (postbyte)
4138: 04           .db $04
4139: A6           .db $A6
413A: 80           .db $80
413B: E6           .db $E6
413C: 80           .db $80
413D: 34 10       PSHS (regs)
413F: BD F2 56    JSR $F256
4142: 35 10       PULS (regs)
4144: 35 04       PULS (regs)
4146: 5A           .db $5A
4147: 10 27 00 05 LBEQ <rel>
414B: 34 04       PSHS (regs)
414D: 16           .db $16
414E: FF           .db $FF
414F: E9           .db $E9
4150: BF C8 9C    STX $C89C
4153: 16           .db $16
4154: 00           .db $00
4155: 14           .db $14
4156: 7F           .db $7F
4157: C8 A1       EORB #$A1
4159: 16           .db $16
415A: 00           .db $00
415B: 0E           .db $0E
415C: EC           .db $EC
415D: 84           .db $84
415E: FD C8 9C    STD $C89C
4161: 7F           .db $7F
4162: C8 A2       EORB #$A2
4164: 16           .db $16
4165: 00           .db $00
4166: 03           .db $03
4167: 16           .db $16
4168: 00           .db $00
4169: 00           .db $00
416A: B6 C8 A5    LDA $C8A5
416D: 10 27 00 03 LBEQ <rel>
4171: BD 41 8B    JSR $418B
4174: 35 08       PULS (regs)
4176: 39           RTS
4177: BF C8 A3    STX $C8A3
417A: 86 01       LDA #$01
417C: B7 C8 A5    STA $C8A5
417F: 39           RTS
4180: B6 C8 A5    LDA $C8A5
4183: 10 27 00 03 LBEQ <rel>
4187: BD 41 8B    JSR $418B
418A: 39           RTS
418B: FE C8 A3    LDU $C8A3
418E: E6           .db $E6
418F: C4 C1       ANDB #$C1
4191: D0           .db $D0
4192: 10 26 00 08 LBNE <rel>
4196: E6           .db $E6
4197: 41           .db $41
4198: C1 20       CMPB #$20
419A: 10 27 00 77 LBEQ <rel>
419E: 31           .db $31
419F: 41           .db $41
41A0: E6           .db $E6
41A1: C4 C5       ANDB #$C5
41A3: 20 10       BRA $41B5
41A5: 27 00       BEQ $41A7
41A7: 10 E6        (unimpl prefix)
41A9: 42           .db $42
41AA: 86 04       LDA #$04
41AC: BD F2 56    JSR $F256
41AF: E6           .db $E6
41B0: 41           .db $41
41B1: 86 05       LDA #$05
41B3: BD F2 56    JSR $F256
41B6: 31           .db $31
41B7: 22 E6       BHI $419F
41B9: C4 C5       ANDB #$C5
41BB: 40           .db $40
41BC: 10 27 00 09 LBEQ <rel>
41C0: E6           .db $E6
41C1: A4           .db $A4
41C2: 86 06       LDA #$06
41C4: BD F2 56    JSR $F256
41C7: 31           .db $31
41C8: 21           .db $21
41C9: E6           .db $E6
41CA: C4 C4       ANDB #$C4
41CC: 0F           .db $0F
41CD: 86 0A       LDA #$0A
41CF: BD F2 56    JSR $F256
41D2: E6           .db $E6
41D3: C4 C5       ANDB #$C5
41D5: 10 10        (unimpl prefix)
41D7: 27 00       BEQ $41D9
41D9: 0D           .db $0D
41DA: F6 C8 07    LDB $C807
41DD: CA           .db $CA
41DE: 04           .db $04
41DF: 86 07       LDA #$07
41E1: BD F2 56    JSR $F256
41E4: 16           .db $16
41E5: 00           .db $00
41E6: 0A           .db $0A
41E7: F6 C8 07    LDB $C807
41EA: C4 FB       ANDB #$FB
41EC: 86 07       LDA #$07
41EE: BD F2 56    JSR $F256
41F1: E6           .db $E6
41F2: C4 C5       ANDB #$C5
41F4: 80           .db $80
41F5: 10 27 00 0D LBEQ <rel>
41F9: F6 C8 07    LDB $C807
41FC: CA           .db $CA
41FD: 20 86       BRA $4185
41FF: 07           .db $07
4200: BD F2 56    JSR $F256
4203: 16           .db $16
4204: 00           .db $00
4205: 0A           .db $0A
4206: F6 C8 07    LDB $C807
4209: C4 DF       ANDB #$DF
420B: 86 07       LDA #$07
420D: BD F2 56    JSR $F256
4210: 10 BF        (unimpl prefix)
4212: C8 A3       EORB #$A3
4214: 39           RTS
4215: 7F           .db $7F
4216: C8 A5       EORB #$A5
4218: 86 0A       LDA #$0A
421A: C6 00       LDB #$00
421C: BD F2 56    JSR $F256
421F: CC 00 00    LDD #$0000
4222: FD C8 A3    STD $C8A3
4225: 39           RTS
4226: 6D           .db $6D
4227: 75           .db $75
4228: 73           .db $73
4229: 69           .db $69
422A: 63           .db $63
422B: 31           .db $31
422C: 80           .db $80
422D-7FFC: [FF padding - 15824 bytes]
7FFD: FF           .db $FF
7FFE: 40           .db $40
7FFF: 00           .db $00
