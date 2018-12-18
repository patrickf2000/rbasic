REM lbl_test2
REM This tests the GOTO command

MAIN:
	PRINTLN "Label test 2"
	PRINTLN "In main function"
	GOTO sublabel
	PRINTLN "If you see this, the test failed."
	EXIT
	
sublabel:
	PRINTLN "Hello!"
	PRINTLN "We are in sublabel."
