# Change the Heatbar color for Yakuza 0
This is a patch that allows you to change the color of the heatbar without
changing the exe, ideal when you want to ship your mod without a modified 
EXE _(kinda more trustworthy)_.

# Instructions
In the release tab, there's a file called `released.zip`, extract
all its content where the `Yakuza0.exe` is located. Modify `colors.ini` 
at your taste.

Supported Sections are `[Brawler], [Beast], [Rush], [Legend]`, if you use
them, you should always include the keys `charged` and `uncharged`. If
you don't want to change a color for a fight style, just don't put the
section.

The colors are in hex format (RRGGBB), similar to CSS, you can use
any color generator for that.

Example of a `colors.ini`:

```
[Brawler]
charged=FF0000
uncharged=AA0000

[Rush]
charged=00FF00
uncharged=00FFFF
```

This one will change the colors of `Brawler` and `Rush`, and will left intact
`Beast` and `Legend`.

**Beware**: This will change the colors for both Kiryu and Majima.
