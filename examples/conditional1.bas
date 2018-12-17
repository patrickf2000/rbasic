REM conditional1.bas
REM Tests conditionals

MAIN:
	let #x = 10
	let #y = 1
	let $result = "Don't know"
	
	if [x > y] then $result = "Greater"
	elif [x < y] then $result = "Less"
	else $result = "Maybe they are equal!"
	
	println result
