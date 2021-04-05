# pakemon-rust

monorepo of new rust rattata/pakemon ideas

- these all need cute names (rattata has "rats" maybe, for plugins, for example)
- it's all rust, initially
- need a good generic plugin system. See [this](https://github.com/luojia65/plugin-system-example). Make sure plugins work on wasm, too, or use separate plugin system for backend vs pakemon
- frontend needs a backend to do anything useful, but can be on the same machine

## sub-projects:
  - rattata
    - backend
      - plugin system
      - grpc
      - tor/encryption for remote, no tor/encryption for local
      - orchestrator plugin (calls back to client and says "I am here, and I have these resources" and allows install/run of other plugins) this creates a single "minion"
      - preload other rats in payload (to make it faster & autonomous to do things like install as windows-service, etc)
    - client lib (librattata)
      - grpc to talk to minion, controls orchestrator, developed in parallell with backend+orchestrator plugin
      - list minions
      - install rats
      - run rats with params
      - remote shell for getting down & dirty
      - file upload/download
    - CLI frontend (rattata-cli)
      - use client lib to control minions
      - develop in parallell with lib
      - remote shell for getting down & dirty
      - file upload/download
  - game frontend (pakemon)
    - use client-lib to control minions
    - pick a game framework that works well with web (so I can make a web-based game with same code) like quicksilver, or need to look into bevy. It looks amazing, but currently no wasm (but they are working on it)
    - mini-games are plugins, maybe even compiled to wasm or using a scripting language, so they are cross-platform and can be zipped up with assets and stuff
    - plugins for other types of hacking (radio, electronic tools, etc)
    - plugins for non-hacking things, for cover, like other minigames, emulators, etc
  - game screen editor
    - just simple placement of 2D things in an ordered graph like images/shapes/text
    - interchange format (JSON, probly) so it can be loaded/saved
    - library to display the graph in quicksilver, and allow user to hook into things in rust (for animation/interaction/sound/etc)
  - docker
    - backend
    - proxy to convert web to grpc (for wasm frotnend)
    - webserver with pre-compiled pakemon on it
    - demo targets
  - OS mods (for gameboy-like pakemon runner)
    - very light, just runs absolute minimum to get pakemon going, no backend by default
  - OS mods (for headless keychain/powerstrip/etc)
    - USB injector stack (to inject backend payload on target over USB)
    - docker (to run above docker stack without demo-targets)

## development

You need `cargo` and `make` installed.

```sh
make help                           show this help
make install                        setup dev-tools used by other targets
make pakemon                        run frontend on current platform
make rattata                        run backend server (payload) on current platform
make rattata-cli                    run CLI frontend on current platform
make release-linux                  create a release for linux (x86_64)
make release-osx                    create a release for osx (x86_64)
make release-win                    create a release for windows (x86_64)
```