use core::os;
use core::vec;

fn grep(append: &str, in : &io::ReaderUtil, recursive : bool) {
    io::println("grep!");
    /*while !in.eof() {*/
        
    /*}*/
}

fn main() {
    let args = os::args();
    if args.len() <= 1 {
        io::stderr().write_line(fmt!("Usage: %s [OPTION]... PATTERN [FILE]...", args[0]));
        return;
    }

    if args.len() == 2 {
        let in = io::stdin();
        /*grep("", in);*/
        return;
    }

    let pattern = copy args[1];
    let files = vec::slice(args, 2, args.len());

    let multiple = files.len()>1;
    for files.each |&d| {
        let p: ~Path = ~Path(d);
        io::println(p.to_str());
    }
}
