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
* The config file's `allow` syntax now requires cidrs to be specified exactly.
  That is to say, before the following were all allowed:
  ```
  allow all
  allow 1.2.3.4
  allow 1.2
  allow 3.4.5
  allow 6.7.8/22
  allow 6.7.8.9/22
  allow 2001:db8::/32
  allow 0/0
  allow ::/0
  allow
  ```
  Now, the only forms that are allowed are:
  ```
  allow all
  allow 1.2.3.4/22
  allow 2001:db8::/32
  ```
  In essence, [rfc4632](https://tools.ietf.org/html/rfc4632) cidr notation must be followed, with the exception of the special 'all' keyword.
