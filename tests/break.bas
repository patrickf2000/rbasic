REM break
REM Tests the break command

MAIN:
	let #no = 0
	let #x = 7
	
	do
		#no = (input)
		if [no == 5] then break
		else println no
	while x > 6
	
	println "Goodbye!!"
	exit
