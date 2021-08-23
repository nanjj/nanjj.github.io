# FreeBSD Cloud Init

`<2017-10-27 Fri>`

How to setup cloudinit for freebsd and how to do disk resize under
freebsd.

## Setup CloudInit for FreeBSD Image

In this guide we will talk about `CloudInit` Setup on FreeBSD, not
`bsd-cloudinit`. It's because `bsd-cloudinit` is based on
`cloudbaseinit`, which has no support on `ConfigDrive`, while our
OpenStack cloud supports only `ConfigDrive`. So for freebsd image we
need to enable `CloudInit`.

## Download Image And Convert

We need packages `qemu-utils`, `wget`:
```
pkg install qemu-utils wget
```

And download freebsd base image:
```
base=FreeBSD-11.1-RELEASE-amd64
release=releases/VM-IMAGES/11.1-RELEASE/amd64/Latest
wget https://download.freebsd.org/ftp/$release/${base}.qcow2.xz
```

Unzip and convert to raw:
```
unxz ${base}.qcow2.xz
qemu-img convert -p -f qcow2 -O raw ${base}.qcow2 ${base}.raw
```

## Mount Image And Chroot

Attach to device:
```
mdconfig -a -t vnode -u 0 -f ${base}.raw
```

Mount last partition:
```
mount /dev/md0p3 /mnt
mount -t devfs devfs /mnt/dev
cp /etc/resolv.conf /mnt/etc
chroot /mnt
```

## Install Cloud-init

Install `cloud-init`:

```
pkg install py27-cloud-init
```

Enable services:

```
cat >> /etc/rc.conf << EOF
cloudinit_enable="YES"
sshd_enable="YES"
EOF
```

Cloudinit call `blkid` to select config drive. But `blkid` doesn't
work for FreeBSD filesytem. Now work around `blkid` as below:
```
cat > /usr/bin/blkid << EOF
#!/bin/sh
[ -e /dev/iso9660/config-2 ] || exit 1
echo /dev/iso9660/config-2
EOF
chmod a+x /usr/bin/blkid
```

Mount for cd9660 on FreeBSD does not support `-o sync`. Patch
cloudinit `mount`:
```
cd /usr/local/lib/python2.7/site-packages/cloudinit
patch -p1 << EOF
--- a/util.py   2017-10-27 07:14:55.968737000 +0000
+++ b/util.py   2017-10-27 07:15:03.481245000 +0000
@@ -1364,6 +1364,7 @@
         if mtypes is None:
             mtypes = ["auto"]
     elif platsys.endswith("bsd"):
+        sync = False
         if mtypes is None:
             mtypes = ['ufs', 'cd9660', 'vfat']
         for index, mtype in enumerate(mtypes):
EOF
```

Cleanup and exit:

```
set history = 0
exit
```

## Umount Image And Convert Back

```
umount /mnt/dev
rm /mnt/etc/resolv.conf
umount /mnt
mdconfig -d -u 0
qemu-img convert -p -f raw -O qcow2 ${base}.raw ${base}.qcow2
```

## Boot Instance

Once you boot an instance, you can access it via ssh, please notice
the default user is `beastie`.


## Disk Resize Under FreeBSD

First to list the geoms:

```
gpart list
=>
Geom name: vtbd0
modified: false
state: CORRUPT
```
Show the Geom:
```
gpart show vtbd0
=>       3  44040315  vtbd0  GPT  (40G) [CORRUPT]
         3       118      1  freebsd-boot  (59K)
       121   2097152      2  freebsd-swap  (1.0G)
   2097273  41943040      3  freebsd-ufs  (20G)
  44040313         5         - free -  (2.5K)
```

If the `state` shows `CORRUPT`, we need to recover it:
```
gpart recover vtbd0
vtbd0 recovered
```

Show again and the `CORRUPT` mark disappeared:
```
gpart show vtbd0
=>       3  83886069  vtbd0  GPT  (40G)
         3       118      1  freebsd-boot  (59K)
       121   2097152      2  freebsd-swap  (1.0G)
   2097273  41943040      3  freebsd-ufs  (20G)
  44040313  39845759         - free -  (19G)
```

Apparently the partition 3 should be resized.

Run resize on partition 3:

```
gpart resize -i 3 -a 4k vtbd0
vtbd0p3 resized
gpart show vtbd0
=>       3  83886069  vtbd0  GPT  (40G)
         3       118      1  freebsd-boot  (59K)
       121   2097152      2  freebsd-swap  (1.0G)
   2097273  81788799      3  freebsd-ufs  (39G)
```

According [FreeBSD Doc](https://www.freebsd.org/doc/handbook/disks-growing.html), `growfs` should be run as below:

```
growfs /dev/vtbd0p3
```

But it doesn't work with below error reported:
```
growfs: /dev/vtbd0p3: Operation not permitted
```

A workaround is to run `service growfs onestart`:
```
service growfs onestart
Growing root partition to fill device
vtbd0 recovering is not needed
vtbd0p3 resized
super-block backups (for fsck_ffs -b #) at:
 42314112, 43596352, 44878592, 46160832, 47443072, ...
 56418752, 57700992, 58983232, 60265472, 61547712, ...
 70523392, 71805632, 73087872, 74370112, 75652352, ...
```

After that I run `shutdown -r now`, the VM hang at:
```
run_interrupt_driven_hooks: still waiting after 60 seconds for xpt_config
run_interrupt_driven_hooks: still waiting after 120 seconds for xpt_config
```

After force restarted it's bootable again.

Looks FreeBSD image is not polished well for cloud using.
