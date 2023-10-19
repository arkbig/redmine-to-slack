#!/usr/bin/env sh
#====================================================================
set -eu
# set -eux
umask 0022
PATH='.:/usr/sbin:/usr/bin:/sbin:/bin'
IFS=$(printf ' \t\n_')
IFS=${IFS%_}
export IFS LC_ALL=C LANG=C PATH
#--------------------------------------------------------------------

# Adjust UID,GID
ug_name=redmine_to_slack
uid=$(id -u)
gid=$(id -g)
if [ "${CONTAINER_GID}" != "${gid}" ]; then
    groupmod -g "${CONTAINER_GID}" -o "${ug_name}"
fi
if [ "${CONTAINER_UID}" != "${uid}" ]; then
    usermod -g "${ug_name}" -o -u "${CONTAINER_UID}" "${ug_name}"
fi

# Adjust UID,GID of the container creation directory.
for mk_dir in /home/${ug_name} /app; do
    uid=$(stat -c "%u" "${mk_dir}")
    gid=$(stat -c "%g" "${mk_dir}")
    if [ "${CONTAINER_GID}" != "${gid}" ]; then
        chgrp -R "${CONTAINER_GID}" "${mk_dir}"
    fi
    if [ "${CONTAINER_UID}" != "${uid}" ]; then
        chown -R "${CONTAINER_UID}" "${mk_dir}"
    fi
done

# If exe is not specified, run redmine-to-slack
if [ -z "$1" ] || [ "${1#-}" != "$1" ]; then
    set -- redmine-to-slack "$@"
fi

# Run as
if [ "$(id -u)" = "${CONTAINER_UID}" ]; then
    exec "$@"
else
    exec su-exec "${ug_name}" "$@"
fi
