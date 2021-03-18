# Specification of a Rust VM

## Object Representation

Each data in the VM is stored as an `object` onto the stack or the heap. An `object` stores a few tags along with the data. The data structure of an `object` is described below, with a field name and a description of the field:

* `type`: identfier that stores the data type
* `len`: the length of the data, if applicable
* `dat`: actual data stored in binary form with length `len`

### Proprietary Heaps

Specifically, functions are stored in specific "function heap" and classes are stored in specific "class heap". They are predefined in related code sections.

Therefore, functions have minimal side-effects: all information it can use is from the input stack and variables. A variable define-before-use will generate error only in runtime.

### Namespace

This is used to associate the nomenclature of functions, classes, and objects between different programs and libraries.

## Instructions

### Stack and Memory Manipulation

1. `pushi <type (u8)> <immediate>`
2. `pushv <var (u8)>`
3. `pop`
4. `load <heap_addr (u16)>`
5. `store <heap_addr (u16)>`
6. `stores <len (u16)> <string (bytes[len])>`
7. `alias <var (u8)> <heap_addr (u16)>` associates a variable name with a heap object

### Arithmetic Operations

1. `add`
2. `sub`
3. `mul`
4. `div`
5. `rem`

### Function Calling

`call <fheap_addr (u8)>` instruction will pop the stack for the input variables required by the function.

## Encoding

