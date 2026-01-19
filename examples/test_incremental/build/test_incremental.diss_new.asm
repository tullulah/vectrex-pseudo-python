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
0034: C8 BA       EORB #$BA
0036: CC 00 00    LDD #$0000
0039: FD C8 B8    STD $C8B8
003C: 7E           .db $7E
003D: 00           .db $00
003E: 3F           SWI
003F: CC 00 00    LDD #$0000
0042: FD C8 BB    STD $C8BB
0045: CC 00 7F    LDD #$007F
0048: FD C8 80    STD $C880
004B: B6 C8 81    LDA $C881
004E: BD F2 AB    JSR $F2AB
0051: CC 00 00    LDD #$0000
0054: FD C8 80    STD $C880
0057: BD 00 5C    JSR $005C
005A: 20 FB       BRA $0057
005C: BD F1 92    JSR $F192
005F: BD F3 54    JSR $F354
0062: BD F1 AA    JSR $F1AA
0065: BD F1 BA    JSR $F1BA
0068: BD F1 AF    JSR $F1AF
006B: FC C8 BB    LDD $C8BB
006E: FD C8 80    STD $C880
0071: FC C8 80    LDD $C880
0074: 34 06       PSHS (regs)
0076: CC 00 00    LDD #$0000
0079: FD C8 80    STD $C880
007C: FC C8 80    LDD $C880
007F: 10 A3        (unimpl prefix)
0081: E1           .db $E1
0082: 10 27 00 06 LBEQ <rel>
0086: CC 00 00    LDD #$0000
0089: 16           .db $16
008A: 00           .db $00
008B: 03           .db $03
008C: CC 00 01    LDD #$0001
008F: FD C8 80    STD $C880
0092: FC C8 80    LDD $C880
0095: 10 27 00 18 LBEQ <rel>
0099: CC 00 01    LDD #$0001
009C: FD C8 80    STD $C880
009F: FC C8 80    LDD $C880
00A2: FD C8 BB    STD $C8BB
00A5: BD 07 15    JSR $0715
00A8: CC 00 00    LDD #$0000
00AB: FD C8 80    STD $C880
00AE: 16           .db $16
00AF: 00           .db $00
00B0: 00           .db $00
00B1: CC FF C4    LDD #$FFC4
00B4: FD C8 80    STD $C880
00B7: FC C8 80    LDD $C880
00BA: FD CF E0    STD $CFE0
00BD: CC 00 78    LDD #$0078
00C0: FD C8 80    STD $C880
00C3: FC C8 80    LDD $C880
00C6: FD CF E2    STD $CFE2
00C9: 8E 07 B4    LDX #$07B4
00CC: BF CF E4    STX $CFE4
00CF: BD 03 54    JSR $0354
00D2: CC 00 00    LDD #$0000
00D5: FD C8 80    STD $C880
00D8: CC 00 00    LDD #$0000
00DB: FD C8 80    STD $C880
00DE: FC C8 80    LDD $C880
00E1: FD C8 9D    STD $C89D
00E4: CC 00 1E    LDD #$001E
00E7: FD C8 80    STD $C880
00EA: FC C8 80    LDD $C880
00ED: FD C8 9F    STD $C89F
00F0: CC FF D8    LDD #$FFD8
00F3: FD C8 80    STD $C880
00F6: FC C8 80    LDD $C880
00F9: 4D           .db $4D
00FA: 2A 05       BPL $0101
00FC: 43           .db $43
00FD: 53           .db $53
00FE: C3           .db $C3
00FF: 00           .db $00
0100: 01           .db $01
0101: FD C8 80    STD $C880
0104: FC C8 80    LDD $C880
0107: FD C8 A1    STD $C8A1
010A: CC 00 46    LDD #$0046
010D: FD C8 80    STD $C880
0110: FC C8 80    LDD $C880
0113: FD C8 A3    STD $C8A3
0116: CC 00 50    LDD #$0050
0119: FD C8 80    STD $C880
011C: FC C8 80    LDD $C880
011F: FD C8 A5    STD $C8A5
0122: BD 03 8B    JSR $038B
0125: CC 00 00    LDD #$0000
0128: FD C8 80    STD $C880
012B: CC 00 00    LDD #$0000
012E: FD C8 80    STD $C880
0131: FC C8 80    LDD $C880
0134: FD C8 9D    STD $C89D
0137: CC 00 1E    LDD #$001E
013A: FD C8 80    STD $C880
013D: FC C8 80    LDD $C880
0140: FD C8 9F    STD $C89F
0143: CC FF EC    LDD #$FFEC
0146: FD C8 80    STD $C880
0149: FC C8 80    LDD $C880
014C: FD C8 82    STD $C882
014F: CC 00 32    LDD #$0032
0152: FD C8 80    STD $C880
0155: FC C8 82    LDD $C882
0158: 10 B3        CMPD <various>
015A: C8 80       EORB #$80
015C: 2F 02       BLE $0160
015E: 20 03       BRA $0163
0160: FD C8 80    STD $C880
0163: FC C8 80    LDD $C880
0166: FD C8 A1    STD $C8A1
0169: CC 00 46    LDD #$0046
016C: FD C8 80    STD $C880
016F: FC C8 80    LDD $C880
0172: FD C8 82    STD $C882
0175: CC FF CE    LDD #$FFCE
0178: FD C8 80    STD $C880
017B: FC C8 82    LDD $C882
017E: 10 B3        CMPD <various>
0180: C8 80       EORB #$80
0182: 2C 02       BGE $0186
0184: 20 03       BRA $0189
0186: FD C8 80    STD $C880
0189: FC C8 80    LDD $C880
018C: FD C8 A3    STD $C8A3
018F: CC 00 50    LDD #$0050
0192: FD C8 80    STD $C880
0195: FC C8 80    LDD $C880
0198: FD C8 A5    STD $C8A5
019B: BD 03 8B    JSR $038B
019E: CC 00 00    LDD #$0000
01A1: FD C8 80    STD $C880
01A4: 86 7F       LDA #$7F
01A6: BD F2 AB    JSR $F2AB
01A9: 86 D0       LDA #$D0
01AB: 1F 8B       TFR
01AD: BD F3 54    JSR $F354
01B0: 86 5A       LDA #$5A
01B2: C6 B7       LDB #$B7
01B4: BD F3 12    JSR $F312
01B7: 7F           .db $7F
01B8: C8 23       EORB #$23
01BA: 86 07       LDA #$07
01BC: C6 FF       LDB #$FF
01BE: BD F3 DF    JSR $F3DF
01C1: 7F           .db $7F
01C2: C8 23       EORB #$23
01C4: 86 05       LDA #$05
01C6: C6 FC       LDB #$FC
01C8: BD F3 DF    JSR $F3DF
01CB: 7F           .db $7F
01CC: C8 23       EORB #$23
01CE: 86 04       LDA #$04
01D0: C6 FB       LDB #$FB
01D2: BD F3 DF    JSR $F3DF
01D5: 7F           .db $7F
01D6: C8 23       EORB #$23
01D8: 86 02       LDA #$02
01DA: C6 F9       LDB #$F9
01DC: BD F3 DF    JSR $F3DF
01DF: 7F           .db $7F
01E0: C8 23       EORB #$23
01E2: 86 FE       LDA #$FE
01E4: C6 F9       LDB #$F9
01E6: BD F3 DF    JSR $F3DF
01E9: 7F           .db $7F
01EA: C8 23       EORB #$23
01EC: 86 FC       LDA #$FC
01EE: C6 FB       LDB #$FB
01F0: BD F3 DF    JSR $F3DF
01F3: 7F           .db $7F
01F4: C8 23       EORB #$23
01F6: 86 FB       LDA #$FB
01F8: C6 FC       LDB #$FC
01FA: BD F3 DF    JSR $F3DF
01FD: 7F           .db $7F
01FE: C8 23       EORB #$23
0200: 86 F9       LDA #$F9
0202: C6 FE       LDB #$FE
0204: BD F3 DF    JSR $F3DF
0207: 7F           .db $7F
0208: C8 23       EORB #$23
020A: 86 F9       LDA #$F9
020C: C6 02       LDB #$02
020E: BD F3 DF    JSR $F3DF
0211: 7F           .db $7F
0212: C8 23       EORB #$23
0214: 86 FB       LDA #$FB
0216: C6 04       LDB #$04
0218: BD F3 DF    JSR $F3DF
021B: 7F           .db $7F
021C: C8 23       EORB #$23
021E: 86 FC       LDA #$FC
0220: C6 05       LDB #$05
0222: BD F3 DF    JSR $F3DF
0225: 7F           .db $7F
0226: C8 23       EORB #$23
0228: 86 FF       LDA #$FF
022A: C6 07       LDB #$07
022C: BD F3 DF    JSR $F3DF
022F: 7F           .db $7F
0230: C8 23       EORB #$23
0232: 86 01       LDA #$01
0234: C6 07       LDB #$07
0236: BD F3 DF    JSR $F3DF
0239: 7F           .db $7F
023A: C8 23       EORB #$23
023C: 86 04       LDA #$04
023E: C6 05       LDB #$05
0240: BD F3 DF    JSR $F3DF
0243: 7F           .db $7F
0244: C8 23       EORB #$23
0246: 86 05       LDA #$05
0248: C6 04       LDB #$04
024A: BD F3 DF    JSR $F3DF
024D: 7F           .db $7F
024E: C8 23       EORB #$23
0250: 86 07       LDA #$07
0252: C6 01       LDB #$01
0254: BD F3 DF    JSR $F3DF
0257: CC 00 00    LDD #$0000
025A: FD C8 80    STD $C880
025D: CC 00 00    LDD #$0000
0260: FD C8 80    STD $C880
0263: B6 C8 81    LDA $C881
0266: B7 C8 82    STA $C882
0269: CC FF EC    LDD #$FFEC
026C: FD C8 80    STD $C880
026F: B6 C8 81    LDA $C881
0272: B7 C8 83    STA $C883
0275: B6 C8 82    LDA $C882
0278: B7 C8 88    STA $C888
027B: B6 C8 83    LDA $C883
027E: B7 C8 89    STA $C889
0281: 7F           .db $7F
0282: C8 9B       EORB #$9B
0284: 7F           .db $7F
0285: C8 9C       EORB #$9C
0287: 7F           .db $7F
0288: C8 8A       EORB #$8A
028A: BD F1 AA    JSR $F1AA
028D: 8E 07 CE    LDX #$07CE
0290: BD 04 50    JSR $0450
0293: 8E 07 FB    LDX #$07FB
0296: BD 04 50    JSR $0450
0299: 8E 08 22    LDX #$0822
029C: BD 04 50    JSR $0450
029F: 8E 08 34    LDX #$0834
02A2: BD 04 50    JSR $0450
02A5: 8E 08 46    LDX #$0846
02A8: BD 04 50    JSR $0450
02AB: 8E 08 6D    LDX #$086D
02AE: BD 04 50    JSR $0450
02B1: 8E 08 94    LDX #$0894
02B4: BD 04 50    JSR $0450
02B7: BD F1 AF    JSR $F1AF
02BA: CC 00 00    LDD #$0000
02BD: FD C8 80    STD $C880
02C0: CC 00 00    LDD #$0000
02C3: FD C8 80    STD $C880
02C6: B6 C8 81    LDA $C881
02C9: B7 C8 88    STA $C888
02CC: CC FF 92    LDD #$FF92
02CF: FD C8 80    STD $C880
02D2: B6 C8 81    LDA $C881
02D5: B7 C8 89    STA $C889
02D8: CC 00 01    LDD #$0001
02DB: FD C8 80    STD $C880
02DE: F6 C8 81    LDB $C881
02E1: 7F           .db $7F
02E2: C8 9B       EORB #$9B
02E4: 7F           .db $7F
02E5: C8 9C       EORB #$9C
02E7: C1 01       CMPB #$01
02E9: 10 26 00 05 LBNE <rel>
02ED: 86 01       LDA #$01
02EF: B7 C8 9B    STA $C89B
02F2: C1 02       CMPB #$02
02F4: 10 26 00 05 LBNE <rel>
02F8: 86 01       LDA #$01
02FA: B7 C8 9C    STA $C89C
02FD: C1 03       CMPB #$03
02FF: 10 26 00 08 LBNE <rel>
0303: 86 01       LDA #$01
0305: B7 C8 9B    STA $C89B
0308: B7 C8 9C    STA $C89C
030B: CC 00 7F    LDD #$007F
030E: FD C8 80    STD $C880
0311: B6 C8 81    LDA $C881
0314: B7 C8 8A    STA $C88A
0317: BD F1 AA    JSR $F1AA
031A: 8E 07 CE    LDX #$07CE
031D: BD 04 50    JSR $0450
0320: 8E 07 FB    LDX #$07FB
0323: BD 04 50    JSR $0450
0326: 8E 08 22    LDX #$0822
0329: BD 04 50    JSR $0450
032C: 8E 08 34    LDX #$0834
032F: BD 04 50    JSR $0450
0332: 8E 08 46    LDX #$0846
0335: BD 04 50    JSR $0450
0338: 8E 08 6D    LDX #$086D
033B: BD 04 50    JSR $0450
033E: 8E 08 94    LDX #$0894
0341: BD 04 50    JSR $0450
0344: BD F1 AF    JSR $F1AF
0347: 7F           .db $7F
0348: C8 8A       EORB #$8A
034A: CC 00 00    LDD #$0000
034D: FD C8 80    STD $C880
0350: BD 06 02    JSR $0602
0353: 39           RTS
0354: 86 98       LDA #$98
0356: B7 D0 0C    STA $D00C
0359: BD F1 AA    JSR $F1AA
035C: FE CF E4    LDU $CFE4
035F: B6 CF E3    LDA $CFE3
0362: F6 CF E1    LDB $CFE1
0365: BD F3 7A    JSR $F37A
0368: BD F3 5B    JSR $F35B
036B: BD F1 AF    JSR $F1AF
036E: 39           RTS
036F: 34 16       PSHS (regs)
0371: 34 06       PSHS (regs)
0373: EC           .db $EC
0374: 64           .db $64
0375: 10 A3        (unimpl prefix)
0377: 62           .db $62
0378: 35 06       PULS (regs)
037A: 2D 0A       BLT $0386
037C: AE 62       LDX (indexed)
037E: EC           .db $EC
037F: E4           .db $E4
0380: 30 8B       LEAX (indexed)
0382: AF           .db $AF
0383: 62           .db $62
0384: 20 EB       BRA $0371
0386: EC           .db $EC
0387: 62           .db $62
0388: 32 64       LEAS (indexed)
038A: 39           RTS
038B: 86 98       LDA #$98
038D: B7 D0 0C    STA $D00C
0390: 86 D0       LDA #$D0
0392: 1F 8B       TFR
0394: B6 C8 A6    LDA $C8A6
0397: BD F2 AB    JSR $F2AB
039A: B6 C8 A0    LDA $C8A0
039D: F6 C8 9E    LDB $C89E
03A0: BD F3 12    JSR $F312
03A3: FC C8 A1    LDD $C8A1
03A6: B3           .db $B3
03A7: C8 9D       EORB #$9D
03A9: FD C8 A7    STD $C8A7
03AC: FC C8 A3    LDD $C8A3
03AF: B3           .db $B3
03B0: C8 9F       EORB #$9F
03B2: FD C8 A9    STD $C8A9
03B5: FC C8 A9    LDD $C8A9
03B8: 10 83        CMPD <various>
03BA: 00           .db $00
03BB: 7F           .db $7F
03BC: 2F 04       BLE $03C2
03BE: 86 7F       LDA #$7F
03C0: 20 0D       BRA $03CF
03C2: 10 83        CMPD <various>
03C4: FF           .db $FF
03C5: 80           .db $80
03C6: 2C 04       BGE $03CC
03C8: 86 80       LDA #$80
03CA: 20 03       BRA $03CF
03CC: B6 C8 AA    LDA $C8AA
03CF: B7 C8 AC    STA $C8AC
03D2: FC C8 A7    LDD $C8A7
03D5: 10 83        CMPD <various>
03D7: 00           .db $00
03D8: 7F           .db $7F
03D9: 2F 04       BLE $03DF
03DB: C6 7F       LDB #$7F
03DD: 20 0D       BRA $03EC
03DF: 10 83        CMPD <various>
03E1: FF           .db $FF
03E2: 80           .db $80
03E3: 2C 04       BGE $03E9
03E5: C6 80       LDB #$80
03E7: 20 03       BRA $03EC
03E9: F6 C8 A8    LDB $C8A8
03EC: F7 C8 AB    STB $C8AB
03EF: 7F           .db $7F
03F0: C8 23       EORB #$23
03F2: B6 C8 AC    LDA $C8AC
03F5: F6 C8 AB    LDB $C8AB
03F8: BD F3 DF    JSR $F3DF
03FB: FC C8 A9    LDD $C8A9
03FE: 10 83        CMPD <various>
0400: 00           .db $00
0401: 7F           .db $7F
0402: 2E 08       BGT $040C
0404: 10 83        CMPD <various>
0406: FF           .db $FF
0407: 80           .db $80
0408: 2D 02       BLT $040C
040A: 20 3F       BRA $044B
040C: FC C8 A9    LDD $C8A9
040F: 10 83        CMPD <various>
0411: 00           .db $00
0412: 7F           .db $7F
0413: 2E 05       BGT $041A
0415: C3           .db $C3
0416: 00           .db $00
0417: 80           .db $80
0418: 20 03       BRA $041D
041A: 83           .db $83
041B: 00           .db $00
041C: 7F           .db $7F
041D: FD C8 AD    STD $C8AD
0420: FC C8 A7    LDD $C8A7
0423: 10 83        CMPD <various>
0425: 00           .db $00
0426: 7F           .db $7F
0427: 2F 05       BLE $042E
0429: 83           .db $83
042A: 00           .db $00
042B: 7F           .db $7F
042C: 20 0E       BRA $043C
042E: 10 83        CMPD <various>
0430: FF           .db $FF
0431: 80           .db $80
0432: 2C 05       BGE $0439
0434: C3           .db $C3
0435: 00           .db $00
0436: 80           .db $80
0437: 20 03       BRA $043C
0439: CC 00 00    LDD #$0000
043C: FD C8 AF    STD $C8AF
043F: B6 C8 AE    LDA $C8AE
0442: F6 C8 B0    LDB $C8B0
0445: 7F           .db $7F
0446: C8 23       EORB #$23
0448: BD F3 DF    JSR $F3DF
044B: 86 C8       LDA #$C8
044D: 1F 8B       TFR
044F: 39           RTS
0450: B6 C8 8A    LDA $C88A
0453: 26 04       BNE $0459
0455: A6           .db $A6
0456: 80           .db $80
0457: 20 02       BRA $045B
0459: 30 01       LEAX (indexed)
045B: BD F2 AB    JSR $F2AB
045E: E6           .db $E6
045F: 80           .db $80
0460: 7D           .db $7D
0461: C8 9C       EORB #$9C
0463: 27 01       BEQ $0466
0465: 50           .db $50
0466: FB           .db $FB
0467: C8 89       EORB #$89
0469: A6           .db $A6
046A: 80           .db $80
046B: 7D           .db $7D
046C: C8 9B       EORB #$9B
046E: 27 01       BEQ $0471
0470: 40           .db $40
0471: BB           .db $BB
0472: C8 88       EORB #$88
0474: FD C8 86    STD $C886
0477: 7F           .db $7F
0478: D0           .db $D0
0479: 0A           .db $0A
047A: 86 CC       LDA #$CC
047C: B7 D0 0C    STA $D00C
047F: 7F           .db $7F
0480: D0           .db $D0
0481: 01           .db $01
0482: 86 82       LDA #$82
0484: B7 D0 00    STA $D000
0487: 12           .db $12
0488: 12           .db $12
0489: 12           .db $12
048A: 12           .db $12
048B: 12           .db $12
048C: 86 83       LDA #$83
048E: B7 D0 00    STA $D000
0491: FC C8 86    LDD $C886
0494: F7 D0 01    STB $D001
0497: 34 02       PSHS (regs)
0499: 86 CE       LDA #$CE
049B: B7 D0 0C    STA $D00C
049E: 7F           .db $7F
049F: D0           .db $D0
04A0: 00           .db $00
04A1: 86 01       LDA #$01
04A3: B7 D0 00    STA $D000
04A6: 35 02       PULS (regs)
04A8: B7 D0 01    STA $D001
04AB: 86 7F       LDA #$7F
04AD: B7 D0 04    STA $D004
04B0: 7F           .db $7F
04B1: D0           .db $D0
04B2: 05           .db $05
04B3: 30 02       LEAX (indexed)
04B5: B6 D0 0D    LDA $D00D
04B8: 84           .db $84
04B9: 40           .db $40
04BA: 27 F9       BEQ $04B5
04BC: A6           .db $A6
04BD: 80           .db $80
04BE: 81 02       CMPA #$02
04C0: 10 27 00 BE LBEQ <rel>
04C4: 81 01       CMPA #$01
04C6: 10 27 00 37 LBEQ <rel>
04CA: E6           .db $E6
04CB: 80           .db $80
04CC: 7D           .db $7D
04CD: C8 9C       EORB #$9C
04CF: 27 01       BEQ $04D2
04D1: 50           .db $50
04D2: A6           .db $A6
04D3: 80           .db $80
04D4: 7D           .db $7D
04D5: C8 9B       EORB #$9B
04D7: 27 01       BEQ $04DA
04D9: 40           .db $40
04DA: 34 02       PSHS (regs)
04DC: F7 D0 01    STB $D001
04DF: 7F           .db $7F
04E0: D0           .db $D0
04E1: 00           .db $00
04E2: 86 01       LDA #$01
04E4: B7 D0 00    STA $D000
04E7: 35 02       PULS (regs)
04E9: B7 D0 01    STA $D001
04EC: 7F           .db $7F
04ED: D0           .db $D0
04EE: 05           .db $05
04EF: 86 FF       LDA #$FF
04F1: B7 D0 0A    STA $D00A
04F4: B6 D0 0D    LDA $D00D
04F7: 84           .db $84
04F8: 40           .db $40
04F9: 27 F9       BEQ $04F4
04FB: 7F           .db $7F
04FC: D0           .db $D0
04FD: 0A           .db $0A
04FE: 16           .db $16
04FF: FF           .db $FF
0500: BB           .db $BB
0501: 1F 10       TFR
0503: 34 06       PSHS (regs)
0505: B6 C8 8A    LDA $C88A
0508: 26 04       BNE $050E
050A: A6           .db $A6
050B: 80           .db $80
050C: 20 02       BRA $0510
050E: 30 01       LEAX (indexed)
0510: 34 02       PSHS (regs)
0512: E6           .db $E6
0513: 80           .db $80
0514: 7D           .db $7D
0515: C8 9C       EORB #$9C
0517: 27 01       BEQ $051A
0519: 50           .db $50
051A: FB           .db $FB
051B: C8 89       EORB #$89
051D: A6           .db $A6
051E: 80           .db $80
051F: 7D           .db $7D
0520: C8 9B       EORB #$9B
0522: 27 01       BEQ $0525
0524: 40           .db $40
0525: BB           .db $BB
0526: C8 88       EORB #$88
0528: FD C8 86    STD $C886
052B: 35 02       PULS (regs)
052D: BD F2 AB    JSR $F2AB
0530: 35 06       PULS (regs)
0532: C3           .db $C3
0533: 00           .db $00
0534: 03           .db $03
0535: 1F 01       TFR
0537: 7F           .db $7F
0538: D0           .db $D0
0539: 0A           .db $0A
053A: 86 CC       LDA #$CC
053C: B7 D0 0C    STA $D00C
053F: 7F           .db $7F
0540: D0           .db $D0
0541: 01           .db $01
0542: 86 82       LDA #$82
0544: B7 D0 00    STA $D000
0547: 12           .db $12
0548: 12           .db $12
0549: 12           .db $12
054A: 12           .db $12
054B: 12           .db $12
054C: 86 83       LDA #$83
054E: B7 D0 00    STA $D000
0551: FC C8 86    LDD $C886
0554: F7 D0 01    STB $D001
0557: 34 02       PSHS (regs)
0559: 86 CE       LDA #$CE
055B: B7 D0 0C    STA $D00C
055E: 7F           .db $7F
055F: D0           .db $D0
0560: 00           .db $00
0561: 86 01       LDA #$01
0563: B7 D0 00    STA $D000
0566: 35 02       PULS (regs)
0568: B7 D0 01    STA $D001
056B: 86 7F       LDA #$7F
056D: B7 D0 04    STA $D004
0570: 7F           .db $7F
0571: D0           .db $D0
0572: 05           .db $05
0573: 30 02       LEAX (indexed)
0575: B6 D0 0D    LDA $D00D
0578: 84           .db $84
0579: 40           .db $40
057A: 27 F9       BEQ $0575
057C: 7F           .db $7F
057D: D0           .db $D0
057E: 0A           .db $0A
057F: 16           .db $16
0580: FF           .db $FF
0581: 3A           .db $3A
0582: 39           RTS
0583: BC           .db $BC
0584: C8 B3       EORB #$B3
0586: 26 05       BNE $058D
0588: B6 C8 B6    LDA $C8B6
058B: 26 0E       BNE $059B
058D: BF C8 B1    STX $C8B1
0590: BF C8 B3    STX $C8B3
0593: 7F           .db $7F
0594: C8 B7       EORB #$B7
0596: 86 01       LDA #$01
0598: B7 C8 B6    STA $C8B6
059B: 39           RTS
059C: 86 01       LDA #$01
059E: B7 C8 B5    STA $C8B5
05A1: B6 C8 B6    LDA $C8B6
05A4: 27 4E       BEQ $05F4
05A6: BE           .db $BE
05A7: C8 B1       EORB #$B1
05A9: 27 49       BEQ $05F4
05AB: E6           .db $E6
05AC: 80           .db $80
05AD: 27 39       BEQ $05E8
05AF: C1 FF       CMPB #$FF
05B1: 27 3A       BEQ $05ED
05B3: 34 04       PSHS (regs)
05B5: A6           .db $A6
05B6: 80           .db $80
05B7: E6           .db $E6
05B8: 80           .db $80
05B9: 34 10       PSHS (regs)
05BB: B7 D0 01    STA $D001
05BE: 86 19       LDA #$19
05C0: B7 D0 00    STA $D000
05C3: 86 01       LDA #$01
05C5: B7 D0 00    STA $D000
05C8: B6 D0 01    LDA $D001
05CB: F7 D0 01    STB $D001
05CE: C6 11       LDB #$11
05D0: F7 D0 00    STB $D000
05D3: C6 01       LDB #$01
05D5: F7 D0 00    STB $D000
05D8: 35 10       PULS (regs)
05DA: 35 04       PULS (regs)
05DC: 5A           .db $5A
05DD: 27 04       BEQ $05E3
05DF: 34 04       PSHS (regs)
05E1: 20 D2       BRA $05B5
05E3: BF C8 B1    STX $C8B1
05E6: 20 0C       BRA $05F4
05E8: 7F           .db $7F
05E9: C8 B6       EORB #$B6
05EB: 20 07       BRA $05F4
05ED: EC           .db $EC
05EE: 84           .db $84
05EF: FD C8 B1    STD $C8B1
05F2: 20 00       BRA $05F4
05F4: 7F           .db $7F
05F5: C8 B5       EORB #$B5
05F7: 39           RTS
05F8: 7F           .db $7F
05F9: C8 B6       EORB #$B6
05FB: 7F           .db $7F
05FC: C8 B1       EORB #$B1
05FE: 7F           .db $7F
05FF: C8 B2       EORB #$B2
0601: 39           RTS
0602: 34 08       PSHS (regs)
0604: 86 D0       LDA #$D0
0606: 1F 8B       TFR
0608: B6 C8 B6    LDA $C8B6
060B: 27 60       BEQ $066D
060D: B6 C8 B7    LDA $C8B7
0610: 27 0F       BEQ $0621
0612: 4A           .db $4A
0613: B7 C8 B7    STA $C8B7
0616: 81 00       CMPA #$00
0618: 26 55       BNE $066F
061A: BE           .db $BE
061B: C8 B1       EORB #$B1
061D: 27 4E       BEQ $066D
061F: 20 0F       BRA $0630
0621: BE           .db $BE
0622: C8 B1       EORB #$B1
0624: 27 47       BEQ $066D
0626: E6           .db $E6
0627: 80           .db $80
0628: C1 FF       CMPB #$FF
062A: 27 37       BEQ $0663
062C: C1 00       CMPB #$00
062E: 26 0A       BNE $063A
0630: E6           .db $E6
0631: 80           .db $80
0632: 27 2A       BEQ $065E
0634: C1 FF       CMPB #$FF
0636: 27 2B       BEQ $0663
0638: 20 09       BRA $0643
063A: 5A           .db $5A
063B: F7 C8 B7    STB $C8B7
063E: BF C8 B1    STX $C8B1
0641: 20 2C       BRA $066F
0643: 34 04       PSHS (regs)
0645: A6           .db $A6
0646: 80           .db $80
0647: E6           .db $E6
0648: 80           .db $80
0649: 34 10       PSHS (regs)
064B: BD F2 56    JSR $F256
064E: 35 10       PULS (regs)
0650: 35 04       PULS (regs)
0652: 5A           .db $5A
0653: 27 04       BEQ $0659
0655: 34 04       PSHS (regs)
0657: 20 EC       BRA $0645
0659: BF C8 B1    STX $C8B1
065C: 20 11       BRA $066F
065E: 7F           .db $7F
065F: C8 B6       EORB #$B6
0661: 20 0C       BRA $066F
0663: EC           .db $EC
0664: 84           .db $84
0665: FD C8 B1    STD $C8B1
0668: 7F           .db $7F
0669: C8 B7       EORB #$B7
066B: 20 02       BRA $066F
066D: 20 00       BRA $066F
066F: B6 C8 BA    LDA $C8BA
0672: 27 03       BEQ $0677
0674: BD 07 27    JSR $0727
0677: 35 08       PULS (regs)
0679: 39           RTS
067A: FE C8 B8    LDU $C8B8
067D: E6           .db $E6
067E: C4 C1       ANDB #$C1
0680: D0           .db $D0
0681: 10 26 00 B1 LBNE <rel>
0685: E6           .db $E6
0686: 41           .db $41
0687: C1 20       CMPB #$20
0689: 10 27 01 16 LBEQ <rel>
068D: 31           .db $31
068E: 41           .db $41
068F: E6           .db $E6
0690: C4 C5       ANDB #$C5
0692: 20 10       BRA $06A4
0694: 27 00       BEQ $0696
0696: B7 E6 42    STA $E642
0699: 86 04       LDA #$04
069B: BD F2 56    JSR $F256
069E: E6           .db $E6
069F: 41           .db $41
06A0: 86 05       LDA #$05
06A2: BD F2 56    JSR $F256
06A5: 31           .db $31
06A6: 22 E6       BHI $068E
06A8: C4 C5       ANDB #$C5
06AA: 40           .db $40
06AB: 10 27 00 AE LBEQ <rel>
06AF: E6           .db $E6
06B0: A4           .db $A4
06B1: 86 06       LDA #$06
06B3: BD F2 56    JSR $F256
06B6: 31           .db $31
06B7: 21           .db $21
06B8: E6           .db $E6
06B9: C4 C4       ANDB #$C4
06BB: 0F           .db $0F
06BC: 86 0A       LDA #$0A
06BE: BD F2 56    JSR $F256
06C1: E6           .db $E6
06C2: C4 C5       ANDB #$C5
06C4: 10 10        (unimpl prefix)
06C6: 27 00       BEQ $06C8
06C8: AF           .db $AF
06C9: F6 C8 07    LDB $C807
06CC: CA           .db $CA
06CD: 04           .db $04
06CE: 86 07       LDA #$07
06D0: BD F2 56    JSR $F256
06D3: 16           .db $16
06D4: 00           .db $00
06D5: AC           .db $AC
06D6: F6 C8 07    LDB $C807
06D9: C4 FB       ANDB #$FB
06DB: 86 07       LDA #$07
06DD: BD F2 56    JSR $F256
06E0: E6           .db $E6
06E1: C4 C5       ANDB #$C5
06E3: 80           .db $80
06E4: 10 27 00 AC LBEQ <rel>
06E8: F6 C8 07    LDB $C807
06EB: CA           .db $CA
06EC: 20 86       BRA $0674
06EE: 07           .db $07
06EF: BD F2 56    JSR $F256
06F2: 16           .db $16
06F3: 00           .db $00
06F4: A9           .db $A9
06F5: F6 C8 07    LDB $C807
06F8: C4 DF       ANDB #$DF
06FA: 86 07       LDA #$07
06FC: BD F2 56    JSR $F256
06FF: 10 BF        (unimpl prefix)
0701: C8 B8       EORB #$B8
0703: 39           RTS
0704: 7F           .db $7F
0705: C8 BA       EORB #$BA
0707: 86 0A       LDA #$0A
0709: C6 00       LDB #$00
070B: BD F2 56    JSR $F256
070E: CC 00 00    LDD #$0000
0711: FD C8 B8    STD $C8B8
0714: 39           RTS
0715: BF C8 B8    STX $C8B8
0718: 86 01       LDA #$01
071A: B7 C8 BA    STA $C8BA
071D: 39           RTS
071E: B6 C8 BA    LDA $C8BA
0721: 27 03       BEQ $0726
0723: BD 07 27    JSR $0727
0726: 39           RTS
0727: FE C8 B8    LDU $C8B8
072A: E6           .db $E6
072B: C4 C1       ANDB #$C1
072D: D0           .db $D0
072E: 26 06       BNE $0736
0730: E6           .db $E6
0731: 41           .db $41
0732: C1 20       CMPB #$20
0734: 27 6D       BEQ $07A3
0736: 31           .db $31
0737: 41           .db $41
0738: E6           .db $E6
0739: C4 C5       ANDB #$C5
073B: 20 27       BRA $0764
073D: 10 E6        (unimpl prefix)
073F: 42           .db $42
0740: 86 04       LDA #$04
0742: BD F2 56    JSR $F256
0745: E6           .db $E6
0746: 41           .db $41
0747: 86 05       LDA #$05
0749: BD F2 56    JSR $F256
074C: 31           .db $31
074D: 22 E6       BHI $0735
074F: C4 C5       ANDB #$C5
0751: 40           .db $40
0752: 27 09       BEQ $075D
0754: E6           .db $E6
0755: A4           .db $A4
0756: 86 06       LDA #$06
0758: BD F2 56    JSR $F256
075B: 31           .db $31
075C: 21           .db $21
075D: E6           .db $E6
075E: C4 C4       ANDB #$C4
0760: 0F           .db $0F
0761: 86 0A       LDA #$0A
0763: BD F2 56    JSR $F256
0766: E6           .db $E6
0767: C4 C5       ANDB #$C5
0769: 10 27 0C F6 LBEQ <rel>
076D: C8 07       EORB #$07
076F: CA           .db $CA
0770: 04           .db $04
0771: 86 07       LDA #$07
0773: BD F2 56    JSR $F256
0776: 20 0A       BRA $0782
0778: F6 C8 07    LDB $C807
077B: C4 FB       ANDB #$FB
077D: 86 07       LDA #$07
077F: BD F2 56    JSR $F256
0782: E6           .db $E6
0783: C4 C5       ANDB #$C5
0785: 80           .db $80
0786: 27 0C       BEQ $0794
0788: F6 C8 07    LDB $C807
078B: CA           .db $CA
078C: 20 86       BRA $0714
078E: 07           .db $07
078F: BD F2 56    JSR $F256
0792: 20 0A       BRA $079E
0794: F6 C8 07    LDB $C807
0797: C4 DF       ANDB #$DF
0799: 86 07       LDA #$07
079B: BD F2 56    JSR $F256
079E: 10 BF        (unimpl prefix)
07A0: C8 B8       EORB #$B8
07A2: 39           RTS
07A3: 7F           .db $7F
07A4: C8 BA       EORB #$BA
07A6: 86 0A       LDA #$0A
07A8: C6 00       LDB #$00
07AA: BD F2 56    JSR $F256
07AD: CC 00 00    LDD #$0000
07B0: FD C8 B8    STD $C8B8
07B3: 39           RTS
07B4: 54           .db $54
07B5: 45           .db $45
07B6: 53           .db $53
07B7: 54           .db $54
07B8: 20 53       BRA $080D
07BA: 55           .db $55
07BB: 49           .db $49
07BC: 54           .db $54
07BD: 45           .db $45
07BE: 80           .db $80
07BF: 07           .db $07
07C0: 07           .db $07
07C1: CE 07 FB    LDU #$07FB
07C4: 08           .db $08
07C5: 22 08       BHI $07CF
07C7: 34 08       PSHS (regs)
07C9: 46           .db $46
07CA: 08           .db $08
07CB: 6D           .db $6D
07CC: 08           .db $08
07CD: 94           .db $94
07CE: 7F           .db $7F
07CF: 13           .db $13
07D0: AE 00       LDX (indexed)
07D2: 00           .db $00
07D3: FF           .db $FF
07D4: EF           .db $EF
07D5: 06           .db $06
07D6: FF           .db $FF
07D7: 02           .db $02
07D8: 07           .db $07
07D9: FF           .db $FF
07DA: D6           .db $D6
07DB: 09           .db $09
07DC: FF           .db $FF
07DD: 0B           .db $0B
07DE: 11 FF        (prefix2)
07E0: 0C           .db $0C
07E1: FC FF 0D    LDD $FF0D
07E4: 10 FF        (unimpl prefix)
07E6: 0B           .db $0B
07E7: 09           .db $09
07E8: FF           .db $FF
07E9: 0C           .db $0C
07EA: 01           .db $01
07EB: FF           .db $FF
07EC: 08           .db $08
07ED: F8           .db $F8
07EE: FF           .db $FF
07EF: 02           .db $02
07F0: F0           .db $F0
07F1: FF           .db $FF
07F2: FC F1 FF    LDD $F1FF
07F5: F8           .db $F8
07F6: EA           .db $EA
07F7: FF           .db $FF
07F8: 00           .db $00
07F9: 00           .db $00
07FA: 02           .db $02
07FB: 7F           .db $7F
07FC: FB           .db $FB
07FD: E3           .db $E3
07FE: 00           .db $00
07FF: 00           .db $00
0800: FF           .db $FF
0801: E7 F8       STB (indexed)
0803: FF           .db $FF
0804: 04           .db $04
0805: 10 FF        (unimpl prefix)
0807: 0C           .db $0C
0808: 02           .db $02
0809: FF           .db $FF
080A: 03           .db $03
080B: 0B           .db $0B
080C: FF           .db $FF
080D: FA           .db $FA
080E: 00           .db $00
080F: FF           .db $FF
0810: 03           .db $03
0811: 0D           .db $0D
0812: FF           .db $FF
0813: 22 F7       BHI $080C
0815: FF           .db $FF
0816: FD F1 FF    STD $F1FF
0819: F5           .db $F5
081A-081A: [FF padding - 1 bytes]
081B: FF           .db $FF
081C: F5           .db $F5
081D: F7 FF 00    STB $FF00
0820: 00           .db $00
0821: 02           .db $02
0822: 7F           .db $7F
0823: 07           .db $07
0824: CE 00 00    LDU #$0000
0827: FF           .db $FF
0828: F8           .db $F8
0829: 02           .db $02
082A: FF           .db $FF
082B: 07           .db $07
082C: 08           .db $08
082D: FF           .db $FF
082E: 01           .db $01
082F: F6 FF 00    LDB $FF00
0832: 00           .db $00
0833: 02           .db $02
0834: 7F           .db $7F
0835: 06           .db $06
0836: F4           .db $F4
0837: 00           .db $00
0838: 00           .db $00
0839: FF           .db $FF
083A: F6 FD FF    LDB $FDFF
083D: 02           .db $02
083E: 07           .db $07
083F: FF           .db $FF
0840: 08           .db $08
0841: FC FF FE    LDD $FFFE
0844: 01           .db $01
0845: 02           .db $02
0846: 7F           .db $7F
0847: F3           .db $F3
0848: 0A           .db $0A
0849: 00           .db $00
084A: 00           .db $00
084B: FF           .db $FF
084C: 29 02       BVS $0850
084E: FF           .db $FF
084F: 02           .db $02
0850: 0D           .db $0D
0851: FF           .db $FF
0852: EB           .db $EB
0853: 0A           .db $0A
0854: FF           .db $FF
0855: 1A           .db $1A
0856: 07           .db $07
0857: FF           .db $FF
0858: 03           .db $03
0859: 14           .db $14
085A: FF           .db $FF
085B: D8           .db $D8
085C: EF           .db $EF
085D: FF           .db $FF
085E: FE F3 FF    LDU $F3FF
0861: 0D           .db $0D
0862: F8           .db $F8
0863: FF           .db $FF
0864: EE FC       LDU (indexed)
0866: FF           .db $FF
0867: FC F6 FF    LDD $F6FF
086A: 00           .db $00
086B: 00           .db $00
086C: 02           .db $02
086D: 7F           .db $7F
086E: 06           .db $06
086F: 45           .db $45
0870: 00           .db $00
0871: 00           .db $00
0872: FF           .db $FF
0873: 08           .db $08
0874: F5           .db $F5
0875: FF           .db $FF
0876: F4           .db $F4
0877: F7 FF F7    STB $FFF7
087A: 01           .db $01
087B: FF           .db $FF
087C: FE 0C FF    LDU $0CFF
087F: 03           .db $03
0880: FA           .db $FA
0881: FF           .db $FF
0882: 05           .db $05
0883: 01           .db $01
0884: FF           .db $FF
0885: 02           .db $02
0886: 17 FF F3    LBSR <rel>
0889: FD FF F9    STD $FFF9
088C: EE FF       LDU (indexed)
088E: 04           .db $04
088F: F0           .db $F0
0890: FF           .db $FF
0891: 0B           .db $0B
0892: F8           .db $F8
0893: 02           .db $02
0894: 7F           .db $7F
0895: 06           .db $06
0896: 45           .db $45
0897: 00           .db $00
0898: 00           .db $00
0899: FF           .db $FF
089A: 00           .db $00
089B: 0C           .db $0C
089C: FF           .db $FF
089D: 0C           .db $0C
089E: F8           .db $F8
089F: FF           .db $FF
08A0: 03           .db $03
08A1: F0           .db $F0
08A2: FF           .db $FF
08A3: FB           .db $FB
08A4: FC 02 00    LDD $0200
08A7: 06           .db $06
08A8: 00           .db $00
08A9: 66           .db $66
08AA: 01           .db $01
08AB: 01           .db $01
08AC: 08           .db $08
08AD: 0F           .db $0F
08AE: 09           .db $09
08AF: 00           .db $00
08B0: 0A           .db $0A
08B1: 00           .db $00
08B2: 07           .db $07
08B3: FE 19 06    LDU $1906
08B6: 00           .db $00
08B7: 1C           .db $1C
08B8: 01           .db $01
08B9: 01           .db $01
08BA: 08           .db $08
08BB: 0F           .db $0F
08BC: 09           .db $09
08BD: 00           .db $00
08BE: 0A           .db $0A
08BF: 00           .db $00
08C0: 07           .db $07
08C1: FE 19 06    LDU $1906
08C4: 00           .db $00
08C5: EF           .db $EF
08C6: 01           .db $01
08C7: 00           .db $00
08C8: 08           .db $08
08C9: 0F           .db $0F
08CA: 09           .db $09
08CB: 00           .db $00
08CC: 0A           .db $0A
08CD: 00           .db $00
08CE: 07           .db $07
08CF: FE 19 06    LDU $1906
08D2: 00           .db $00
08D3: B3           .db $B3
08D4: 01           .db $01
08D5: 00           .db $00
08D6: 08           .db $08
08D7: 0F           .db $0F
08D8: 09           .db $09
08D9: 00           .db $00
08DA: 0A           .db $0A
08DB: 00           .db $00
08DC: 07           .db $07
08DD: FE 19 06    LDU $1906
08E0: 00           .db $00
08E1: EF           .db $EF
08E2: 01           .db $01
08E3: 00           .db $00
08E4: 08           .db $08
08E5: 0F           .db $0F
08E6: 09           .db $09
08E7: 00           .db $00
08E8: 0A           .db $0A
08E9: 00           .db $00
08EA: 07           .db $07
08EB: FE 18 06    LDU $1806
08EE: 00           .db $00
08EF: 1C           .db $1C
08F0: 01           .db $01
08F1: 01           .db $01
08F2: 08           .db $08
08F3: 0F           .db $0F
08F4: 09           .db $09
08F5: 00           .db $00
08F6: 0A           .db $0A
08F7: 00           .db $00
08F8: 07           .db $07
08F9: FE 1A 06    LDU $1A06
08FC: 00           .db $00
08FD: 66           .db $66
08FE: 01           .db $01
08FF: 01           .db $01
0900: 08           .db $08
0901: 0F           .db $0F
0902: 09           .db $09
0903: 00           .db $00
0904: 0A           .db $0A
0905: 00           .db $00
0906: 07           .db $07
0907: FE 32 FF    LDU $32FF
090A: 08           .db $08
090B: A6           .db $A6
090C: A0           .db $A0
090D: 00           .db $00
090E: AA           .db $AA
090F: AE 00       LDX (indexed)
0911: CA           .db $CA
0912: AD           .db $AD
0913: 00           .db $00
0914: EA           .db $EA
0915: AC           .db $AC
0916: 01           .db $01
0917: 0A           .db $0A
0918: AC           .db $AC
0919: 01           .db $01
091A: 2A AC       BPL $08C8
091C: 01           .db $01
091D: 4A           .db $4A
091E: AC           .db $AC
091F: 01           .db $01
0920: 6A           .db $6A
0921: AC           .db $AC
0922: 01           .db $01
0923: 8A A6       ORA #$A6
0925: 01           .db $01
0926: AA           .db $AA
0927: D0           .db $D0
0928: 20 A0       BRA $08CA
092A: 0F           .db $0F
092B: FF           .db $FF
092C: 8F           .db $8F
092D: 8D 8D       BSR $08BC
092F: 8D 8D       BSR $08BE
0931: 8D 8D       BSR $08C0
0933: 8D 8D       BSR $08C2
0935: 8D 8D       BSR $08C4
0937: 8D 8D       BSR $08C6
0939: 8D 8D       BSR $08C8
093B: 8D 8D       BSR $08CA
093D: 8D 8D       BSR $08CC
093F: 8D 8D       BSR $08CE
0941: 8D 8D       BSR $08D0
0943: 8D 8D       BSR $08D2
0945: 8D 8D       BSR $08D4
0947: 8D 8D       BSR $08D6
0949: 8D 8D       BSR $08D8
094B: 8D 8D       BSR $08DA
094D: 8D 8D       BSR $08DC
094F: 8D 8D       BSR $08DE
0951: 8D 8D       BSR $08E0
0953: 8D 8D       BSR $08E2
0955: 8D 8D       BSR $08E4
0957: 8D 8D       BSR $08E6
0959: 8D 8D       BSR $08E8
095B: 8D 8D       BSR $08EA
095D: 8D 8D       BSR $08EC
095F: 8D 8D       BSR $08EE
0961: 8D 8D       BSR $08F0
0963: 8D 8D       BSR $08F2
0965: 8D 8D       BSR $08F4
0967: 8D 8D       BSR $08F6
0969: 8D 8D       BSR $08F8
096B: 8D 8D       BSR $08FA
096D: 8D 8D       BSR $08FC
096F: 8D 8D       BSR $08FE
0971: 8D 8D       BSR $0900
0973: 8D 8D       BSR $0902
0975: 8D 8D       BSR $0904
0977: 8D 8D       BSR $0906
0979: 8D 8D       BSR $0908
097B: 8D 8D       BSR $090A
097D: 8D 8D       BSR $090C
097F: 8D 8D       BSR $090E
0981: D0           .db $D0
0982: 20 60       BRA $09E4
0984: 0F           .db $0F
0985: FF           .db $FF
0986: 00           .db $00
0987: 0E           .db $0E
0988: 0E           .db $0E
0989: 0E           .db $0E
098A: 0D           .db $0D
098B: 0D           .db $0D
098C: 0D           .db $0D
098D: 0C           .db $0C
098E: 0C           .db $0C
098F: 0C           .db $0C
0990: 0C           .db $0C
0991: 0C           .db $0C
0992: 0C           .db $0C
0993: 0C           .db $0C
0994: 0C           .db $0C
0995: 0C           .db $0C
0996: 0C           .db $0C
0997: 0C           .db $0C
0998: 0C           .db $0C
0999: 0C           .db $0C
099A: 0C           .db $0C
099B: 0C           .db $0C
099C: 0C           .db $0C
099D: D0           .db $D0
099E: 20 60       BRA $0A00
09A0: 00           .db $00
09A1: 02           .db $02
09A2: 1A           .db $1A
09A3: 07           .db $07
09A4: 0E           .db $0E
09A5: 0E           .db $0E
09A6: 0E           .db $0E
09A7: 0E           .db $0E
09A8: 0D           .db $0D
09A9: 0D           .db $0D
09AA: 0D           .db $0D
09AB: 0D           .db $0D
09AC: 0C           .db $0C
09AD: 0C           .db $0C
09AE: 0C           .db $0C
09AF: 0B           .db $0B
09B0: 0B           .db $0B
09B1: 0B           .db $0B
09B2: 0B           .db $0B
09B3: 0B           .db $0B
09B4: 0B           .db $0B
09B5: 0B           .db $0B
09B6: 0B           .db $0B
09B7: 0B           .db $0B
09B8: 0B           .db $0B
09B9: 0B           .db $0B
09BA: 0B           .db $0B
09BB: 0B           .db $0B
09BC: 0B           .db $0B
09BD: 0B           .db $0B
09BE: 0B           .db $0B
09BF: 0B           .db $0B
09C0: 0B           .db $0B
09C1: 0B           .db $0B
09C2: 0B           .db $0B
09C3: 0B           .db $0B
09C4: 0B           .db $0B
09C5: 0B           .db $0B
09C6: 0B           .db $0B
09C7: D0           .db $D0
09C8: 20 A0       BRA $096A
09CA: 00           .db $00
09CB: 5F           CLRB
09CC: A7 00       STA (indexed)
09CE: 5F           CLRB
09CF: AF           .db $AF
09D0: 00           .db $00
09D1: 5F           CLRB
09D2: AD           .db $AD
09D3: 00           .db $00
09D4: 5F           CLRB
09D5: AB           .db $AB
09D6: 00           .db $00
09D7: 5F           CLRB
09D8: A9           .db $A9
09D9: 00           .db $00
09DA: 55           .db $55
09DB: A7 00       STA (indexed)
09DD: 55           .db $55
09DE: A7 00       STA (indexed)
09E0: 55           .db $55
09E1: A7 00       STA (indexed)
09E3: 55           .db $55
09E4: A7 00       STA (indexed)
09E6: 5F           CLRB
09E7: A7 00       STA (indexed)
09E9: 5F           CLRB
09EA: A7 00       STA (indexed)
09EC: 5F           CLRB
09ED: A7 00       STA (indexed)
09EF: 5F           CLRB
09F0: A7 00       STA (indexed)
09F2: 65           .db $65
09F3: A7 00       STA (indexed)
09F5: 65           .db $65
09F6: A7 00       STA (indexed)
09F8: 65           .db $65
09F9: A7 00       STA (indexed)
09FB: 65           .db $65
09FC: A7 00       STA (indexed)
09FE: 65           .db $65
09FF: A7 00       STA (indexed)
0A01: 47           .db $47
0A02: A7 00       STA (indexed)
0A04: 47           .db $47
0A05: A7 00       STA (indexed)
0A07: 47           .db $47
0A08: A7 00       STA (indexed)
0A0A: 47           .db $47
0A0B: A6           .db $A6
0A0C: 00           .db $00
0A0D: 4B           .db $4B
0A0E: A5           .db $A5
0A0F: 00           .db $00
0A10: 4B           .db $4B
0A11: A4           .db $A4
0A12: 00           .db $00
0A13: 4B           .db $4B
0A14: A3           .db $A3
0A15: 00           .db $00
0A16: 4B           .db $4B
0A17: A2           .db $A2
0A18: 00           .db $00
0A19: 5F           CLRB
0A1A: A1           .db $A1
0A1B: 00           .db $00
0A1C: 5F           CLRB
0A1D: A0           .db $A0
0A1E: 00           .db $00
0A1F: 5F           CLRB
0A20: D0           .db $D0
0A21: 20 60       BRA $0A83
0A23: 00           .db $00
0A24: 8C           .db $8C
0A25: 08           .db $08
0A26: 6F           .db $6F
0A27: 00           .db $00
0A28: AA           .db $AA
0A29: 08           .db $08
0A2A: 6F           .db $6F
0A2B: 00           .db $00
0A2C: C8 08       EORB #$08
0A2E: 6E           .db $6E
0A2F: 00           .db $00
0A30: E6           .db $E6
0A31: 08           .db $08
0A32: 6D           .db $6D
0A33: 01           .db $01
0A34: 04           .db $04
0A35: 08           .db $08
0A36: 6C           .db $6C
0A37: 01           .db $01
0A38: 22 08       BHI $0A42
0A3A: 6C           .db $6C
0A3B: 01           .db $01
0A3C: 40           .db $40
0A3D: 08           .db $08
0A3E: 6C           .db $6C
0A3F: 01           .db $01
0A40: 5E           .db $5E
0A41: 08           .db $08
0A42: 6C           .db $6C
0A43: 01           .db $01
0A44: 7C 08 6C    INC $086C
0A47: 01           .db $01
0A48: 9A           .db $9A
0A49: 08           .db $08
0A4A: 6C           .db $6C
0A4B: 01           .db $01
0A4C: B8           .db $B8
0A4D: 08           .db $08
0A4E: 6C           .db $6C
0A4F: 01           .db $01
0A50: D6           .db $D6
0A51: 08           .db $08
0A52: 69           .db $69
0A53: 01           .db $01
0A54: F4           .db $F4
0A55: 08           .db $08
0A56: 66           .db $66
0A57: 02           .db $02
0A58: 12           .db $12
0A59: 08           .db $08
0A5A: 63           .db $63
0A5B: 02           .db $02
0A5C: 30 08       LEAX (indexed)
0A5E: D0           .db $D0
0A5F: 20 A0       BRA $0A01
0A61: 00           .db $00
0A62: 34 AF       PSHS (regs)
0A64: 00           .db $00
0A65: 3A           .db $3A
0A66: AC           .db $AC
0A67: 00           .db $00
0A68: 42           .db $42
0A69: AC           .db $AC
0A6A: 00           .db $00
0A6B: 48           .db $48
0A6C: AC           .db $AC
0A6D: 00           .db $00
0A6E: 4E           .db $4E
0A6F: AC           .db $AC
0A70: 00           .db $00
0A71: 56           .db $56
0A72: AC           .db $AC
0A73: 00           .db $00
0A74: 5C           .db $5C
0A75: AC           .db $AC
0A76: 00           .db $00
0A77: 62           .db $62
0A78: AC           .db $AC
0A79: 00           .db $00
0A7A: 6A           .db $6A
0A7B: AC           .db $AC
0A7C: 00           .db $00
0A7D: 70           .db $70
0A7E: AC           .db $AC
0A7F: 00           .db $00
0A80: 76           .db $76
0A81: AC           .db $AC
0A82: 00           .db $00
0A83: 7C AC 00    INC $AC00
0A86: 84           .db $84
0A87: AC           .db $AC
0A88: 00           .db $00
0A89: 8A AC       ORA #$AC
0A8B: 00           .db $00
0A8C: 90           .db $90
0A8D: AC           .db $AC
0A8E: 00           .db $00
0A8F: 98           .db $98
0A90: AC           .db $AC
0A91: 00           .db $00
0A92: 9E           .db $9E
0A93: AC           .db $AC
0A94: 00           .db $00
0A95: A4           .db $A4
0A96: AC           .db $AC
0A97: 00           .db $00
0A98: AC           .db $AC
0A99: AC           .db $AC
0A9A: 00           .db $00
0A9B: B2           .db $B2
0A9C: AC           .db $AC
0A9D: 00           .db $00
0A9E: B8           .db $B8
0A9F: A9           .db $A9
0AA0: 00           .db $00
0AA1: C0           .db $C0
0AA2: A7 00       STA (indexed)
0AA4: C6 A4       LDB #$A4
0AA6: 00           .db $00
0AA7: CC A2 00    LDD #$A200
0AAA: D4           .db $D4
0AAB: D0           .db $D0
0AAC: 20 60       BRA $0B0E
0AAE: 00           .db $00
0AAF: 01           .db $01
0AB0: 1E           .db $1E
0AB1: 6F           .db $6F
0AB2: 01           .db $01
0AB3: 74           .db $74
0AB4: 1E           .db $1E
0AB5: 6A           .db $6A
0AB6: 02           .db $02
0AB7: E8           .db $E8
0AB8: 1E           .db $1E
0AB9: 6A           .db $6A
0ABA: 04           .db $04
0ABB: 5C           .db $5C
0ABC: 1E           .db $1E
0ABD: 6A           .db $6A
0ABE: 05           .db $05
0ABF: D0           .db $D0
0AC0: 1E           .db $1E
0AC1: 6A           .db $6A
0AC2: 07           .db $07
0AC3: 44           .db $44
0AC4: 1E           .db $1E
0AC5: 6A           .db $6A
0AC6: 08           .db $08
0AC7: B8           .db $B8
0AC8: 1E           .db $1E
0AC9: 6A           .db $6A
0ACA: 0A           .db $0A
0ACB: 2C 1E       BGE $0AEB
0ACD: 6A           .db $6A
0ACE: 0B           .db $0B
0ACF: A2           .db $A2
0AD0: 1E           .db $1E
0AD1: 6A           .db $6A
0AD2: 0D           .db $0D
0AD3: 16           .db $16
0AD4: 1E           .db $1E
0AD5: 6A           .db $6A
0AD6: 0E           .db $0E
0AD7: 8A 1E       ORA #$1E
0AD9: 6A           .db $6A
0ADA: 0F           .db $0F
0ADB: FE 1E 6A    LDU $1E6A
0ADE: 0F           .db $0F
0ADF: FF           .db $FF
0AE0: 1E           .db $1E
0AE1: 0A           .db $0A
0AE2: 0A           .db $0A
0AE3: 0A           .db $0A
0AE4: 0A           .db $0A
0AE5: 0A           .db $0A
0AE6: 0A           .db $0A
0AE7: 0A           .db $0A
0AE8: 0A           .db $0A
0AE9: 0A           .db $0A
0AEA: 0A           .db $0A
0AEB: D0           .db $D0
0AEC: 20 FF       BRA $0AED
0AEE-7FFE: [FF padding - 29969 bytes]
7FFF: FF           .db $FF
