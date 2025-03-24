# Node.js, stdout, and disappearing bytes

Someone in one of my online communities recently asked about why their Node
command was dropping output when piped to another command:

> [command] spits out 2319 lines of valid json to stdout. If I redirect it into
> a file then I can open that file and yes, see all 2319 lines and well formatted
> json.
>
> But when I pipe it to `jq`, it complains that the json terminates on line 1394
>
> If I pipe to `wc` then it reports there are 1394 lines
>
> If I pipe to `tee` to write it to a file and then pipe **that** to `wc` it
> reports 1394 and the file indeed cuts off my json part way through the 1394th
> line
>
> There is nothing unusual about this line
>
> What could be going on?

A mystery! And I still have chores and work to do, so what better time to get
nerdsniped[^nerdsniped]?

## Digging in

I'd seen something like this before years ago, but didn't really remember any
details from that besides that Node does some weird things with pipes.

If it's truncating output from 2k+ lines to ~1.5k lines of JSON, well, hm - depending
on how big those lines are, it seems very likely that there's some kind of "pipe
is full" limit getting triggered in some shape or fashion here.

> me: what do you get when piping to `wc -c`?

> them: actual character count is `104283` with the pipe its `65536`

Aha! A power of two - ✨magic✨ numbers are always a good time. (Seriously. It's so
much easier once you have a hint about what to look for.)

Indeed, looking at [`man(7) pipe`](https://man7.org/linux/man-pages/man7/pipe.7.html):

> Before Linux 2.6.11, the capacity of a pipe was the same as the
> system page size (e.g., 4096 bytes on i386).  Since Linux 2.6.11,
> the pipe capacity is 16 pages (i.e., 65,536 bytes in a system
> with a page size of 4096 bytes).

## But I know I can pipe more than 64KiB!

So do I. It's pretty easy to test too:

```
$ node -e "process.stdout.write('@'.repeat(128 * 1024));" | wc -c
  131072
```

...but if pipes are capped at 64KiB, how does this work?

If you've looked up the documentation for `printf` or `std::cout` or
`process.stdout.write` enough times, you'll notice - and hopefully remember -
that they all make some mention of buffering and blocking, which you usually
just gloss over because it's entirely irrelevant to what you're doing.

This, though, seems like something where that might be relevant!

Let's look at the Node.js docs for
[`process.stdout`](https://nodejs.org/api/process.html#a-note-on-process-io):

> `process.stdout` and `process.stderr` differ from other Node.js streams in
> important ways:
>
> [...]
> 
> 2. Writes may be synchronous depending on what the stream is connected to
>    and whether the system is Windows or POSIX:
>    * Files: _synchronous_ on Windows and POSIX
>    * TTYs (Terminals): _asynchronous_ on Windows, _synchronous_ on POSIX
>    * Pipes (and sockets): _synchronous_ on Windows, _asynchronous_ on POSIX

Huh. So on POSIX, `process.stdout.write` is synchronous when writing to a file or
terminal, but asynchronous when writing to a pipe? Interesting. Let's read on:

> These behaviors are partly for historical reasons, as changing them would
> create backward incompatibility, but they are also expected by some users.
> 
> Synchronous writes avoid problems such as output written with `console.log()` or
> `console.error()` being unexpectedly interleaved, or not written at all if
> `process.exit()` is called before an asynchronous write completes. See
> `process.exit()` for more information.
> 
> _**Warning**_: Synchronous writes block the event loop until the write has
> completed. This can be near instantaneous in the case of output to a file, but
> under high system load, pipes that are not being read at the receiving end, or
> with slow terminals or file systems, it's possible for the event loop to be
> blocked often enough and long enough to have severe negative performance
> impacts. This may not be a problem when writing to an interactive terminal
> session, but consider this particularly careful when doing production logging to
> the process output streams.

Oho. This looks promising: without even reading it, just looking at how much
information there is, it's pretty clear that Here Be Dragons.

Essentially, what this is saying is two things:

  1. `process.stdout.write(data)` will buffer data in Node.js and block if `stdout`
        is a file or terminal until `data` is written, but
  2. if `stdout` is a pipe, and the pipe is full, `process.stdout.write(data)`
        will immediately return `false` and will not block until `data` is written to
        the pipe.

Frustratingly, you have to go to the docs about [stream
buffering](https://nodejs.org/api/stream.html#buffering) and
[`writable.write`](https://nodejs.org/api/stream.html#writablewritechunk-encoding-callback)
to actually understand this.

## So... how do you actually trigger this?

This took me a second to figure out.

I thought `gpt-4o` and `o1-preview` would be able to do this pretty easily, but
surprisingly not. They did both point out that to fix this, there's a [`drain`
event](https://nodejs.org/api/stream.html#event-drain) that you can subscribe
to, so you can resume writing to `stdout` after `process.stdout` has filled up;
but nothing that either suggested was repro'ing this issue.

So I went back and re-read the docs about `process.stdout`, and this bit jumped
out at me:

> Synchronous writes avoid problems such as output [not being] written at all if
> `process.exit()` is called before an asynchronous write completes.

Let's try that:

```
$ node -e "process.stdout.write('@'.repeat(128 * 1024)); process.exit(0);" | wc -c
  65536
```

And what if we write to a file?

```
$ node -e "process.stdout.write('@'.repeat(128 * 1024)); process.exit(0);" >node-stdout-test.out && wc -c node-stdout-test.out
  131072 node-stdout-test.out
```

Well, there we go.

(Verified on both `darwin-arm64` and `linux-x86_64`[^colab].)

[^colab]: If, like me, you're on a ARM Mac, and you're looking for a x86_64 Linux
    machine to fiddle around with, Google Colab seems to be the easiest way to get a
    hold of one: [open a notebook](https://colab.research.google.com), type a
    command into a cell like `!uname -a`, and hit "Run cell".

[^nerdsniped]: Congratulations to [George Mauer](https://georgemauer.net/) for
    the successful nerdsnipe!
