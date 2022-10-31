// Lists vs subscripting


list = [1, 2, 3]

list[0]

get_list()[0]

parentheses priority is higher than subscripting
syntax node is subscriptable if it is already satisfied
if the previous node isn't satisfied/requires some operand that it doesn't have,
then the brackets are treated as a list

statement:
`get_list()[0]`

tokens:
get_list ( ) [ 0 ]

nodes after conversion pass:
get_list () []

nodes after first parsing pass:
() []

second pass:
[]

// Call function from list
`list[0]()`

list [ 0 ] ( )

list [] ()

[] ()

()

`(get_list())[0]`

( get_list ( ) ) [ 0 ]

[ and ( have the same base priority. whichever comes first gets evaluated first

list vs subscripting

`[1][0]`
[ 1 ] [ 0 ]

([])[3]
foo([], [])[3]

list rules:
- first token of the statement
- required by another token??
- after a comma (commas are not self-stable)

subscript rules:
- not first token in the statement
- previous tokens are self-stable

