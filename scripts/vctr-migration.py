from argparse import ArgumentParser
import subprocess
import psycopg2


def gen_id():
    try:
        out = subprocess.Popen(["idgen", "xid"], stdout=subprocess.PIPE)
        return out.stdout.read().decode('utf-8')
    except FileNotFoundError:
        print("Error: idgen is not installed!")
        print("Please install it from here and make it available to your PATH:")  # noqa
        print("https://github.com/zekroTJA/idgen/releases")
        exit(1)


def parse_args():
    p = ArgumentParser()

    p.add_argument("--origin", "-o", type=str, required=True,
                   help="Origin database connection string.")
    p.add_argument("--target", "-t", type=str, required=True,
                   help="Target database connection string.")

    return p.parse_args()


def main():
    args = parse_args()

    originConn = psycopg2.connect(args.origin)
    cur = originConn.cursor()
    cur.execute(
        'SELECT "Guid", "Ident", "Destination", "Enabled", "PermanentRedirect", "Created" '  # noqa
        'FROM "Links"')
    links = cur.fetchall()
    cur = originConn.cursor()
    cur.execute(
        'SELECT "LinkGuid", "Created" '
        'FROM "Accesses"')
    accesses = cur.fetchall()

    targetConn = psycopg2.connect(args.target)
    cur = targetConn.cursor()
    cur.execute('SELECT "username" FROM "auth" LIMIT 1')
    res = cur.fetchone()
    if not res:
        print("Error: tenso is not initialized!")
        print("This needs to be done before you can migrate.")
        exit(1)
    [(username)] = res

    id_mapping = {}
    i = 0
    lstr = str(len(links))
    for [guid, ident, destination, enabled, permanent_redirect, created] in links:  # noqa
        i += 1
        xid = gen_id()
        cur.execute(
            'INSERT INTO "links" '
            '("id", "ident", "creator_id", "created_date", "destination", "enabled", "permanent_redirect") '  # noqa
            'VALUES (%s, %s, %s, %s, %s, %s, %s)',
            (xid, ident, username, created,
             destination, enabled, permanent_redirect,))
        istr = str(i).rjust(len(lstr), "0")
        id_mapping[guid] = xid
        print(f"[{istr}/{lstr}] Migrated Link: {ident} -> {destination}")

    i = 0
    lstr = str(len(accesses))
    for [guid, created] in accesses:  # noqa
        i += 1
        istr = str(i).rjust(len(lstr), "0")
        mapped_id = id_mapping.get(guid)
        if not mapped_id:
            print(f"[{istr}/{lstr}] Migration skipped: unknown link for Guid {guid}")  # noqa
            continue
        xid = gen_id()
        cur.execute(
            'INSERT INTO "stats" '
            '("id", "link_id", "created_date", "user_agent") '  # noqa
            'VALUES (%s, %s, %s, %s)',
            (xid, mapped_id, created, "migrated",))
        print(f"[{istr}/{lstr}] Migrated Stat: {guid} / {xid}")

    targetConn.commit()
    print("Changes have been committed.")


if __name__ == '__main__':
    main()
