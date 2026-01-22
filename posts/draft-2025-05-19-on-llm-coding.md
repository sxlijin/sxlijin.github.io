
thoughts on llm coding and abstractions

I've had a few conversations over the past few months about this, and wanted to put the words to paper about how I feel like software engineering is changing, in the moment. There's a few pieces here, this is one of two.

first one: what skills are important in software

second one: how to use LLMs effectively

so with that framing in mind let's go

on what skills are important in software:

LLMs are absolutely _fantastic_ at streamlining nitty gritty implementation work. i can ask cursor to convert a camelCaseFunctionName to snake_case_function_name and it will not only do that, but also automatically rename camelCaseInputVar and camelCaseReturnType to snake_case_input_var and snake_case_return_type. i can ask for an aws cdk stack that sets up an auto-scaling EC2 VM pool without knowing anything about EC2's autoscaling abstractions and skip the requisite 90 minutes of googling and stackoverflow-fu that i would otherwise need to realize that oh, this is not a thing that EC2 has native abstractions for, this is the type of use case that you go to EKS or GCP Cloud Run or fly.io for. i can ask Claude to write me a dockerfile that mounts a volume from my host at a specific dir path and it will tell me that i need a docker-compose file, not a dockerfile.

but here's the thing: i have spent enough years building software that i *know* the questions i need to ask. i have intuition around things that are hard technical problems and things that are easy technical problems. i know how, when presented with a given technical solution to a problem, how to zoom in and out of it at different levels of abstraction. i know how to break down a problem into smaller technical problems. these are things that an LLM - at least today - are not going to warn you about, out of the box.

(aside: take a second and pause. this is a thing that may change tomorrow, in the next 3 months, or even just a year from now.)

let's run through an exercise: let's build youtube. and because this is an article about *technical* problem solving, i'm going to ignore most of the (very complicated!) questions about product design and things that come into play around building for a global audience (scale, a11y, i18n, l10n). so we'll start with these things being in scope:

- i want a website, myvideos.com, where users can upload videos and watch videos.

OK, so immediately, i have a question: what URL do users go to, to upload videos, and what URL do they go to, to watch videos? (assume, for the sake of argument, that this matters because sharing links by copy-pasting them is a thing a lot of people do.)

youtube went with "go to youtube.com/upload to upload, and go to youtube.com/watch?v=AbCd01234" for these links. let's copy that decision.

---

but i could easily come up with a bunch of different schemes:

if i want to upload a video titled "i skied at palisades today!" i could say that user "John Doe" can upload this video by going to myvideos.com/john-doe/i-skied-at-palisades-today and clicking "upload" and then john doe can text that link to his friends who aren't skiing with him today and they can click it and open it in their browser and watch the video at that same link.

or i could say that john has to go to myvideos.com/john-doe/upload to upload his video, and then it will get uploaded as myvideos.com/videos/i-skied-at-palisades-today and that's the link he can text his friends.

or i could just have an upload button on the myvideos.com homepage and it would 

myvideos.com/sports/skiing-freeride-2024-world-championships 

---

let's go back to upload and playback. we'll forget about upload for a second, and just focus on playback. we're going to copy youtube here, again, and say that we want to be able to watch videos in 720p, 1080p, or 4k[footnote]. again, we've made a decision.

[footnote]: the product reason for this? some people want to watch videos while they're on the bus, and their data plan sucks, so those people want 720p. other people have great internet at home and a big fancy 72" TV so they want 4k.

ok, so now we have at minimum, two computers involved: (1) the user's computer, where they're going to go to myvideos.com/watch?v=1234 and see my awesome skiing video, and also say that the user wants to watch the video in 720p, and (2) my video server, which is going to show my awesome skiing video in 720p.

now, we're copying youtube, so we want the user to be able to stream the video - that is, they should be able to open myvideos.com/watch?v=1234, click play, and the video should just start playing.[decision]. if the video is 20 minutes long, i don't want to force them to wait 5 minutes to download the entire thing, but hey, now that i have streaming, i need to buffer the stream.

oh, and if i want the user to be able to switch between 720p and 1080p and 4k on the fly, now that stream needs to be able to re-negotiate the contents mid-stream. and maybe, if my server is serving a lot of streams at once, it should be able to operate in a degraded mode and serve all streams at 480p.

there's a gajillion questions here, and honestly, it's basically a system design interview.

[decision]: notice that i'm ignoring the question of offline playback/storage.

---

let's go a little smaller, and say that, oh, i want a UI for managing env vars in my CI system. where on the page does the "create new environment variable" button go? do i want to use lucide or material for the visibility toggle icons? if i want to support environment var values with newlines in them, i can't use `<input>` elements because those are explicitly single-line and browsers strip newlines from values that are copy-pasted into them.

there are a lot of decisions that have to be made, and even though i can ask cursor or claude code or v0.dev "make me a react component for managing env vars", to deliver that solution, it's going to make a lot of decisions for me.

if my goal is to just create an env var management UI, and i just want something functional, great! but let's say i work at a big company with an established design language and we agreed 3 years ago that we were going to use material icons for everything. and we already have our own in-house `MultiLineInput` react component that under the hood is really a `textbox` with visibility controls that make it functionally behave identically to `input type="password"`. well, now i need to start being a lot more prescriptive about the solution that the AI builds for me and i can't let it make those decisions for me.

---

this is what, imo, the thing that's becoming increasing important in software right now: the ability to zoom in and out of a tech stack and ask questions at different levels of abstraction.

- build a youtube.com clone
- build a youtube-like VideoPlayback react component
- build a youtube-like VideoPlayback react component with a menu that allows switching between different resolutions
- build a youtube-like VideoPlayback react component with a menu that allows switching between different resolutions (480p, 720p, 4k) and manages a Websocket connection that sends a `setResolutionRequest { newResolution: "720p" }` message when the user chooses 720p
- build a youtube-like VideoPlayback react component with a menu that allows switching between different resolutions (480p, 720p, 4k) and manages a Websocket connection that sends a `setResolutionRequest { newResolution: "720p" }` message when the user chooses 720p
- build a youtube-like VideoPlayback react component with a menu that allows switching between different resolutions (480p, 720p, 4k) and manages a Websocket connection that sends a `setResolutionRequest { newResolution: "720p" }` message when the user chooses 720p, and allows the server to force a downgrade to `{ newResolution: "480p" }` if the server is in overload

Notice how these prompts are getting increasingly specific in how prescriptive they are about what kind of video playback component i want

but also consider that asking these questions about every little detail is super fatiguing! let's say i want a UI app to, given a csv with two columns, date and price, render a line chart. if i ask v0 or claude to do this, it will use a specific react component for the chart that allows me to hover my mouse over a datapoint and see a label showing the date and price for that datapoint. that same component will also work on mobile, if i tap the datapoint.

if this works for me, and solves my given use case, that's *amazing* - 