;;; gcc for m6809 : Mar 17 2019 15:46:08
;;; 4.3.6 (gcc6809)
;;; ABI version 1
;;; -mabi=bx -mint16 -fomit-frame-pointer -O3
	.module	malban.cpp
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
L15:
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
L19:
	ldx	7,s
	tfr	x,d
	addd	#-1; addhi3,3
	std	7,s
	ldx	7,s
	bgt	L19
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
	ldb	-2,u
	stb	1,s
	lbne	L17
	tst	-1,u
	lbne	L17
L18:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L18
L27:
	ldb	,u
	blt	L30
L8:
	tstb
	lbne	L11
	ldb	1,u
	stb	3,s
	lbeq	L12
	ldb	2,u
	stb	,s
L13:
	ldb	3,s
	stb	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	ldb	,s
	stb	-12288
	clr	-12283
L14:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L14
	leau	3,u
L31:
	ldb	,u
	bge	L8
L30:
	ldb	1,u
	stb	-12288
	clr	-12286
	ldb	#1
	stb	-12286
	ldb	2,u
	stb	-12288
	clr	-12283
	ldb	#-1
	stb	-12198
L9:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L9
	clr	-12198
	leau	3,u
	bra	L31
L17:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L17
	ldb	1,s
	stb	-12288
	ldb	#-50
	stb	-12277
	clr	-12286
	ldb	#1
	stb	-12286
	ldb	-1,u
	stb	-12288
	clr	-12283
L6:
	ldb	-12275
	clra		;zero_extendqihi: R:b -> R:d
	clra	;andqi(ZERO)
	andb	#64
	cmpd	#0
	beq	L6
	lbra	L27
L12:
	ldb	2,u
	stb	,s
	lbne	L13
	leau	3,u
	lbra	L31
L11:
	cmpb	#2	;cmpqi:
	lbne	L15
	leas	9,s
	puls	u,pc
