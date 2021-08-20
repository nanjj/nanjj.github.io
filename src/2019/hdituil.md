# Manipulate Disk Images Under Mac

> Created at <2019-12-03 二>

## Mac Case Insensitive File System

By default, file system under Mac is case insensitive. It may cause
trouble when you work with some projects like Linux Kernel. To solve
the problem, you can create a disk image with case sensitive file
system, and attach to a mount point to use.

Say we want to create disk image at `/usr/local`, and attach it to
`/usr/local/src`. All operations are executed under `/usr/local`.

## Create Disk Image

```bash
$ hdiutil create -size 10G src -type SPARSE -fs \
	'Case-sensitive Journaled HFS+' -volname src
```

Image `src.sparseimage` will be created, with case sensitive journaled
file system and the size limit 20G.

## Attach Disk Image

```bash
$ mkdir src
$ hdiutil attach -mountpoint src src.sparseimage #=>
/dev/disk4              GUID_partition_scheme
/dev/disk4s1            EFI
/dev/disk4s2            Apple_HFS            /usr/local/src
```

## Detach Disk Image

```
$ hdiutil detach src #=>
"disk4" unmounted.
"disk4" ejected.
```

## Resize Disk Image If needed

If `20G` is not enough you can resize the image:

```
$ hdiutil resize -size 20G src.sparseimage
```
