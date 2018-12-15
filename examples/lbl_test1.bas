REM lbl_test1
REM This tests the GOSUB command

MAIN:
	PRINTLN "GOSUB command test."
	PRINTLN ""
	GOSUB label1
	PRINTLN "Back from label 1"
	EXIT
	
label1:
	PRINTLN "In Label 1"
	GOSUB label2
	PRINTLN "Back from label 2"
	
label2:
	PRINTLN "In label 2"
	GOSUB label3
	PRINTLN "Back from label 3"

label3:
	PRINTLN "In label 3"
