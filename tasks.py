from invoke import task


@task
def watch(c, tests=None):
    """
    Cargo watch oneshot (lint/test/run).

    :param str tests: If given, added as parameter to the 'test' subcall.
    """
    base = "cargo watch --clear"
    clippy = "-x 'clippy -- --deny warnings'"
    test = f"-x 'test {tests or ''}'"
    aoc = "-x aoc"
    c.run(f"{base} {clippy} {test} {aoc}", pty=True)
