WASM 2D experiments
===================

Random circles (2000):

| Language/Library      | Graphics API                | Env.     | FPS     |
|-----------------------|-----------------------------|----------|---------|
| Javascript            | WebGL                       | Firefox  | 60 fps  |
| Javascript            | Canvas API                  | Firefox  | 12 fps  |
| Javascript            | Canvas API                  | Chromium | 12 fps  |
| WASM/Piet             | Canvas API                  | Firefox  | 12 fps  |
| WASM/Piet             | Canvas API                  | Chromium | 12 fps  |
| WASM/raqote           | Canvas API                  | Firefox  | 9  fps  |

Random circles (400):

| Library               | Graphics API                | Env.     | FPS     |
|-----------------------|-----------------------------|----------|---------|
| WASM/quicksilver/lyon | WebGL                       | Firefox  | 18 fps  |
| WASM/quicksilver/lyon | WebGL                       | Chromium | 15 fps  |
| WASM/quicksilver      | WebGL                       | Firefox  | 8 fps   |


Canvas API comparisons:

| Language/Library      | Graphics API / variation     | Env.     | FPS     |
|-----------------------|------------------------------|----------|---------|
| Javascript            | Canvas API                   | Firefox  | 25 fps  |
| Javascript            | Canvas API                   | Chromium | 34 fps  |
| Javascript            | Canvas API / 2nd Canvas      | Firefox  | 21 fps  |
| Javascript            | Canvas API / non-transparent | Firefox  | 28 fps  |
| Javascript            | Canvas API / 1 color         | Firefox  | 31 fps  |
