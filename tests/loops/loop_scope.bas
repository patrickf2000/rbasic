REM loop_scope.bas
REM Tests the scope of variables in loops

MAIN:
    let $str = "Hello!"
    let #index = 0

    do
        let $str2 = "Yo!"

        if [index == 5] then $str = "Howdy!"
        elif [index == 4] then $str2 = "Yo ho!"

        println str
        println str2

        #index = index + 1
    while index < 10

    println str
    println str2

    exit