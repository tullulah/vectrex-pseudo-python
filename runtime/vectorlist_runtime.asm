; Reusable compact vector list runtime for Vectrex
; Safe include: guarded & no END/ORG side effects.
; Format per list:
;   FCB count
;   then exactly 'count' command triples (y,x,cmd). For CMD_INT an extra byte follows (intensity value).
; Commands:
;   0 = START (absolute move)
;   1 = LINE  (delta draw)
;   2 = END   (terminate early, stop even if triples remain)
;   3 = INT   (set intensity: consumes NEXT BYTE)
;
; Caller must have already included VECTREX.I

        IFNDEF RVL_RUNTIME_INCLUDED
RVL_RUNTIME_INCLUDED EQU 1

CMD_START EQU 0
CMD_LINE  EQU 1
CMD_END   EQU 2
CMD_INT   EQU 3
CMD_ZERO  EQU 4 ; call Reset0Ref

; Public entry: X points to list start
; Clobbers: A,B,X,U (preserves none)
Run_VectorList:
        PSHS U
        TFR X,U
        LDA ,U+          ; count
        BEQ RVL_Done
        STA RVL_REMAIN
RVL_Next:
        LDA RVL_REMAIN
        BEQ RVL_Done
        DEC RVL_REMAIN
        LDA ,U+          ; y / ignored
        STA RVL_TMP_Y
        LDB ,U+          ; x / ignored
        STB RVL_TMP_X
        LDA ,U+          ; cmd
        STA RVL_CMD
        CMPA #CMD_START
        BEQ RVL_Start
        CMPA #CMD_LINE
        BEQ RVL_Line
        CMPA #CMD_END
        BEQ RVL_Done
        CMPA #CMD_INT
        BEQ RVL_Int
        CMPA #CMD_ZERO
        BEQ RVL_Zero
        BRA RVL_Next     ; skip unknown
RVL_Start:
        LDA RVL_TMP_Y
        LDB RVL_TMP_X
        JSR Moveto_d
        BRA RVL_Next
RVL_Line:
        LDA RVL_TMP_Y
        LDB RVL_TMP_X
        CLR Vec_Misc_Count
        JSR Draw_Line_d
        BRA RVL_Next
RVL_Int:
        LDA ,U+          ; intensity value
        JSR Intensity_a
        BRA RVL_Next
RVL_Zero:
        JSR Reset0Ref
        BRA RVL_Next
RVL_Done:
        PULS U,PC

; RAM variable addresses (no ORG side effects)
RVL_TMP_Y   EQU $C840
RVL_TMP_X   EQU $C841
RVL_CMD     EQU $C842
RVL_REMAIN  EQU $C843

        ENDIF ; RVL_RUNTIME_INCLUDED
