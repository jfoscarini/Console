# Console

Console is a piece of a bigger software that handles user commands in a terminal-like interaction environment.

The goal was to make it easy to add self documented commands/command variations to explore and manipulate data within that bigger software.

There are no intended changes to be made to this project.

## What is it

This program reads user input and calls some function if it matches any of the commands' many possible predefined patterns.

Patterns can be composed by an arrangement of two things: ***Keyword*** and ***Some***.
* ***Keyword*** expects an exact word match.
* ***Some*** expects any word.

For example, if you want a ***math*** command that can sum or subtract two numbers, you can have two patterns:
* `math some keyword(+) some` calling the ***sum*** function
* `math some keyword(-) some` calling the ***sub*** function

Commands are self documented through the use of two functions: *desc* and *usage*. Shown when an invalid pattern is entered or the help function is called.

## How to run it

```console
$ ./console
```

## Example

One of the provided example commands, ***math***, either sums or subtracts two values. It has two executing functions, *sum* and *sub*, that are called when the informed patterns are matched with the user's input.

`/src/commands/math.rs`
```rust
pub struct CmdMath;
impl CmdMath {
    fn sum(...) -> Result<(), Box<dyn Error>> {
        ...
    }

    fn sub(...) -> Result<(), Box<dyn Error>> {
        ...
    }
}

impl Command for CmdMath {
    fn register(cmds: &mut Commands) {
        cmds.register_command_meta("math", Self::desc, Self::usage);
        cmds.register_pattern("math", "math_sum", Self::sum, vec![
            CommandArg::Some,
            CommandArg::Keyword("+"),
            CommandArg::Some,
        ]);
        cmds.register_pattern("math", "math_sub", Self::sub, vec![
            CommandArg::Some,
            CommandArg::Keyword("-"),
            CommandArg::Some,
        ]);
    }

    fn desc() { ... }
    fn usage() { ... }
}
```

It is already registered to the console.

`/src/console.rs`
```rust
...

pub fn register_commands(&mut self) -> &mut Self {
    CmdMath::register(&mut self.commands);
    ...
}

...

```

Try it out:

```
>> help math
- : Sum or subtract two numbers
USAGE: math value_one <+/-> value_two
>> math 10 + 15
25
>>
```
