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
;
; -Ofast
;
; n * (n + 1) / 2


.global sum_to_n

sum_to_n:
	sub w8, w0, #1         ; w8 = w0 - 1; don't know why
	cmp w0, #0             ; w0 == 0?
	umull x8, w0, w8       ; x8 = w0 * w8
	lsr x8, x8, #1         ; x8 >>= 1; divide by two
	add w8, w8, w0         ; w8 += w0
	csel w0, wzr, w8, lt   ; w0 = wzr < w8 ? wzr : w8
	ret
