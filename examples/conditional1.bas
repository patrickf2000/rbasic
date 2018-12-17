REM conditional1.bas
REM Tests conditionals

MAIN:
	let #x = 1
	let #y = 10
	let $result = "Don't know"
	
	if [x > y] then $result = "Greater"
	elif [x < y] then $result = "Less"
	
	println result
