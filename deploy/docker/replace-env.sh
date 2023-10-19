#!/usr/bin/env bash
#====================================================================
# .envを用意します。
# コンテナ内のUIDやGIDを揃えるために、ローカルの値を設定します。
#====================================================================
# begin of 定型文
# このスクリプトを厳格に実行
set -eu
# set -eux
# 環境に影響を受けないようにしておく
umask 0022
# PATH='/usr/bin:/bin'
IFS=$(printf ' \t\n_')
IFS=${IFS%_}
export IFS LC_ALL=C LANG=C PATH
# end of 定型文
#--------------------------------------------------------------------

# 簡潔にするためカレント移動
self_dir="$(readlink -fn "$(dirname "$0")")"
pushd "$self_dir" >/dev/null

# .envを用意
if [ ! -f ".env" ]; then
    cp sample.env .env
fi
# UID:GIDの置換
sed "s/^CONTAINER_UID=.*/CONTAINER_UID=$(id -u)/" .env 1<>.env
sed "s/^CONTAINER_GID=.*/CONTAINER_GID=$(id -g)/" .env 1<>.env
