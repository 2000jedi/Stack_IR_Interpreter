# Specification of the Intermediate Representation for a Rust VM

> The project is defered as the 3-address-form IR is prioritized for enhanced JIT support.

## Object Representation

Each data in the VM is stored as an `atom` onto the stack or the heap. An `atom` stores a few tags along with the data. The data structure of an `object` is described below, with a field name and a description of the field:

* `type`: identfier that stores the data type
* `len`: the length of the data, if applicable
* `dat`: actual data stored in binary form with length `len`

### Proprietary Heaps

Specifically, functions are stored in specific "function heap" and classes are stored in specific "class heap". They are predefined in related code sections.

Therefore, functions have minimal side-effects: all information it can use is from the input stack and variables. A variable define-before-use will generate error only in runtime.

### Namespace

This is used to associate the nomenclature of functions, classes, and objects between different programs and libraries.

## Code Section

There are two code sections.

1. `.class` to define program-level structures.
2. `.function` to define functions and procedures.

## Virtual Instructions

### Class and Function Declaration

1. `defun <fheap_addr (String)> <par_size (usize)> [list of <par_type(String)] <return_type>` & `endef`: applied to top levels of `.function` section only
2. `defcl` & `endcl`: applied to all levels of `.class` section.

### Stack and Memory Manipulation

1. `pushi <immediate (i32)>` / `pushf <immediate (f32)>`
2. `pop`
3. `load <heap_addr (u16)>`
4. `store <heap_addr (u16)>`
5. `stores <heap_addr (u16)> <len (u16)> <string (bytes[len])>`
6. `dup` duplicates the top of stack

[comment]: # (6. `alias <var (String)> <heap_addr (u16)>` associates a variable name with a heap object)

### Arithmetic Operations

1. `add`
2. `sub`
3. `mul`
4. `div`
5. `rem`

### Comparison Operation

1. `eq`
2. `ne`
3. `gt`
4. `lt`
5. `ge`
6. `le`

### Control Flow

1. `label <label (usize)>`
2. `goto <label (usize)>`
3. `branch <label (usize)>` 
    goto the specific label when the stack top has value other than 0
    the stack top will be poped

### Function Calling

`call <fheap_name (String)>` instruction will pop the stack for the input variables required by the function.

## Encoding

There are two encodings of the RVM language.

One encoding is the `raw encoding`, where each operation and namespace are encoded in english.
The `raw encoding` is more convenient and it should be used for debugging only.

The second encoding is the `byte encoding`, which is designed for space efficiency.
Therefore, `byte encoding` should be the only encoding used in production.

### Byte Encoding
