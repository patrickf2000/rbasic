REM mp_vars.bas
REM Tests multiplication with variables

MAIN:
	GOSUB mp_int
	GOSUB mp_db
	
REM Expected output
REM 5
REM 4
REM 20
mp_int:
	PRINTLN "Mp Int Test"
	LET #x = 5
	LET #y = 4
	LET #answer = x*y
	
	PRINTLN x
	PRINTLN y
	PRINTLN answer

REM Expected output:
REM 3.3
REM 4.4
REM 14.52
mp_db:
	PRINTLN "Mp double test"
	LET .x = 3.3
	LET .y = 4.4
	
	PRINTLN x
	PRINTLN y
	
	.y = x * y
	
	PRINTLN y
