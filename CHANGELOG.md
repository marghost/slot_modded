# Changelog

Fork versions read `v<slot version>-mod.<feature>.<update>`: the first mod number goes up with new fork features, the
second with upstream merges, fixes and small changes.

## v1.6.0-mod.4.0 (marghost fork, not released yet)

Changes made in this fork since v1.6.0-mod.3.1. Written by Claude Opus (AI), see the
[Fork AI Disclosure](README.md#fork-ai-disclosure):

- Merged the original slot's latest work (main at f62a4a7, not released yet): on 4:3 screens such as the RG35XXSP
  the shelves now fill the whole screen
  ([slot issue #91](https://github.com/BrandonKowalski/slot/issues/91)), MENU → General → **Show Titles** fades the
  shelf name into the selected game's name, Wallpaper Tint reads as a percentage, and RG34XX volume and brightness
  chords.
- Favorites now use the original's storage, from Samuel Alder's work
  ([slot #89](https://github.com/BrandonKowalski/slot/issues/89)): `Config/favorites.txt`. An existing
  `Config/favourites.txt` is moved in once and then removed. The heart, `System/favorite.png`, the Show/Hide row and
  the banner stay as before.
- Recents are now kept in `Config/recents.txt`. Recents saved by v1.6.0-mod.3.1 in `Config/slot.state` are moved
  there on the first save.
- On 4:3 screens such as the RG35XXSP, Settings, About, Date & Time and the Button Cheat Sheet now fill the whole
  screen, with no black strip under them.
- The Button Cheat Sheet has a new look that matches Settings: your wallpaper or theme behind it, each button in a
  white key cap, page dots, and the usual Page / Back hints. It looks the same in game.
- Settings: MENU → General → Favorites shows or hides the Favorites shelf (shown by default). Your favorites and
  hearts are kept while it is hidden.
- In a GB or GBC game, the bezel now stays until the in-game menu or the save state switcher actually opens, instead
  of flashing the GBA SP frame first.
- The save state switcher reads "START Button Help".
- Settings → About: the badge reads "_modded" and the full version (v1.6.0-mod.4.0) sits in the bottom-right corner.

## [v1.6.0-mod.3.1](https://github.com/marghost/slot_modded/releases/tag/v1.6.0-mod.3.1) (marghost fork)

Changes made in this fork since v1.6.0-mod.3 (2026-10-14). Written by Claude Opus (AI), see the
[Fork AI Disclosure](README.md#fork-ai-disclosure):

- Merged the original slot's latest work (main at f63b009, not released yet): hand-drawn carts, slot and shelf by
  Markus Maribu with a light and a dark theme and wallpapers, a General settings page, an in-game menu on a tap of
  MENU, a Shader and a Color Correction row for each system, held directions repeating in menus, logs in a `Logs`
  folder, and the original's own Recents shelf.
- Recents is now the original's: kept in `Config/slot.state` and shown or hidden with MENU → General → Recents. An
  existing `Config/recents.txt` is moved in once, with Recents switched on, and then removed.
- Shelves now run Recents, Favorites, GBA, GB, GBC.
- Settings moved: Startup Screen is on the General page, Button Cheat Sheet sits above About, Keep FF Pitch is also on
  the in-game Gameplay page. The scrolling settings pages are gone, as every page now fits on the screen.
- The in-game menu has a Button Cheat Sheet row. The GB/GBC bezel and the battery badge step aside while the menu is
  open.
- Button Cheat Sheet: lists MENU (tap) for the in-game menu, and B closes it (the footer used to say hold MENU).
- Settings → About reads "_modded v3.1".

## [v1.6.0-mod.3](https://github.com/marghost/slot_modded/releases/tag/v1.6.0-mod.3) (marghost fork)

Changes made in this fork since v1.6.0-mod.2 (2026-10-13). Written by Claude Opus (AI), see the
[Fork AI Disclosure](README.md#fork-ai-disclosure):

- Game Boy and Game Boy Color games show a bezel, on by default, with a set drawn for 640-wide screens such as the
  RG35XXSP, and the GBA SP frame steps aside for them. Settings: MENU → Screen → GB / GBC Bezels picks another PNG from the
  card's `bezels` folders, or None; a picked PNG that is missing falls back to the shipped bezel. (From
  [slot PR #77](https://github.com/BrandonKowalski/slot/pull/77) by Markus Maribu, not merged in the original yet.)
- New RG35XXSP versions of the GB and GBC bezels, whose window fits the game picture exactly, with no black gap.
  They are the default on 640-wide screens; Markus Maribu's originals stay selectable in GB / GBC Bezels.
- Each shelf (GBA, GB, GBC and Favorites) opens on the game it was left on, also after a restart.
- Holding Left or Right for about two seconds on a shelf of 60 games or more moves three carts a step. (Adapted from
  [stevolarn/slot](https://github.com/stevolarn/slot).)
- Fast scrolling no longer shows an empty shelf while the carts catch up.
- With a GB or GBC bezel up, the volume, brightness, rewind and fast-forward bars and the banners show inside the game
  window instead of under the bezel, and the save state switcher and link menu take the bezel away while they are open.
- Faster boot with big Game Boy libraries: a cart's shell is read when it is first drawn, not for every cart before the
  shelf appears. (Adapted from [slot PR #75](https://github.com/BrandonKowalski/slot/pull/75) by felipecpaiva.)
- The slot._modded boot logo is built into `slot`, so it no longer needs `System/bootlogo.bmp`. A logo at
  `Config/bootlogo.bmp` still replaces it.
- Settings → About shows the fork author's picture under the sticker, with "_modded v3" across it.

## [v1.6.0-mod.2](https://github.com/marghost/slot_modded/releases/tag/v1.6.0-mod.2) (marghost fork)

Changes made in this fork since v1.6.0-mod.1 (2026-10-13). Written by Claude Opus (AI), see the
[Fork AI Disclosure](README.md#fork-ai-disclosure):

- Favorites: Y on the shelf adds or removes the selected game, and a pixel heart on the cart marks it on every shelf. A
  Favorites shelf after GBC (L1/R1) lists them A to Z, GBA, GB and GBC together; Y there removes a game. They are kept
  in `Config/favourites.txt`. The heart is `System/favorite.png`: put your own PNG there, or remove it for the built-in
  heart. (Adapted from [slot.advance](https://github.com/suprsomething/slot.advance) by suprsomething and
  [jgraving/slot](https://github.com/jgraving/slot) by jgraving.)
- Recents: a Recents shelf after Favorites holds the last 10 games started, newest first, and opens on the game just
  played. Kept in `Config/recents.txt`. (Adapted from slot.advance and jgraving/slot.)
- Startup Screen on the main settings page: the shelf slot opens on, GBA (default), GB, GBC, Favorites or Recents. An
  empty choice opens the next shelf with games. A game left in the slot still resumes at boot, and ejecting it goes back
  to the startup shelf when the game is on it.
- Keep FF Pitch on the Gameplay page: with Fast Forward Sound on, fast-forward sound keeps its normal pitch instead of
  going higher. (Adapted from [snowslots](https://github.com/chiyuki0325/snowslots) by chiyuki0325.)
- A "Battery Low" banner at 15% and again at 10%, once per discharge; charging resets both. (Adapted from slot.advance.)
- Standby: while the lid is shut, or after a tap of POWER, slot stops drawing until it wakes, to use less battery. The
  3-minute power-off is unchanged. (Adapted from slot.advance.)
- Settings pages with more than six rows scroll, and ▲ ▼ show when there are more rows above or below.
- A new boot logo: "slot." with "_modded" under it.
- The Button Cheat Sheet's carousel page lists Y (add / remove favorite), and L1/R1 now switch shelf.
- Includes the original slot's latest changes, not in a slot release yet: a 1 frame Run-Ahead setting for GBA games on
  mGBA, brightness steps on a curve so the lowest steps are dimmer, and steady frame rate while the sound is muted.

## [v1.6.0-mod.1](https://github.com/marghost/slot_modded/releases/tag/v1.6.0-mod.1) (marghost fork)

Changes made in this fork, on top of slot 1.6.0 (2026-10-12). Written by Claude Opus (AI), see the
[Fork AI Disclosure](README.md#fork-ai-disclosure):

- In game, on 4:3 screens (RG35XXSP), a battery gauge and the battery percentage stay on screen in the bottom-right
  corner, over the GBA SP bezel. A bolt appears while charging. The badge is half transparent, and the gauge fill goes
  from green to yellow to orange to red as the battery drains.
- A Button Cheat Sheet entry in the settings menu, between Date & Time and About, lists every button shortcut on four
  pages: carousel, in game, screen & sound, and the save state switcher. D-pad Left/Right or L1/R1 turn the pages; B or
  MENU goes back to the menu.
- START in the save state switcher opens the Button Cheat Sheet on its In Game page; B goes back to the switcher. Its
  bottom bar shows "START Help" left of "A Load", except while the undo hint is offered.

## [1.6.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.6.0)

- Runs on the RG28XX: its panel is mounted portrait, so slot turns the picture onto it. (#35)
- Pokémon Pinball: Ruby & Sapphire rumble. mGBA now sees the game's Game Boy Player check. You still need to turn rumble on in game settings. (#25)
- A red alarm shows when the SD card turns read-only or a save fails to write.
- A new shortcut for mute: SELECT + VOL+ / VOL-. The 34XX has a fake volume wheel so VOL+ + VOL- isn't possible. (#39)
- Game Boy cartridge clocks are kept in the .sav. (#22)
- The GBA cartridge clock keeps running on gpSP while the game is off.
- Large libraries now should boot faster, and the first frame shows sooner.
- Games start faster: resting on a cart reads its ROM ahead, so big ROMs no longer wait on the SD card when you insert them.
- Wallpapers are dealt from a shuffled deck, so none repeats until all have shown.
- PNG art is turned the way its EXIF orientation tag says.
- When slot crashes it logs where, and keeps that run's log in `crash/` to attach to a bug report.
- The BaseOS startup sound no longer leaves slot silent. (#20)
- A user defined `Config/bootlogo.bmp` will take precedence over the shipped one. (#30)
- Cart shell, core and video mode files are now `.txt` instead of `.ini`, so Chrome on Windows opens them. slot renames them on first boot. (#15)
- The grid shader's lines stay even on the RG34XX and at fractional scales, and the dot shader no longer darkens a white screen as much. (#26, #36)

## [1.5.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.5.0)

- Rumble regression fixed.
- The boot logo is built to the size of the one BaseOS installed, so it fits every panel instead of only 720x480.
- Runs on the RG35XXSP: the picture fills the 4:3 screen's width and a GBA SP bezel sits underneath while a game plays.
- Screen shaders chosen per platform: Off, LCD3x, Grid or Dot for GBA, and Off, Grid or Simpletex for Game Boy and Game Boy Color.
- SELECT + B / A in game steps back and forth through the screen shaders. A game's own soft reset that holds SELECT with A or B no longer reaches it; hold A on the carousel to start a game fresh instead.
- New settings: auto save on eject, turbo buttons and rewind can each be switched off.
- Settings are grouped into Screen and Gameplay pages, and every value wraps from the last choice to the first.
- Label art is cached on the card, so the shelf no longer stutters building cart faces. New labels are cached once at boot behind a Caching New Labels screen.
- Named palettes for Game Boy games: turn on GB Palettes in the Screen settings, then SELECT + L2 / R2 in game steps back and forth through mGBA's 48 palettes.
- The speaker no longer buzzes while nothing is playing. slot lets go of the audio device after 3 seconds of silence and takes it back when sound starts. Thanks to flo333 for finding it.

## [1.4.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.4.0)

- The volume bar shows a headphones icon while they are plugged in.
- Speaker and headphones keep their own volume and mute, and switch the moment you plug / unplug.
- Settings moved from `System/` to a new `Config/` folder, so replacing `System/` on update no longer resets them. slot moves them on first boot.
- The quick menu and power menu wrap from the last item to the first, and back.

## [1.3.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.3.0)

- The SP boots to a slot logo instead of BaseOS's. slot installs `System/bootlogo.bmp` on first run and keeps the original as `bootlogo.baseos.bmp`.

## [1.2.2](https://github.com/BrandonKowalski/slot/releases/tag/v1.2.2)

- Fix for hitch in games caused by autosave. The autosave now writes to the card in the background.

## [1.2.1](https://github.com/BrandonKowalski/slot/releases/tag/v1.2.1)

- Shell colors from Cart Studio now apply to games with accented names.
- Clear GBA carts no longer show their circuit board above the label.

## [1.2.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.2.0)

- Hold X for turbo A and Y for turbo B.
- Carts on the shelf are larger, and shrink into the slot as they go in.
- Carts are drawn at their real proportions, so full label scans fit without cropping.
- Jumping a letter with Up or Down shows the letter in the slot.
- Saves now stick for games whose save is smaller than the core first reports, such as Golden Sun on mGBA.
- The seated cart no longer shows through the slot as a game starts.
- SELECT reaches the game as soon as it can no longer be the start of a shortcut.

## [1.1.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.1.0)

- Fast-forward runs several game frames per refresh, so it is properly fast, and rewind no longer records while fast forwarding.
- The About label credits BaseOS.

## [1.0.0](https://github.com/BrandonKowalski/slot/releases/tag/v1.0.0)

- Game Boy and Game Boy Color, each on its own shelf. L1 and R1 switch shelves, and carts are drawn at their own size in their own plastics.
- Two cores, mGBA and gpSP. Press START on a cart to open it and swap the chip.
- Link play between two RG SPs over WiFi: the link cable and the Wireless Adapter for Pokémon trades and Advance Wars, mGBA's link mode for Mario Kart, and Game Boy link for Tetris.
- A quick menu on the shelf for fast forward speed and sound, rumble, color correction, the date and time, and About.
- Game Boy games stretch to fill the screen with L1 and go back with R1.
- The real BIOS boot animation plays when the card has one.
- Per-cart shell colors, set in `cart_shell.ini`.
- Up and Down jump through the shelf a letter at a time.
- Your own folder layout on the card is kept, and slot runs on upstream BaseOS.

## [0.1.1](https://github.com/BrandonKowalski/slot/releases/tag/v0.1.1)

- The game holds still behind the power menu.
- Rewind records at every speed and replays exactly what happened.

## [0.1.0](https://github.com/BrandonKowalski/slot/releases/tag/v0.1.0)

- First release: a GBA-only shelf of cartridges with an insert scrape and clunk.
- The game in the slot resumes on the next boot. Closing the lid sleeps and saves.
- Save states with a switcher, rewind, fast-forward, brightness, blue light and volume.
- A power menu on a held POWER, and an About label.
