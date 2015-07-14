.org 0x10000000
.equ true 1
.equ false 0
li $sp, 0x10fffffc

call BasicArithmatic_BasicArithmatic_static_init
nop
call BasicArithmatic_main
nop
j end
nop

call_buffer:
	.word 0
caller:
	.word 0
arg_stack:
	.word 0

