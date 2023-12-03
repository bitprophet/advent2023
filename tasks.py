from invoke import task


@task
def watch(c):
    """
    Cargo watch oneshot (lint/test/run).
    """
    c.run(
        "cargo watch --clear -x 'clippy -- --deny warnings' -x test -x aoc",
        pty=True,
    )
