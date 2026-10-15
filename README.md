<div align="center">

<img src=".github/resources/banner.png" width="100%" alt="slot_modded. A bespoke, Game Boy-centric frontend for the Anbernic RG SP and Anbernic RG35XXSP of course!">

<a href="LICENSE"><img src="https://img.shields.io/github/license/BrandonKowalski/slot?style=for-the-badge&labelColor=242429&color=4d4d57" height="36" alt="License"></a>
<a href="https://slot-cfw.fyi"><img src="https://img.shields.io/badge/Guide-slot--cfw.fyi-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="Guide"></a>
<a href="CHANGELOG.md"><img src="https://img.shields.io/badge/Changelog-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="Changelog"></a>
<a href="https://github.com/marghost/slot_modded/wiki"><img src="https://img.shields.io/badge/Wiki-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="Wiki"></a>

</div>

---

# About this fork

An **unofficial fork** of [slot](https://github.com/BrandonKowalski/slot) by Brandon Kowalski. Not affiliated with or endorsed by him: please report this fork's problems here. Same license, GPL-3.0-or-later.

Some features are adapted from other slot forks and pull requests: [slot.advance](https://github.com/suprsomething/slot.advance)
by suprsomething (battery warnings, standby, favorites), [snowslots](https://github.com/chiyuki0325/snowslots) by
chiyuki0325 (fast-forward pitch), [jgraving/slot](https://github.com/jgraving/slot) by jgraving (favorites and recents),
Markus Maribu's [GB/GBC bezels](https://github.com/BrandonKowalski/slot/pull/77), felipecpaiva's
[faster boot](https://github.com/BrandonKowalski/slot/pull/75) and [stevolarn/slot](https://github.com/stevolarn/slot)
(faster scrolling). Thank you!

**Tuned and tested on the Anbernic RG35XXSP**: GB/GBC bezels fitted to its 640×480 screen, the battery badge in its
SP frame, and every feature checked on the device. The other handhelds slot supports should work as before, but this
fork is not tested on them.

[User guide](https://slot-cfw.fyi) · [Changelog](CHANGELOG.md) · [Wiki](https://github.com/marghost/slot_modded/wiki)

# Documentation - [The Wiki](https://github.com/marghost/slot_modded/wiki)

Everything technical lives in the wiki, one page per topic all the documentation you need:

| Page | What it contains |
|---|---|
| [Settings and Controls](https://github.com/marghost/slot_modded/wiki/Settings-and-Controls) | Every button and settings row the fork adds, with values and defaults |
| [SD Card Files](https://github.com/marghost/slot_modded/wiki/SD-Card-Files) | Files you can add to the card (favorites, recents, boot logo, sticker, bezels) and the `slot.state` keys |
| [RG35XXSP](https://github.com/marghost/slot_modded/wiki/RG35XXSP) | How the 640×480 screen is laid out, with pixel positions, and what is tuned for it |
| [Troubleshooting](https://github.com/marghost/slot_modded/wiki/Troubleshooting) | Where the logs are, common surprises, going back to the original slot |
| [Building](https://github.com/marghost/slot_modded/wiki/Building) | Run on a PC, build for the console, run the tests |

# What's new on this Fork

| Feature | How to get there |
|---|---|
| **Battery %** in game | Always on, any game and any console, bottom-right of the bezel (4:3 screens like the RG35XXSP are compatible). Green → red as it drains. |
| **Button Cheat Sheet** | Settings: MENU → **Button Cheat Sheet**.  This will tell you all the hotkeys! |
| **Button Cheat Sheet** in game | In-game menu: tap MENU → **Button Cheat Sheet**, or in the save state switcher (double-tap MENU) → **START** (Button Help) |
| **Shelves** | Other name for folder, in this modded version you have access to a few of them: Recents, Favorites, GBA, GB or GBC. **L1/R1** switch shelf. |
| **Favorites** | On the shelf, **Y** adds or removes a game to your favorites; a heart marks it. **L1** from GBA opens the Favorites shelf. To set your own icon on the cartridge: put your PNG at `System/favorite.png`. Settings: MENU → General → **Favorites** to show or hide the shelf |
| **Recents** | The first shelf: your last 10 games, newest first. Settings: MENU → General → **Recents** to show or hide it |
| **Startup Screen** | Settings: MENU → General → **Startup Screen** (Recents, Favorites, GBA, GB or GBC).  Choose folder/shelf at boot. |
| **Shelf memory** | Automatic: each shelf opens on the game you left it on, also after a restart |
| **Keep FF Pitch** | Settings: MENU → Gameplay → **Keep FF Pitch** (also in game: MENU → Settings → Gameplay), with Fast Forward Sound on.  This lower the pitch of the game music when you FF.  |
| **Low-battery warning** | Automatic alerts, at 15% and 10% |
| **Standby** | Automatic: with the lid shut, slot stops drawing screen image to save battery |
| **GB / GBC bezels** | On by default in GB and GBC games. Settings: MENU → Screen → **GB / GBC Bezels** to change or turn off |
| **Fullscreen menus** | Automatic on 4:3 screens like the RG35XXSP: the shelves, Settings, About, Date & Time and the Button Cheat Sheet fill the whole screen, no black strip at the bottom |
| **Faster scrolling** | Hold Left or Right for 2 s on a shelf of 60+ games: it moves 3 carts a step |

# Try this fork on your device

Both options use the [latest release](https://github.com/marghost/slot_modded/releases/latest). slot's SD card is the
card in SD1 (also labelled TF1) if you use one card, or the card in SD2 if you use two.

## Quick: replace the slot file

For a handheld that already runs slot OS. Your games, saves and states stay as they are.

1. Download [`slot`](https://github.com/marghost/slot_modded/releases/latest/download/slot).
2. Turn the handheld off and put slot's SD card in your computer.
3. In the card's `System` folder, rename the original `slot` file to `slot.orig`. That keeps it as a backup.
4. Copy the downloaded `slot` into the card's `System` folder.
5. Optional, if you use the gpSP emulator: also download
   [`gpsp_libretro.so`](https://github.com/marghost/slot_modded/releases/latest/download/gpsp_libretro.so) and copy it
   over `System/gpsp_libretro.so`, for the original author's latest gpSP fix.
6. Eject the card safely, put it back in the handheld and turn it on.

To go back to the original slot, delete `System/slot` and rename `System/slot.orig` back to `slot`.

## Fresh install

For a handheld without slot, or to start over. slot runs on top of
[Base OS](https://github.com/pvaibhav/BaseOS), so Base OS has to be installed first. The original author's
[installation guide](https://slot-cfw.fyi/#install) walks through every step, with single and dual SD card setups; in
short:

1. Flash the [latest Base OS release](https://github.com/pvaibhav/BaseOS/releases/latest) to an SD card, for example
   with Raspberry Pi Imager, and insert it in SD1.
2. Turn the handheld on once so Base OS expands its storage, then turn it off when it says no frontend was found.
3. Download `slot-<version>-h700.zip` from the [latest release](https://github.com/marghost/slot_modded/releases/latest)
   and unzip it.
4. Copy everything inside the unzipped `slot-<version>/` folder onto the root of the Base OS card (one card), or onto a
   second card formatted FAT32 or exFAT with an MBR partition table, for SD2 (two cards).
5. Add your games, unzipped, to `Games/GBA`, `Games/GB` or `Games/GBC`, and optionally `gba_bios.bin` to `BIOS/`.
6. Put the card back in the handheld and turn it on.

# Screenshots

<p align="center">
<img src=".github/resources/screenshots/fork-favorite-page.png" width="60%" alt="The Favorites shelf: a pixel heart on the top-left corner of a GBA, a GB and a GBC cart, and FAVORITES on the slot"><br>
<em>The Favorites shelf, with a heart on each favorite.</em>
</p>

<p align="center">
<img src=".github/resources/screenshots/fork-recent-page.png" width="60%" alt="The Recents shelf: the last game played in the middle, with its heart, and RECENTS on the slot"><br>
<em>The Recents shelf: your last games, newest first.</em>
</p>

<p align="center">
<img src=".github/resources/screenshots/fork-settings-cheat-sheet.png" width="60%" alt="The settings menu with the new Startup Screen row set to Favorites, and the Button Cheat Sheet row selected"><br>
<em>The settings menu, with Startup Screen and the Button Cheat Sheet.</em>
</p>

<p align="center">
<img src=".github/resources/screenshots/fork-startingpage.png" width="60%" alt="The settings menu with the Startup Screen row selected and set to Recents"><br>
<em>Startup Screen: pick the shelf slot opens on at boot.</em>
</p>

<p align="center">
<img src=".github/resources/screenshots/fork-keep-ff-pitch.png" width="60%" alt="The Gameplay settings page: Keep FF Pitch selected under Fast Forward Sound, Run-Ahead, and a down arrow under the last visible row"><br>
<em>The Gameplay page: Keep FF Pitch, and ▼ when more settings are below.</em>
</p>

<p align="center">
<img src=".github/resources/screenshots/fork-cheat-sheet-page.png" width="60%" alt="The In Game page of the Button Cheat Sheet, buttons in yellow and what they do in white"><br>
<em>Button Cheat Sheet</em>
</p>

<p align="center">
<img src=".github/resources/screenshots/fork-switcher-battery.png" width="60%" alt="The save state switcher with START Help left of A Load in its bottom bar, and the battery badge in the bezel's bottom-right corner"><br>
<em>The battery percentage in the bottom-right corner, and START Help to open the cheat sheet from the save state switcher.</em>
</p>

---

# Fork AI Disclosure

- **Written by AI.** Every change in this fork, both the Rust code and the documentation in the [CHANGELOG](CHANGELOG.md), was written by Claude Opus.
  The original project has its own
  [AI Disclosure](https://github.com/BrandonKowalski/slot#ai-disclosure). Its
  code was also put together by Claude Opus.
- **Not reviewed by the original author.** Brandon Kowalski has not reviewed, tested or approved anything in this
  fork. Any bug here may be the fork's own, so check whether it also happens on the original slot before reporting it
  there.
- **No warranty.** As the GPL-3.0-or-later license says, this software comes with no warranty. AI-written code can
  contain mistakes that tests do not catch. Use it at your own risk, and keep a backup of your saves and SD card.

---

# Support the original author

If you enjoy slot, consider supporting its original author, Brandon Kowalski, on
[Ko-fi](https://ko-fi.com/kwallbmoc). This fork does not take donations.
