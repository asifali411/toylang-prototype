<center>
<img src="./public/banner.png" alt="banner" width="100%">
</center>

# ToyLang
![prototype](https://img.shields.io/badge/prototype-orange?style=for-the-badge)

A simple, readable, dynamically typed programming language.

> [!NOTE]
> Currently implemented as a tree-walking interpreter written in **Rust**. A future rewrite in **C with bytecode compilation** is planned for better performance.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Syntax Overview](#syntax-overview)
  - [Variables](#variables)
  - [Comments](#comments)
  - [Operators](#operators)
  - [Built-in Functions](#built-in-functions)
  - [Functions](#functions)
  - [Control Flow](#control-flow)
  - [Loops](#loops)
  - [Classes & Inheritance](#classes--inheritance)
  - [Arrays](#arrays)
  - [Hashmaps](#hashmaps)
  - [Null](#null)
- [Types](#types)
- [Example Program](#example-program)
- [Project Status](#project-status)
- [Inspiration](#inspiration)

---

## Features

- Clean, beginner-friendly syntax (Python meets JavaScript)
- Dynamic typing with explicit type conversion
- Functions, classes, and inheritance
- Arrays and hashmaps
- Built-in I/O functions
- `loop` keyword unifying counted and conditional loops

---

## Installation

ToyLang source files use the `.toy` extension.

### Quick install

On macOS and Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/asifali411/toylang-prototype/main/scripts/install.sh | sh
```

On Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/asifali411/toylang-prototype/main/scripts/install.ps1 | iex
```

### Build from source

```bash
# Clone the repository
git clone https://github.com/asifali411/toylang-prototype.git
cd toylang-prototype

# Build with Cargo
cargo build --release

# Run a ToyLang program
cargo run -- path/to/program.toy
```

---

## Syntax Overview

### Variables

```
var name = "ToyLang";
var age = 18;
```

Variable shadowing is allowed (re-declaring a variable in the same scope overwrites it).

> [!WARNING]
> Variable shadowing will be **removed** in future updates.

### Comments

```
// this is a single line comment
/* 
    this is a multi line comment
*/
```

### Operators

| Category   | Operators                      |
|------------|--------------------------------|
| Arithmetic | `+` `-` `*` `/` `%`            |
| Comparison | `<` `>` `<=` `>=` `==` `!=`    |
| Logical    | `(& / and)` `(\| / or)` `!`    |
| Assignment | `=` `+=` `-=` `/=` `*=` `%=`   |

> [!NOTE]
> `+` is also used for string concatenation, e.g. `"Hai, " + name`.

### Built-in Functions

| Function         | Description                        |
|------------------|------------------------------------|
| `output(value)`  | Print a value to the console       |
| `input(prompt)`  | Read a line of input from the user |
| `number(value)`  | Convert a value to a number        |
| `string(value)`  | Convert a value to a string        |
| `boolean(value)` | Convert a value to a boolean       |
| `type(value)`    | Extract type of a value            |

### Functions

```
func greet(name) {
    output("Hai, " + name);
}

func add(a, b) {
    return a + b;
}
```

### Control Flow

```
if i < 10 {
    
} else if i < 20 {
    
} else {
    
}
```

### Loops

ToyLang uses a single `loop` keyword for both counted and conditional loops:

```
// Executes exactly 6 times
loop 6 {
    // ...
}

// Loops if the condition is true
loop if i > 10 {
    // ...
}

// Loop through array
loop i in [10, 9, 8, 7, 6] {
    output(i);
}
```

### Classes & Inheritance

```
class Person {
    func Person(name, age) {
        this.name = name;
        this.age = age;
    }

    func display() {
        output("Name: " + this.name + "\n");
        output("Age: " + this.age + "\n");
    }
}

var person1 = Person("Spongebob", 79);
person1.display();
```

Inheritance uses the `inherit` keyword. Use `super` to refer to the parent class:

```
class Student inherit Person {
    func Student(name, age, marks) {
        super(name, age);
        this.marks = marks;
    }

    func display() {
        output("Name: " + this.name + "\n");
        output("Age: " + this.age + "\n");
        output("Marks: " + this.marks + "\n");
    }
}

var student1 = Student("Spongebob", 79, 99.98);
student1.display();
```

### Arrays

```
var array = [1, 2, 3, 4, 5];

// Access by index
output(array[0]);
```

### Hashmaps

```
var hashmap = {
    name: "Spongebob",
    age: 79,
};

// Access by key
output(hashmap.name);
```

### Null

```
var nothing = null;
```

---

## Types

ToyLang is **dynamically typed**. The following types are supported:

- `number` — floating point numbers
- `string` — text values
- `boolean` — `true` or `false`
- `null` — absence of a value

Use the built-in conversion functions (`number()`, `string()`, `boolean()`) to explicitly convert between types.

> [!WARNING]
> ToyLang does not have error handling (no try/catch). Invalid operations (e.g. type mismatches, undefined variables) will cause the interpreter to crash.

---

## Example Program

```
// Greet the user
func greet(name) {
    output("Hai, " + name);
}

var name = input("Enter your name: ");
var age = number(input("Enter your age: "));
greet(name);

// Class example
class Person {
    func Person() {
        this.name = "unknown";
    }
    func introduce() {
        output(this.name);
    }
}

var p = Person();
p.introduce();
```

---

## Project Status

This is a **prototype** project. The current Rust implementation is a working tree-walking interpreter.

### Planned: C Rewrite with Bytecode VM

The next major milestone is a rewrite in C with a bytecode compiler and stack-based virtual machine, similar to how Lua and CPython work. This will involve:

1. **Lexer** — tokenizer
2. **Parser** — builds an AST
3. **Compiler** — walks the AST and emits bytecode
4. **VM** — a stack-based loop that executes bytecode instructions
5. **Value system** — tagged union to represent ToyLang values in C

This rewrite will significantly improve performance.

### Planned Language Features

- String interpolation: `"Hello, {name}"`
- Improved error messages

---

## Inspiration

- Syntax inspired by Python, JavaScript, and Rust