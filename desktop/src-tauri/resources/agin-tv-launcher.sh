#!/bin/bash
export WEBKIT_DISABLE_DMABUF_RENDERER=1
exec "$(dirname "$0")/agin-tv" "$@"
