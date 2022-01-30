# swimscript
A language for writing swim practices.

## Table of contents
* [Purpose](#purpose)
* [Language Spec](#language-spec)

## Purpose
The goal of swimscript is to create a universal langauge that can be written to create swim practices.
Having an easy to write format that is compiled into a universal standard such as a JSON, would make it easier
to write and export swim sets into a prefered format. In the future we plan on writing a front end to generate
LaTeX from swimscript.

## Language Spec
Currently swimscript is in very early development, but here is some example code.
```
- A sample practice
Warm Up: {
    5 * 125 75 swim, @1:45 , 25 drill, 25 kick
    200 IM, Drill / Swim
    4 * 50 Choice, @:55, Variable
}

Prep Set: {
    4 * 25 IMO, @:45
    6 * 50 Free, @1:00, Maxi Mini
    2 * 125 Choice, @1:25/1:30/1:40, Just make it
}

Main Set: {
    3 * {
        100 Free, @1:20/1:30
        200 Free, @2:40/3:00
        300 Free, @4:00/4:30
        400 Free, @5:20/6:00
        - Very short rest to get pull stuff together
        400 Pull, @5:20/6:00
        300 Pull, @4:00/4:30
        200 Pull, @2:40/3:00
        100 Pull, @1:20/1:30
    }
}

Cool Down: {
    50 Destroy
    50 EZ
}
```
