# An example project

## Output

This program demonstrates linking the application to Configtpl library.
It builds a simple configuration and prints it to console.
The expected output would be something like:

```
Operation succeeded. Here is the result:
.urls.base=example.com
.urls.mail=mail.example.com
.server.host=example.com
.server.port=1234
.my_overidden_key=my_overidden_value
```

## Building

### Linux

```bash
make all
```

If you use clang:

```bash
make all CXX=clang
```

If you wish to link the libary statically (i.e. no `libconfigtpl.so` dependency):

```
make all static=true
```

To launch the binary:

```bash
# Set path to the compiled library (libconfigtpl.so, libconfigtpl.a).
# Change the last part of path to `debug` if you had built the lib in debug mode
export LD_LIBRARY_PATH=$PWD/../../../../target/release

./my_test_app ../../../../tests/t000_simple/config.cfg
```

### Windows

In the first, copy the `configtpl.lib` and `configtpl.dll` from Rust build directory to this dir. Then run:

```
build.bat
```

To launch the binary:
```
main.exe ..\..\..\..\tests\t000_simple\config.cfg
```
