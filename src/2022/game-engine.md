<div style="text-align: right"><code>2022年05月08日 星期日 下午</code></div>

## 0

游戏引擎要构建一个虚拟现实（Virtual Reality）世界。此世界是虚拟的，万
物以数字形式存在；此世界是现实的，万物拟现实以存在。 现实的要素：
1. 时间，
2. 空间，
3. 声音。

花香，美味暂时不管。

## 1

虚拟现实以1/30秒每帧的速度滴答前行。在此1/30秒，游戏引擎要为下1/30秒准备好：
1. 万物的运动，生命值，组合与拆分，以及相互作用，
2. 万物的表面纹理，反射这是，
3. 环境渲染，
4. 万物的声音渲染。

虚拟现实两个状态：
1. 构建态，
2. 运行时。

## 2

英伟达Omniverse解决虚拟现实的构建问题：
1. 基于USD框架构建多人，多工具协作的载体，
2. 构建Nucleus平台提供以数据为中心的协作，
3. 提供虚拟物交流的平台。

云渲染，渲染的操作发生在云端。云渲染管理云端GPU资源：
1. 多GPU协作，
2. 多渲染调度。


## 3

BBR (Bottleneck Bandwidth and Round-trip propagation time)是一套拥塞控
制算法，适合在一定丢包的弱网环境下使用。

BBR进入了Linux Kernel 4.9。之前，TCP CUBIC（以及更早的拥塞控制算法），
是基于丢包的拥塞控制，一有丢包发生，就减小发送窗口，拥塞控制和可靠传输
是耦合的。 BBR解耦了二者：

```c
static void tcp_cong_control(struct sock *sk, u32 ack, u32 acked_sacked,
			     int flag, const struct rate_sample *rs)
{
	const struct inet_connection_sock *icsk = inet_csk(sk);

	if (icsk->icsk_ca_ops->cong_control) { // BBR 接手
		icsk->icsk_ca_ops->cong_control(sk, rs);
		return;
	}

	if (tcp_in_cwnd_reduction(sk)) {
		/* Reduce cwnd if state mandates */
		tcp_cwnd_reduction(sk, acked_sacked, rs->losses, flag);
	} else if (tcp_may_raise_cwnd(sk, flag)) {
		/* Advance cwnd if state allows */
		tcp_cong_avoid(sk, ack, acked_sacked);
	}
	tcp_update_pacing_rate(sk);
}
```

BBR只需在服务端启用，客户端无需改动：
```sh
cat > /etc/sysctl.conf.d/bbr.conf <<EOF
net.core.default_qdisc=fq
net.ipv4.tcp_congestion_control=bbr
EOF

sysctl -p
modprobe tcp_bbr
```

`tc`工具可以用来模拟传输时延和丢包率：

```sh
tc qdisc add dev eth0 root netem loss 1% latency 25ms
```




