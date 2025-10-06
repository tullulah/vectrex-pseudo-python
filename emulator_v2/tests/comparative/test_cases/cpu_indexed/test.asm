; Test CPU Indexed Addressing - Various indexed modes
; Verify indexed addressing with offsets and auto-increment

        ORG $C800

start:
        ; Setup base address
        LDX  #$C850      ; X = 0xC850
        
        ; Test indexed no offset
        LDA  #$AA
        STA  ,X          ; Store to [X]
        
        ; Test indexed with 5-bit offset
        LDA  #$BB
        STA  5,X         ; Store to [X+5]
        
        ; Test indexed with 8-bit offset  
        LDA  #$CC
        STA  $10,X       ; Store to [X+0x10]
        
        ; Test auto-increment
        LDA  ,X+         ; Load from [X], then X++
        LDA  ,X+         ; Load again, X is now 0xC852
        
        ; Load back values
        LDX  #$C850
        LDA  ,X          ; Should be 0xAA
        LDB  5,X         ; Should be 0xBB

loop:   BRA  loop
