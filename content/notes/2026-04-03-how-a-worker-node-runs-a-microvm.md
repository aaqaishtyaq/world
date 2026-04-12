+++
title = "How a Worker Node Runs a MicroVM"
date = "2026-04-03"
draft = false

[taxonomies]
series = ["MicroVM Platform"]
+++
After removing the earlier draft for this slot, I still wanted a post here that explains one of the most important parts of the platform: what actually happens on the worker node when a machine gets created.

This part matters because a lot of the rest of the system only makes sense once you know what the worker is responsible for.

The control plane can pick a node. The proxy can expose traffic. DNS can hand out names. None of that means much until one worker host actually does the job of turning a request into a running microVM.

## The worker is where the request becomes real

At a high level, the worker daemon is the part of the system that takes a machine request and does the host-side work needed to bring it to life.

That usually means:

- validating the machine request
- preparing the filesystem
- setting up the network
- starting Firecracker
- waiting for the guest to come up
- exposing the machine to the rest of the platform
- cleaning up if something fails halfway through

That is a lot of responsibility for one service, but it is the right place for it. The worker host is where the machine actually lives, so it needs the code that knows how to deal with the messy details.

## The flow I keep in my head

This is the simplest version of the worker-side flow:

```
create request arrives
        |
        v
  validate + record intent
        |
        v
  prepare root filesystem
        |
        v
  setup network namespace, tap, routes
        |
        v
  write VM config + start Firecracker
        |
        v
  wait for guest agent to become ready
        |
        v
  register machine with proxy / DNS / control plane
```

Real life is messier than that, but if this mental model is clear, the rest of the platform gets easier to follow.

## What the worker leaves behind on disk

Even a short-lived machine usually leaves behind a few concrete things on the host while it is running:

- a kernel image
- a root filesystem or overlay
- a VM config
- logs
- sockets or control files

That is another reason cleanup matters so much. These are not abstract objects. They are real files and real resources on a real box.

## Step 1: take the request seriously

The first part sounds boring, but it is important.

When the worker gets a create request, it cannot just charge ahead and hope for the best. It needs to make sure:

- the shape of the machine makes sense
- the node has enough room
- the image or base is available
- the request can be tracked if the host dies halfway through

This is one reason I like durable lifecycle logic so much. If the process crashes after step three out of seven, the worker still needs enough information to continue or clean up properly when it comes back.

## Step 2: prepare the filesystem

Before the guest boots, the worker has to prepare the disk the machine will use.

Depending on the path, that can mean:

- pulling an image
- unpacking a root filesystem
- creating an overlay
- cloning from a prepared base
- restoring from a snapshot-backed setup

This step has a huge effect on startup time.

It is also one of the places where wasted work hurts the most. If two nodes both prepare filesystems for one request and only one wins, that is real IO and real time thrown away.

That is why I care so much about reservations and prepared capacity in the later posts.

## The slow parts

If I had to point to the places that most often dominate startup time, it would usually be these:

- filesystem prep
- network bring-up
- waiting for the guest to be ready

The worker does all three, which is why it ends up sitting right on the critical path.

## Step 3: build the network around the guest

Once the filesystem is ready, the worker still has to build the network shape the machine expects.

That usually means creating or wiring up:

- a network namespace
- a tap device
- routes
- addressing
- any host-side policy needed for isolation

This is the part that makes a machine feel less like "a Firecracker process" and more like "a guest that actually lives on a host and can talk to things."

It is also the step that causes a lot of annoying bugs if you are sloppy. A machine can boot fine and still be useless because the host-side network was only half-configured.

## Step 4: start Firecracker

Once the assets are in place, the worker can finally hand things over to Firecracker.

That means building the VM config, pointing Firecracker at the right kernel and drives, starting the process, and then watching closely.

This is the step people usually think is the whole problem.

It is not.

It is a very important step, but by the time you get here, the worker has already done a lot of heavy lifting.

## Step 5: wait for the guest to be usable

A VM process existing is not the same thing as a usable machine.

This is one of those details that matters a lot in practice.

If you return success too early, the rest of the system starts acting as if the machine is ready when it really is not. Then you get weird timing bugs:

- the proxy points to a machine that is not listening yet
- the control plane thinks startup succeeded
- the user connects before the guest finished booting

So the worker needs a clear readiness signal from inside the guest. Until that signal shows up, the machine is still starting.

## Step 6: plug the machine into the platform

Once the guest is actually ready, the worker can expose it to the rest of the system.

That can include:

- registering routes with the proxy
- publishing private network identity
- publishing DNS data
- reporting machine state back to the control plane

This is the point where the machine stops being "something being built on one host" and becomes "something the platform can route to and reason about."

## Step 7: clean up hard when things go wrong

This part is easy to underestimate.

Machines do not only fail at clean points. They fail halfway through.

You can end up with:

- a prepared filesystem but no VM
- a VM process but broken networking
- a route registered for a machine that never became ready
- a half-created overlay

That is why the worker daemon cannot just know how to start machines. It has to know how to recover and how to clean up.

For me, that is one of the main differences between a demo and a platform.

## The worker is the heart of the node

Once I started looking at the system this way, a lot of other design choices made more sense.

The control plane should stay focused on placement and coordination.

The worker should own the machine lifecycle on the node.

The proxy should care about exposure.

DNS should care about names.

The host agent should care about host-level drift and health.

That split keeps the jobs clear.

## Why I wanted this post in the series

I wanted one post early in the series that answers a simple question:

When a machine gets created, what does one worker host actually do?

For me, the answer is:

It takes a request, prepares the machine, builds the network around it, starts the guest, waits until it is actually usable, plugs it into the rest of the platform, and cleans up if anything goes wrong.

That is a lot of work.

It is also where the platform becomes real. The worker does much more than start Firecracker. It builds the machine around it.
