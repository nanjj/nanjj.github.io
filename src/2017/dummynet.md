# Dummynet Usage on Mac

`<2017-10-26 Thu>`

[PfOnMacOSX]: https://pleiades.ucsc.edu/hyades/PF_on_Mac_OS_X
[DummynetRevisited]: http://info.iet.unipi.it/~luigi/papers/20091201-dummynet.pdf
[epoll example]: https://banu.com/blog/2/how-to-use-epoll-a-complete-example-in-c

Here some `dummynet` usages on Mac.

## Configure Bandwidth

1. Create pipe with bandwidth, redirect packets to the pipe

    ```
    sudo dnctl pipe 1 config bw 1Kbit/s
    echo "dummynet out proto tcp from any to wordpress.org pipe 1" |sudo pfctl -f -
	```

2. Enable if not

    ```
	sudo pfctl -e
	```

4. do download and watch the speed

    ```
	wget -O /dev/null wordpress.org/latest.zip #=>
    ... 3.02KB/s ...
	```

5. Adjust the bandwitch on fly and watch the speed

	 ```
     sudo dnctl pipe 1 config bw 10Kbit/s #=>
     ... 61.0KB/s ...
	 ```
6. Reset and disable

   ```
   sudo pfctl -f /etc/pf.conf
   sudo pfctl -d
   ```

