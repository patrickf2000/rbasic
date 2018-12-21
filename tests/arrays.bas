REM arrays
REM Tests the arrays feature

MAIN:
    array $responses = "hi", "how", "are", "you"
    println responses

    let #index = 0

    do
        println "Enter something: "
        let $str = (input)
        push str to responses
        #index = index + 1
    while index < 3

    println "Final result:"
    println responses
    exit
