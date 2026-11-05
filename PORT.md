# Porting YMap to Mind

Everything learned building YMap, for starting the new app **Mind** in
`~/Developer` with its own GitHub repository. YMap's code is in this repo at
commit `e50e92d`. Read README.md for the requirements (R1 to R13) and
PROGRESS.md for the history.

## The user and how they want to work

- They want pdfs (questions, solutions, lecture notes) annotated with Apple
  Pencil, with the notes stored as files next to the pdfs in iCloud Drive.
  Files and Finder come first: no separate library, no OneNote.
- Write as little code as possible; they will read it. Few comments, only
  where something isn't obvious. Few docs.
- README.md holds requirements and decisions; lines are kept to 80 characters.
  PROGRESS.md is append-only, one dated header per session.
- Pause and ask when a design choice is theirs (e.g. "if folder context menus
  aren't simple, ask me").
- Commit only when asked. They sometimes commit themselves.

## Machine and toolchain

- Mac on macOS 27.2, Xcode 27.2 (27B52), iOS SDK 27.2, XcodeGen 2.46
  (`/opt/homebrew/bin/xcodegen`). The `.xcodeproj` is generated from
  `project.yml` and git-ignored, as are `build/` and `iOS/Info.plist`.
- Already done once, not needed again: accepting the Xcode license, signing
  in to Xcode with the Apple account, Developer Mode on the iPad, trusting the
  developer profile on the iPad (Settings ▸ General ▸ VPN & Device
  Management).
- Signing: team `8495B78D24`, identity "Apple Development:
  calebyates42@gmail.com". `-allowProvisioningUpdates` makes the profile.
  A new bundle ID (for Mind) gets its profile the same way; the iPad may ask
  to trust the profile again. Free personal-team profiles expire after 7 days.
- No iOS Simulator runtime is installed; all testing is on the real iPad.
- iPad: iPad Pro 11-inch (3rd generation), iPad13,4, UDID
  `00008103-001560CE01E3401E`, connected over USB.

Build, install and launch:

```
xcodegen generate
D=00008103-001560CE01E3401E
xcodebuild -project YMap.xcodeproj -scheme YMap-iOS -destination "id=$D" \
  -derivedDataPath build -allowProvisioningUpdates build
xcrun devicectl device install app --device $D \
  build/Build/Products/Debug-iphoneos/YMap.app
xcrun devicectl device process launch --terminate-existing --device $D \
  com.calebyates.ymap
```

Add `--console` to the launch to see `print` output from the app.

## What YMap does (all on iPad, SwiftUI + UIKit, iOS 18 deployment target)

- Root: pick an iCloud Drive folder with `fileImporter`; its security-scoped
  bookmark is kept in `@AppStorage`. A `NavigationStack` of folder lists, with
  the path in an `@Observable Navigator` so code can push a document.
- Folder list: tap a pdf → creates or opens `X.ymap`. Long-press a folder →
  "Convert Folder to YMap". Long-press or swipe a `.ymap` → "Reload PDFs".
  Each `.ymap` row shows its `Preview/thumbnail.png`.
- Document screen: one pdf at a time on a PencilKit canvas, with a sidebar,
  hidden by default, listing the package's pdfs. Undo and redo in the toolbar.
- `.ymap` is declared as an exported UTI (`com.calebyates.ymap`, conforms to
  `com.apple.package`) with `CFBundleDocumentTypes` and
  `LSSupportsOpeningDocumentsInPlace`, and `.onOpenURL` pushes it, so Files
  can open it in the app. Not tested.

## Package format 2 (`.ymap`)

```
X.ymap/
├── Info.json        formatVersion, id, created, sha256 {pdf: hash}, lastOpened
├── Source/*.pdf     the pdfs; changed only by Reload PDFs
├── Drawings/<pdf name without .pdf>.pkdrawing   one PKDrawing per pdf
└── Preview/thumbnail.png   first pdf's page 1 + its ink; safe to delete
```

- Drawings are in points relative to the top-left of the pdf's first page,
  not to on-screen positions, so the file doesn't depend on the UI.
- Format 1 had one pdf, `source`/`sourceSHA256` in Info.json and a single
  `Drawing.pkdrawing`. Making new Info fields Optional lets format 1 decode.
- Default pdf: `lastOpened`, else the first by `localizedStandardCompare`,
  with names matching `(?i)(^|[^a-z])sol(ution)?s?([^a-z]|$)` last (tested:
  "Q4_sol", "Q4 Solutions", "SOL" are solutions; "Console", "Solve" aren't).
  Use `#/…/#` regex literals; bare `/…/` needs a compiler flag in Swift 5 mode.
- Reload sources: every pdf in folder `X/` beside `X.ymap`, plus any pdf
  beside the package named exactly like one in `Source/`. Adds or replaces,
  never removes, keeps drawings, deletes `Preview/`.

## Technical lessons

- **Opening and saving:** `UIDocument` with a `FileWrapper` package does the
  file coordination, autosave, iCloud download and reload-on-remote-change
  for little code. `load(fromContents:)` runs on the main thread; keep and
  mutate the loaded wrapper and return it from `contents(forType:)`. Conflicts
  currently keep this device's version (`NSFileVersion`), which loses the
  other device's strokes; a stroke merge would be better.
- **Writing outside UIDocument** (create, reload): `NSFileCoordinator`
  coordinated reads also download undownloaded iCloud files.
- **iCloud placeholders:** files not yet downloaded are listed as
  `.Name.pdf.icloud`; map them back to their real names.
  `startDownloadingUbiquitousItem` before opening a document.
- **The pdf under the ink:** a subview inserted inside `PKCanvasView` is
  hidden by PencilKit's own views. What works: canvas `backgroundColor =
  .clear`, `isOpaque = false`, the pdf view as a sibling behind it, and in
  `scrollViewDidScroll`/`scrollViewDidZoom` set the pdf view's transform to the
  zoom scale and its origin to `origin * zoom - contentOffset`.
- **Known bug, not fixed:** that pdf view draws over the sidebar and the
  navigation bar buttons (but not the PencilKit tool picker). Probably fixed
  by `view.clipsToBounds = true` on the canvas controller, or better, by a
  design where the pdf lives in the same scroll/zoom container as the ink.
- **Sharp pdf at any zoom:** a `UIView` backed by `CATiledLayer`
  (`levelsOfDetail 4`, `levelsOfDetailBias 5`, 512pt tiles) drawing
  `CGPDFPage`s: flip the context, then
  `getDrawingTransform(.cropBox, rect:rotate:0, preserveAspectRatio:true)`.
  Swap width and height for pages rotated 90° or 270°.
- **Canvas size:** `PKCanvasView` doesn't grow by itself. Set `contentSize =
  area * zoomScale` after every drawing change. Put the pdf at a 1000pt
  margin (more if ink lies further left or up) so there's room to draw on all
  sides; the drawing is translated by that origin when shown and back when
  saved.
- **Pencil:** `PKToolPicker` handles Pencil double-tap (eraser switch) on its
  own. `stateAutosaveName` keeps the chosen tool across canvases. Undo on
  `canvas.undoManager`; clear it when switching to another pdf's canvas.
- **Dark mode:** set the canvas controller to `.light`, and render ink for
  thumbnails inside `UITraitCollection(userInterfaceStyle: .light)
  .performAsCurrent`, otherwise PencilKit inverts ink colours.
- **SwiftUI:** a `UIViewControllerRepresentable` with `.id(pdfName)` gives a
  new controller per pdf; pass a counter to tell it to reload after a remote
  change. A weak handle class lets toolbar buttons reach the controller.
  `contextMenu` and `swipeActions` on `List` rows are simple; folder rows can
  have context menus too.
- **Security scope:** call `startAccessingSecurityScopedResource` on the root
  folder (children are covered) and on URLs opened from Files.

## Still open from YMap

- Quick Look thumbnail extension so Files and Finder show previews.
- Mac app (R9, R11, R12): double-click in Finder opens it; view and manage.
- Live two-device sync and conflict merging untested.
- Opening `.ymap` from the Files app untested.

## Repo conventions carried over

- `.ai-agent-coordination/coord`: bash tool for several AI agents at once
  (join, lock folders, message). State lives in `.git/ai-agent-coordination/`.
  It needs a working `git` (it failed while the Xcode license was unaccepted
  and wrote its state into the repo instead).
- Commit messages end with a `Co-Authored-By` line for Claude.
