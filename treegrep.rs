#!/usr/bin/rustc

extern mod pcre;
use core::os;
use core::vec;
use pcre::pcre::*;
use pcre::consts::*;

fn indent_level(line: &str, tabwidth: int) -> int {
    let mut indent = 0;

    for str::each_char(line) |ch| {
        match ch {
            ' '  => { indent += 1; }
            '\t' => { indent += tabwidth; }
            _    => { return indent; }
        }
    }

    return indent;
}

// I don't use this anymore
/*fn peek_line(in : @io::Reader) -> ~str {
    let mark = in.tell() as int;
    if in.eof() {
        return ~"";
    }
    let line = in.read_line();
    in.seek(mark, io::SeekSet);
    line
}*/

fn grep(
    pattern: &str,
    filename: &str,
    in : @io::Reader,
    tabwidth: int
) {
    let mut queue : ~[(uint, ~str)] = ~[];

    // keeps track of relative indentation
    // the arbitrarily-sized "tabs" of the document
    let mut tabs = ~[0];
    let mut line_no = 0u;
    let mut last_indent = 0;

    while !in.eof() {
        let line = in.read_line();
        line_no += 1;

        let cur_indent = indent_level(line, tabwidth);
        /*let next_line = peek_line(in);*/
        /*let next_indent = indent_level(next_line, tabwidth);*/

        let mut diff = cur_indent - last_indent;
        if diff == 0 {
            if !queue.is_empty() {
                queue.pop();
            }
            queue += [(line_no, copy line)];
            
        } else if diff > 0 {
            queue += [(line_no, copy line)];
            tabs += [cur_indent - last_indent];
        } else {
            while diff < 0 {
                diff += tabs.pop();
                if !queue.is_empty() {
                    queue.pop();
                }
            }
            if !queue.is_empty() {
                queue.pop();
            }
            queue += [(line_no, copy line)];
        }

        match search(pattern, line, PCRE_CASELESS) {
            Ok(_) => {
                for queue.each |&e| {
                    if filename.is_empty() {
                        println(e.second());
                    } else {
                        println(fmt!("%s(%u): %s", filename, e.first(), e.second()));
                    }
                }
                queue= ~[];
            }
            Err(e) => match e {
                CompileErr(_) => {
                    io::stderr().write_line("error: bad pattern");
                    return;
                }
                ExecErr(_) => {
                }
            }
        }
        
        last_indent = cur_indent;
    }
}

fn main() {

    // TODO: make options program arguments
    let tabwidth = 4;
    //let print_leaves = true;
    //let print_parents = true;

    let args = os::args();
    if args.len() <= 1 {
        io::stderr().write_line(fmt!(
            "Usage: %s [OPTION]... PATTERN [FILE]...",
            Path(args[0]).filename().unwrap()
        ));
        return;
    }

    let pattern = copy args[1];

    if args.len() == 2 {
        let in = io::stdin();
        grep(pattern, "", in, tabwidth);
        return;
    }

    let files = vec::slice(args, 2, args.len());

    let multiple = files.len()>1;
    for files.each |&d| {
        let p: ~Path = ~Path(d);
        let in = io::file_reader(p).unwrap();
        grep(pattern, if multiple {p.to_str()} else {~""}, in, tabwidth);
    }
}
