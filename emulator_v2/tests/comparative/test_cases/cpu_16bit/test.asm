; Test CPU 16-bit Operations - LDD/STD/ADDD/SUBD/CMPD
; Verify 16-bit D register operations

        ORG $C800

start:
        LDS  #$CFFF      ; Initialize stack
        
        ; Test LDD (Load D - 16 bit)
        LDD  #$1234      ; D = 0x1234 (A=0x12, B=0x34)
        
        ; Test ADDD (Add to D)
        ADDD #$0100      ; D = 0x1234 + 0x0100 = 0x1334
        
        ; Test SUBD (Subtract from D)
        SUBD #$0034      ; D = 0x1334 - 0x0034 = 0x1300
        
        ; Test STD (Store D to memory)
        STD  $C850       ; Store D to RAM
        
        ; Test CMPD (Compare D)
        CMPD #$1300      ; Should set Z=1 (equal)
        
        ; Load different value
        LDD  #$ABCD      ; D = 0xABCD
        
        ; Load back from memory
        LDD  $C850       ; D should be 0x1300 again

loop:   BRA  loop
