## 0

[cloud-init]是GPLv3和Apache2.0双协议发布的自由软件, 于2009年，最初
[Canonical]为其服务器版操作系统Ubuntu运行于亚马逊EC2而设计。
[cloud-init]使得Ubuntu灵活运行于云端，因此Canonical就把自己的操作系统
叫云原生操作系统。

[cloud-init]连接云平台和操作系统。

## 1

[cloud-init]所支持的云平台叫数据源（Data Source），也就是初始化操作系
统所用数据的来源。如果把数据源看作一个服务，那就又提metadata service的
概念了。

以下列出[cloud-init]所支持的数据源：

| 数据源         | 创建日期 | 贡献者                                   |
|----------------|----------|------------------------------------------|
| NoCloud        | 2009     | [Canonical], HP                          |
| EC2            | 2009     | [Canonical], HP                          |
| AltCloud       | 2009     | [Canonical], HP, Yahoo!                  |
| OVF            | 2011     | [Canonical], HP                          |
| None           | 2012     | Yahoo!                                   |
| [MAAS]         | 2012     | [Canonical]                              |
| ConfigDriver   | 2012     | [Canonical]                              |
| CloudStack     | 2012     | [Canonical], Cosmin Luta                 |
| [OpenNebula]   | 2012     | [Canonical], Yahoo!, Cerit-Science Cloud |
| [SmartOS]      | 2013     | [Canonical]                              |
| [OpenStack]    | 2014     | Yahoo!                                   |
| [CloudSigma]   | 2014     | [CloudSigma]                             |
| [DigitalOcean] | 2014     | [DigitalOcean]                           |
| [GCE]          | 2014     | [GCE]                                    |
| [Azure]        | 2013     | Canonical                                |
| [Bigstep]      | 2015     | [Bigstep]                                |
| [Aliyun]       | 2016     | [Aliyun]                                 |
| [Scaleway]     | 2017     | [Scaleway]                               |
| [Hetzner]      | 2018     | [Hetzner]                                |
| [IBMCloud]     | 2018     | [IBMCloud]                               |
| [OracleCloud]  | 2018     | [Canonical]                              |
| [RbxCloud]     | 2018     | [RbxCloud]                               |
| [VMware]       | 2018     | [VMware]                                 |
| [Exoscale]     | 2019     | [Exoscale]                               |
| [UpCloud]      | 2021     | [UpCloud]                                |
| [LXD]          | 2021     | [Canonical]                              |
| [Vultr]        | 2021     | [Vultr]                                  |
| [NWCS]         | 2022     | [NWCS]                                   |


[Bigstep]: https://docs.bigstep.com/en/latest/
[Aliyun]: https://aliyun.com/
[Canonical]: https://canonical.com/
[Azure]: https://azure.microsoft.com/
[CloudSigma]: https://cloudsigma.com
[CloudStack]: https://cloudstack.apache.org/
[DigitalOcean]: https://digitalocean.com
[Exoscale]: https://exoscale.com
[GCE]: https://cloud.google.com
[IBMCloud]: https://ibm.com/cloud
[LXD]: https://linuxcontainers.org/lxd/
[MAAS]: https://maas.io/
[OpenNebula]: https://opennebula.io/
[OpenStack]: https://openstack.org/
[OracleCloud]: https://www.oracle.com/cloud/
[RbxCloud]: https://rootbox.com
[Scaleway]: https://scaleway.com
[SmartOS]: https://wiki.smartos.org/
[UpCloud]: https://upcloud.com
[Vultr]: https://vultr.com

## 2

其中EC2最早。著名IP `169.254.169.254`就来自此。基于EC2的变种有：
1. aliyun,
2. aws,
3. brightbox,
4. e24cloud,
5. outscale,
6. zstack.

其中阿里云后来加了自己的数据源：

```python
class DataSourceAliYun(EC2.DataSourceEc2):

    dsname = "AliYun"
    metadata_urls = ["http://100.100.100.200"]

    # The minimum supported metadata_version from the ec2 metadata apis
    min_metadata_version = "2016-01-01"
    //
```

## 3

以上Metadata来自网络，可以叫​`Metadata Service`。

## 4
[LXD Agent]是[LXD]运行在[LXD]所起虚机里的Agent。

```

                       +--------------HOST-----------------+
                       |                +-------VM-------+ |
+--------------+       |  +-----+       |  +-----------+ | |
| lxc shell vm +--http-+->| LXD +-vsock-+->| LXD Agent | | |
+--------------+       |  +-----+       |  +-----------+ | |
                       |                +----------------+ |
                       +-----------------------------------+
```
须知：
1. [LXD]运行在Host机器上，
2. [LXD Agent]运行在虚机上，
3. [LXD Agent]通过管控面[LXD]暴露给用户的服务：
   1. `exec` - 执行虚机里任意命令，
   2. `sftp` - 上传或下载虚机里任意文件。

[LXD Agent]监听[vsock]。[LXD]通过[vsock]与[LXD Agent]通讯。

[LXD Agent]: https://github.com/lxc/lxd/tree/master/lxd-agent
[LXD]: https://github.com/lxc/lxd
[vsock]: https://www.man7.org/linux/man-pages/man7/vsock.7.html

执行命令和上传下载文件是容器技术的标配。[LXD]把系统容器管理做到了极致，
逐渐拓展到虚机管理。由此[LXD]兼具系统容器管理和虚机管理。在跨界的过程
中（从API角度看并未跨界），`exec`和`sftp`也从系统容器带到了虚机。

## 5

[LXD]做系统容器管理的时候，系统容器支持[cloud-init]。一般容器里应用配
置都固化在容器镜像里，比如Docker容器镜像。容器启动应用运行，并不需要沉
重的[cloud-init]配置。但[LXD]支持的是系统容器。系统容器要和虚机越像越
好。系统容器里的配置要和虚机里的配置相同。因此[LXD]就支持了
[cloud-init]。

那如何支持呢？

两种：
1. NoCloud，
2. LXD

其实现是显然的，这里不做展开。

[cloud-init]: https://cloudinit.readthedocs.io/en/latest/index.html


