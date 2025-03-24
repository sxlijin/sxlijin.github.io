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

export PANDOC_VERSION=3.6.4
export SASS_VERSION=1.86.0

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

export PANDOC_ARCHIVE_NAME=pandoc-${PANDOC_VERSION}-${PANDOC_OS_ARCH_PKG}
export PANDOC_BIN_DIR="${BIN_DIR}/pandoc-${PANDOC_VERSION}"
if [ ! -e "${BIN_DIR}/${PANDOC_ARCHIVE_NAME}" ]
then
  curl \
    -L https://github.com/jgm/pandoc/releases/download/${PANDOC_VERSION}/pandoc-${PANDOC_VERSION}-${PANDOC_OS_ARCH_PKG} \
    -o "${BIN_DIR}/${PANDOC_ARCHIVE_NAME}"
  mkdir -p "${PANDOC_BIN_DIR}"
  case "${PANDOC_OS_ARCH_PKG}" in
    *.tar.gz)
      tar xf "${BIN_DIR}/${PANDOC_ARCHIVE_NAME}" -C "${PANDOC_BIN_DIR}"
      ;;
    *.zip)
      unzip "${BIN_DIR}/${PANDOC_ARCHIVE_NAME}" -d "${PANDOC_BIN_DIR}"
      ;;
    *)
      echo "Failed to unpack pandoc"
      exit 2
      ;;
    esac
fi
case "${PANDOC_OS_ARCH_PKG}" in
  *.tar.gz)
    export PANDOC_CLI="$(pwd)/${PANDOC_BIN_DIR}/pandoc-${PANDOC_VERSION}/bin/pandoc"
    ;;
  *.zip)
    export PANDOC_CLI="$(pwd)/${PANDOC_BIN_DIR}/pandoc-${PANDOC_VERSION}-arm64/bin/pandoc"
    ;;
  *)
    echo "Failed to resolve pandoc"
    exit 2
    ;;
esac
function pandoc {
  $PANDOC_CLI --verbose --standalone --from gfm --to html --mathjax \
    --template lib/template.html --metadata-file lib/metadata.yaml $@
}

export SASS_ARCHIVE_NAME=dart-sass-${SASS_VERSION}-${SASS_OS_ARCH}.tar.gz
export SASS_BIN_DIR="${BIN_DIR}/dart-sass-${SASS_VERSION}"
if [ ! -e "${BIN_DIR}/${SASS_ARCHIVE_NAME}" ]
then
  curl \
    -L https://github.com/sass/dart-sass/releases/download/${SASS_VERSION}/${SASS_ARCHIVE_NAME} \
    -o "${BIN_DIR}/${SASS_ARCHIVE_NAME}"
  mkdir -p "${SASS_BIN_DIR}"
  tar xf "${BIN_DIR}/${SASS_ARCHIVE_NAME}" -C "${SASS_BIN_DIR}"
fi
export SASS_CLI="$(pwd)/${SASS_BIN_DIR}/dart-sass/sass"
function sass {
  $SASS_CLI $@
}

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

sass scss/:$OUTPUT_DIR/css/

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
