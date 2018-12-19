REM char_cmd
REM Tests the char command

MAIN:
	let $str = "Hello!"
	let #len = (len str)
	let #index = 0
	
	do
		println (char index in str)
		#index = index + 1
	while index < len
