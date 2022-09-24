; https://c.godbolt.org/
;
; int sum_to_n(int n) {
;     int total = 0;
;     
;     for (int i = 0; i <= n; i++)
;         total += i;
; 
;     return total;
; }


.global sum_to_n

sum_to_n:
	; stack frame?
	sub sp, sp, #16             ; subtract 16 from stack pointer?
  str w0, [sp, #12]           ; store the value of word0 into stack pointer w/ 12 offset

; total = 0; i = 0
	; wzr is the zeroed value register
	; here we're storing 0 into sp offset 4 and 8 (32 bits each; aka 4x 8-bits)
	str wzr, [sp, #8]           ; total = 0
	str wzr, [sp, #4]           ; i = 0
	b .for_header               ; jump to .for_header label

; i <= n
.for_header:
	ldr w8, [sp, #4]            ; load stack pointer offset 4 into the lower 32 bits of x8 register
	ldr w9, [sp, #12]           ; load stack pointer offset 12 into the lower 32 bits of x9 register
	subs w8, w8, w9             ; w8 = w8 - w9
	cset w8, gt                 ; condition greater then?
	tbnz w8, #0, .return_fn     ; test bit, branch not zero; if i <= 0; jmp to end of fn
	b .for_body

; total += i
.for_body:
	ldr w9, [sp, #4]            ; load stack pointer offset 4 into the lower 32 bits of x9 register
	ldr w8, [sp, #8]            ; load stack pointer offset 8 into the lower 32 bits of x8 register
	add w8, w8, w9              ; w8 += w9; w8 = total + i
	str w8, [sp, #8]            ; store w8 into stack pointer (total); total += i
	b .for_inc                  ; jump to .for_inc label

; i++
.for_inc:
	ldr w8, [sp, #4]            ; load sp into w8, offset 4
	add w8, w8, #1              ; w8 + 1; i + 1
	str w8, [sp, #4]            ; store it back; i += 1
	b .for_header               ; jump to begining of loop

.return_fn:
  ldr w0, [sp, #4]            ; load sp into w8, offset 4
	add sp, sp, #16             ; remove fn from stack frame?
	ret
