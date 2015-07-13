.org 0x10000000
.equ true 1
.equ false 0
li $sp, 0x10fffffc

# Initialize the static memory of all classes
call BasicArithmatic_static_init
nop
# Run main, then stop the program
call BasicArithmatic_main
nop
j end
nop

# --Allocate static memory for program control--
# The call buffer is used to keep track of accessors (e.g. point.x)
call_buffer:
	.word 0

# Caller is used to keep track of the caller of a method (e.g. in 'point.clone()' the caller of clone() is 'point')
caller:
	.word 0

# Pointer to the argument stack for a method call
arg_stack:
	.word 0

BasicArithmatic_static:
	.space 1

BasicArithmatic_static_init:
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {constant}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_static
	lw $t3, 0($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 0
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	li $t3, 10
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)

# Method declaration: casualMethod() in namespace BasicArithmatic
BasicArithmatic_casualMethod_static:
	.space 0
BasicArithmatic_casualMethod:
	# Push local memory of this method to the stack, and restore it at the end of the method call
	li $t0, BasicArithmatic_casualMethod_static

	# Save the location of the previous argument stack
	li $t0, arg_stack
	lw $t1, 0($t0)
	push $t1

	# The pointer to the agument stack for this method call is stored in $a0...
	# ...Load it as the current argument stack
	sw $a0, 0($t0)

	# Save the previous caller reference to the stack
	li $t0, caller
	lw $t1, 0($t0)
	push $t1

	# The caller of this method call is stored in the call_buffer...
	# ...Load it as the current caller
	li $t1, call_buffer
	lw $t1, 0($t1)
	sw $t1, 0($t0)

	# Start of method body
	li $v0, 1
	push $v0
	pop $v0
	j BasicArithmatic_casualMethod_return
	nop
	# End of method body

	# Start of method return
	BasicArithmatic_casualMethod_return:
	# Restore the previous caller
	li $t0, caller
	pop $t1
	sw $t1, 0($t0)

	# Restore the pointer to the previous argument stack
	li $t0, arg_stack
	pop $t1
	sw $t1, 0($t0)

	# Restore the static memory of the previous call to this method
	li $t0, BasicArithmatic_casualMethod_static

	return
	nop
	# End of method return
# End of method declaration: casualMethod() in namespace BasicArithmatic

# Method declaration: main(args) in namespace BasicArithmatic
BasicArithmatic_main_static:
	.space 13
BasicArithmatic_main:
	# Push local memory of this method to the stack, and restore it at the end of the method call
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

	# Save the location of the previous argument stack
	li $t0, arg_stack
	lw $t1, 0($t0)
	push $t1

	# The pointer to the agument stack for this method call is stored in $a0...
	# ...Load it as the current argument stack
	sw $a0, 0($t0)

	# Save the previous caller reference to the stack
	li $t0, caller
	lw $t1, 0($t0)
	push $t1

	# The caller of this method call is stored in the call_buffer...
	# ...Load it as the current caller
	li $t1, call_buffer
	lw $t1, 0($t1)
	sw $t1, 0($t0)

	# Start of method body
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 0
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	li $t3, 4
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 4
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	li $t3, 2
	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {casualMethod}--
	# Start method call
	# Evaluate method arguments, and push each argument to the stack
	# End argument evaluation
	call BasicArithmatic_casualMethod
	nop
	# End method call
	# Retreive return value from method call
	move $t3, $v0
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	# Start of conditional chain
	# Evaluate if condition
	li $t3, true
	push $t3
	pop $t3
	push $t3
	pop $t3
	beq $t3, $0, BasicArithmatic_main_conditional0_else0
	# Start if body
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 4
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# End of conditional chain

	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {c}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 8($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 8
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {constant}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	# Start of conditional chain
	# Evaluate if condition
	li $t3, false
	push $t3
	pop $t3
	push $t3
	pop $t3
	beq $t3, $0, BasicArithmatic_main_conditional1_else0
	# Start if body
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {c}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 8($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 8
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# Start else body
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {c}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 8($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 8
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	li $t3, 1
	push $t3
	pop $t3
	pop $t4
	lw $t0, 0($t4)
	addu $t3, $t3, $t0
	sw $t3, 0($t4)
	# End else body
	BasicArithmatic_main_conditional1_end:
	# End of conditional chain

	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {sum}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 12($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 12
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	pop $t3
	pop $t0
	addu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {product}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 16($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 16
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	pop $t3
	pop $t0
	mullo $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {difference}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 20($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 20
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	pop $t3
	pop $t0
	subu $t3, $t0, $t3
	push $t3
	pop $t3
	pop $t4
	sw $t3, 0($t4)
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {addImmediate}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 24($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 24
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {subImmediate}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 28($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 28
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {multiplyImmediate}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 32($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 32
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {parenthesis}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 36($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 36
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {pemdasFull}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 40($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 40
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {pemdasFullAlternate}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 44($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 44
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {multipleParenthis}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 48($t1)
	# Save the address of the symbol so that it can be assigned later
	li $t2, 48
	addu $t4, $t1, $t2
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t4
	li $t3, 1
	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {a}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 0($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

	push $t3
	# Save call buffer
	li $t1, call_buffer
	lw $t1, 0($t1)
	push $t1
	# End save call buffer

	# --Evaluate the symbol {b}--
	# Load the symbol from memory
	li $t1, BasicArithmatic_main_static
	lw $t3, 4($t1)
	# Load the result into the call_buffer for the next token
	li $t1, call_buffer
	sw $t3, 0($t1)
	# --Symbol evaluation complete--
	# Restore call buffer
	li $t2, call_buffer
	pop $t1
	sw $t1, 0($t2)
	# End restore call buffer

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
	# End of method body

	# Start of method return
	BasicArithmatic_main_return:
	# Restore the previous caller
	li $t0, caller
	pop $t1
	sw $t1, 0($t0)

	# Restore the pointer to the previous argument stack
	li $t0, arg_stack
	pop $t1
	sw $t1, 0($t0)

	# Restore the static memory of the previous call to this method
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
	# End of method return
# End of method declaration: main(args) in namespace BasicArithmatic
end:
