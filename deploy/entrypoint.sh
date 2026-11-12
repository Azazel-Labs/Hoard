#!/bin/sh
# Starts as root just long enough to make the mounted folders usable, then runs Hoard as an ordinary user.
#
# Which user: PUID/PGID if they're set, otherwise whoever owns /library (so a host folder you mount keeps working
# and the files Hoard writes there stay yours), otherwise 10001. /config and /cache are Hoard's own, so they're
# handed to that user; /library is never chown'd, because it can be a big share Hoard doesn't own.
set -eu

# Started as a user already (compose `user:`, rootless runtimes): nothing to fix up, and no way to.
if [ "$(id -u)" != 0 ]; then
    exec "$@"
fi

owner_uid=$(stat -c %u /library 2>/dev/null || echo 0)
owner_gid=$(stat -c %g /library 2>/dev/null || echo 0)
if [ "$owner_uid" = 0 ]; then owner_uid=10001; owner_gid=10001; fi
uid=${PUID:-$owner_uid}
gid=${PGID:-$owner_gid}

for dir in /config /cache; do
    mkdir -p "$dir"
    # Only when the top folder is someone else's, so a restart doesn't walk the whole cache again.
    if [ "$(stat -c %u:%g "$dir")" != "$uid:$gid" ]; then
        chown -R "$uid:$gid" "$dir" 2>/dev/null \
            || echo "hoard: couldn't give $dir to $uid:$gid (a network share that refuses chown?); set PUID/PGID to its owner" >&2
    fi
done

# A fresh named volume is root's and empty: give it over. A folder with files in it is left as it is.
if [ "$(stat -c %u /library)" = 0 ] && [ -z "$(ls -A /library 2>/dev/null)" ]; then
    chown "$uid:$gid" /library 2>/dev/null || true
fi
if ! setpriv --reuid="$uid" --regid="$gid" --clear-groups test -w /library; then
    echo "hoard: /library isn't writable by $uid:$gid; set PUID/PGID to the folder's owner or change its permissions" >&2
fi

# Keep groups the container was given (compose group_add, such as the GPU's render group), but not root's.
groups=$(id -G | tr ' ' '\n' | grep -vx 0 | paste -sd, - || true)
if [ -n "$groups" ]; then
    set -- setpriv --reuid="$uid" --regid="$gid" --groups="$groups" -- "$@"
else
    set -- setpriv --reuid="$uid" --regid="$gid" --clear-groups -- "$@"
fi
HOME=/config exec "$@"
