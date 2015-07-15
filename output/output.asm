BasicArithmatic_static:
	.space 1

BasicArithmatic_static_init:
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_static
	lw $t3, 0($t1)
	li $t2, 0
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t3, 10
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	return
	nop

BasicArithmatic_local_init:
	return
	nop

BasicArithmatic_casualMethod_static:
	.space 0
BasicArithmatic_casualMethod:
	li $t0, BasicArithmatic_casualMethod_static
	li $t0, arg_stack
	lw $t1, 0($t0)
	push $t1
	sw $a0, 0($t0)
	li $t0, caller
	lw $t1, 0($t0)
	push $t1
	li $t1, call_buffer
	lw $t1, 0($t1)
	sw $t1, 0($t0)
	li $v0, 1
	push $v0
	pop $v0
	j BasicArithmatic_casualMethod_return
	nop

	BasicArithmatic_casualMethod_return:
	li $t0, caller
	pop $t1
	sw $t1, 0($t0)
	li $t0, arg_stack
	pop $t1
	sw $t1, 0($t0)
	li $t0, BasicArithmatic_casualMethod_static
	return
	nop

BasicArithmatic_main_static:
	.space 13
BasicArithmatic_main:
	li $t0, BasicArithmatic_main_static
	lw $t1, 0($t0)
	push $t1
	lw $t1, 4($t0)
	push $t1
	lw $t1, 8($t0)
	push $t1
	lw $t1, 12($t0)
	push $t1
	lw $t1, 16($t0)
	push $t1
	lw $t1, 20($t0)
	push $t1
	lw $t1, 24($t0)
	push $t1
	lw $t1, 28($t0)
	push $t1
	lw $t1, 32($t0)
	push $t1
	lw $t1, 36($t0)
	push $t1
	lw $t1, 40($t0)
	push $t1
	lw $t1, 44($t0)
	push $t1
	lw $t1, 48($t0)
	push $t1
	li $t0, arg_stack
	lw $t1, 0($t0)
	push $t1
	sw $a0, 0($t0)
	li $t0, caller
	lw $t1, 0($t0)
	push $t1
	li $t1, call_buffer
	lw $t1, 0($t1)
	sw $t1, 0($t0)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t2, 0
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t3, 4
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t2, 4
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t3, 2
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	call BasicArithmatic_casualMethod
	nop
	move $t3, $v0
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t3, true
	push $t3
	pop $t3
	push $t3
	pop $t3
	beq $t3, $0, BasicArithmatic_main_conditional0_else0
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t2, 4
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t3, 1
	push $t3
	pop $t3
	pop $t4
	lw $t0, 0($t4)
	addu $t3, $t3, $t0
	sw $t3, 0($t4)
	j BasicArithmatic_main_conditional0_end
	nop
	BasicArithmatic_main_conditional0_else0:
	BasicArithmatic_main_conditional0_end:
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 8($t1)
	li $t2, 8
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t3, false
	push $t3
	pop $t3
	push $t3
	pop $t3
	beq $t3, $0, BasicArithmatic_main_conditional1_else0
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 8($t1)
	li $t2, 8
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t3, 89
	push $t3
	pop $t3
	pop $t4
	lw $t0, 0($t4)
	addu $t3, $t3, $t0
	sw $t3, 0($t4)
	j BasicArithmatic_main_conditional1_end
	nop
	BasicArithmatic_main_conditional1_else0:
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 8($t1)
	li $t2, 8
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t3, 1
	push $t3
	pop $t3
	pop $t4
	lw $t0, 0($t4)
	addu $t3, $t3, $t0
	sw $t3, 0($t4)
	BasicArithmatic_main_conditional1_end:
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 12($t1)
	li $t2, 12
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 16($t1)
	li $t2, 16
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	pop $t3
	pop $t0
	mullo $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 20($t1)
	li $t2, 20
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	pop $t3
	pop $t0
	subu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 24($t1)
	li $t2, 24
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t3, 2
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 28($t1)
	li $t2, 28
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t3, 2
	push $t3
	pop $t3
	pop $t0
	subu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 32($t1)
	li $t2, 32
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t3, 0x05
	push $t3
	pop $t3
	pop $t0
	mullo $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 36($t1)
	li $t2, 36
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 40($t1)
	li $t2, 40
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	push $t3
	li $t3, 3
	push $t3
	li $t3, 10
	push $t3
	pop $t3
	pop $t0
	subu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t0
	mullo $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 44($t1)
	li $t2, 44
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t3, 3
	push $t3
	li $t3, 10
	push $t3
	pop $t3
	pop $t0
	subu $t3, $t0, $t3
	push $t3
	pop $t3
	push $t3
	pop $t3
	pop $t0
	mullo $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 48($t1)
	li $t2, 48
	addu $t4, $t1, $t2
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t4
	li $t3, 1
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	li $t1, call_buffer
	sw $t3, 0($t1)
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	push $t3
	li $t3, 5
	push $t3
	pop $t3
	pop $t0
	subu $t3, $t0, $t3
	push $t3
	pop $t3
	push $t3
	li $t3, 2
	push $t3
	pop $t3
	pop $t0
	mullo $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	push $t3
	li $t3, 7
	push $t3
	pop $t3
	pop $t0
	mullo $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)

	BasicArithmatic_main_return:
	li $t0, caller
	pop $t1
	sw $t1, 0($t0)
	li $t0, arg_stack
	pop $t1
	sw $t1, 0($t0)
	li $t0, BasicArithmatic_main_static
	pop $t1
	sw $t1, 48($t0)
	pop $t1
	sw $t1, 44($t0)
	pop $t1
	sw $t1, 40($t0)
	pop $t1
	sw $t1, 36($t0)
	pop $t1
	sw $t1, 32($t0)
	pop $t1
	sw $t1, 28($t0)
	pop $t1
	sw $t1, 24($t0)
	pop $t1
	sw $t1, 20($t0)
	pop $t1
	sw $t1, 16($t0)
	pop $t1
	sw $t1, 12($t0)
	pop $t1
	sw $t1, 8($t0)
	pop $t1
	sw $t1, 4($t0)
	pop $t1
	sw $t1, 0($t0)
	return
	nop
end:
