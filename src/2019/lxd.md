# LXD
> Created at <2019-01-03 Thu>

LXD setup, usage and more.

## LXD Setup

So easy to setup LXD!

The existing lxd packages in ubuntu 16.04 are too old to use, which
need to be purged:

```bash
apt-get -qq purge lxd lxd-client
snap install lxd
```

`lxd` is lxd daemon and `lxc` is the lxd client:
```
which lxd #=>
/snap/bin/lxd
which lxc #=>
/snap/bin/lxc
```

## Init

`lxd` provides a sub command =init= to init lxd:

```
lxd init
```

`lxd init` can configure `lxd` run in clustering mode or standalone
mode.

## Snap and Lxd

You can use `snap` to run `lxd`, or the opposite. Inside `lxd` you
can run `docker`.

```
     +----------+       +-----------+       +------------+        +-----------+
     |          |       |           |       |            |        |           |
     |    vm    +------>|    snap   +------>|     lxd    +------->|   docker  |
     |          |       |           |       |            |        |           |
     +----------+       +-----------+       +------+-----+        +-----------+
                              ^                    |
                              |                    |
                              +--------------------+
```

## Run lxd in Snap

By default ubuntu xenial with lxd and lxd client installed. You can
check this by running:

```bash
which lxd                       # => /usr/bin/lxd
which lxc                       # => /usr/bin/lxc
```

The lxd in apt repo is very low. We need to switch to the lxd in snap
store:

```bash
# Uninstall existing lxd in ubuntu repo.
apt-get purge -qq lxd lxd-client
# Install lxd in snap
snap install lxd
# Check lxd and lxc
which lxd                       # => /snap/bin/lxd
lxd --version                   # => 3.3
which lxc                       # => /snap/bin/lxc
lxc --version                   # => 3.3
# Init lxd
lxd init
# Start lxd
snap restart lxd
```

## Launcn Lxd container

Now launch a vm like `lxc` container:

```bash
# List images
snap image list ubuntu:xenial
# Launch a container
lxc launch ubuntu:xenial/amd64
# List containers
lxc list
# exec bash in the container
lxc exec steady-dingo bash
# Now you can do anything in the container
```

## Run snap inside Lxd Container

Snap as a container, can run `lxd` and launch `lxd` container. Inside
the `lxc` container, you can run `snap` container, say to install go
snap:

```bash
lxc exec steady-dingo bash
snap install go --classic
echo 'export GOPATH=/usr/local' >> ~/.bashrc
apt-get install build-essential
cat > ~/.netrc <<EOF
your github toke  n
EOF
go get github.ibm.com/nanjj/ncatd
ncatd --version                 # ncatd version 0.0.1
```

## Run Lxd inside Lxd Container

Run `lxd` inside lxd container nesting:

```bash
lxc config set steady-dingo security.nesting true
lxc exec steady-dingo bash
apt-get purge -qq lxd lxd-client
snap install lxd
lxd init
lxc launch ubuntu:xenial/amd64
```

## Run Docker inside Lxd Container

With `security.nesting true`, run docker:

```bash
apt-get install docker.io
docker run hello-world
```

## Use Lxd Container as a Router

```
lxc launch ubuntu:16.04 router #=>
lxc list #=>
+--------+---------+-----------------------+------+------------+-----------+
|  NAME  |  STATE  |         IPV4          | IPV6 |    TYPE    | SNAPSHOTS |
+--------+---------+-----------------------+------+------------+-----------+
| router | RUNNING |  10.149.11.203 (eth0) |      | PERSISTENT |           |
+--------+---------+-----------------------+------+------------+-----------+
lxc exec router -- bash -i
# configure vpn if needed

# check ip forward setting
sysctl net.ipv4.ip_forward #=>
net.ipv4.ip_forward = 1
# iptables
iptables -t nat -A POSTROUTING -s 10.149.11.0/24 ! -d 10.0.2.0/24 -j MASQUERADE
apt-get update
apt-get install iptables-persistent
```

Now you can use this as a router

## Clustering Mode

  Lxd clustering mode makes lxd run in multiple nodes. Each node lxd
  is running on is a cluster member. The whole set of the cluster
  members is called a cluster.

