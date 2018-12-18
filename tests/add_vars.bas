MAIN:
	GOSUB add_int
	GOSUB add_db
	GOSUB add_str

REM Expected output:
REM 20
REM 16
add_int:
	PRINTLN "Add Int Test"
	LET #x = 10
	LET #y = x + 6
	#x = y + 4
	PRINTLN x
	PRINTLN y

REM Expected output:
REM 1.1
REM 2.2
REM 3.3
add_db:
	PRINTLN "Add Double Test"
	LET .x = 1.1
	LET .y = 2.2
	LET .answer = x + y
	
	PRINTLN x
	PRINTLN y
	PRINTLN answer

REM Expected output:
REM Hello, 
REM how are you?
REM Hello, how are you?
add_str:
	PRINTLN "Add string test"
	LET $str1 = "Hello, "
	LET $str2 = "how are you?"
	LET $combine = str1 + str2
	
	PRINTLN str1
	PRINTLN str2
	PRINTLN combine
