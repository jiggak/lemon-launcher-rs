## Build rom database

Download `genre.ini`

    https://raw.githubusercontent.com/mamesupport/catver.ini/refs/heads/master/genre.ini

Save `mame.xml`

    mame -listxml > mame.xml

Build database

    lemon-launcher scan mame.xml genre.ini path/to/roms/

## Menu format

The menu file requires a `[main]` menu section.

```toml
[main]
entries = [
   # ...
]
```

Optionally, you can define several sub-menu's.

```toml
[menus.sub_menu1]
entries = [
   # ...
]
```

Menu entries come in four flavours:

### Open sub-menu

```toml
entries = [
   { title = "Open Sub Menu", action.menu = "sub_menu1" }
]
```

### Execute command

```toml
entries = [
   { title = "Hello, World!", action.exec = "echo", action.args = ["Hello, World!"] }
]
```

### Launch rom

```toml
entries = [
   { title = "Street Fighter II", action = { rom = "sf2" } }
]
```

Launch rom. Optionally pass extra args to mame with `action.args = "-some arg=val"`.

### Query

> Query action requires a rom database, generated with the `lemon-launcher scan` command.

Query the rom database to generate menu entries.

The format of the `action.query` depends on the type of query:

#### Categories

List of rom categories. Each category entry will open a list of roms in the category.

``` toml
entries = [
   { title = "Categories", action.query = "categories" }
]
```

#### Roms

List of roms, optionally limited to a given category.

```toml
entries = [
   { title = "All Games", action.query = "roms" },
   { title = "Fighting Games", action = { query = "roms", genre = "Fighter" } }
]
```

#### Favourites

List of favourite roms. Favourite status can be toggled in any list of roms
using the "favourite" key (default is 'F').

```toml
entries = [
   { title = "Favourites", action = { query = "favourites", count = 15 } }
]
```

#### Popular

List most played roms. The play count of a rom is incremented each time a rom
is launched.

```toml
entries = [
   { title = "Most Played", action = { query = "popular", count = 15 } }
]
```

## Examples

[Minimal basic](config/minimal/)

[Styled, rom db menus](config/full/)