REM conditional1.bas
REM Tests conditionals

MAIN:
	let #x = 0
	let #y = 1
	let $result = "Less"
	
	if [x > y] then $result = "Greater"
	
	println result
