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

pandoc --lua-filter index.lua pages/index.md >$OUTPUT_DIR/index.html

ls pages/ | sed "s/.md//" | grep -v index | \
while read path
do
  pandoc "pages/${path}.md" >"${OUTPUT_DIR}/${path}.html"
done

ls posts/ | sed "s/.md//" | \
while read path 
do
  pandoc --metadata="basepath:${path}" --lua-filter post.lua \
    "posts/${path}.md" >"${OUTPUT_DIR}/${path}.html"
done
