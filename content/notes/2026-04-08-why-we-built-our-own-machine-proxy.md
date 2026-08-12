+++
title = "Why We Built Our Own Machine Proxy"
description = "Why we built a custom machine proxy for exposing services from microVMs instead of reaching for an off-the-shelf option."
date = "2026-04-08"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
When I started thinking about exposing services from machines, one plain question kept showing up:

How does traffic reach a machine?

That question sounds smaller than it is.

Public ingress is not just HTTP. It is TLS, SNI, raw TCP, SSH, WebSSH, machine ownership, and route cleanup when the guest disappears in the middle of the day.

That is a lot of stuff to hand-wave.

In this post, I want to explain why I think a machine platform often wants a proxy layer that is tightly tied to the machine lifecycle.

## The normal answer

The normal answer is to reach for a big proxy.

That is often reasonable. Envoy, HAProxy, Nginx, Caddy, and friends exist for a reason. You should not rebuild them out of ego.

But sometimes the fit is bad.

In a microVM platform, the proxy has to care about things that general-purpose ingress does not naturally care about:

- per-machine route ownership
- fast route add and delete
- namespace-aware connections
- SSH and WebSSH alongside HTTP
- tight integration with machine lifecycle

When that friction grows, a custom proxy stops sounding like vanity and starts sounding like less glue code.

## What I wanted from the proxy

I wanted the host-level proxy to do a few things well:

- terminate TLS
- expose stable public endpoints
- route traffic to the right machine
- support HTTP, raw TCP, and SSH flows
- clean up routes when machines die

That is not a generic public ingress problem. That is a machine platform problem.

## Why lifecycle integration matters

A machine proxy should not feel like a detached box on the side.

It needs to move with the lifecycle:

- machine starts
- route gets registered
- DNS gets updated
- traffic flows
- machine stops
- route disappears

If the route layer lags behind machine truth, you get bad failure modes:

- traffic to dead machines
- ghost routes
- wrong backend after a restart
- delayed exposure after machine readiness

The proxy has to speak the same language as the machine system.

## The shape of the design

This is the rough picture:

```
internet
   |
   v
+---------+
| proxy   |
| TLS     |
| SSH     |
| routes  |
+----+----+
     |
     v
+----------+
| machine  |
| netns or |
| backend  |
+----------+
```

The important part is not the box. It is the ownership.

The proxy should know which machine owns which route and update that state as the machine lifecycle changes.

## What the proxy really stores

At a minimum, the proxy layer usually needs to know:

- hostname
- target machine
- protocol
- port
- route ownership

That is enough to make route updates explicit instead of magical.

## Why not push this into every guest

You could make every machine solve its own public ingress.

That would be a mistake.

Now every guest needs:

- TLS setup
- cert management
- public binding logic
- exposure policy
- maybe even SSH exposure logic

That is duplicated work and a bigger security surface.

A host-level proxy gives the platform one clean place to manage exposure.

That is simpler for operators and better for users.

There is also a very real cleanup race here:

```
machine dies
route still exists
next request arrives
request goes nowhere
```

That is why I keep treating ingress as part of the machine lifecycle instead of a detached networking concern.

## One underrated benefit: better product shape

Once ingress is owned at the platform layer, you can do nicer things:

- stable public hostnames per machine
- platform-managed certs
- clean SSH entry points
- policy around what gets exposed
- shared observability around requests and failures

That turns raw compute into a product with doors and handles.

## Building your own does not mean building everything

There is a good way to do this and a bad way.

The good way is to build the narrow thing your platform actually needs:

- route storage
- lifecycle hooks
- protocol support you really use
- cert modes you can operate

The bad way is to convince yourself you are now in the business of recreating every feature a major proxy has learned over a decade.

I have no interest in the second path.

The goal is not "beat Envoy." The goal is "remove enough mismatch that the system gets simpler."

## The test I use

If a machine starts, can the route appear quickly?

If a machine dies, can the route disappear cleanly?

If an operator asks who owns a route, is the answer obvious?

If the answer to those questions is yes, the proxy is doing its job.

That is the level I care about. Not theoretical flexibility. Real ownership. That is what I care about here. Ingress is a lifecycle problem as much as a networking problem.
