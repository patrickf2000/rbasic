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
    
    ln
    println "Press any key to continue..."
    input

    goto int_arr

int_arr:
    cls
    array #numbers
    let #index = 1

    do
        push index to numbers
        #index = index + 1
    while index < 11

    println "The numbers 1-10 are:"
    println numbers

    ln
    println "Press any key to continue..."
    input

    goto db_arr

db_arr:
    cls
    array .numbers
    let #index = 0

    do
        let .no = (rand 100)
        push no to numbers
        #index = index + 1
    while index < 10

    println "Below are 10 random numbers:"
    println numbers

    ln
    println "Press any key to continue..."
    input

    exit