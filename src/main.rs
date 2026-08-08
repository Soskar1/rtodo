use clap::Parser;
use rtodo::commands::RTodoArgs;

fn main() {
    let args = RTodoArgs::parse();
    rtodo::run(args);
}
