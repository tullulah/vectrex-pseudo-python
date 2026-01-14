; ========================================
; BANK #0 (offset $000000-$003FFF)
; ========================================
; --- ROM HEADER ---
$0000:  ; Signature: "g GCE 1982"
$000A:  ; Music pointer: $80FD
$0010:  ; Title: "�CALL GRAPH TES"

$0020:  80           FCB $80
$0021:  00           FCB $00
$0022:  86 D0        LDA #$D0
$0024:  1F 8B        TFR A,DP
$0026:  7F C8 0E     CLR $C80E
$0029:  86 80        LDA #$80
$002B:  B7 D0 04     STA $D004
$002E:  10 CE        PAGE2 $CE
$0030:  CB           FCB $CB
$0031:  FF           FCB $FF
$0032:  86 00        LDA #$00
$0034:  B7 C8 80     STA $C880
$0037:  CC FF CE     LDD #$FFCE
$003A:  FD C8 9A     STD $C89A
$003D:  CC 00 3C     LDD #$003C
$0040:  FD C8 9C     STD $C89C
$0043:  CC 00 00     LDD #$0000
$0046:  FD C8 9E     STD $C89E
$0049:  CC 00 00     LDD #$0000
$004C:  FD C8 A0     STD $C8A0
$004F:  CC 00 32     LDD #$0032
$0052:  FD C8 A2     STD $C8A2
$0055:  CC FF C4     LDD #$FFC4
$0058:  FD C8 A4     STD $C8A4
$005B:  CC 00 00     LDD #$0000
$005E:  FD C8 A6     STD $C8A6
$0061:  CC 00 7F     LDD #$007F
$0064:  FD C8 81     STD $C881
$0067:  FC C8 81     LDD $C881
$006A:  FD C8 A8     STD $C8A8
$006D:  BD 04 92     JSR $0492
$0070:  4F           CLRA
$0071:  5F           CLRB
$0072:  FD C8 81     STD $C881
$0075:  BD F1 AF     JSR $F1AF
$0078:  7F C8 23     CLR $C823
$007B:  86 01        LDA #$01
$007D:  B7 C8 1A     STA $C81A
$0080:  86 01        LDA #$01
$0082:  B7 C8 1F     STA $C81F
$0085:  86 03        LDA #$03
$0087:  B7 C8 20     STA $C820
$008A:  86 00        LDA #$00
$008C:  B7 C8 21     STA $C821
$008F:  B7 C8 22     STA $C822
$0092:  86 80        LDA #$80
$0094:  B7 D0 04     STA $D004
$0097:  BD 00 9C     JSR $009C
$009A:  20 D9        BRA $D9
$009C:  32 7E        LEAS 126,S
$009E:  BD F1 92     JSR $F192
$00A1:  BD F1 AA     JSR $F1AA
$00A4:  BD F1 BA     JSR $F1BA
$00A7:  BD F1 AF     JSR $F1AF
$00AA:  CC 00 0C     LDD #$000C
$00AD:  FD C8 81     STD $C881
$00B0:  BE           FCB $BE
$00B1:  C8           FCB $C8
$00B2:  81           FCB $81
$00B3:  AF           FCB $AF
$00B4:  60           FCB $60
$00B5:  EC           FCB $EC
$00B6:  60           FCB $60
$00B7:  FD C8 81     STD $C881
$00BA:  FC C8 81     LDD $C881
$00BD:  FD C8 83     STD $C883
$00C0:  34 06        PSHS B,A
$00C2:  CC 00 0F     LDD #$000F
$00C5:  FD C8 81     STD $C881
$00C8:  FC C8 81     LDD $C881
$00CB:  FD C8 87     STD $C887
$00CE:  35 06        PULS B,A
$00D0:  FD C8 83     STD $C883
$00D3:  FC C8 83     LDD $C883
$00D6:  F3           FCB $F3
$00D7:  C8           FCB $C8
$00D8:  87           FCB $87
$00D9:  FD C8 81     STD $C881
$00DC:  BE           FCB $BE
$00DD:  C8           FCB $C8
$00DE:  81           FCB $81
$00DF:  AF           FCB $AF
$00E0:  60           FCB $60
$00E1:  CC 00 64     LDD #$0064
$00E4:  FD C8 81     STD $C881
$00E7:  FC C8 81     LDD $C881
$00EA:  FD C8 A8     STD $C8A8
$00ED:  BD 04 92     JSR $0492
$00F0:  4F           CLRA
$00F1:  5F           CLRB
$00F2:  FD C8 81     STD $C881
$00F5:  EC           FCB $EC
$00F6:  60           FCB $60
$00F7:  FD C8 81     STD $C881
$00FA:  FC C8 81     LDD $C881
$00FD:  B7 C0 02     STA $C002
$0100:  02           FCB $02
$0101:  F7           FCB $F7
$0102:  C0           FCB $C0
$0103:  00           FCB $00
$0104:  86 FE        LDA #$FE
$0106:  B7 C0 01     STA $C001
$0109:  8E 00 00     LDX #$0000
$010C:  BF           FCB $BF
$010D:  C0           FCB $C0
$010E:  04           FCB $04
$010F:  16 00 02     LBRA $0002
$0112:  61           FCB $61
$0113:  00           FCB $00
$0114:  CC 00 00     LDD #$0000
$0117:  FD C8 81     STD $C881
$011A:  32 62        LEAS 98,S
$011C:  39           RTS
$011D:  BD 01 24     JSR $0124
$0120:  BD 01 25     JSR $0125
$0123:  39           RTS
$0124:  39           RTS
$0125:  39           RTS
$0126:  FC C8 9A     LDD $C89A
$0129:  FD C8 81     STD $C881
$012C:  FC C8 81     LDD $C881
$012F:  FD C8 83     STD $C883
$0132:  34 06        PSHS B,A
$0134:  CC 00 01     LDD #$0001
$0137:  FD C8 81     STD $C881
$013A:  FC C8 81     LDD $C881
$013D:  FD C8 87     STD $C887
$0140:  35 06        PULS B,A
$0142:  FD C8 83     STD $C883
$0145:  FC C8 83     LDD $C883
$0148:  F3           FCB $F3
$0149:  C8           FCB $C8
$014A:  87           FCB $87
$014B:  FD C8 81     STD $C881
$014E:  BE           FCB $BE
$014F:  C8           FCB $C8
$0150:  81           FCB $81
$0151:  CE           FCB $CE
$0152:  C8           FCB $C8
$0153:  9A           FCB $9A
$0154:  FF           FCB $FF
$0155:  C8           FCB $C8
$0156:  8B           FCB $8B
$0157:  AF           FCB $AF
$0158:  C4           FCB $C4
$0159:  FC C8 9A     LDD $C89A
$015C:  FD C8 81     STD $C881
$015F:  FC C8 81     LDD $C881
$0162:  FD C8 83     STD $C883
$0165:  CC 00 64     LDD #$0064
$0168:  FD C8 81     STD $C881
$016B:  FC C8 81     LDD $C881
$016E:  FD C8 87     STD $C887
$0171:  FC C8 83     LDD $C883
$0174:  B3           FCB $B3
$0175:  C8           FCB $C8
$0176:  87           FCB $87
$0177:  10 2E 00 09  LBGT $0009
$017B:  CC 00 00     LDD #$0000
$017E:  FD C8 81     STD $C881
$0181:  16 00 06     LBRA $0006
$0184:  CC 00 01     LDD #$0001
$0187:  FD C8 81     STD $C881
$018A:  FC C8 81     LDD $C881
$018D:  10 27 00 14  LBEQ $0014
$0191:  CC FF 9C     LDD #$FF9C
$0194:  FD C8 81     STD $C881
$0197:  BE           FCB $BE
$0198:  C8           FCB $C8
$0199:  81           FCB $81
$019A:  CE           FCB $CE
$019B:  C8           FCB $C8
$019C:  9A           FCB $9A
$019D:  FF           FCB $FF
$019E:  C8           FCB $C8
$019F:  8B           FCB $8B
$01A0:  AF           FCB $AF
$01A1:  C4           FCB $C4
$01A2:  16 00 00     LBRA $0000
$01A5:  FC C8 A0     LDD $C8A0
$01A8:  FD C8 81     STD $C881
$01AB:  FC C8 81     LDD $C881
$01AE:  FD C8 83     STD $C883
$01B1:  34 06        PSHS B,A
$01B3:  CC 00 01     LDD #$0001
$01B6:  FD C8 81     STD $C881
$01B9:  FC C8 81     LDD $C881
$01BC:  FD C8 87     STD $C887
$01BF:  35 06        PULS B,A
$01C1:  FD C8 83     STD $C883
$01C4:  FC C8 83     LDD $C883
$01C7:  F3           FCB $F3
$01C8:  C8           FCB $C8
$01C9:  87           FCB $87
$01CA:  FD C8 81     STD $C881
$01CD:  BE           FCB $BE
$01CE:  C8           FCB $C8
$01CF:  81           FCB $81
$01D0:  CE           FCB $CE
$01D1:  C8           FCB $C8
$01D2:  A0           FCB $A0
$01D3:  FF           FCB $FF
$01D4:  C8           FCB $C8
$01D5:  8B           FCB $8B
$01D6:  AF           FCB $AF
$01D7:  C4           FCB $C4
$01D8:  FC C8 A0     LDD $C8A0
$01DB:  FD C8 81     STD $C881
$01DE:  FC C8 81     LDD $C881
$01E1:  FD C8 83     STD $C883
$01E4:  CC 00 64     LDD #$0064
$01E7:  FD C8 81     STD $C881
$01EA:  FC C8 81     LDD $C881
$01ED:  FD C8 87     STD $C887
$01F0:  FC C8 83     LDD $C883
$01F3:  B3           FCB $B3
$01F4:  C8           FCB $C8
$01F5:  87           FCB $87
$01F6:  10 2E 00 09  LBGT $0009
$01FA:  CC 00 00     LDD #$0000
$01FD:  FD C8 81     STD $C881
$0200:  16 00 06     LBRA $0006
$0203:  CC 00 01     LDD #$0001
$0206:  FD C8 81     STD $C881
$0209:  FC C8 81     LDD $C881
$020C:  10 27 00 14  LBEQ $0014
$0210:  CC FF 9C     LDD #$FF9C
$0213:  FD C8 81     STD $C881
$0216:  BE           FCB $BE
$0217:  C8           FCB $C8
$0218:  81           FCB $81
$0219:  CE           FCB $CE
$021A:  C8           FCB $C8
$021B:  A0           FCB $A0
$021C:  FF           FCB $FF
$021D:  C8           FCB $C8
$021E:  8B           FCB $8B
$021F:  AF           FCB $AF
$0220:  C4           FCB $C4
$0221:  16 00 00     LBRA $0000
$0224:  FC C8 A2     LDD $C8A2
$0227:  FD C8 81     STD $C881
$022A:  FC C8 81     LDD $C881
$022D:  FD C8 83     STD $C883
$0230:  34 06        PSHS B,A
$0232:  CC 00 01     LDD #$0001
$0235:  FD C8 81     STD $C881
$0238:  FC C8 81     LDD $C881
$023B:  FD C8 87     STD $C887
$023E:  35 06        PULS B,A
$0240:  FD C8 83     STD $C883
$0243:  FC C8 83     LDD $C883
$0246:  B3           FCB $B3
$0247:  C8           FCB $C8
$0248:  87           FCB $87
$0249:  FD C8 81     STD $C881
$024C:  BE           FCB $BE
$024D:  C8           FCB $C8
$024E:  81           FCB $81
$024F:  CE           FCB $CE
$0250:  C8           FCB $C8
$0251:  A2           FCB $A2
$0252:  FF           FCB $FF
$0253:  C8           FCB $C8
$0254:  8B           FCB $8B
$0255:  AF           FCB $AF
$0256:  C4           FCB $C4
$0257:  FC C8 A4     LDD $C8A4
$025A:  FD C8 81     STD $C881
$025D:  FC C8 81     LDD $C881
$0260:  FD C8 83     STD $C883
$0263:  34 06        PSHS B,A
$0265:  CC 00 01     LDD #$0001
$0268:  FD C8 81     STD $C881
$026B:  FC C8 81     LDD $C881
$026E:  FD C8 87     STD $C887
$0271:  35 06        PULS B,A
$0273:  FD C8 83     STD $C883
$0276:  FC C8 83     LDD $C883
$0279:  B3           FCB $B3
$027A:  C8           FCB $C8
$027B:  87           FCB $87
$027C:  FD C8 81     STD $C881
$027F:  BE           FCB $BE
$0280:  C8           FCB $C8
$0281:  81           FCB $81
$0282:  CE           FCB $CE
$0283:  C8           FCB $C8
$0284:  A4           FCB $A4
$0285:  FF           FCB $FF
$0286:  C8           FCB $C8
$0287:  8B           FCB $8B
$0288:  AF           FCB $AF
$0289:  C4           FCB $C4
$028A:  FC C8 A2     LDD $C8A2
$028D:  FD C8 81     STD $C881
$0290:  FC C8 81     LDD $C881
$0293:  FD C8 83     STD $C883
$0296:  CC FF 9C     LDD #$FF9C
$0299:  FD C8 81     STD $C881
$029C:  FC C8 81     LDD $C881
$029F:  FD C8 87     STD $C887
$02A2:  FC C8 83     LDD $C883
$02A5:  B3           FCB $B3
$02A6:  C8           FCB $C8
$02A7:  87           FCB $87
$02A8:  10 2D        PAGE2 $2D
$02AA:  00           FCB $00
$02AB:  09           FCB $09
$02AC:  CC 00 00     LDD #$0000
$02AF:  FD C8 81     STD $C881
$02B2:  16 00 06     LBRA $0006
$02B5:  CC 00 01     LDD #$0001
$02B8:  FD C8 81     STD $C881
$02BB:  FC C8 81     LDD $C881
$02BE:  10 27 00 14  LBEQ $0014
$02C2:  CC 00 64     LDD #$0064
$02C5:  FD C8 81     STD $C881
$02C8:  BE           FCB $BE
$02C9:  C8           FCB $C8
$02CA:  81           FCB $81
$02CB:  CE           FCB $CE
$02CC:  C8           FCB $C8
$02CD:  A2           FCB $A2
$02CE:  FF           FCB $FF
$02CF:  C8           FCB $C8
$02D0:  8B           FCB $8B
$02D1:  AF           FCB $AF
$02D2:  C4           FCB $C4
$02D3:  16 00 00     LBRA $0000
$02D6:  FC C8 A4     LDD $C8A4
$02D9:  FD C8 81     STD $C881
$02DC:  FC C8 81     LDD $C881
$02DF:  FD C8 83     STD $C883
$02E2:  CC FF 9C     LDD #$FF9C
$02E5:  FD C8 81     STD $C881
$02E8:  FC C8 81     LDD $C881
$02EB:  FD C8 87     STD $C887
$02EE:  FC C8 83     LDD $C883
$02F1:  B3           FCB $B3
$02F2:  C8           FCB $C8
$02F3:  87           FCB $87
$02F4:  10 2D        PAGE2 $2D
$02F6:  00           FCB $00
$02F7:  09           FCB $09
$02F8:  CC 00 00     LDD #$0000
$02FB:  FD C8 81     STD $C881
$02FE:  16 00 06     LBRA $0006
$0301:  CC 00 01     LDD #$0001
$0304:  FD C8 81     STD $C881
$0307:  FC C8 81     LDD $C881
$030A:  10 27 00 14  LBEQ $0014
$030E:  CC 00 64     LDD #$0064
$0311:  FD C8 81     STD $C881
$0314:  BE           FCB $BE
$0315:  C8           FCB $C8
$0316:  81           FCB $81
$0317:  CE           FCB $CE
$0318:  C8           FCB $C8
$0319:  A4           FCB $A4
$031A:  FF           FCB $FF
$031B:  C8           FCB $C8
$031C:  8B           FCB $8B
$031D:  AF           FCB $AF
$031E:  C4           FCB $C4
$031F:  16 00 00     LBRA $0000
$0322:  39           RTS
$0323:  BD 03 2A     JSR $032A
$0326:  BD 03 6A     JSR $036A
$0329:  39           RTS
$032A:  CC 00 00     LDD #$0000
$032D:  FD C8 81     STD $C881
$0330:  B6 C8 82     LDA $C882
$0333:  B7 C8 8B     STA $C88B
$0336:  CC 00 00     LDD #$0000
$0339:  FD C8 81     STD $C881
$033C:  B6 C8 82     LDA $C882
$033F:  B7 C8 8C     STA $C88C
$0342:  B6 C8 8B     LDA $C88B
$0345:  B7 C8 95     STA $C895
$0348:  B6 C8 8C     LDA $C88C
$034B:  B7 C8 96     STA $C896
$034E:  7F C8 97     CLR $C897
$0351:  7F C8 98     CLR $C898
$0354:  7F C8 99     CLR $C899
$0357:  BD F1 AA     JSR $F1AA
$035A:  8E 00 00     LDX #$0000
$035D:  BD 05 CC     JSR $05CC
$0360:  BD F1 AF     JSR $F1AF
$0363:  CC 00 00     LDD #$0000
$0366:  FD C8 81     STD $C881
$0369:  39           RTS
$036A:  FC C8 9A     LDD $C89A
$036D:  FD C8 81     STD $C881
$0370:  B6 C8 82     LDA $C882
$0373:  B7 C8 8B     STA $C88B
$0376:  FC C8 9C     LDD $C89C
$0379:  FD C8 81     STD $C881
$037C:  B6 C8 82     LDA $C882
$037F:  B7 C8 8C     STA $C88C
$0382:  B6 C8 8B     LDA $C88B
$0385:  B7 C8 95     STA $C895
$0388:  B6 C8 8C     LDA $C88C
$038B:  B7 C8 96     STA $C896
$038E:  7F C8 97     CLR $C897
$0391:  7F C8 98     CLR $C898
$0394:  7F C8 99     CLR $C899
$0397:  BD F1 AA     JSR $F1AA
$039A:  8E 00 00     LDX #$0000
$039D:  BD 05 CC     JSR $05CC
$03A0:  BD F1 AF     JSR $F1AF
$03A3:  CC 00 00     LDD #$0000
$03A6:  FD C8 81     STD $C881
$03A9:  FC C8 9E     LDD $C89E
$03AC:  FD C8 81     STD $C881
$03AF:  B6 C8 82     LDA $C882
$03B2:  B7 C8 8B     STA $C88B
$03B5:  FC C8 A0     LDD $C8A0
$03B8:  FD C8 81     STD $C881
$03BB:  B6 C8 82     LDA $C882
$03BE:  B7 C8 8C     STA $C88C
$03C1:  B6 C8 8B     LDA $C88B
$03C4:  B7 C8 95     STA $C895
$03C7:  B6 C8 8C     LDA $C88C
$03CA:  B7 C8 96     STA $C896
$03CD:  7F C8 97     CLR $C897
$03D0:  7F C8 98     CLR $C898
$03D3:  7F C8 99     CLR $C899
$03D6:  BD F1 AA     JSR $F1AA
$03D9:  8E 00 00     LDX #$0000
$03DC:  BD 05 CC     JSR $05CC
$03DF:  BD F1 AF     JSR $F1AF
$03E2:  CC 00 00     LDD #$0000
$03E5:  FD C8 81     STD $C881
$03E8:  FC C8 A2     LDD $C8A2
$03EB:  FD C8 81     STD $C881
$03EE:  B6 C8 82     LDA $C882
$03F1:  B7 C8 8B     STA $C88B
$03F4:  FC C8 A4     LDD $C8A4
$03F7:  FD C8 81     STD $C881
$03FA:  B6 C8 82     LDA $C882
$03FD:  B7 C8 8C     STA $C88C
$0400:  B6 C8 8B     LDA $C88B
$0403:  B7 C8 95     STA $C895
$0406:  B6 C8 8C     LDA $C88C
$0409:  B7 C8 96     STA $C896
$040C:  7F C8 97     CLR $C897
$040F:  7F C8 98     CLR $C898
$0412:  7F C8 99     CLR $C899
$0415:  BD F1 AA     JSR $F1AA
$0418:  8E 00 00     LDX #$0000
$041B:  BD 05 CC     JSR $05CC
$041E:  BD F1 AF     JSR $F1AF
$0421:  CC 00 00     LDD #$0000
$0424:  FD C8 81     STD $C881
$0427:  39           RTS
$0428:  34 10        PSHS X
$042A:  BD F1 AA     JSR $F1AA
$042D:  BD F1 F5     JSR $F1F5
$0430:  BD F1 AF     JSR $F1AF
$0433:  F6           FCB $F6
$0434:  C8           FCB $C8
$0435:  1B           FCB $1B
$0436:  1D           FCB $1D
$0437:  C3           FCB $C3
$0438:  00           FCB $00
$0439:  02           FCB $02
$043A:  35 10        PULS X
$043C:  39           RTS
$043D:  34 10        PSHS X
$043F:  BD F1 AA     JSR $F1AA
$0442:  BD F1 F5     JSR $F1F5
$0445:  BD F1 AF     JSR $F1AF
$0448:  F6           FCB $F6
$0449:  C8           FCB $C8
$044A:  1C           FCB $1C
$044B:  1D           FCB $1D
$044C:  C3           FCB $C3
$044D:  00           FCB $00
$044E:  02           FCB $02
$044F:  35 10        PULS X
$0451:  39           RTS
$0452:  B6 C8 11     LDA $C811
$0455:  84           FCB $84
$0456:  01           FCB $01
$0457:  10 27 00 04  LBEQ $0004
$045B:  CC 00 01     LDD #$0001
$045E:  39           RTS
$045F:  CC 00 00     LDD #$0000
$0462:  39           RTS
$0463:  B6 C8 11     LDA $C811
$0466:  84           FCB $84
$0467:  02           FCB $02
$0468:  10 27 00 04  LBEQ $0004
$046C:  CC 00 01     LDD #$0001
$046F:  39           RTS
$0470:  CC 00 00     LDD #$0000
$0473:  39           RTS
$0474:  B6 C8 11     LDA $C811
$0477:  84           FCB $84
$0478:  04           FCB $04
$0479:  10 27 00 04  LBEQ $0004
$047D:  CC 00 01     LDD #$0001
$0480:  39           RTS
$0481:  CC 00 00     LDD #$0000
$0484:  39           RTS
$0485:  B6 C8 11     LDA $C811
$0488:  84           FCB $84
$0489:  08           FCB $08
$048A:  10 27 00 04  LBEQ $0004
$048E:  CC 00 01     LDD #$0001
$0491:  39           RTS
$0492:  CC 00 00     LDD #$0000
$0495:  39           RTS
$0496:  B6 C8 A8     LDA $C8A8
$0499:  B7 C0 02     STA $C002
$049C:  B6 C8 A9     LDA $C8A9
$049F:  B7 C0 00     STA $C000
$04A2:  86 42        LDA #$42
$04A4:  B7 C0 01     STA $C001
$04A7:  39           RTS
$04A8:  86 98        LDA #$98
$04AA:  B7 D0 0C     STA $D00C
$04AD:  86 D0        LDA #$D0
$04AF:  1F 8B        TFR A,DP
$04B1:  B6 C8 A9     LDA $C8A9
$04B4:  BD 04 B8     JSR $04B8
$04B7:  39           RTS
$04B8:  1F 98        TFR B,A
$04BA:  7E           FCB $7E
$04BB:  F2           FCB $F2
$04BC:  AB           FCB $AB
$04BD:  7E           FCB $7E
$04BE:  F3           FCB $F3
$04BF:  54           FCB $54
$04C0:  A6           FCB $A6
$04C1:  62           FCB $62
$04C2:  7E           FCB $7E
$04C3:  F3           FCB $F3
$04C4:  12           NOP
$04C5:  A6           FCB $A6
$04C6:  62           FCB $62
$04C7:  7E           FCB $7E
$04C8:  F3           FCB $F3
$04C9:  DF           FCB $DF
$04CA:  A6           FCB $A6
$04CB:  80           FCB $80
$04CC:  BD F2 AB     JSR $F2AB
$04CF:  E6           FCB $E6
$04D0:  80           FCB $80
$04D1:  A6           FCB $A6
$04D2:  80           FCB $80
$04D3:  FD C8 8F     STD $C88F
$04D6:  7F D0 0A     CLR $D00A
$04D9:  86 CC        LDA #$CC
$04DB:  B7 D0 0C     STA $D00C
$04DE:  7F D0 01     CLR $D001
$04E1:  86 82        LDA #$82
$04E3:  B7 D0 00     STA $D000
$04E6:  12           NOP
$04E7:  12           NOP
$04E8:  12           NOP
$04E9:  12           NOP
$04EA:  12           NOP
$04EB:  86 83        LDA #$83
$04ED:  B7 D0 00     STA $D000
$04F0:  FC C8 8F     LDD $C88F
$04F3:  F7           FCB $F7
$04F4:  D0           FCB $D0
$04F5:  01           FCB $01
$04F6:  34 02        PSHS A
$04F8:  86 CE        LDA #$CE
$04FA:  B7 D0 0C     STA $D00C
$04FD:  7F D0 00     CLR $D000
$0500:  86 01        LDA #$01
$0502:  B7 D0 00     STA $D000
$0505:  35 02        PULS A
$0507:  B7 D0 01     STA $D001
$050A:  86 7F        LDA #$7F
$050C:  B7 D0 04     STA $D004
$050F:  7F D0 05     CLR $D005
$0512:  30 02        LEAX 2,X
$0514:  B6 D0 0D     LDA $D00D
$0517:  84           FCB $84
$0518:  40           FCB $40
$0519:  10 27 FF F7  LBEQ $FFF7
$051D:  A6           FCB $A6
$051E:  80           FCB $80
$051F:  81           FCB $81
$0520:  02           FCB $02
$0521:  10 27 00 A6  LBEQ $00A6
$0525:  81           FCB $81
$0526:  01           FCB $01
$0527:  10 27 00 30  LBEQ $0030
$052B:  7F C8 23     CLR $C823
$052E:  E6           FCB $E6
$052F:  80           FCB $80
$0530:  A6           FCB $A6
$0531:  80           FCB $80
$0532:  34 02        PSHS A
$0534:  F7           FCB $F7
$0535:  D0           FCB $D0
$0536:  01           FCB $01
$0537:  7F D0 00     CLR $D000
$053A:  86 01        LDA #$01
$053C:  B7 D0 00     STA $D000
$053F:  35 02        PULS A
$0541:  B7 D0 01     STA $D001
$0544:  7F D0 05     CLR $D005
$0547:  86 FF        LDA #$FF
$0549:  B7 D0 0A     STA $D00A
$054C:  B6 D0 0D     LDA $D00D
$054F:  84           FCB $84
$0550:  40           FCB $40
$0551:  10 27 FF F7  LBEQ $FFF7
$0555:  7F D0 0A     CLR $D00A
$0558:  16 FF C2     LBRA $FFC2
$055B:  1F 10        TFR X,D
$055D:  34 06        PSHS B,A
$055F:  A6           FCB $A6
$0560:  80           FCB $80
$0561:  34 02        PSHS A
$0563:  E6           FCB $E6
$0564:  80           FCB $80
$0565:  A6           FCB $A6
$0566:  80           FCB $80
$0567:  FD C8 8F     STD $C88F
$056A:  35 02        PULS A
$056C:  34 02        PSHS A
$056E:  86 D0        LDA #$D0
$0570:  1F 8B        TFR A,DP
$0572:  35 02        PULS A
$0574:  BD F2 AB     JSR $F2AB
$0577:  35 06        PULS B,A
$0579:  C3           FCB $C3
$057A:  00           FCB $00
$057B:  03           FCB $03
$057C:  1F 01        TFR D,X
$057E:  7F D0 0A     CLR $D00A
$0581:  86 CC        LDA #$CC
$0583:  B7 D0 0C     STA $D00C
$0586:  7F D0 01     CLR $D001
$0589:  86 82        LDA #$82
$058B:  B7 D0 00     STA $D000
$058E:  12           NOP
$058F:  12           NOP
$0590:  12           NOP
$0591:  12           NOP
$0592:  12           NOP
$0593:  86 83        LDA #$83
$0595:  B7 D0 00     STA $D000
$0598:  FC C8 8F     LDD $C88F
$059B:  F7           FCB $F7
$059C:  D0           FCB $D0
$059D:  01           FCB $01
$059E:  34 02        PSHS A
$05A0:  86 CE        LDA #$CE
$05A2:  B7 D0 0C     STA $D00C
$05A5:  7F D0 00     CLR $D000
$05A8:  86 01        LDA #$01
$05AA:  B7 D0 00     STA $D000
$05AD:  35 02        PULS A
$05AF:  B7 D0 01     STA $D001
$05B2:  86 7F        LDA #$7F
$05B4:  B7 D0 04     STA $D004
$05B7:  7F D0 05     CLR $D005
$05BA:  30 02        LEAX 2,X
$05BC:  B6 D0 0D     LDA $D00D
$05BF:  84           FCB $84
$05C0:  40           FCB $40
$05C1:  10 27 FF F7  LBEQ $FFF7
$05C5:  7F D0 0A     CLR $D00A
$05C8:  16 FF 52     LBRA $FF52
$05CB:  39           RTS
$05CC:  B6 C8 99     LDA $C899
$05CF:  10 26 00 05  LBNE $0005
$05D3:  A6           FCB $A6
$05D4:  80           FCB $80
$05D5:  16 00 02     LBRA $0002
$05D8:  30 01        LEAX 1,X
$05DA:  BD F2 AB     JSR $F2AB
$05DD:  E6           FCB $E6
$05DE:  80           FCB $80
$05DF:  7D           FCB $7D
$05E0:  C8           FCB $C8
$05E1:  98           FCB $98
$05E2:  10 27 00 01  LBEQ $0001
$05E6:  50           FCB $50
$05E7:  FB           FCB $FB
$05E8:  C8           FCB $C8
$05E9:  96           FCB $96
$05EA:  A6           FCB $A6
$05EB:  80           FCB $80
$05EC:  7D           FCB $7D
$05ED:  C8           FCB $C8
$05EE:  97           FCB $97
$05EF:  10 27 00 01  LBEQ $0001
$05F3:  40           FCB $40
$05F4:  BB           FCB $BB
$05F5:  C8           FCB $C8
$05F6:  95           FCB $95
$05F7:  FD C8 8F     STD $C88F
$05FA:  7F D0 0A     CLR $D00A
$05FD:  86 CC        LDA #$CC
$05FF:  B7 D0 0C     STA $D00C
$0602:  7F D0 01     CLR $D001
$0605:  86 82        LDA #$82
$0607:  B7 D0 00     STA $D000
$060A:  12           NOP
$060B:  12           NOP
$060C:  12           NOP
$060D:  12           NOP
$060E:  12           NOP
$060F:  86 83        LDA #$83
$0611:  B7 D0 00     STA $D000
$0614:  FC C8 8F     LDD $C88F
$0617:  F7           FCB $F7
$0618:  D0           FCB $D0
$0619:  01           FCB $01
$061A:  34 02        PSHS A
$061C:  86 CE        LDA #$CE
$061E:  B7 D0 0C     STA $D00C
$0621:  7F D0 00     CLR $D000
$0624:  86 01        LDA #$01
$0626:  B7 D0 00     STA $D000
$0629:  35 02        PULS A
$062B:  B7 D0 01     STA $D001
$062E:  86 7F        LDA #$7F
$0630:  B7 D0 04     STA $D004
$0633:  7F D0 05     CLR $D005
$0636:  30 02        LEAX 2,X
$0638:  B6 D0 0D     LDA $D00D
$063B:  84           FCB $84
$063C:  40           FCB $40
$063D:  10 27 FF F7  LBEQ $FFF7
$0641:  A6           FCB $A6
$0642:  80           FCB $80
$0643:  81           FCB $81
$0644:  02           FCB $02
$0645:  10 27 00 CD  LBEQ $00CD
$0649:  81           FCB $81
$064A:  01           FCB $01
$064B:  10 27 00 3D  LBEQ $003D
$064F:  E6           FCB $E6
$0650:  80           FCB $80
$0651:  7D           FCB $7D
$0652:  C8           FCB $C8
$0653:  98           FCB $98
$0654:  10 27 00 01  LBEQ $0001
$0658:  50           FCB $50
$0659:  A6           FCB $A6
$065A:  80           FCB $80
$065B:  7D           FCB $7D
$065C:  C8           FCB $C8
$065D:  97           FCB $97
$065E:  10 27 00 01  LBEQ $0001
$0662:  40           FCB $40
$0663:  34 02        PSHS A
$0665:  F7           FCB $F7
$0666:  D0           FCB $D0
$0667:  01           FCB $01
$0668:  7F D0 00     CLR $D000
$066B:  86 01        LDA #$01
$066D:  B7 D0 00     STA $D000
$0670:  35 02        PULS A
$0672:  B7 D0 01     STA $D001
$0675:  7F D0 05     CLR $D005
$0678:  86 FF        LDA #$FF
$067A:  B7 D0 0A     STA $D00A
$067D:  B6 D0 0D     LDA $D00D
$0680:  84           FCB $84
$0681:  40           FCB $40
$0682:  10 27 FF F7  LBEQ $FFF7
$0686:  7F D0 0A     CLR $D00A
$0689:  16 FF B5     LBRA $FFB5
$068C:  1F 10        TFR X,D
$068E:  34 06        PSHS B,A
$0690:  B6 C8 99     LDA $C899
$0693:  10 26 00 05  LBNE $0005
$0697:  A6           FCB $A6
$0698:  80           FCB $80
$0699:  16 00 02     LBRA $0002
$069C:  30 01        LEAX 1,X
$069E:  34 02        PSHS A
$06A0:  E6           FCB $E6
$06A1:  80           FCB $80
$06A2:  7D           FCB $7D
$06A3:  C8           FCB $C8
$06A4:  98           FCB $98
$06A5:  10 27 00 01  LBEQ $0001
$06A9:  50           FCB $50
$06AA:  FB           FCB $FB
$06AB:  C8           FCB $C8
$06AC:  96           FCB $96
$06AD:  A6           FCB $A6
$06AE:  80           FCB $80
$06AF:  7D           FCB $7D
$06B0:  C8           FCB $C8
$06B1:  97           FCB $97
$06B2:  10 27 00 01  LBEQ $0001
$06B6:  40           FCB $40
$06B7:  BB           FCB $BB
$06B8:  C8           FCB $C8
$06B9:  95           FCB $95
$06BA:  FD C8 8F     STD $C88F
$06BD:  35 02        PULS A
$06BF:  BD F2 AB     JSR $F2AB
$06C2:  35 06        PULS B,A
$06C4:  C3           FCB $C3
$06C5:  00           FCB $00
$06C6:  03           FCB $03
$06C7:  1F 01        TFR D,X
$06C9:  7F D0 0A     CLR $D00A
$06CC:  86 CC        LDA #$CC
$06CE:  B7 D0 0C     STA $D00C
$06D1:  7F D0 01     CLR $D001
$06D4:  86 82        LDA #$82
$06D6:  B7 D0 00     STA $D000
$06D9:  12           NOP
$06DA:  12           NOP
$06DB:  12           NOP
$06DC:  12           NOP
$06DD:  12           NOP
$06DE:  86 83        LDA #$83
$06E0:  B7 D0 00     STA $D000
$06E3:  FC C8 8F     LDD $C88F
$06E6:  F7           FCB $F7
$06E7:  D0           FCB $D0
$06E8:  01           FCB $01
$06E9:  34 02        PSHS A
$06EB:  86 CE        LDA #$CE
$06ED:  B7 D0 0C     STA $D00C
$06F0:  7F D0 00     CLR $D000
$06F3:  86 01        LDA #$01
$06F5:  B7 D0 00     STA $D000
$06F8:  35 02        PULS A
$06FA:  B7 D0 01     STA $D001
$06FD:  86 7F        LDA #$7F
$06FF:  B7 D0 04     STA $D004
$0702:  7F D0 05     CLR $D005
$0705:  30 02        LEAX 2,X
$0707:  B6 D0 0D     LDA $D00D
$070A:  84           FCB $84
$070B:  40           FCB $40
$070C:  10 27 FF F7  LBEQ $FFF7
$0710:  7F D0 0A     CLR $D00A
$0713:  16 FF 2B     LBRA $FF2B
$0716:  39           RTS
$0717:  01           FCB $01
$0718:  00           FCB $00
$0719:  00           FCB $00
$071A:  7F 0F 00     CLR $0F00
$071D:  00           FCB $00
$071E:  00           FCB $00
$071F:  FF           FCB $FF
$0720:  E2           FCB $E2
$0721:  F1           FCB $F1
$0722:  FF           FCB $FF
$0723:  00           FCB $00
$0724:  1E           FCB $1E
$0725:  FF           FCB $FF
$0726:  1E           FCB $1E
$0727:  F1           FCB $F1
$0728:  02           FCB $02
$0729:  01           FCB $01
$072A:  00           FCB $00
$072B:  00           FCB $00
$072C:  64           FCB $64
$072D:  0A           FCB $0A
$072E:  F6           FCB $F6
$072F:  00           FCB $00
$0730:  00           FCB $00
$0731:  FF           FCB $FF
$0732:  00           FCB $00
$0733:  14           FCB $14
$0734:  FF           FCB $FF
$0735:  EC           FCB $EC
$0736:  00           FCB $00
$0737:  FF           FCB $FF
$0738:  00           FCB $00
$0739:  EC           FCB $EC
$073A:  FF           FCB $FF
$073B:  14           FCB $14
$073C:  00           FCB $00
$073D:  02           FCB $02
$073E:  FF           FCB $FF
; [Rest of bank is 0xFF padding]

