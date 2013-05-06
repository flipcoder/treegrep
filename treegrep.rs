#!/usr/bin/rustc

/*
   treegrep
   Like grep, but for trees, outlines, and when indentation matters
 * Not yet finished!
 */

extern mod std;
extern mod pcre;
use core::os;
use core::vec;
use pcre::pcre::*;
use pcre::consts::*;

fn indent_level(line: &str) -> int {
    0
}

fn grep(pattern: &str, filename: &str, in : @io::Reader) {
    let mut indent = -1;
    let mut stack : ~[~str] = ~[];

    while !in.eof() {
        let line = in.read_line();

        let cur_indent = indent_level(line);

        match search(pattern, line, PCRE_CASELESS) {
            Ok(_) => {
                io::println(fmt!("%s: %s", filename, line));
            }
            Err(e) => match e {
                CompileErr(_) => {
                    io::println("error: bad pattern");
                    return;
                }
                ExecErr(_) => {
                }
            }
        }

        // TODO: if cur => indent, push
        if cur_indent >= indent {
            stack += [line];
        } else {
            for stack.each |&blah| {
                io::println(fmt!("%s: %s", filename, blah));
            }
        }
        indent = cur_indent;
    }
}

fn main() {
    let args = os::args();
    if args.len() <= 1 {
        io::stderr().write_line(fmt!("Usage: %s [OPTION]... PATTERN [FILE]...", args[0]));
        return;
    }

    let pattern = copy args[1];

    if args.len() == 2 {
        let in = io::stdin();
        grep(pattern, ~"stdin", in);
        return;
    }

    let files = vec::slice(args, 2, args.len());

    /*let multiple = files.len()>1;*/
    for files.each |&d| {
        let p: ~Path = ~Path(d);
        let in = result::unwrap(io::file_reader(p));
        grep(pattern, p.to_str(), in);
    }
}
