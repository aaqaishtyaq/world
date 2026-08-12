+++
title = "Private Networking for MicroVM Fleets with WireGuard and IPv6"
description = "Building private networking for microVM fleets using WireGuard and IPv6."
date = "2026-04-06"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
After getting machines to boot reliably, networking was the next thing that forced me to slow down and think properly.

Private networking sounds simple when you say it quickly.

"Just let the machines talk to each other."

Then you try to do it across hosts, across regions, across reboots, without handing every guest a pile of custom firewall rules and mystery routing.

That is where the clean model matters.

In this post, I want to explain the network shape I find the most reasonable for a microVM fleet.

## What I wanted from the network

For a microVM platform, the private network should feel boring from inside the guest.

The guest should not care:

- which host it landed on
- which physical node hosts its peer
- whether a node got replaced
- how the control plane exchanged peer state

The guest should know one thing: it has a private address and packets to that private world work.

That is the standard.

## Why IPv6 helps

IPv4 can do this, but it starts fights you do not need.

With IPv6, you can give every machine a clean private identity without squeezing everything through tiny address space and heavy NAT habits. It makes the addressing model easier to reason about.

That matters because the address can carry meaning:

- node identity
- machine identity
- network membership
- policy boundaries

Not magical meaning. Useful meaning.

## Why WireGuard fits

Once you have more than one host, you need a private fabric between them.

WireGuard is a good fit because it is simple, fast, and kernel-native. It also gives you a clean identity model with keys instead of a giant pile of TLS setup for every internal hop.

At a high level:

```
guest VM --tap--> worker host --WireGuard--> remote worker host --tap--> guest VM
```

The worker hosts form the mesh. The guests ride on top of it.

That lets the platform keep a simple promise:

"Your machine has a private address, and that address works across the fleet."

Here is the packet path I keep in my head:

```
guest
  -> tap device
  -> local worker host
  -> WireGuard mesh
  -> remote worker host
  -> remote tap device
  -> remote guest
```

## Do not mix guest routing and DNS identity

One subtle mistake is to blur two separate ideas:

- the next hop a guest should route through
- the stable identity of the DNS service it should query

Those are not the same thing.

The next hop is local. It is about how a packet leaves the guest and reaches the host.

The DNS service identity should be stable. It should not change just because the guest moved or the node got replaced.

Once those concepts stay separate, the network gets less weird.

## The control-plane trap

There is a bigger design lesson here too.

Do not make your whole control plane depend on the same network it is still trying to heal.

That circular dependency creates awful failure modes.

If peer updates only arrive over the mesh, and the mesh is broken because peer updates are stale, you have built a trap for yourself.

The data plane and the control plane can talk to each other, but they should not hold each other hostage.

## Failure modes that matter

The failure modes here are not always dramatic. They are often annoying:

- a stale peer endpoint means packets go to the wrong place
- a bad route means the guest has an address but no path
- the guest can reach the network, but DNS still fails

Those are the cases that make a platform feel flaky even when part of the design is working.

## What "good" looks like

A good private network for microVMs has a few traits:

- every guest gets a stable private identity
- east-west traffic works across hosts
- guest routing is host-local and predictable
- policy can be enforced without hand-crafted snowflake rules
- DNS does not depend on the guest knowing host details
- host replacement does not force the guest model to change

That is the shape I want:

```
            private fleet network

   VM A            worker A           worker B            VM B
  fdaa::10  <-->   mesh0    <====>    mesh0    <-->     fdaa::20
                     WG mesh          WG mesh
```

The guest sees a private world. The host does the ugly work.

## Why this matters for compute products

Private networking is not a side quest.

It is what makes the platform useful for anything beyond isolated toy workloads.

You need it for:

- service-to-service communication
- private databases
- multi-machine test environments
- agent sandboxes that call internal tools
- distributed jobs that should not be public on the internet

Without a clean private network, everything gets forced through public exposure or weird host tunnels. That is not a platform. That is a workaround farm.

## The real win

The best network design is the one users forget.

They should not need to know which node owns the WireGuard peer, which route got reconciled, or which host-local device delivered the packet.

They should know their machine has a private address and the packet gets where it should go. That is boring. Boring is the goal. The guest should see one private network, even if the hosts underneath are messy.
