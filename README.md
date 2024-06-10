`fdtshim-mapping-generator`
===========================

This tool generates a device tree source file from a mainline-flavoured `install_dtbs` output.

The source file can either be compiled as-is or be included from a `.dts` file with additional manually-authored mappings.

The compiled `mapping.dtb` can be used with `fdtshim` to properly map between ambiant data and the correct `.dtb` file.

See the [`fdtshim`](https://github.com/fdtshim/) project for more information.


Usage
-----

```
 $ fdtshim-mapping-generator path/to/dtbs > output.dtsi
```
