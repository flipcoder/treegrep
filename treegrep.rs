#!/usr/bin/rustc

/*
 * treegrep
 * Like grep, but for trees, outlines, and when indentation matters
 * Not yet finished!
 */

extern mod pcre;
use core::os;
use core::vec;
use pcre::pcre::*;
use pcre::consts::*;

fn indent_level(line: &str, tabwidth: uint) -> uint {
    let mut indent = 0u;

    for line.each_char |ch| {
        if ch == ' ' {
            indent += 1u;
        } else if ch == '\t' {
            indent += tabwidth;
        } else {
            return indent;
        }
    }
}

fn peek_line(in : @io::Reader) -> ~str {
    let mark = in.tell();
    let line = in.read_line();
    in.seek(mark, io::SeekSet);
    line
}

fn grep(
    pattern: &str,
    filename: &str,
    in : @io::Reader,
    tabwidth: uint
) {
    // queue of "pending" lines
    let mut queue : ~[~str] = ~[];
    let mut last_indent = 0u;
    let last_line = ~"";

    // keeps track of relative indentation
    // the arbitrarily-sized "tabs" of the document
    let mut tabs = ~[0u];
    let mut line_no = 0;

    while !in.eof() {
        let line = in.read_line();
        line_no += 1;

        let cur_indent = indent_level(line, tabwidth);
        let next_line = peek_line(in);
        let next_indent = indent_level(next_line, tabwidth);
        
        match search(pattern, line, PCRE_CASELESS) {
            Ok(_) => {
                for queue.each |&e| {
                    io::println(fmt!("%s: %s", filename, e));
                }
                queue= ~[];
            }
            Err(e) => match e {
                CompileErr(_) => {
                    io::stderr().writeln("error: bad pattern");
                    return;
                }
                ExecErr(_) => {
                }
            }
        }

        if next_indent <= cur_indent {
            
            let mut indent_diff: int = cur_indent as int - next_indent as int;
            while indent_diff > 0 {
                queue.pop();
                indent_diff -= tabs.pop();
                if indent_diff < 0 {
                    io::stderr().writeln("line %s: abnormal indentation");
                    return;
                }
            }

        } else {
            queue += [line];
            tabs += [next_indent - cur_indent];
        }

        /*last_indent = cur_indent;*/
    }
}

fn main() {

    // TODO: make options program arguments
    let tabwidth = 4u;
    //let print_leaves = true;
    //let print_parents = true;

    let args = os::args();
    if args.len() <= 1 {
        io::stderr().write_line(fmt!("Usage: %s [OPTION]... PATTERN [FILE]...", args[0]));
        return;
    }

    let pattern = copy args[1];

    if args.len() == 2 {
        let in = io::stdin();
        grep(pattern, ~"stdin", in, tabwidth);
        return;
    }

    let files = vec::slice(args, 2, args.len());

    //let multiple = files.len()>1;
    for files.each |&d| {
        let p: ~Path = ~Path(d);
        let in = result::unwrap(io::file_reader(p));
        grep(pattern, p.to_str(), in);
    }
}
