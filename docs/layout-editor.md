# Arranging Widgets

Widgets are repositioned and resized from the settings dialog, which is the
single place all configuration is edited.

## Turning Arranging On

Open **Settings** from the tray, then flip **Arrange widgets** in the footer.

Two shortcuts open the dialog with arranging already on:

- Tray menu: **Edit Layout**
- CLI: `zokute edit`

## While Arranging

- Widgets show resize handles and a drag area
- Drag to reposition on screen
- Drag a resize handle to change a widget's size
- The X on a widget removes it

## Saving and Discarding

Nothing is written to disk until you save. **Save** applies every change in the
dialog at once — colors, data, layout and startup together. **Discard** returns
everything to the last saved state.

Closing the window with unsaved changes prompts you to save, discard, or keep
editing.

There is no undo. Discard is the way back.

## Moving Everything to Another Display

The tray menu item **Move Widgets to Next Display** shifts every widget one
display forward. A widget on the first display moves to the second, one on the
second moves to the third, and one on the last wraps back to the first. Widgets
move together, so their arrangement across displays is preserved. The change is
saved immediately and survives a restart.

The item is greyed out when only one display is connected.

Positions are carried over unchanged, which matters only for widgets you have
moved or resized. Either action fixes a widget to exact coordinates, so sending
it to a smaller display can leave it partly off screen; **Edit Layout** is the
way to bring it back. Widgets you have never arranged stay tied to a corner or
edge and re-derive their spot from the new display, whatever its size.

Whether a display is connected is checked when Zokute starts. Connect a display
while it is running and you will need to restart before the item becomes
available. Disconnect a display and the item stays enabled but does nothing.
