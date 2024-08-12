fn main() {
    println!(
        "Greetings from process {}! (parent: {})",
        nix::unistd::getpid(),
        nix::unistd::getppid()
    );

    unsafe {
        nix::unistd::fork().expect("fork failed");
    };
    println!(
        "Bye-bye from process {}! (parent: {})",
        nix::unistd::getpid(),
        nix::unistd::getppid()
    );
}
