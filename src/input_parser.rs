pub fn parse_input(args: &[String]) -> u32 {

    if args.len() != 2{
        panic!("Enter exactly 2 arguments")
    }

    let size = args[1].trim().parse().expect("Enter a number");
    return size;
}