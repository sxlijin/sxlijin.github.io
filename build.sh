#!/usr/bin/env bash

set -euxo pipefail

export BIN_DIR=_bin
export OUTPUT_DIR=_site

mkdir -p ${BIN_DIR}

# Avoid removing the output directory itself: this screws with hot reload.
rm -rf ${OUTPUT_DIR}/*
mkdir -p ${OUTPUT_DIR}

##############################################################################
#                                                                            #
#  Auto-download pandoc and dart-sass CLIs                                   #
#                                                                            #
##############################################################################

if [ ! -e "${BIN_DIR}/pandoc.tar.gz" ]
then
  curl \
    -L https://github.com/jgm/pandoc/releases/download/3.1.9/pandoc-3.1.9-linux-amd64.tar.gz \
    -o "${BIN_DIR}/pandoc.tar.gz"
  tar xf "${BIN_DIR}/pandoc.tar.gz" -C "${BIN_DIR}"
fi
export PANDOC_CLI="$(pwd)/${BIN_DIR}/pandoc-3.1.9/bin/pandoc"

if [ ! -e "${BIN_DIR}/dart-sass.tar.gz" ]
then
  curl \
    -L https://github.com/sass/dart-sass/releases/download/1.69.5/dart-sass-1.69.5-linux-x64.tar.gz \
    -o "${BIN_DIR}/dart-sass.tar.gz"
  tar xf "${BIN_DIR}/dart-sass.tar.gz" -C "${BIN_DIR}"
fi
export SASS_CLI="$(pwd)/${BIN_DIR}/dart-sass/sass"

##############################################################################
#                                                                            #
#  Build all assets                                                          #
#                                                                            #
##############################################################################

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

pandoc --lua-filter lib/index.lua pages/index.md >$OUTPUT_DIR/index.html

ls pages/ | sed "s/.md//" | grep -v index | \
while read path
do
  pandoc "pages/${path}.md" >"${OUTPUT_DIR}/${path}.html"
done

ls posts/ | sed "s/.md//" | \
while read path 
do
  pandoc --metadata="basepath:${path}" --lua-filter lib/post.lua \
    "posts/${path}.md" >"${OUTPUT_DIR}/${path}.html"
done
