REM sub_vars.bas
REM Tests the subtraction function on variables

MAIN:
	GOSUB sub_int
	GOSUB sub_db
	GOSUB sub_str

REM Expected output
REM 10
REM 6
REM 4
REM 9
sub_int:
	PRINTLN "Subtraction int test"
	LET #x = 10
	LET #y = 6
	LET #answer = x - y
	
	PRINTLN x
	PRINTLN y
	PRINTLN answer
	
	#answer = answer + 5
	PRINTLN answer

REM Expected output:
REM 4.4
REM 1.2
sub_db:
	PRINTLN "Subtraction div test"
	LET .x = 4.4
	LET .y = x - 3.2
	
	PRINTLN x
	PRINTLN y

REM This should simply throw an error
sub_str:
	PRINTLN "Subtraction string test"
	LET $str = "Hello"
	LET $str2 = " how r u?"
	$str = str2 - str
	PRINTLN str
