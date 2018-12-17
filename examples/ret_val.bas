REM ret_val
REM Tests returning to a variable

MAIN:
	let #x = 0
	gosub add
	memset x
	println x

add:
	let #x = 5
	let #y = 4
	#y = x + y
	
	return y
