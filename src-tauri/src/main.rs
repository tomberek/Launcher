#[cfg(desktop)]
mod desktop;

#[cfg(mobile)]
mod mobile;

fn main() {
    #[cfg(desktop)]
    desktop::main();

    #[cfg(mobile)]
    mobile::main();
}
