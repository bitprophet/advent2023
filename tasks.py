from invoke import task


@task
def watch(c):
    """
    Cargo watch oneshot (lint/test/run latest exercise).
    """
    c.run(
        "cargo watch -x 'clippy -- --deny warnings' -x test -x run",
        pty=True,
    )
