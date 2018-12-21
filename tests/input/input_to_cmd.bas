REM input_to_cmd
REM Tests input directly to a variable/print statement

MAIN:
	let $str = ""
	let #index = 0
	
	do
		$str = (input)
		println str
		#index = index + 1
	while index < 5
	
	exit
