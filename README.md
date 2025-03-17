<div align="center">

# brewr

**brewr** updates brew and adds descriptions for new and outdated formulae

Built with Rust as a first project while I was learning it. With multi-threaded lookups.

![Crates.io](https://img.shields.io/crates/v/brewr?link=https://crates.io/crates/brewr)
![Crates.io](https://img.shields.io/crates/l/brewr?link=https://github.com/iceman/brewr/blob/main/master/LICENCE)

</div>

![Screenshot of brewr](https://raw.githubusercontent.com/iceman/brewr/master/screenshot3.jpg)

```
Usage: brewr [OPTIONS] (no options defaults to update brew)

Options:
  -a, --all      List all installed formulae with descriptions
  -l, --leaves   List all manually installed formulae with descriptions
  -g, --grid     Display results with grid lines
```