# This is a comment

# Variable assignment
let var = 1
let string = "hello"

# Print variables
print(var)

# Do maths
print(1 + 1)

# Variables have weak dynamic typing
let var = 1
let var = "hello"

# Different types can be used together in operations
print("hello" + 1)

# Functions

fun add(a, b) {
    return a + b
}

print(add(1, 2))

# Data types
let number = 1
let string = "hello"
let boolean = true
let list = [1, 2, 3]

# Lists can be indexed
print(list[0])

# Control flow

if 1==1 {
    print("true")
} else {
    print("false")
}

# Loops

for i in [1, 2, 3] {
    print(i)
}

for i in 5 {
    print(i)
}

for char in "hello" {
    print(char)
}

while true {
    print("looping")
}


# Scope

let global = 1
{
    let outer = 2
    {
        let inner = 3
        # Inner scope can see global scopes and outer scopes
        print(inner)
        print(outer)
        print(global)
    }
    # Outer scope can only see its local scopes and global scope
    print(outer)
}
# Global scope can only see global variables
print(global)
