#!/usr/bin/env bash

set -e

/bin/server migrate
exec /bin/server start
