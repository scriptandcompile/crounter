# Overview #

Crounter is a command line driven multi-language code metrics tool. 

## Goals ##

We will strive to be user friendly, configurable, fast, smart, powerful, and modular. In that order.

As with anything involving languages, there will always be multiple ways to do something.

When possible we will do the smart or useful thing by default and provide a way to alter this behavior with through configuration.

For example. The most common and useful command to use should be the bare crounter command
within the root of your source directory. The command should search this directory, find
any project descriptor files, the language types, ignore any of the standard files which should be ignored, and return a human readable, easily understood summary.

All of this should be modifiable through a command line switch or configuration file.

The user should be able to send the output to a file, change the output to a computer usable standard format, switch to a 'dumb' count rather than a 'syntactically significant' version, and so on.

For most users, they should be able to make reasonable assessments based on the least amount of effort and have the power to control the tool to their own preferences.

## Roadmap ##

- [ ] Line Count.
    - [x] Standard (LoC) line of code count.
    - [ ] 'Syntactically significant' (LoC) line of code count.
        - [ ] C#.
    let mut code_file_count = 0;

    let mut code_lines_total = 0;
    - [ ] Delphi.
    - [ ] Cargo Rust.

- [x] .gitignore support.
- [x] .ignore support.