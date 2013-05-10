# treegrep

Grep utility for outlines, trees, and when indentation matters.

Useful for working with TODO lists when you need the full context of a match.

## Compiling

You first need rust-pcre.  Then compile with:
```
rustc treegrep.rs -L /path/to/rust-pcre
```

## Usage

Usage is similar to grep.
It works with one or more files, including piping from stdin.

```
treegrep [OPTION]... PATTERN [FILE]...
```

Example:

```
treegrep "\[>\]" ~/.tasks
```

Output:

```
[ ] some group of tasks
    [ ] subtasks
        [>] current task here!
```

## Ideas

There are no options yet but I have a few ideas listed in comments (tab size, leaves only, etc.)

The cli program *tree* outputs directory trees, but it doesn't work with treegrep since
the indentation style is different.  It would be cool to see treegrep take a directory tree instead of
files as input (maybe -d?).  Or alternatively, the ability to treat certain characters as whitespace so
you could just do tree | treegrep "blah"

Pull requests and other ideas are welcome. :)

## Contributers

You may add your name here if you add/fix something, etc.

```
Grady O'Connell <github.com/flipcoder>
```

Thanks to #rust and /r/rust for helping with bugs and giving tips!

## License

The MIT License (MIT)

Copyright (c) 2013 Grady O'Connell

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

