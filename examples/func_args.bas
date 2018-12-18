REM func_args
REM Tests passing arguments to a function

MAIN:
	let #x = 5
	let #y = 10
	
	gosub double[x,y]
	goto name_test
	
double[#x, #y]:
	println "In double func"
	
	#x = x * 2
	#y = y * 2
	
	println x
	println y
	
name_test:
	println " "
	let $name = ""
	
	print "What is your name? "
	input name
	
	gosub print_name[name]

print_name[$name]:
	print "Hello, "
	print name
	println " "
