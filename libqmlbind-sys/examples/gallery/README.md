# gallery

This example requires a build script to compile the resource as a library
and link it.

## Compile

```
cargo build
```

## Fix rpath

Until the `rpath` issue is fixed on OSX, run this:

```
./fix_qt_rpath.sh
```

## Run

Note that the `rpath` for the resource library is asbent completely. For now,
simply copy the library files beside the binary:

```
cd target/debug
cp build/gallery-*/out/*galleryresources* .
./gallery
```
