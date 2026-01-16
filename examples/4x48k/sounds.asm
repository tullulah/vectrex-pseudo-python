                    INCLUDE  "ymPlayer.i"
                    INCLUDE  "warp_start_2.asm"


playSound
                    lda      BackGndCtr
                    cmpa     #1
                    bne      checkRestart
                    jsr      Clear_Sound
                    bra      aftSound
checkRestart        cmpa     #0
                    beq      restartBck
                    jsr      do_ym_sound
                    dec      BackGndCtr
                    bra      aftSound
restartBck          lda      #70
                    sta      BackGndCtr
                    ldu      #warp_start_2
                    jsr      init_ym_sound
aftSound            rts