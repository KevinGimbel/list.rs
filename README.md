# list.rs
> List directory files in current directory and all sub-directories

## HEY, LISTEN 🧚‍
This is my first ever thing in Rust and it is not optimized or tested or whatever. It's probably a very bad idea to run this script from the `/` directory since it doesn't stop - it just prints all directories and files.

## Goal

The Goal is to have a tool similar to `ls` and `tree` - basically a mix of both.

## Why Rust? 
Why not? I haven't tried Rust yet so I thought this was a good idea.

## Known Issues

- [ ] The indention logic is still wrong
- [ ] All values are written in quotes and I don't know how to get rid of them

## Roadmap

### v0.0.2
- [ ] Add limit
- [ ] Add limit from environment variable

## v0.0.3
- [ ] Implement colors (optional, CLI flag)
- [ ] Implement different modes (CLI flags)
