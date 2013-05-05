#!/usr/bin/rustc
use core::os;
use core::vec;

fn grep(pattern: &str, filename: &str, in : @io::Reader) {
    if in.eof() { return; }

    io::println(fmt!("%s: %s", filename, in.read_line()));
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

    let multiple = files.len()>1;
    for files.each |&d| {
        let p: ~Path = ~Path(d);
        let in = result::unwrap(io::file_reader(p));
        grep(pattern, p.to_str(), in);
    }
}
