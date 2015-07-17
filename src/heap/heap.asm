# Simplified heap implementation. The heap is a fixed-size, and its size cannot be dynamically adjusted.
# Memory is allocated from the "bottom" of the heap, and the "bottom" is redefined as the first untouched word in the heap.
# Currently, memory is never reclaimed.
#
# A bitmap is used to keep track of which words in the heap are free and which are in use.
# Future versions will feature a "free" subroutine in addition to "calloc"
.equ heap_size 4096

# Allocates contiguous memory of a specified size
# The desired size should be specified in 32-bit words, 
# and stored in $a0 before calling malloc.
# 
# A pointer to the allocated memory will be returned via $v0
malloc:
	# TODO
	# TODO: search the memory map for an open address in the middle of the heap
	li $t0, heap_pointer
	lw $v0, 0($t0)
	#Calculate amount of memory to allocate, in bytes
	li $t1, 4
	mullo $t1, $a0, $t1
	#Calculate max heap pointer
	li $t9, 4
	li $t8, heap_size
	mullo $t9, $t9, $t8 # max offset
	li $t8, heap # base address
	addu $t9, $t9, $t8 # max value of pointer
	# TODO: check for overflow
	# Calculate the new heap pointer
	addu $t7, $v0, $t1
	sw $t7, 0($t0)
	#Update the memory map
	## Calculate the corresponding bits
	#subu $t6, $v0, $t7 #t6 = start_pointer - base_address_of_heap
	#srl $t6, $t6, 2 # divide offset by 4 (to get the word number)
	#srl $t5, $t6, 5 # get the word of the bitmap that maps to this range
	#andi $t4, $t6, 0b11111 # get the bit corresponding to the first word in this range
	#addiu $t4, $t4

	return

init_heap:
	li $t0, heap
	swm $t0, heap_pointer
	return
	
# A pointer to the highest word in the heap that has not yet been allocated
# Note that this value will not go down, unless a contiguous block at the top of the heap is deallocated
# Thus, there will never be a word in the heap that is occupied at an address greater than that which is stored in this pointer
heap_pointer:
	.word 0

# A bitmap representing which words in the heap are currently free
# Each bit represents one word (32-bits), with the first bit (left-most) representing the first word (left-most) in the heap
# If the bit is set to 1, the word is occupied
# If the bit is set to 0, the word is free
#
# 128 words are consumed by this map, which means 2**12 (4096) words in the heap can be addressed.
# This value corresponds directly to the static size of the heap
heap_memory_map:
	.space 128
heap:
	.space 4096

end:
