REM func_to_var
REM This tests assigning the result of a function call to a variable

MAIN:
	let #x = 0
	print "Enter a number: "
	input x
	
	println (gosub double[x])

double[#x]:
	let #no = x * 2
	return no
