;;; gcc for m6809 : Mar 17 2019 15:46:08
;;; 4.3.6 (gcc6809)
;;; ABI version 1
;;; -mabi=bx -mint16 -fomit-frame-pointer -O3
	.module	malban_full.c
	.area	.text
	.globl	_draw_synced_list_c
_draw_synced_list_c:
	pshs	u
	leas	-9,s
	leau	,x
	ldb	18,s
	stb	2,s
	ldb	14,s
	stb	4,s
	ldb	16,s
	stb	5,s
	ldb	20,s
	stb	6,s
L14:
	clr	-12198
	ldb	#-52
	stb	-12277
	clr	-12288
	ldb	#-126
	stb	-12286
	ldb	2,s
	stb	-12284
	ldd	#5
	std	7,s
	ldx	7,s
	ble	L2
L18:
	ldx	7,s
	tfr	x,d
	addd	#-1; addhi3,3
	std	7,s
	ldx	7,s
	bgt	L18
L2:
	ldb	#-125
	stb	-12286
	ldb	4,s
	stb	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	ldb	5,s
	stb	-12288
	clr	-12283
	ldb	6,s
	stb	-12284
	leau	3,u
L4:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L4
	ldb	-2,u
	stb	1,s
	lbne	L5
	tst	-1,u
	bne	L16
L17:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L17
	clr	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	clr	-12288
	clr	-12283
L8:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L8
L16:
	tst	1,s
	beq	L10
	ldb	1,u
	stb	,s
	ldb	2,u
	stb	3,s
L11:
	ldb	,s
	stb	-12288
	clr	-12286
	ldb	#1
	stb	-12286
	ldb	3,s
	stb	-12288
	clr	-12283
L13:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L13
	leau	3,u
	ldb	,u
	stb	1,s
	bge	L16
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L14
L29:
	leas	9,s
	puls	u,pc
L10:
	ldb	1,u
	stb	,s
	beq	L12
	ldb	2,u
	stb	3,s
	bra	L11
L12:
	ldb	2,u
	stb	3,s
	bne	L11
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L14
	bra	L29
L5:
	tst	1,s
	lbge	L16
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L14
	bra	L29
	.globl	__start
__start:
	pshs	u
	leas	-5,s
;----- asm -----
; 114 "malban_full.c" 1
	LDA #$7F
; 115 "malban_full.c" 1
	JSR $F2AB
;--- end asm ---
L44:
;----- asm -----
; 105 "malban_full.c" 1
	JSR $F192
;--- end asm ---
	ldu	#_square_data
L43:
	clr	-12198
	ldb	#-52
	stb	-12277
	clr	-12288
	ldb	#-126
	stb	-12286
	ldb	#127
	stb	-12284
	ldd	#5
	std	3,s
	ldx	3,s
	ble	L31
L48:
	ldx	3,s
	tfr	x,d
	addd	#-1; addhi3,3
	std	3,s
	ldx	3,s
	bgt	L48
L31:
	ldb	#-125
	stb	-12286
	clr	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	clr	-12288
	clr	-12283
	ldb	#127
	stb	-12284
	leau	3,u
L33:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L33
	ldb	-2,u
	stb	1,s
	lbne	L34
	tst	-1,u
	bne	L46
L47:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L47
	clr	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	clr	-12288
	clr	-12283
L38:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L38
L46:
	tst	1,s
	beq	L39
	ldb	1,u
	stb	,s
	ldb	2,u
	stb	2,s
L40:
	ldb	,s
	stb	-12288
	clr	-12286
	ldb	#1
	stb	-12286
	ldb	2,s
	stb	-12288
	clr	-12283
L42:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L42
	leau	3,u
	ldb	,u
	stb	1,s
	bge	L46
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L43
	lbra	L44
L39:
	ldb	1,u
	stb	,s
	beq	L41
	ldb	2,u
	stb	2,s
	bra	L40
L41:
	ldb	2,u
	stb	2,s
	bne	L40
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L43
	lbra	L44
L34:
	tst	1,s
	lbge	L46
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L43
	lbra	L44
	.globl	_main_loop
_main_loop:
	pshs	u
	leas	-5,s
;----- asm -----
; 105 "malban_full.c" 1
	JSR $F192
;--- end asm ---
	ldu	#_square_data
L70:
	clr	-12198
	ldb	#-52
	stb	-12277
	clr	-12288
	ldb	#-126
	stb	-12286
	ldb	#127
	stb	-12284
	ldd	#5
	std	3,s
	ldx	3,s
	ble	L58
L74:
	ldx	3,s
	tfr	x,d
	addd	#-1; addhi3,3
	std	3,s
	ldx	3,s
	bgt	L74
L58:
	ldb	#-125
	stb	-12286
	clr	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	clr	-12288
	clr	-12283
	ldb	#127
	stb	-12284
	leau	3,u
L60:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L60
	ldb	-2,u
	stb	1,s
	lbne	L61
	tst	-1,u
	bne	L72
L73:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L73
	clr	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	clr	-12288
	clr	-12283
L65:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L65
L72:
	tst	1,s
	beq	L66
	ldb	1,u
	stb	,s
	ldb	2,u
	stb	2,s
L67:
	ldb	,s
	stb	-12288
	clr	-12286
	ldb	#1
	stb	-12286
	ldb	2,s
	stb	-12288
	clr	-12283
L69:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L69
	leau	3,u
	ldb	,u
	stb	1,s
	bge	L72
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L70
L83:
	leas	5,s
	puls	u,pc
L66:
	ldb	1,u
	stb	,s
	beq	L68
	ldb	2,u
	stb	2,s
	bra	L67
L68:
	ldb	2,u
	stb	2,s
	bne	L67
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L70
	bra	L83
L61:
	tst	1,s
	lbge	L72
	ldb	,u
	cmpb	#2	;cmpqi:
	lbne	L70
	bra	L83
	.globl	_square_data
	.area	.data
_square_data:
	.byte	0
	.byte	0
	.byte	0
	.byte	-128
	.byte	80
	.byte	0
	.byte	-128
	.byte	0
	.byte	80
	.byte	-128
	.byte	-80
	.byte	0
	.byte	-128
	.byte	0
	.byte	-80
	.byte	2
	.byte	0
	.byte	0
