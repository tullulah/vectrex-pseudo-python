; Test ROM simple para verificar carga
; Genera patrones en pantalla para confirmare que se carga

        org $0000       ; ROM inicia en $0000

start:
        ; Inicializar
        lds #$cbff      ; Stack pointer
        ldx #$d000      ; VIA base
        
        ; Configurar VIA
        lda #$ff
        sta $d002,x     ; DDRA - todos outputs
        sta $d003,x     ; DDRB - todos outputs
        
        ; Loop simple que dibuja líneas
main_loop:
        ; Mover a posición inicial
        lda #$80
        sta $d000,x     ; ORA
        
        ; Delay simple
        ldy #$1000
delay:
        leay -1,y
        bne delay
        
        ; Cambiar posición
        lda #$40
        sta $d000,x     ; ORA
        
        ; Delay
        ldy #$1000
delay2:
        leay -1,y
        bne delay2
        
        bra main_loop   ; Loop infinito

        ; Vectores al final de la ROM (asumiendo ROM 8K en $0000-$1FFF)
        org $1FF8
        fdb start       ; IRQ
        fdb start       ; FIRQ  
        fdb start       ; NMI
        fdb start       ; Reset vector