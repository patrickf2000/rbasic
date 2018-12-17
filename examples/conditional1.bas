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
	
	goto cmp_double
	
cmp_double:
	println "Cmp double"
	
	let .x = 4.6
	let .y = 4.5
	
	if [x > y] then println "Greater"
	elif [x < y] then println "Less"
	else println "They are equal!"
	
	
	if [x == y] then println "They are equal!"
	elif [x != y] then println "They are not equal."
