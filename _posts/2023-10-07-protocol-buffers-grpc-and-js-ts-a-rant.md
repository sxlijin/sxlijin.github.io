---
layout: post
title: Protocol Buffers, gRPC, and JS/TS - a Rant
---

Protocol Buffers and gRPC are amazing. But also, fuck them.

---

Google built Protocol Buffers a long time ago, and the competitive landscape of solutions was _very_ different at the time. Even as I write this, in 2023, https://protobuf.dev still has this as the intro text to protobuf:

> Protocol buffers are Google’s language-neutral, platform-neutral, extensible mechanism for serializing structured data – think XML, but smaller, faster, and simpler. 

XML? Seriously? There's a lot more than SOAP out there these days.

---

Protobuf went through a lot of iterations internally at Google before it was open-sourced (I wrote go/wtf-are-protos when I was there to make sense of it, because I had the distinct pleasure of being on a team that was on the long tail of proto1 users who couldn't move to proto2).

This meant that, before being open-sourced, it was designed to solve Google problems, created by Google scale, for Google engineers. And actually, I should really say it was designed to solve google3 problems by google3 engineers.

This also meant that _after_ being open-sourced, it... kind of continued to stay that way.

---

Let's step away from the history of protobuf for a second. Here's the problem that we've dealt with at trunk.io for the past few years:

* we decided to use Bazel for our build system, because we knew we were going to be a polyglot repo, and we all had experience with it;
* we wanted to use protos and gRPC, because for all their warts, they're still fundamentally sound technologies with decades of design decisions underpinning them;
* and then we decided to build our services using Node.js and Typescript.

Now, in theory, using Bazel should've made it _easy_ to get going with protos and gRPC: having intermediate codegen steps and modelling them in the dependency graph of your build system of choice (to the extent that you're willing to describe your Lovecraftian Makefile horror as a build system) is always a pain. (At least, I assume so. I've never actually had to set up Makefiles for anything that wasn't a toy.)

In practice?

* [Dig-Doug/rules_typescript_proto](https://github.com/Dig-Doug/rules_typescript_proto) were the rules that we set up our stack with, because
  * we needed codegen for our backends, i.e. gRPC for Node, and for our frontend, i.e. gRPC for Web,
  * which meant that at the time - 2021? or so - this repo was the only one that worked
* Unfortunately, that repo depends on tooling from `improbable-eng`, which time has not been kind to:
  * [`improbable-eng/ts-protoc-gen`](https://github.com/improbable-eng/ts-protoc-gen), the protoc plugin that generates the gRPC-Node and gRPC-Web code, which inevitably lends itself to code like this:

    ```
    const toFooMsgPb = (fooObj) => {
        const fooMsg = new fooPb.FooMsg();
        fooMsg.setAlpha(fooObj.alpha);
        fooMsg.setBeta(fooObj.beta);
        fooMsg.setGamma(fooObj.gamma);
        return fooMsg;
    }
    ```

    Notice how that really should've been just `new FooPb.FooMsg.FromObject(fooObj)`? Well, that's because [`protobuf-javascript`](https://github.com/protocolbuffers/protobuf-javascript), the underlying JS protoc plugin, cargo cults the C++ API without any respect for building ergonomic APIs using standard JS conventions.

    (I'm being a bit facetious here. There are legitimate design trade-offs to be had, from a memory footprint perspective to enabling code minification. But IIRC, the Python protoc plugin has exactly the same problem - even inside google3, at least when I left in 2021, people had been asking for the ability to use `**kwargs` with protobuf constructors for _years_ and no one ever built it.)
  * [`improbable-eng/grpc-web`](https://github.com/improbable-eng/grpc-web), which is deprecated in favor of [`grpc/grpc-web`](https://github.com/grpc/grpc-web)... except apparently TS support in the latter has been [experimental since 2018](https://github.com/grpc/grpc-web/blame/49d3b7086895de22b44ec7be29b4c259b553bff8/README.md#L100-L103)?
  * (There's some deeper history here where Improbable and Google started building out gRPC Web tooling in parallel, realized that they were duplicating effort, and then tried to link up but by then had already implemented slightly divergent stacks - I'm not very familiar with the details of this though.)

* But none of the alternatives are good:

  * neither [`stephenh/ts-proto`](https://github.com/stephenh/ts-proto) and [`thesayyn/protoc-gen-ts`](https://github.com/thesayyn/protoc-gen-ts) have native Bazel support, and
    * adding Bazel rules for a given `protoc` plugin is not exactly... _easy_, particularly since [`rules_proto_grpc` ripped out aspect-based compilation support in 4.0.0](https://rules-proto-grpc.com/en/latest/changelog.html#4.0.0) (which... IMO is a super critical piece of making protoc plugins _usable_, despite that I do empathize with the reasoning w.r.t. 3p deps; and IIRC both `cc_proto_library` and `java_proto_library` are natively implemented in Bazel, so they don't have to struggle with this issue - not sure what `go_proto_library` does though)
  * `rules_proto_grpc`, by ripping out aspect-based compilation, made it very hard to use their `js_grpc_node_library` and `js_grpc_web_library`, and
    * even if we _did_ get them to work, the latter appears to have some subtle behavior divergences from `improbable-eng/grpc-web` - finding that out after I proved we could use those rules was an unpleasant surprise;
  * all this leaves [Buf](https://buf.build/) as the only real player in town here
    * which, let's also note - the fact that there is an _entire company_ built around making protobufs usable, that enough people use protobufs that this was a necessary and viable business - that's utterly insane to me,
    * but unfortunately their Bazel rules aren't yet ready for primetime (and they're not even building them! it's Aspect that's building them!)

And so that's how we at trunk.io have been stuck with a hideously un-ergonomic protobuf API and tooling setup for the past 2 years with an inability to get off this.

P.S. what the frickity frack is going on with the runtime libraries here? `grpc` doesn't [receive security updates anymore](https://www.npmjs.com/package/grpc) and you're supposed to use `@grpc/grpc-js` now. Except there's no actual published documentation for `@grpc/grpc-js`, and you're supposed to use the `grpc` docs instead (fortunately the two appear to be _mostly_ API compatible, but I don't have a good sense of the edge cases where we do have divergent behavior.)

---

I blame Google for this state of affairs. (And now that I'm on the outside looking in, it's easy for me to throw stones at glass houses - I'm sure some of the stuff I'm about to say is blatantly wrong because there are <insert internal design constraints here>.)

Google had to figure out Javascript in the early 2000s, when they were inventing Gmail and the notion of a Web UI that wasn't just a pre-compiled Xanga blog or something. That led to Closure, which... worked. Right up until the rest of the Internet got big and people started loading bigger pages on beefier machines.

Fast forward to the early 2020s, and Google has managed to make bad bet after bad bet in frontend/JS land:

* AngularJS (aka Angular 1), back when Coffeescript was still becoming a thing,
* Angular (aka Angular 2), which has now lost to React, and
* don't even get me started on Wiz. that's one project that I will never believe was anything except a promo project.

And as a consequence, the JS tooling - and TS tooling as well - has just never been properly developed. I suspect organizationally, the protobuf team was also never appropriately staffed to make JS/TS development sane, which is why the rest of the open-source world has had to pick up the slack here (in fact, when I was at Google, the only team staffing that I remember existing for protobuf was for getting google3 off proto1 - I don't think their mandate allowed them to tackle any major projects that weren't "rip out all the dependencies on XYZ legacy technology").

---

I don't really have a conclusion here. This piece has just been floating around in the back of my mind for a long time, in some shape or form, and I wanted to get the ideas and thoughts out in words.