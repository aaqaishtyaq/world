+++
title = "Linux Container Networking from Scratch"
date = "2023-01-26"
+++
In this article, we will be looking into setting up networking on a linux box from scratch.

We will be creating a fresh new VM using Lima on macOS.
You can create a new VM using VirtualBox, Vagrant, or even create a VM on OCI for free.

## Isolation based on Network Namespace

The common thing that constitutes a network stack includes the:

- Network Devices
- Routing Rules
- Netfilter Hooks

We can create a non-comprehensive `inspect-net-stack` script.

```bash
#!/usr/bin/env bash

echo "> Network devices"
ip link

echo -e "\n> Route table"
ip route

echo -e "\n> Iptables rules"
iptables --list-rules
```

After the execution of the inspect script on my machine produces the following input:

```bash
% sudo ./inspect-net-stack
> Network devices
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1500 qdisc fq_codel state UP mode DEFAULT group default qlen 1000
    link/ether 52:55:55:a3:ad:92 brd ff:ff:ff:ff:ff:ff
    altname enp0s1

> Route table
default via 192.168.5.3 dev eth0 proto dhcp src 192.168.5.15 metric 100
192.168.5.0/24 dev eth0 proto kernel scope link src 192.168.5.15 metric 100

> Iptables rules
-P INPUT ACCEPT
-P FORWARD ACCEPT
-P OUTPUT ACCEPT
```

We want to make sure that each of the containers we are going to create soon will get a separate network stack.

Linux namespaces used for container isolation are called [network namespaces](https://man7.org/linux/man-pages/man8/ip-netns.8.html). We won't be creating the fully isolated container, rather restrict the scope to only the network stack. One of the ways to create a network stack is by using the `ip` tool, part of [`iproute2`](https://en.wikipedia.org/wiki/Iproute2)

```bash
% sudo ip netns add netns0
% ip netns
netns0
```

We have a new network namespace but to start using the network namespace, We can use a command called `nsenter`. It enters one or more of the specified namespaces and then executes the given program:

```bash
# This is create a new bash process under netns0 namespace
% sudo nsenter --net=/var/run/netns/netns0 bash


% sudo ./inspect-net-stack
> Network devices
1: lo: <LOOPBACK> mtu 65536 qdisc noop state DOWN mode DEFAULT group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00

> Route table

> Iptables rules
-P INPUT ACCEPT
-P FORWARD ACCEPT
-P OUTPUT ACCEPT
```

As you can see from the above output, the `bash` process running inside the `netns0` namespace sees a different network stack. There are no routing rules and no custom iptables chain. Only one loopback device (`lo`).

```bash
                        Network Namespace
┌─────────────────────────────────────────────────────────────────────┐
│                                                                     │
│                                          Root Namespace             │
│                                                                     │
│ lo0                    ┌────────────────────────────────────────┐   │
│ ┌─┐                    │                                        │   │
│ │ │                    │    lo0             Container Namespace │   │
│ │ │                    │    ┌─┐              netns0             │   │
│ │ │    ┌────────────┐  │    │ │                                 │   │
│ │ │    │            │  │    │ │  ┌───────────┐                  │   │
│ │ │    │ routes     │  │    │ │  │           │                  │   │
│ │ │    │            │  │    │ │  │routes     │                  │   │
│ │ │    └────────────┘  │    │ │  └───────────┘                  │   │
│ └─┘                    │    └─┘                                 │   │
│                        │                                        │   │
│ ┌─┐    ┌────────────┐  │         ┌───────────┐                  │   │
│ │ │    │            │  │         │           │                  │   │
│ │ │    │ iptables   │  │         │iptables   │                  │   │
│ │ │    │            │  │         └───────────┘                  │   │
│ │ │    └────────────┘  │                                        │   │
│ │ │                    │                                        │   │
│ │ │                    │                                        │   │
│ └─┘                    │                                        │   │
│ eth0                   └────────────────────────────────────────┘   │
│                                                                     │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

*Beware*: The `nsenter` command from above started a nested bash session in the netns0 network namespace. Don't forget to exit from it.

---

In the upcoming posts, I will discuss connecting the host with this namespace. Interconnecting various network switches, Just like patching the physical switch with a LAN cable.
