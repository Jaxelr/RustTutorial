fn main() {
    println!("Hello, world!");
}

/* The purpose of this repo is to validate how cargo manages the dependencies from a github repository.

    If you validate the cargo.toml file, you can see that the dependency from rand obtained from github
    does not include the commit sha. So in order to guarantee consistency as we commit and pass the lib around
    the Cargo.lock file includes information about the commit.

    In order to upgrade the commit the following command can be executed:
        cargo update -p rand

    If we want to upgrade all packages included we can run
        cargo update

*/