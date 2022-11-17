mod log;
mod style;

fn main() {
    info!(("gaming"), (" testing", red));
    warn!(("gaming"), (" testing", red));
    error!(("gaming"), (" testing", red));
}