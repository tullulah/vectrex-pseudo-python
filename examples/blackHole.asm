; Extracted blackHole.asm (partial) for analysis
; NOTE: Likely depends on additional include files/macros not provided here.

; MEGA simple particles and emitters
; one object only has 6 byte
; thus nearly 140 objects can be created!
;
; the demo runs with abour 135 dots
PARTICLE_BH_MAX_COUNT  =     80                           ; max with below RAM 
;
; RAM
                    bss      
                    org      startParticleRAM 
;
animCounter         ds       1 
animStep            ds       1 
random_seed         ds       1 
plist_empty_head    ds       2                            ; if empty these contain a value that points to a RTS, smaller than OBJECT_LIST_COMPARE_ADDRESS 
plist_objects_head  ds       2                            ; if greater OBJECT_LIST_COMPARE_ADDRESS, than this is a pointer to a RAM location of an Object 
pCount              ds       1 
pobject_list        ds       BHParticle*PARTICLE_BH_MAX_COUNT 
pobject_list_end    ds       0 
;
; ROM
                    code     
initBlackHole 
                    clr      animCounter 
                    clr      animStep 
                    INIT_OBJECTLIST  PARTICLE_BH_MAX_COUNT, BHParticle, main 
;
                    jsr      buildBlackHoleEmitter 
                    lda      #60;0                           ; start angle 
                    sta      START_ANGLE,u 
;
                    jsr      buildBlackHoleEmitter 
                    lda      #160;137                         ; start angle 
                    sta      START_ANGLE,u 
;
                    rts      

;***************************************************************************
ANIMDELAY           =        3 
MAX_ANIM            =        24                           ; double of anim steps, since its a word pointer 
blackHole 
                    dec      animCounter 
                    bpl      nostepChange 
                    lda      #ANIMDELAY 
                    sta      animCounter 
                    dec      animStep 
                    dec      animStep 
                    bpl      nostepChange 
                    lda      #MAX_ANIM 
                    sta      animStep 
nostepChange 
; pointer to circle data - is a constant!
                    ldy      #circleData 
                    lds      plist_objects_head 
                    puls     d,pc                         ; (D = y,x) ; do all objects 
;***************************************************************************
;...........................................................................
buildBlackHoleEmitter 
                    jsr      newObject 
                    cmpu     #PLIST_COMPARE_ADDRESS 
                    bls      noNewBHEmitter 
                    ldd      #blackHoleEmitterBehaviour 
                    std      BEHAVIOUR, u 
noNewBHEmitter 
                    rts      

;...........................................................................
blackHoleEmitterBehaviour 
                    RANDOM_A  
                    cmpa     #40 
                    bhi      noNewBHParticle 
                    NEW_OBJECT  
                    cmpy     #PLIST_COMPARE_ADDRESS 
                    bls      noNewBHParticle 
                    RANDOM_B  
                    andb     #%10001111 
                    tfr      b,a 
                    addb     START_ANGLE+u_offset1,s      ; start angle 
                    anda     #%0111 
                    sta      LISTSCALE, y                 ; pseudo random object size!
                    bne      noZero 
                    lda      #5 
                    sta      LISTSCALE, y                 ; start object scale 
noZero 
                    adda     #$80                         ; start scale 
                    std      P_SCALE,y 
                    ldd      #bhParticleBehaviour 
                    std      BEHAVIOUR, y 
                    lda      #50 
                    sta      REMAIN , y 
                    ldx      #debrisList ; random of 4 object list
                    RANDOM_A  
                    anda     #%11 
                    asla     
                    ldd      a,x 
                    std      VLIST, y 
noNewBHParticle 
                    ldy      #circleData 
                    lds      NEXT_OBJECT+u_offset1,s      ; preload next user stack 
                    puls     d,pc 
;...........................................................................
bhParticleBehaviour 
; position to scale in A
                    sta      <VIA_t1_cnt_lo 
                    clra     
; angle in B
                    MY_LSL_D 
; leay d,y 
; lda 90,y
; ldb 51+90,y
     
               ldd      d,y 
 asra
 asra
 suba #150
; adda #32 ; 0 - 64

; adda P_SCALE+u_offset1,s
 adda REMAIN+u_offset1,s 
 adda REMAIN+u_offset1,s 
 adda REMAIN+u_offset1,s 
 adda REMAIN+u_offset1,s 
 sta tmp
                    MY_MOVE_TO_D_START  