; ========================================
; BANK #1 (offset $004000-$007FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #2 (offset $008000-$00BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #3 (offset $00C000-$00FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #4 (offset $010000-$013FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #5 (offset $014000-$017FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #6 (offset $018000-$01BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #7 (offset $01C000-$01FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #8 (offset $020000-$023FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #9 (offset $024000-$027FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #10 (offset $028000-$02BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #11 (offset $02C000-$02FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #12 (offset $030000-$033FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #13 (offset $034000-$037FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #14 (offset $038000-$03BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #15 (offset $03C000-$03FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #16 (offset $040000-$043FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #17 (offset $044000-$047FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #18 (offset $048000-$04BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #19 (offset $04C000-$04FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #20 (offset $050000-$053FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #21 (offset $054000-$057FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #22 (offset $058000-$05BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #23 (offset $05C000-$05FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #24 (offset $060000-$063FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #25 (offset $064000-$067FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #26 (offset $068000-$06BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #27 (offset $06C000-$06FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #28 (offset $070000-$073FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #29 (offset $074000-$077FFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #30 (offset $078000-$07BFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; BANK #31 (offset $07C000-$07FFFF)
; ========================================
; [EMPTY BANK - all 0xFF]

; ========================================
; RESET VECTOR (Bank #31 offset $3FFE)
; Points to: $FFFF
; ========================================
