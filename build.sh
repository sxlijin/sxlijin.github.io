#!/usr/bin/env bash

set -euxo pipefail

export PANDOC_CLI
export SASS_CLI
export OUTPUT_DIR=_site

# Avoid removing the output directory itself: this screws with hot reload.
rm -rf ${OUTPUT_DIR}/*
mkdir -p ${OUTPUT_DIR}

# Everything in root assets used to be at the root of the repo. Keeping it that
# way for backwards compatibility, just in case there's a reason I wanted them
# there that I forgot about.
cp root-assets/* $OUTPUT_DIR/

# Create an assets/ dir in the outputs.
cp -R assets/ $OUTPUT_DIR/

$SASS_CLI scss/:$OUTPUT_DIR/css/

function pandoc {
  $PANDOC_CLI --verbose --standalone --from gfm --to html \
    --template lib/template.html --metadata-file lib/metadata.yaml $@
}

pandoc index.md >$OUTPUT_DIR/index.html
pandoc resume.md >$OUTPUT_DIR/resume.html
pandoc bookshelf.md >$OUTPUT_DIR/bookshelf.html

cat lib/blog.md <(ls -r posts | sed -E 's|(.*).md|\n  * [\1](/\1.html)|') | \
  pandoc >$OUTPUT_DIR/blog.html

pandoc posts/2017-06-01-was-my-degree-worth-it.md >$OUTPUT_DIR/2017-06-01-was-my-degree-worth-it.html
pandoc posts/2023-10-07-protocol-buffers-grpc-and-js-ts-a-rant.md >$OUTPUT_DIR/2023-10-07-protocol-buffers-grpc-and-js-ts-a-rant.html
