# Rust: Getting started

- actively developed.
- to update: `rustup update`
- local docs: `rustup doc`
- compiling single file code: `rustc <file name>`
    - this would create two files
        - <filenam>.pdb
        - <filename>.exe
    - executing exe doesn't require rust, and it should be able to run on any Win machine   

### Working with a rust project

Rust uses a tool named __cargo__ to manage project.

__Create project using cargo__ 
-
```
cargo new <projectname>
```
The above command would generate a project folder with the same name and some default details.

__Compile project__

From inside the project root folder, run below command
```
cargo run
```
see run logs to know details about what just happend. You are smart enough to do that!!

__initialize non-empty dir__
```
cd <proj-dir>
cargo init
```

---

## Primitive datatypes

- `let x=10;`
- immutable by default 
- to declare mutable var: `let mut x = 10;`
- prefered naming of vars, snake_case

__Four datatypes__
- Integers
- Floating point
    - f32
    - f64
- Boolean
- Characters



