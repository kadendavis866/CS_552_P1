# CS 552 Operating Systems Project 1

This is a very simple circular doubly-linked list written in Rust.
It closely follows the C template that was provided with this assignment but some small changes have been made.
These changes should not affect functionality but arose due to fundamental differences between the languages.

Note: The library has been tested but does contain code in unsafe blocks. Use at your own risk.

Steps to configure, build, run, and test the project.

## Building
To build a shared rust library:
```bash
make
```

## Testing

```bash
make check
```

## Clean

```bash
make clean
```

## Install Dependencies

If needed, the rust build system (cargo) can be installed by running the following command:

```bash
make install-deps
```