### New Cluster
```
lxd init
Would you like to use LXD clustering? (yes/no) [default=no]: yes
What name should be used to identify this node in the cluster? [default=hypercube01]:
What IP address or DNS name should be used to reach this node? [default=192.168.0.46]:
Are you joining an existing cluster? (yes/no) [default=no]:
Setup password authentication on the cluster? (yes/no) [default=yes]: yes
Trust password for new clients:
Again:
Do you want to configure a new local storage pool? (yes/no) [default=yes]:
Name of the storage backend to use (btrfs, dir, lvm, zfs) [default=zfs]:
Create a new ZFS pool? (yes/no) [default=yes]:
Would you like to use an existing block device? (yes/no) [default=no]:
Size in GB of the new loop device (1GB minimum) [default=31GB]: 128GB
Do you want to configure a new remote storage pool? (yes/no) [default=no]:
Would you like to connect to a MAAS server? (yes/no) [default=no]:
Would you like to configure LXD to use an existing br...? (yes/no) [default=no]:
Would you like to create a new Fan overlay network? (yes/no) [default=yes]:
What subnet should be used as the Fan underlay? [default=auto]:
Would you like stale cached images to be updated automatically? (yes/no) [default=yes]
Would you like a YAML "lxd init" preseed to be printed? (yes/no) [default=no]:
```
### Join Cluster
```
lxd init
Would you like to use LXD clustering? (yes/no) [default=no]: yes
What name should be used to identify this node in the cluster? [default=hypercube06]:
What IP address or DNS name should be used to reach this node? [default=192.168.0.21]:
Are you joining an existing cluster? (yes/no) [default=no]: yes
IP address or FQDN of an existing cluster node: 192.168.0.30
Cluster fingerprint: 6ab5b519ffbb309cb38b73657299dd9b0b8c6f2bd5b3....
You can validate this fingerprint by running "lxc info" locally on an existing node.
Is this the correct fingerprint? (yes/no) [default=no]: yes
Cluster trust password:
All existing data is lost when joining a cluster, continue? (yes/no) [default=no] yes
Choose "size" property for storage pool "local": 128GB
Choose "source" property for storage pool "local":
Choose "zfs.pool_name" property for storage pool "local":
Would you like a YAML "lxd init" preseed to be printed? (yes/no) [default=no]:
```
### List Cluster
```
lxc cluster list

+-------------+---------------------------+----------+--------+-------------------+
|    NAME     |            URL            | DATABASE | STATE  |      MESSAGE      |
+-------------+---------------------------+----------+--------+-------------------+
| hypercube01 | https://192.168.0.46:8443 | YES      | ONLINE | fully operational |
+-------------+---------------------------+----------+--------+-------------------+
| hypercube02 | https://192.168.0.47:8443 | YES      | ONLINE | fully operational |
+-------------+---------------------------+----------+--------+-------------------+
| hypercube03 | https://192.168.0.48:8443 | YES      | ONLINE | fully operational |
+-------------+---------------------------+----------+--------+-------------------+
| hypercube04 | https://192.168.0.51:8443 | NO       | ONLINE | fully operational |
+-------------+---------------------------+----------+--------+-------------------+
| hypercube05 | https://192.168.0.30:8443 | NO       | ONLINE | fully operational |
+-------------+---------------------------+----------+--------+-------------------+
| hypercube06 | https://192.168.0.21:8443 | NO       | ONLINE | fully operational |
+-------------+---------------------------+----------+--------+-------------------+
| hypercube07 | https://192.168.0.26:8443 | NO       | ONLINE | fully operational |
+-------------+---------------------------+----------+--------+-------------------+
```

It's a 7 nodes lxd cluster with 3 database nodes and 7 service nodes.

