REM modo_vars
REM Tests the modolus function

MAIN:
	GOSUB mod_int
	GOSUB mod_dec

REM Expected output
REM 10
REM 2
REM 0
REM 1
mod_int:
	PRINTLN "Mod Int Part 1"
	LET #x = 10
	LET #y = 2
	LET #answer = x% y
	
	PRINTLN x
	PRINTLN y
	PRINTLN answer
	
	PRINTLN " "
	PRINTLN "Part 2"
	
	#answer = 11 %y
	PRINTLN answer
	
	PRINTLN " "

REM Expected output
REM 88.32
REM 32.32
REM 23.68
mod_dec:
	PRINTLN "Mod Dec Part 2"
	LET .x = 88.32
	LET .y = 32.32
	LET .answer = x%y
	
	PRINTLN x
	PRINTLN y
	PRINTLN answer
