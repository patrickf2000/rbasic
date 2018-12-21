REM conditional2
REM Tests conditionals with strings

MAIN:
	let $str = ""
	let #no = 7
	
	do
		$str = (input)
		if [str == "exit" ] then break
		else println str
	while no < 8
	
	println "Goodbye!!"
	exit
