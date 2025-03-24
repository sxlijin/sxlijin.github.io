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

case "$(uname -sm)" in
  "Darwin arm64")
    PANDOC_OS_ARCH_PKG="arm64-macos.zip"
    SASS_OS_ARCH="macos-arm64"
    ;;
  "Linux x86_64")
    PANDOC_OS_ARCH_PKG="linux-amd64.tar.gz"
    SASS_OS_ARCH="linux-x64"
    ;;
  *)
    echo "Unrecognized platform"
    exit 2
    ;;
esac

if [ ! -e "${BIN_DIR}/pandoc-${PANDOC_OS_ARCH_PKG}" ]
then
  curl \
    -L https://github.com/jgm/pandoc/releases/download/3.1.9/pandoc-3.1.9-${PANDOC_OS_ARCH_PKG} \
    -o "${BIN_DIR}/pandoc-${PANDOC_OS_ARCH_PKG}"
  case "${PANDOC_OS_ARCH_PKG}" in
    *.tar.gz)
      tar xf "${BIN_DIR}/pandoc-${PANDOC_OS_ARCH_PKG}" -C "${BIN_DIR}"
      ;;
    *.zip)
      unzip "${BIN_DIR}/pandoc-${PANDOC_OS_ARCH_PKG}" -d "${BIN_DIR}"
      ;;
    *)
      echo "Failed to unpack pandoc"
      exit 2
      ;;
    esac
fi
case "${PANDOC_OS_ARCH_PKG}" in
  *.tar.gz)
    export PANDOC_CLI="$(pwd)/${BIN_DIR}/pandoc-3.1.9/bin/pandoc"
    ;;
  *.zip)
    export PANDOC_CLI="$(pwd)/${BIN_DIR}/pandoc-3.1.9-arm64/bin/pandoc"
    ;;
  *)
    echo "Failed to resolve pandoc"
    exit 2
    ;;
esac

if [ ! -e "${BIN_DIR}/dart-sass-${SASS_OS_ARCH}.tar.gz" ]
then
  curl \
    -L https://github.com/sass/dart-sass/releases/download/1.69.5/dart-sass-1.69.5-${SASS_OS_ARCH}.tar.gz \
    -o "${BIN_DIR}/dart-sass-${SASS_OS_ARCH}.tar.gz"
  tar xf "${BIN_DIR}/dart-sass-${SASS_OS_ARCH}.tar.gz" -C "${BIN_DIR}"
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
# cp -R assets/ $OUTPUT_DIR/
ln -s "../assets" "${OUTPUT_DIR}"

$SASS_CLI scss/:$OUTPUT_DIR/css/

function pandoc {
  $PANDOC_CLI --verbose --standalone --from gfm --to html --mathjax \
    --template lib/template.html --metadata-file lib/metadata.yaml $@
}

pandoc --lua-filter lib/index.lua pages/index.md >$OUTPUT_DIR/index.html

ls pages/ | sed "s/.md//" | grep -v index | \
while read path
do
  mkdir -p "${OUTPUT_DIR}/${path}"
  pandoc "pages/${path}.md" | tee \
    "${OUTPUT_DIR}/${path}.html" \
    "${OUTPUT_DIR}/${path}/index.html" \
    >/dev/null

done

ls posts/ | sed "s/.md//" | \
while read path 
do
  mkdir -p "${OUTPUT_DIR}/${path}"
  pandoc --metadata="basepath:${path}" --lua-filter lib/post.lua \
    "posts/${path}.md" | tee \
    "${OUTPUT_DIR}/${path}.html" \
    "${OUTPUT_DIR}/${path}/index.html" \
    >/dev/null
done
