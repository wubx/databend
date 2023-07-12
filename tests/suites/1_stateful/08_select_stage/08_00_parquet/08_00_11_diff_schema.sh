#!/usr/bin/env bash

CURDIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
. "$CURDIR"/../../../../shell_env.sh
DATADIR_PATH="$CURDIR/../../../../data/"

echo "drop stage if exists data;" | $MYSQL_CLIENT_CONNECT
echo "create stage data url = 'fs://$DATADIR_PATH' " | $MYSQL_CLIENT_CONNECT
echo "select \$1 from @data/ (files=>('mytime.parquet', 'complex.parquet'))" | $MYSQL_CLIENT_CONNECT 2>&1 | grep -c "diff schema"