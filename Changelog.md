## Unreleased

This initial version is based on the C versoin of the code. Below are the differences from the C version:

* Written in rust (self-explanatory)
* The ability to daemonize, and related configuration, has been removed.
    Modern linux services no longer daemonize. Init systems, such as systemd,
    prefer a foreground process for various reasons, and if you still need to
    daemonize, you can use a double-fork in a shell or something like debian's
    start-stop-daemon(8).
* Support for a pidfile, and related configuration, has been removed.
    Init systems, such as systemd, are more than capable of managing pidfiles
    themselves and ensuring duplicate processes do not run.
