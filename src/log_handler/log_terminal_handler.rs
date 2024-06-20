use atty::Stream;

pub fn run_in_terminal<F: Fn()>(function: F) {
    let is_terminal = atty::is(Stream::Stdout);
    if is_terminal {
        function();
    }
}

pub fn run_in_terminal_or_not<F1, F2>(if_terminal: F1, if_not_terminal: F2)
where
    F1: Fn(),
    F2: Fn(),
{
    let is_terminal = atty::is(Stream::Stdout);
    if is_terminal {
        if_terminal();
    } else {
        if_not_terminal();
    }
}
