REM func_to_var
REM This tests assigning the result of a function call to a variable

MAIN:
	let #x = 0
	print "Enter a number: "
	input x
	
	let #result = (gosub double[x])
	println result
	
	let #result2 = 0
	#result2 = (gosub double[result])
	println result2

double[#x]:
	let #no = x * 2
	return no
