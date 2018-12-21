REM int_vars1
REM This tests integer variables

MAIN:
	LET #x = 100
	LET #y = 200
	
	PRINTLN "X: "
	PRINTLN x
	
	PRINTLN " "
	
	PRINTLN "Y: "
	PRINTLN y
	
	PRINTLN " "
	
	#x = 300
	
	PRINTLN "X after redef:"
	PRINTLN x
	
	PRINTLN " "
	
	#y = x
	
	PRINTLN "Y after redef:"
	PRINTLN y