; ldy      #circleData 
                    inc      P_ANGLE+u_offset1,s 
; scale reduce in relation to scale (further out -> slower)
; if scale > 100, than only each 4th round...
                    lda      P_SCALE+u_offset1,s 
                    cmpa     #110 
                    blo      smaller100 
                    ldb      Vec_Loop_Count+1 
                    andb     #%11 ; at first reduce scaling only each 4th round
                    bne      noScaling 
                    dec      P_SCALE+u_offset1,s 
                    bra      noScaling 

smaller100 
                    ldb      Vec_Loop_Count+1 
                    andb     #1 
                    bne      noSpeedUp 
                    inc      P_ANGLE+u_offset1,s 
noSpeedUp 
                    cmpa     #80 
                    bhi      nospeeding 
                    ldb      Vec_Loop_Count+1 
                    andb     #1 
                    beq      nospeeding
                    inc      P_ANGLE+u_offset1,s 
nospeeding 
                    cmpa     #60 
                    blo      smaller80 
                    ldb      Vec_Loop_Count+1 
                    andb     #$01 
                    bne      noScaling 
                    dec      P_SCALE+u_offset1,s 
                    bra      noScaling 

smaller80 
                    ldb      Vec_Loop_Count+1 
                    andb     #1 
                    bne      noSpeedUp3 
                    inc      P_ANGLE+u_offset1,s 
noSpeedUp3 
                    cmpa     #30 
                    blo      smaller60 
                    dec      P_SCALE+u_offset1,s 
                    bra      noScaling 

smaller60 
                    inc      P_ANGLE+u_offset1,s 
                    dec      P_SCALE+u_offset1,s 
                    dec      P_SCALE+u_offset1,s 
INNERMOST_CIRCLE = 20

                    cmpa     #INNERMOST_CIRCLE 
                    bhi      noScaling 
                    lda      #INNERMOST_CIRCLE
                    sta      P_SCALE+u_offset1,s 
                    dec      REMAIN+u_offset1,s 
                    lbeq      destroyPObject 
                    inc      P_ANGLE+u_offset1,s 
                    inc      P_ANGLE+u_offset1,s 
; brightness in relation to scale (further out less bright)
; disappear at scale XXX
; object depending 
noScaling 
; object scaling
                    lda      LISTSCALE+u_offset1,s        ; start angle 
                    sta      <VIA_t1_cnt_lo 
                    ldu      VLIST+u_offset1,s            ; preload next user stack 
                    ldb      animStep 
; load smartlist address to U
                    ldu      b,u 
 
                    lda      P_SCALE+u_offset1,s 
                    nega     
                    adda     #147 
 bpl noInOverflow
 lda #127
noInOverflow
 ldb REMAIN+u_offset1,s 
 cmpb #50
 beq intok
 tfr b,a
 asla
 bra intremain
intok
 ldb tmp
 subb #30
 asrb
 stb tmp

 suba tmp
; sta tmp


intremain
                    lds      NEXT_OBJECT+u_offset1,s      ; preload next user stack 
                    MY_MOVE_TO_B_END  
                    _INTENSITY_A  
                    pulu     d,pc 
debrisList 
                    dw       threeDebris 
                    dw       quadDebris 
                    dw       twoDebris 
                    dw       four2Debris 
quadDebris 
                    DW       quadDebris_12 
                    DW       quadDebris_11 
                    DW       quadDebris_10 
                    DW       quadDebris_9 
                    DW       quadDebris_8 
                    DW       quadDebris_7 
                    DW       quadDebris_6 
                    DW       quadDebris_5 
                    DW       quadDebris_4 
                    DW       quadDebris_3 
                    DW       quadDebris_2 
                    DW       quadDebris_1 
                    DW       quadDebris_0                 ; list of all single vectorlists in this 
                    DW       0 
quadDebris_0 
                    db       $2A, -$23, hi(SM_continue_d), lo(SM_continue_d) 
                    db       $0E, $46, hi(SM_startDraw_d), lo(SM_startDraw_d) 
                    db       -$70, $1C, hi(SM_continue_d3), lo(SM_continue_d3) 
                    db       $0E, -$7E 
                    db       $54, $1C 
                    db       $00, $00, hi(SM_rts), lo(SM_rts) 
... (truncated for brevity from original) ...
; End of extracted portion
