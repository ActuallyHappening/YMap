# Progress
Record your progress here for a new AI Agent session to pick up where you left off.
Organise by headers for each session in an append-only fashion.
Don't worry about tokens here, we can purge and summarise later.
Add the date and time when you first started to the header.
Append your progress only to your header, you may not be the only agent writing,
make sure to use the agent coordination scheme in the README.md.
## 2026-11-05 13:00 — iPad app scaffold
- Decided: iPad app first; Mac app later. Conversion copies the pdf (original
  stays). XcodeGen `project.yml` instead of a checked-in `.xcodeproj`.
- Written, never compiled: `project.yml`, `iOS/YMapApp.swift` (folder picker,
  bookmark in UserDefaults), `iOS/FolderView.swift` (browser, tap pdf to
  convert), `Shared/YMapPackage.swift` (creates the package via
  NSFileCoordinator + FileWrapper).
- Blocker: this Mac has only Command Line Tools, no Xcode. Install Xcode, then
  `sudo xcode-select -s /Applications/Xcode.app/Contents/Developer`.
- Next: build and run on iPad, fix compile errors, test with iCloud Drive
  (especially undownloaded `.icloud` placeholders). Then the PencilKit canvas
  for opening a `.ymap` (R1, R2, R7).

## 2026-11-05 13:15 — Agent coordination
- Added `.ai-agent-coordination/coord` (bash) and the README section on it.
  Locks are `mkdir` (atomic); messages are written to a temp file and renamed
  into the recipient's inbox. Tested: 20 concurrent `lock` calls, one winner.
- No automatic breaking of stale locks: doing that without a race needs more
  machinery. A human or agent runs `coord DEADNAME leave` instead.

## 2026-11-05 14:10 — Reload PDF
- Canvas fix: the pdf now sits behind a transparent PKCanvasView and follows
  its scroll/zoom; inside PKCanvasView it was hidden.
- Added Reload PDF (context menu and swipe on a `.ymap` row), shown only when
  the sibling pdf has exactly the name stored in Info.json. Replaces the pdf in
  `Source/`, updates the hash and deletes `Preview/` (regenerated on next save).
- Ink isn't moved if the new pdf's pages differ in size or count.

## 2026-11-05 14:20 — Many pdfs per .ymap
- Format 2: `Source/` holds many pdfs, `Drawings/<pdf stem>.pkdrawing` one per
  pdf, `Info.json` has `sha256` per pdf and `lastOpened`. Format 1 packages
  load; their `Drawing.pkdrawing` moves to `Drawings/` on save.
- Canvas screen: hidden sidebar (toolbar button) lists pdfs; a new
  CanvasController per pdf (`.id(name)`), undo cleared on switch, tool picker
  state shared via `stateAutosaveName`.
- Folder list: long-press folder → Convert Folder to YMap (top-level pdfs
  only). Reload PDFs = folder X/ for X.ymap plus same-named sibling pdfs.
- Untested on device beyond launch: conversion of a folder, switching pdfs,
  opening a format 1 package.
