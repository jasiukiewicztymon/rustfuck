; puts.rbf - print some brainfuck code for a given string (print + clear)
;
; Dean Scarff's brain was last raped 27 July 2003.
;
; <Rich BrainFuck source file>
; To extract pure brainfuck source (and format it as well!):
; sed -e 's/;.*//' puts.rbf | tr -d '[:space:]' \
; | sed -e 's/\(.\{80\}\)/\1\n/g' > puts.bf
;
; Some usage examples:
; $ echo "My string" | bfi puts.bf > mystring.bf
;
; $ bfi puts # Then just type, hit Ctrl+D twice or ÿ when done
;

; *** Initialize increment and print output registers ***
->++; a[0] = -1; a[1] = 2;
>>+++++++++++; a[3] = 11; p = 3;
[<++++[<+<+>>-]>-]<; a[0], a[1] += 4 * a[3]; a[3] = 0; p = 2;

; *** Read a character ***
,+[-; if input != EOF (or ÿ)

; *** Print "increment and print" sequence ***
[<<.>>-]<.>; print ("+" x a[2] "."); a[2] = 0;

; *** Initialize clear output ***
>>+++++++++; a[4] = 9; p = 4;
[[<+<++>>-]<[>+<-]>-]<<+; a[2] = 91; p = 2;

; Print "clear" sequence
.<-.+>++.; print ("[-]");

[-],+]; cleanup, loop
