# Tips

`<2017-12-20 Mon>`

[cat]: https://nmap.org/ncat/
[ncat guide]: https://nmap.org/ncat/guide/index.html
[nmap]: https://nmap.org/

Some tips about linux kernel, ssh, qemu, ncat and more.

## Linux Kernel

Kernel building takes too long time. We can speed up the process via:
1. Enable `tmpfs` to build in memory,
2. Enable `ccache` to avoid unnecessary recompile,
3. Enable `distcc` to build on multiple machines.

### Mount tmpfs

```
   mkdir src .ccache
   mount -t tmpfs -o size=30G,mode=0755 tmpfs src
   mount -t tmpfs -o size=10G,mode=0755 tmpfs .ccache
```
Speed up kernel building via `ccache` and `distcc`:

### Enable ccache

```
apt-get install ccache
```

### Enable distcc
```
apt-get install distcc
echo 10.113.111.160 10.113.111.187 >> /etc/distcc/hosts
```

On `10.113.111.160` and  `10.113.111.187`:

```
apt-get install distcc
cat > /etc/default/distcc <<EOF
STARTDISTCC="true"
ALLOWEDNETS="10.113.111.152"    # master build ip
LISTENER="10.113.111.187"       # local ip
EOF
systemctl restart distcc
```

### Build

Build ubuntu kernel `Ubuntu-4.4.0-102.125` as below:

```bash
cd src
git clone git://kernel.ubuntu.com/ubuntu/ubuntu-xenial
cd ubuntu-xenial
git checkout Ubuntu-4.4.0-102.125
sed 's/CONFIG_VIRTIO_NET=y/CONFIG_VIRTIO_NET=m/g'
export PATH=/usr/lib/ccache:$PATH
export CCACHE_PREFIX="distcc"
fakeroot debian/rules clean
fakeroot debian/rules binary
```

It takes around 9 minutes on my machine.

## Ssh

[Using SSH Multiplexing](https://blog.scottlowe.org/2015/12/11/using-ssh-multiplexing/):

```
ControlMaster auto
ControlPath ~/.ssh/sockets/%r@%h-%p
ControlPersist 600
```

## QEMU

1. which nic models qemu supports?
   ```
   qemu-system-x86_64 -net nic,model=?  #=>
   qemu: Supported NIC models: ne2k_pci,i82551,i82557b,i82559er,...
   ```
2. Quit qemu `C-A C`.

## Ncat
[Ncat] is a general-purpose command-line tool for reading,
writing,redirecting, and encrypting data across a network. It is along
with [nmap], but not so famous.

### Install

To install ncat just install nmap, Ubuntu:
```
apt-get install nmap
```

Mac:

```
brew install nmap
```

### Use Ncat to Transfer Files

1. Transfer Single File

   ```
   # on machine A
   ncat -p 80 -l --send-only < file
   # on machine B
   ncat <machine A ip> 80 --recv-only > file
   ```

2. Transfer Multiple Files

	```
    # on machine A
    for file in *.deb; do
        echo $file | ncat -p 80 -l --send-only
        ncat -p 80 -l --send-only < $file
    done
    # on machine B
    while true; do
        file=$(ncat <machine A ip> 80 --recv-only) || break
        echo $file
        ncat <machine A ip> 80 --recv-only > $file
    done
    ```
### Http Proxy

Ncat can act as a http proxy server, which

Run:

```
ncat -l -p 8888 --proxy-type http --allow 127.0.0.1
```

Test it(on the machine running ncat):

```
export https_proxy=http://127.0.0.1:8888
curl https://www.google.com
```

Jump over the wall via ssh:

```
ssh -L 127.0.0.1:8888:127.0.0.1:8888 <your machine outside the wall>
```

Test it again(on the machine running `ssh -L`):

```
export http_proxy=http://127.0.0.1:8888
curl http://www.google.com
```

Http request will go through your local port 8888, encrypted and jump
over the wall, go to your target machine local port 8888, via ncat to
access the world. It's safe, simple, stable and fast.

### Proxy Ssh

Ncat http proxy mode support =CONNECT= method also. So it can be used
as a ssh proxy:

```
ssh -o "ProxyCommand=ncat --proxy 127.0.0.1:8888 %h %p" user@host
```

Ssh traffic will go to your local port 8888, ssh will forward it to
your jumpbox, which ncat is running there, to visit all the machines
running behind the jumpbox. In this way, you can keep your ssh keys in
your local, no need to copy anywhere in order to access your machines.

You can put it to your `~/.ssh/config`:

```
Host: host
Hostname: host
ProxyCommand: ncat --proxy 127.0.0.1:8888 %h %p
```

### Proxy Ansible

You can run ansible with below configuration in your inventory:

```
[all:vars]
ansible_ssh_common_args='-o ProxyCommand="ncat --proxy 127.0.0.1:8888 %h %p"'
```

## Permission Bits

1. Read, write and execute permissions

   | Permission | Octal | Description             |
   |------------|-------|-------------------------|
   | rwx        |     7 | Read, write and execute |
   | rw-        |     6 | Read and write          |
   | r-x        |     5 | Read and execute        |
   | r--        |     4 | Read                    |
   | -wx        |     3 | Write and execute       |
   | -w-        |     2 | Write                   |
   | --x        |     1 | Execute                 |
   | ---        |     0 | no permissions          |



2. User, group and others

   | Permission | Octal | Field      |
   |------------|-------|------------|
   | rwx------  |  0700 | User       |
   | ---rwx---  |  0070 | Group      |
   | ------rwx  |  0007 | All Others |
