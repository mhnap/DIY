enum Channel {
    Stable(Semver),
    Beta(Semver),
    Nightly(Semver),
}

struct Semver {
    major: u64,
    minor: u64,
    patch: u64,
}

fn release_info() -> Channel {
    Channel::Stable(Semver { major: 1, minor: 88, patch: 0 })
}

fn main() {
    if let Channel::Stable(v) = release_info()
        && let Semver { major, minor, .. } = v
        && major == 1
        && minor == 88
    {
        println!("`let_chains` was stabilized in this version");
    }
}