## Database
  [Lxd
  database](https://github.com/lxc/lxd/blob/master/doc/database.md) is
  based on [distributed
  sqlite](https://github.com/CanonicalLtd/dqlite), which removed lxd's
  dependency on traditional database like postgres or mysql. [Fan
  network](https://wiki.ubuntu.com/FanNetworking) introduced lxd a
  simple and fast container network.

### Global and Local
   For each lxd node, there are two type databases: global and
   local. Data in global database is shared by each lxd cluster
   members, while data in local database can only be accessed in local
   node, does not impact others.
### Lxd sql

```
lxd sql <local|global> <query>  [flags]
```

### Global Schemas
For example, to list global schemas:
```
lxd sql global .schema #=>
PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE schema (
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    version    INTEGER NOT NULL,
    updated_at DATETIME NOT NULL,
    UNIQUE (version)
);
INSERT INTO schema VALUES(1,13,1546788241);
CREATE TABLE "containers" (
    id INTEGER primary key AUTOINCREMENT NOT NULL,
    node_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    architecture INTEGER NOT NULL,
    type INTEGER NOT NULL,
    ephemeral INTEGER NOT NULL DEFAULT 0,
    creation_date DATETIME NOT NULL DEFAULT 0,
    stateful INTEGER NOT NULL DEFAULT 0,
    last_use_date DATETIME,
    description TEXT,
    project_id INTEGER NOT NULL,
    UNIQUE (project_id, name),
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);
CREATE TABLE "images" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    fingerprint TEXT NOT NULL,
    filename TEXT NOT NULL,
    size INTEGER NOT NULL,
    public INTEGER NOT NULL DEFAULT 0,
    architecture INTEGER NOT NULL,
    creation_date DATETIME,
    expiry_date DATETIME,
    upload_date DATETIME NOT NULL,
    cached INTEGER NOT NULL DEFAULT 0,
    last_use_date DATETIME,
    auto_update INTEGER NOT NULL DEFAULT 0,
    project_id INTEGER NOT NULL,
    UNIQUE (project_id, fingerprint),
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);
CREATE TABLE "images_aliases" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    image_id INTEGER NOT NULL,
    description TEXT,
    project_id INTEGER NOT NULL,
    UNIQUE (project_id, name),
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);
CREATE TABLE "operations" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    uuid TEXT NOT NULL,
    node_id TEXT NOT NULL,
    type INTEGER NOT NULL DEFAULT 0,
    project_id INTEGER,
    UNIQUE (uuid),
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);
CREATE TABLE "profiles" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    project_id INTEGER NOT NULL,
    UNIQUE (project_id, name),
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);
CREATE TABLE "storage_volumes" (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    storage_pool_id INTEGER NOT NULL,
    node_id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    description TEXT,
    snapshot INTEGER NOT NULL DEFAULT 0,
    project_id INTEGER NOT NULL,
    UNIQUE (storage_pool_id, node_id, project_id, name, type),
    FOREIGN KEY (storage_pool_id) REFERENCES storage_pools (id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE
);
CREATE TABLE certificates (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    fingerprint TEXT NOT NULL,
    type INTEGER NOT NULL,
    name TEXT NOT NULL,
    certificate TEXT NOT NULL,
    UNIQUE (fingerprint)
);
CREATE TABLE config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    UNIQUE (key)
);
CREATE TABLE containers_backups (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    container_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    creation_date DATETIME,
    expiry_date DATETIME,
    container_only INTEGER NOT NULL default 0,
    optimized_storage INTEGER NOT NULL default 0,
    FOREIGN KEY (container_id) REFERENCES containers (id) ON DELETE CASCADE,
    UNIQUE (container_id, name)
);
CREATE TABLE containers_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    container_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    FOREIGN KEY (container_id) REFERENCES containers (id) ON DELETE CASCADE,
    UNIQUE (container_id, key)
);
CREATE TABLE containers_devices (
    id INTEGER primary key AUTOINCREMENT NOT NULL,
    container_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type INTEGER NOT NULL default 0,
    FOREIGN KEY (container_id) REFERENCES containers (id) ON DELETE CASCADE,
    UNIQUE (container_id, name)
);
CREATE TABLE containers_devices_config (
    id INTEGER primary key AUTOINCREMENT NOT NULL,
    container_device_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    FOREIGN KEY (container_device_id) REFERENCES containers_devices (id) \
     ON DELETE CASCADE,
    UNIQUE (container_device_id, key)
);
CREATE TABLE containers_profiles (
    id INTEGER primary key AUTOINCREMENT NOT NULL,
    container_id INTEGER NOT NULL,
    profile_id INTEGER NOT NULL,
    apply_order INTEGER NOT NULL default 0,
    UNIQUE (container_id, profile_id),
    FOREIGN KEY (container_id) REFERENCES containers(id) ON DELETE CASCADE,
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);
CREATE TABLE images_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    image_id INTEGER NOT NULL,
    node_id INTEGER NOT NULL,
    UNIQUE (image_id, node_id),
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE
);
CREATE TABLE images_properties (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    image_id INTEGER NOT NULL,
    type INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE
);
CREATE TABLE images_source (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    image_id INTEGER NOT NULL,
    server TEXT NOT NULL,
    protocol INTEGER NOT NULL,
    certificate TEXT NOT NULL,
    alias TEXT NOT NULL,
    FOREIGN KEY (image_id) REFERENCES images (id) ON DELETE CASCADE
);
CREATE TABLE networks (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    state INTEGER NOT NULL DEFAULT 0,
    UNIQUE (name)
);
CREATE TABLE networks_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    network_id INTEGER NOT NULL,
    node_id INTEGER,
    key TEXT NOT NULL,
    value TEXT,
    UNIQUE (network_id, node_id, key),
    FOREIGN KEY (network_id) REFERENCES networks (id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE
);
CREATE TABLE networks_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    network_id INTEGER NOT NULL,
    node_id INTEGER NOT NULL,
    UNIQUE (network_id, node_id),
    FOREIGN KEY (network_id) REFERENCES networks (id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE
);
CREATE TABLE nodes (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT DEFAULT '',
    address TEXT NOT NULL,
    schema INTEGER NOT NULL,
    api_extensions INTEGER NOT NULL,
    heartbeat DATETIME DEFAULT CURRENT_TIMESTAMP,
    pending INTEGER NOT NULL DEFAULT 0,
    UNIQUE (name),
    UNIQUE (address)
);
CREATE TABLE profiles_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    profile_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    UNIQUE (profile_id, key),
    FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);
CREATE TABLE profiles_devices (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    profile_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    type INTEGER NOT NULL default 0,
    UNIQUE (profile_id, name),
    FOREIGN KEY (profile_id) REFERENCES profiles (id) ON DELETE CASCADE
);
CREATE TABLE profiles_devices_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    profile_device_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    UNIQUE (profile_device_id, key),
    FOREIGN KEY (profile_device_id) REFERENCES profiles_devices (id) ON DELETE CASCADE
);
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    UNIQUE (name)
);
CREATE TABLE projects_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    project_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    UNIQUE (project_id, key)
);
CREATE TABLE storage_pools (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name TEXT NOT NULL,
    driver TEXT NOT NULL,
    description TEXT,
    state INTEGER NOT NULL DEFAULT 0,
    UNIQUE (name)
);
CREATE TABLE storage_pools_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    storage_pool_id INTEGER NOT NULL,
    node_id INTEGER,
    key TEXT NOT NULL,
    value TEXT,
    UNIQUE (storage_pool_id, node_id, key),
    FOREIGN KEY (storage_pool_id) REFERENCES storage_pools (id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE
);
CREATE TABLE storage_pools_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    storage_pool_id INTEGER NOT NULL,
    node_id INTEGER NOT NULL,
    UNIQUE (storage_pool_id, node_id),
    FOREIGN KEY (storage_pool_id) REFERENCES storage_pools (id) ON DELETE CASCADE,
    FOREIGN KEY (node_id) REFERENCES nodes (id) ON DELETE CASCADE
);
CREATE TABLE storage_volumes_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    storage_volume_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT,
    UNIQUE (storage_volume_id, key),
    FOREIGN KEY (storage_volume_id) REFERENCES storage_volumes (id) ON DELETE CASCADE
);
COMMIT;
```
### Local Schemas
To list local database schemas:

```
lxd sql local .schema #=>
PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE schema (
    id         INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    version    INTEGER NOT NULL,
    updated_at DATETIME NOT NULL,
    UNIQUE (version)
);
INSERT INTO schema VALUES(1,38,1546788240);
CREATE TABLE config (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    key VARCHAR(255) NOT NULL,
    value TEXT,
    UNIQUE (key)
);
CREATE TABLE patches (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    applied_at DATETIME NOT NULL,
    UNIQUE (name)
);
CREATE TABLE raft_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    address TEXT NOT NULL,
    UNIQUE (address)
);
COMMIT;
```

### Raft Nodes
```
 lxd sql local 'select * from raft_nodes' #=>
 +----+-------------------+
 | id |      address      |
 +----+-------------------+
 | 1  | 192.168.0.46:8443 |
 | 2  | 192.168.0.47:8443 |
 | 3  | 192.168.0.48:8443 |
 +----+-------------------+
```
[Distributed Sqlite](https://github.com/CanonicalLtd/dqlite) is using raft to sync sqlite db logs.

### Cluster nodes
```
lxd sql global 'select * from nodes'
+----+-------------+-------------------+--------+----------------+
| id |    name     |      address      | schema | api_extensions |
+----+-------------+-------------------+--------+----------------+
| 1  | hypercube01 | 192.168.0.46:8443 | 13     | 115            |
| 2  | hypercube02 | 192.168.0.47:8443 | 13     | 115            |
| 3  | hypercube03 | 192.168.0.48:8443 | 13     | 115            |
| 4  | hypercube04 | 192.168.0.51:8443 | 13     | 115            |
| 5  | hypercube05 | 192.168.0.30:8443 | 13     | 115            |
| 6  | hypercube06 | 192.168.0.21:8443 | 13     | 115            |
| 7  | hypercube07 | 192.168.0.26:8443 | 13     | 115            |
+----+-------------+-------------------+--------+----------------+
```

### Containers
```
lxd sql global 'select * from containers' #=>
+----+---------+----------------+------------+
| id | node_id |      name      | project_id |
+----+---------+----------------+------------+
| 20 | 2       | grafana        | 1          |
| 30 | 3       | go             | 1          |
| 32 | 1       | guyujie        | 1          |
| 33 | 4       | nginx          | 1          |
| 36 | 1       | lxdui01        | 1          |
| 38 | 1       | crack-mako     | 1          |
| 39 | 5       | lxdui02        | 1          |
| 42 | 6       | fluent-hamster | 1          |
| 43 | 1       | b2             | 1          |
| 44 | 2       | b3             | 1          |
+----+---------+----------------+------------+
```
To select the node with least containers:
```
lxd sql global \
    'select node_id, count(node_id) as node_count from containers
     group by node_id order by node_count'
```
## Network

Lxd can be configured to use [Ubuntu Fan
Network](https://wiki.ubuntu.com/FanNetworking).

Say 2 containers A and B:

| Container | IP            | Hyper       | Hyper IP        |
|-----------|---------------|-------------|-----------------|
| A         | 240.0.46.14/8 | hypercube01 | 192.168.0.46/16 |
| B         | 240.0.47.99/8 | hypercube02 | 192.168.0.47/16 |

Now ping B on A:

```
ping 240.0.47.99 #=>
ARP, Request who-has 240.0.47.99 tell 240.0.46.14, length 28
```

On hypercube01 the arp request being forwarded to hypercube02:
```
17:07:29.650323 IP 192.168.0.46.53730 > 192.168.0.47.8472
ARP, Request who-has 240.0.47.99 tell 240.0.46.14, length 28
```
## Operations

### Launch Container
`lxc launch b --debug` will do:
1. Get version
    ```
    DBUG Connecting to a remote LXD over HTTPs
    DBUG Sending request to LXD method=GET url=https://192.168.0.48:8443/1.0 etag=
    DBUG Got response struct from LXD
    DBUG
            {
                    "config": {
                            "cluster.https_address": "192.168.0.48:8443",
                            "core.https_address": "192.168.0.48:8443",
                            "core.trust_password": true
                    },
                    "api_extensions": [...],
                    "api_status": "stable",
                    "api_version": "1.0",
                    "auth": "trusted",
                    "public": false,
                    "auth_methods": [
                            "tls"
                    ],
                    "environment": {
                            "addresses": [
                                    "192.168.0.48:8443"
                            ],
                            "architectures": [
                                    "x86_64",
                                    "i686"
                            ],
                            "certificate": "...",
                            "certificate_fingerprint": "...",
                            "driver": "lxc",
                            "driver_version": "3.1.0",
                            "kernel": "Linux",
                            "kernel_architecture": "x86_64",
                            "kernel_version": "4.15.0-43-generic",
                            "server": "lxd",
                            "server_pid": 32645,
                            "server_version": "3.9",
                            "storage": "zfs",
                            "storage_version": "0.7.5-1ubuntu16.4",
                            "server_clustered": true,
                            "server_name": "hypercube03",
                            "project": "default"
                    }
            }
    ```
2. Get image
    ```
    Creating the container
    DBUG ... method=GET url=https://192.168.0.48:8443/1.0/images/aliases/b etag=
    DBUG Got response struct from LXD
    DBUG
            {
                    "description": "",
                    "target": "dcbc8e3e5c2ed9fb21c3d0659a0eee004bd...
                    "name": "b"
            }
    DBUG ... method=GET url=https://192.168.0.48:8443/1.0/images/dcbc8e... etag=
    DBUG Got response struct from LXD
    DBUG
            {
                    "auto_update": true,
                    "properties": {
                            "architecture": "amd64",
                            "description": "ubuntu 18.04 LTS amd64 (r...
                            "label": "release",
                            "os": "ubuntu",
                            "release": "bionic",
                            "serial": "20190114",
                            "version": "18.04"
                    },
                    "public": false,
                    "aliases": [
                            {
                                    "name": "b",
                                    "description": ""
                            }
                    ],
                    "architecture": "x86_64",
                    "cached": true,
                    "filename": "ubuntu-18.04-server-cloudimg-amd64-lxd.tar.xz",
                    "fingerprint": "dcbc8e3e5c2ed9fb21c3d0...",
                    "size": 183468820,
                    "update_source": {
                            "alias": "b",
                            "certificate": "",
                            "protocol": "simplestreams",
                            "server": "https://cloud-images.ubuntu.com/releases"
                    },
                    "created_at": "2019-01-14T00:00:00Z",
                    "expires_at": "2023-04-26T00:00:00Z",
                    "last_used_at": "2019-01-18T08:22:28.5476208Z",
                    "uploaded_at": "2019-01-15T00:36:47.651093161Z"
            }
     ```
3. Create Container Operation
    ```
    DBUG Connected to the websocket
    DBUG ... method=POST url=.../1.0/containers etag=
    DBUG
            {
                    "architecture": "",
                    "config": {},
                    "devices": {},
                    "ephemeral": false,
                    "profiles": null,
                    "stateful": false,
                    "description": "",
                    "name": "",
                    "source": {
                            "type": "image",
                            "certificate": "",
                            "fingerprint": "dcbc8e3e5c2ed9fb21c3d065..."
                    },
                    "instance_type": ""
            }
    DBUG Got operation from LXD
    DBUG
            {
                    "id": "1de45646-d209-413f-827a-ef7921c3c7f8",
                    "class": "task",
                    "description": "Creating container",
                    "created_at": "2019-01-22T06:13:23.360302136Z",
                    "updated_at": "2019-01-22T06:13:23.360302136Z",
                    "status": "Running",
                    "status_code": 103,
                    "resources": {
                            "containers": [
                                    "/1.0/containers/fluent-hamster"
                            ]
                    },
                    "metadata": null,
                    "may_cancel": false,
                    "err": ""
            }
    ```
4. Wait Create Operation Done
    ```
    DBUG ... method=GET url=.../1.0/operations/1de45646-... etag=
    DBUG Got response struct from LXD
    DBUG
            {
                    "id": "1de45646-d209-413f-827a-ef7921c3c7f8",
                    "class": "task",
                    "description": "Creating container",
                    "created_at": "2019-01-22T06:13:23.360302136Z",
                    "updated_at": "2019-01-22T06:13:23.360302136Z",
                    "status": "Running",
                    "status_code": 103,
                    "resources": {
                            "containers": [
                                    "/1.0/containers/fluent-hamster"
                            ]
                    },
                    "metadata": null,
                    "may_cancel": false,
                    "err": ""
            }
    Container name is: fluent-hamster
    ```
5. Get container
    ```
    DBUG[01-22|14:13:37] ... method=GET url=.../1.0/containers/fluent-hamster etag=
    DBUG[01-22|14:13:39] Got response struct from LXD
    DBUG[01-22|14:13:39]
            {
                    "architecture": "x86_64",
                    "config": {
                            "image.architecture": "amd64",
                            "image.description": "ubuntu 18.04 LTS 
                            "image.label": "release",
                            "image.os": "ubuntu",
                            "image.release": "bionic",
                            "image.serial": "20190114",
                            "image.version": "18.04",
                            "volatile.apply_template": "create",
                            "volatile.base_image": "dcbc8e3e5c2ed9fb21c3....",
                            "volatile.eth0.hwaddr": "00:16:3e:e3:bf:17",
                            "volatile.idmap.base": "0",
                            "volatile.idmap.next": "[{\"Isuid...
                            "volatile.last_state.idmap": "[{\...
                    },
                    "devices": {},
                    "ephemeral": false,
                    "profiles": [
                            "default"
                    ],
                    "stateful": false,
                    "description": "",
                    "created_at": "2019-01-22T06:13:29.053538619Z",
                    "expanded_config": {
                            "image.architecture": "amd64",
                            "image.description": "ubuntu 18.04 LTS 
                            "image.label": "release",
                            "image.os": "ubuntu",
                            "image.release": "bionic",
                            "image.serial": "20190114",
                            "image.version": "18.04",
                            "volatile.apply_template": "create",
                            "volatile.base_image": "dcbc8e3e5c2ed9fb21...
                            "volatile.eth0.hwaddr": "00:16:3e:e3:bf:17",
                            "volatile.idmap.base": "0",
                            "volatile.idmap.next": "[{\"Isuid\":true,\"Isgid\...]",
                            "volatile.last_state.idmap": "[{\"Isuid...]"
                    },
                    "expanded_devices": {
                            "eth0": {
                                    "name": "eth0",
                                    "nictype": "bridged",
                                    "parent": "lxdfan0",
                                    "type": "nic"
                            },
                            "root": {
                                    "path": "/",
                                    "pool": "local",
                                    "type": "disk"
                            }
                    },
                    "name": "fluent-hamster",
                    "status": "Stopped",
                    "status_code": 102,
                    "last_used_at": "1970-01-01T00:00:00Z",
                    "location": "hypercube06"
            }
    ```
6. Start Container Operation
    ```
    Starting fluent-hamster
    DBUG ... method=PUT url=.../1.0/containers/fluent-hamster/state etag=
    DBUG
            {
                    "action": "start",
                    "timeout": -1,
                    "force": false,
                    "stateful": false
            }
    DBUG Got operation from LXD
    DBUG
            {
                    "id": "46746a23-5873-4755-a0ad-27385370aa39",
                    "class": "task",
                    "description": "Starting container",
                    "created_at": "2019-01-22T06:13:40.232324373Z",
                    "updated_at": "2019-01-22T06:13:40.232324373Z",
                    "status": "Running",
                    "status_code": 103,
                    "resources": {
                            "containers": [
                                    "/1.0/containers/fluent-hamster"
                            ]
                    },
                    "metadata": null,
                    "may_cancel": false,
                    "err": ""
            }
    ```
7. Start Operation Done
    ```
    DBUG[01-22|14:13:40] ... method=GET url=.../1.0/operations/...-27385370aa39 etag=
    DBUG[01-22|14:13:42] Got response struct from LXD
    DBUG[01-22|14:13:42]
            {
                    "id": "46746a23-5873-4755-a0ad-27385370aa39",
                    "class": "task",
                    "description": "Starting container",
                    "created_at": "2019-01-22T06:13:40.232324373Z",
                    "updated_at": "2019-01-22T06:13:40.232324373Z",
                    "status": "Success",
                    "status_code": 200,
                    "resources": {
                            "containers": [
                                    "/1.0/containers/fluent-hamster"
                            ]
                    },
                    "metadata": null,
                    "may_cancel": false,
                    "err": ""
            }
    ```
## Ansible

### Lxd connection
Ansible has a `lxd` connection, which can be used to manage lxd
containers.

```
[lxdui]
lxdui01 ansible_host=lxdui01
lxdui02 ansible_host=lxdui02
[lxdui:vars]
ansible_user=root
ansible_connection=lxd
```

To ping:

```bash
ansible -m ping lxdui -vvvvv #=>

ansible 2.7.6
<lxdui01> ESTABLISH LXD CONNECTION FOR USER: root
<lxdui01> EXEC /bin/sh -c 'echo ~root && sleep 0'
<lxdui02> ESTABLISH LXD CONNECTION FOR USER: root
lxdui02 | SUCCESS => {
    "changed": false,
    "invocation": {
        "module_args": {
            "data": "pong"
        }
    },
    "ping": "pong"
}
lxdui01 | SUCCESS => {
    "changed": false,
    "invocation": {
        "module_args": {
            "data": "pong"
        }
    },
    "ping": "pong"
}
META: ran handlers
META: ran handlers
```

It can work without ssh:
```
ansible (client) -> lxc (client) -> lxd api(server) -> lxd containers
```
