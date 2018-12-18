REM div_vars
REM Tests the division function for integers

MAIN:
	GOSUB div_int
	GOSUB div_dec

REM Expected output:
REM 10
REM 2
REM 5
div_int:
	PRINTLN "Div Int Test"
	LET #x = 10
	LET #y = 2
	LET #answer = x / y
	
	PRINTLN x
	PRINTLN y
	PRINTLN answer

REM Expected output:
REM 55.66
REM 7.32
REM 7.60382513661
div_dec:
	PRINTLN "Div Double Test"
	LET .x = 55.66
	LET .y = 8.32 - 1
	
	PRINTLN x
	PRINTLN y
	
	.y = x/y
	
	PRINTLN y
