<div align="center">

<img src=".github/resources/banner.png" width="100%" alt="slot_modded. A bespoke, Game Boy-centric frontend for the Anbernic RG SP.">

<a href="LICENSE"><img src="https://img.shields.io/github/license/BrandonKowalski/slot?style=for-the-badge&labelColor=242429&color=4d4d57" height="36" alt="License"></a>
<a href="https://slot-cfw.fyi"><img src="https://img.shields.io/badge/Guide-slot--cfw.fyi-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="Guide"></a>
<a href="CHANGELOG.md"><img src="https://img.shields.io/badge/Changelog-4d4d57?style=for-the-badge&labelColor=242429" height="36" alt="Changelog"></a>

</div>

---

# About this fork

This is an **unofficial fork** of [slot](https://github.com/BrandonKowalski/slot) by Brandon Kowalski, maintained by
marghost. All credit for the original project goes to Brandon Kowalski. This fork is not affiliated with or endorsed by
him, so please do not report problems with this fork to the original project.

It stays under the same license, GPL-3.0-or-later (see [LICENSE](LICENSE)).

- User guide (for the original slot, which this fork follows): [slot-cfw.fyi](https://slot-cfw.fyi)
- Release notes, including this fork's changes: [CHANGELOG](CHANGELOG.md)

# Changes from the original

- The battery level (gauge + "NN%", a bolt while charging) stays on screen in game, in the bottom-right corner over
  the GBA SP bezel, on 4:3 screens such as the RG35XXSP. It is half transparent, and the gauge fill goes from green to
  yellow to orange to red as the battery drains.
- A **Button Cheat Sheet** entry in the settings menu, between Date & Time and About, lists every button shortcut on
  four pages: carousel, in game, screen & sound, and the save state switcher. D-pad Left/Right or L1/R1 turn the
  pages; B or MENU goes back to the menu. START in the save state switcher (shown as "START Help" in its bottom
  bar) opens it on its In Game page; B goes back to the switcher.

# Try this fork on your device

Both options use the [latest release](https://github.com/marghost/slot_modded/releases/latest). slot's SD card is the
card in SD1 (also labelled TF1) if you use one card, or the card in SD2 if you use two.

## Quick: replace the slot file

For a handheld that already runs slot OS. Your games, saves and states stay as they are.

1. Download [`slot`](https://github.com/marghost/slot_modded/releases/latest/download/slot).
2. Turn the handheld off and put slot's SD card in your computer.
3. In the card's `System` folder, rename the original `slot` file to `slot.orig`. That keeps it as a backup.
4. Copy the downloaded `slot` into the card's `System` folder.
5. Eject the card safely, put it back in the handheld and turn it on.

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
<img src=".github/resources/screenshots/fork-settings-cheat-sheet.png" width="60%" alt="The settings menu with the new Button Cheat Sheet row between Date & Time and About"><br>
<em>The settings menu, with the new Button Cheat Sheet entry.</em>
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

- **Written by AI.** Every change in this fork, both the Rust code and the documentation (this README and the fork
  entry in the [CHANGELOG](CHANGELOG.md)), was written by Claude Opus, Anthropic's AI model, working in Claude Code.
  The original project has its own
  [AI Disclosure](https://github.com/BrandonKowalski/slot#ai-disclosure), which covers only the original project. Its
  code was also put together by Claude Opus, but its documentation is human-written; this fork's documentation is not.
- **Not reviewed by the original author.** Brandon Kowalski has not reviewed, tested or approved anything in this
  fork. Any bug here may be the fork's own, so check whether it also happens on the original slot before reporting it
  there.
- **No warranty.** As the GPL-3.0-or-later license says, this software comes with no warranty. AI-written code can
  contain mistakes that tests do not catch. Use it at your own risk, and keep a backup of your saves and SD card.

---

# Support the original author

If you enjoy slot, consider supporting its original author, Brandon Kowalski, on
[Ko-fi](https://ko-fi.com/kwallbmoc). This fork does not take donations.
