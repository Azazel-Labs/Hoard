"""Write latest.json, which installed copies of the desktop app read to find and check an update.

    python3 scripts/desktop-updates.py <version> <download url> <dir>

<dir> holds the release's files; each update the build signed has a .sig beside it. With none (the signing key
wasn't set), there are no updates and nothing is written.
"""

import datetime
import json
import os
import sys

# Each platform's update, by how its file name ends, as Tauri's updater names the platforms.
PLATFORMS = {
    'windows-x86_64': '_x64_en-US.msi',
    'darwin-aarch64': '_aarch64.app.tar.gz',
    'linux-x86_64': '_amd64.AppImage',
}


def main(version, url, folder):
    names = sorted(os.listdir(folder))
    platforms = {}
    for platform, ending in PLATFORMS.items():
        for name in names:
            if name.endswith(ending) and name + '.sig' in names:
                with open(os.path.join(folder, name + '.sig'), encoding='utf-8') as f:
                    platforms[platform] = {'signature': f.read().strip(), 'url': f'{url}/{name}'}
    if not platforms:
        print('no signed updates, so no latest.json')
        return 0
    latest = {
        'version': version,
        'notes': f'Hoard {version}',
        'pub_date': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
        'platforms': platforms,
    }
    with open(os.path.join(folder, 'latest.json'), 'w', encoding='utf-8') as f:
        json.dump(latest, f, indent=2)
    print(f'latest.json: {", ".join(platforms)}')
    return 0


if __name__ == '__main__':
    if len(sys.argv) != 4:
        sys.exit(__doc__)
    sys.exit(main(*sys.argv[1:]))
