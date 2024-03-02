#[cfg(desktop)]
mod desktop;


fn main() {
    #[cfg(desktop)]
    desktop::main();
    #[cfg(mobile)]
    mobile::main()
}
