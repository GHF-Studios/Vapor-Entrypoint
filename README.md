# Vapor Entrypoint

Vapor Entrypoint is the Steam-facing terminal adapter for the installed Vapor
app.

It intentionally does not understand Vapor launch targets. Steam starts
`vapor-entrypoint[.exe]`, the entrypoint opens the platform terminal, starts
the matching app-local launch script with the internal `--hold` wrapper flag,
forwards Steam's launch arguments after that flag, waits for the terminal to
close, and exits with that terminal status.

Runtime paths:

```text
Steam
└── bin/<target>/vapor-entrypoint[.exe]
    └── Konsole or cmd
        └── bin/vapor-launch.sh or bin\vapor-launch.cmd
            └── vapor or vapor-installer
```

Logs go to:

```text
<app-root>/.vapor/logs/entrypoint.log
```

The launch scripts, Vapor Installer, and Vapor Shell own all product behavior.
This project owns only app-root discovery, terminal launch, argument forwarding,
and entrypoint logging.
