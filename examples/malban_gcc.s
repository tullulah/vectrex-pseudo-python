;;; gcc for m6809 : Mar 17 2019 15:46:08
;;; 4.3.6 (gcc6809)
;;; ABI version 1
;;; -mabi=bx -mint16 -fno-omit-frame-pointer -O0
	.module	malban.cpp
	.area	.text
	.globl	_draw_synced_list_c
_draw_synced_list_c:
	pshs	y,u
	leas	-6,s
	leau	,s
	stx	2,u
L14:
	ldx	#-12198
	clr	,x
	ldx	#-12277
	ldb	#-52
	stb	,x
	ldx	#-12288
	clr	,x
	ldx	#-12286
	ldb	#-126
	stb	,x
	ldy	#-12284
	ldx	16,u
	tfr	x,d	;movlsbqihi: R:x -> R:b
	stb	,y
	ldx	#5
	stx	4,u
	bra	L2
L3:
	ldx	4,u
	leax	-1,x
	stx	4,u
L2:
	ldx	4,u
	cmpx	#0
	bgt	L3
	ldx	#-12286
	ldb	#-125
	stb	,x
	ldy	#-12288
	ldx	12,u
	tfr	x,d	;movlsbqihi: R:x -> R:b
	stb	,y
	ldx	#-12277
	ldb	#-50
	stb	,x
	ldx	#-12286
	clr	,x
	ldx	#-12286
	ldb	#1
	stb	,x
	ldy	#-12288
	ldx	14,u
	tfr	x,d	;movlsbqihi: R:x -> R:b
	stb	,y
	ldx	#-12283
	clr	,x
	ldy	#-12284
	ldx	18,u
	tfr	x,d	;movlsbqihi: R:x -> R:b
	stb	,y
	ldd	2,u
	addd	#3; addhi3,3
	std	2,u
	ldy	2,u
	leax	-2,y
	ldb	,x
	tstb
	bne	L4
	ldy	2,u
	leax	-1,y
	ldb	,x
	tstb
	lbeq	L5
L4:
	ldx	#-12275
	ldb	,x
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L4
	ldy	#-12288
	ldd	2,u
	addd	#-2; addhi3,3
	std	,u
	ldx	,u
	ldb	,x
	stb	,y
	ldx	#-12277
	ldb	#-50
	stb	,x
	ldx	#-12286
	clr	,x
	ldx	#-12286
	ldb	#1
	stb	,x
	ldy	#-12288
	ldd	2,u
	addd	#-1; addhi3,3
	std	,u
	ldx	,u
	ldb	,x
	stb	,y
	ldx	#-12283
	clr	,x
L6:
	ldx	#-12275
	ldb	,x
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L6
	bra	L7
L5:
	ldx	#-12275
	ldb	,x
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L5
L7:
	ldb	[2,u]
	tstb
	bge	L8
	ldy	#-12288
	ldd	2,u
	addd	#1; addhi3,3
	std	,u
	ldx	,u
	ldb	,x
	stb	,y
	ldx	#-12286
	clr	,x
	ldx	#-12286
	ldb	#1
	stb	,x
	ldy	#-12288
	ldd	2,u
	addd	#2; addhi3,3
	std	,u
	ldx	,u
	ldb	,x
	stb	,y
	ldx	#-12283
	clr	,x
	ldx	#-12198
	ldb	#-1
	stb	,x
L9:
	ldx	#-12275
	ldb	,x
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L9
	ldx	#-12198
	clr	,x
	lbra	L10
L8:
	ldb	[2,u]
	tstb
	lbne	L11
	ldy	2,u
	leax	1,y
	ldb	,x
	tstb
	bne	L12
	ldy	2,u
	leax	2,y
	ldb	,x
	tstb
	beq	L10
L12:
	ldy	#-12288
	ldd	2,u
	addd	#1; addhi3,3
	std	,u
	ldx	,u
	ldb	,x
	stb	,y
	ldx	#-12277
	ldb	#-50
	stb	,x
	ldx	#-12286
	clr	,x
	ldx	#-12286
	ldb	#1
	stb	,x
	ldy	#-12288
	ldd	2,u
	addd	#2; addhi3,3
	std	,u
	ldx	,u
	ldb	,x
	stb	,y
	ldx	#-12283
	clr	,x
L13:
	ldx	#-12275
	ldb	,x
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L13
L10:
	ldd	2,u
	addd	#3; addhi3,3
	std	2,u
	lbra	L7
L11:
	ldb	[2,u]
	cmpb	#2	;cmpqi:
	lbne	L14
	leas	6,s
	puls	y,u,pc
