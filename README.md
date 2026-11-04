# YMap
YMap - Yates Mind Map.
A mind mapping software for iPad pencil note taking, synced with iCloud, with the ability to view drawings on MacOS.
This app does nothing else.
This app allows editting and scribbling on iPad.
The app allows viewing and managing drawings on MacOS.

## Vision
I want to take notes on my iPad using my Apple Pencil so I get the double tap to erase functionality.
I want the notes to be in a custom file format, which losslessly stores everything relevant for a specific
question or lecture.
So, say I want to solve a past exam question, firstly I prepare the relevant LaTeX paragraph (using AI)
including fonts and sizing and everything else. This plain text is then rendered
into an image on my Mac, which I then export

> _(The sentence above is unfinished in the original. It is kept exactly as written; the
> requirements below continue the thought.)_

---

## Requirements log (verbatim)

Everything I ask for is copied here word for word, newest at the bottom, so nothing gets lost
when it is summarised further down.

### 2026-11-04 — first brainstorm request
> Help me brainstorm this project. Read what I have so far. I want to do everything losslessly,
> write this down and everything else here in the README.md.
> I want to prepare a space on my Mac, with perfect live updating and syncing between Mac and iPad,
> this space having the original question image. Then on my iPad in a UI which is centered around
> the file system of the iCloud where I already store all my notes and want to work in, I want to
> select this newly prepared file and it open up onto a scribbleable space where I can draw around
> it. I want the original image to be stored permanently seperately to the full scribble data, I
> want the scribble data to be stored in Apple's format so no custom conversions, I want this
> completely lossless and future proof with Apple's systems. Record all of this as I go.

### 2026-11-04 — feedback on the first design
> I like this design! It is Finder focussed and file system focussed, which I like, record this
> opinion. Can you confirm that double clicking this in Finder will open the file package on both
> MacOS and iPad OS? I really care about the macOS link opening funcionality, don't let this
> technical requirement on iOS hold annythign back.

### 2026-11-04 — decisions on Mac links and app structure
> Reject the ymap:// proposal. I like the two native app design.

### Requirements extracted from the above
| # | Requirement | Source |
|---|-------------|--------|
| R1 | Notes are taken on iPad with Apple Pencil; Pencil double-tap switches to the eraser. | Vision |
| R2 | One file per question or lecture holds everything relevant to it, losslessly. | Vision |
| R3 | The question starts as LaTeX (written with AI help, including fonts and sizing), rendered to an image on the Mac. | Vision |
| R4 | The Mac prepares a "space" (a new file) that contains the original question image. | 2026-11-04 |
| R5 | That space syncs between Mac and iPad and updates live. | 2026-11-04 |
| R6 | The iPad UI is built around the iCloud Drive folders where all my notes already are. No separate library. | 2026-11-04 |
| R7 | Opening the file on iPad gives a canvas with the image on it, and I can draw around the image. | 2026-11-04 |
| R8 | The original image is stored permanently, in its own file, apart from the drawing data. | 2026-11-04 |
| R9 | The drawing data is stored in Apple's own format (PencilKit), with no custom conversion. | 2026-11-04 |
| R10 | Everything is lossless and future-proof as Apple's systems change. | 2026-11-04 |
| R11 | The Mac can view and manage drawings. It does not need to edit them. | Original header |
| R12 | The app does nothing else. | Original header |
| R13 | Requests and decisions are written down in this README as the project goes. | 2026-11-04 |
| R14 | Opening a `.ymap` on macOS by double-clicking it in Finder, or by following a link to it, opens it straight in YMap. This is a top priority. | Feedback 2026-11-04 |
| R15 | Opening it from the Files app on iPad opens it in YMap too. | Feedback 2026-11-04 |
| R16 | iPad needs never limit the Mac side. The Mac app is designed on its own terms. | Feedback 2026-11-04 |

### My opinions so far
- **2026-11-04:** I like that the design centres on Finder and the file system. Files and folders I
  can see, move and back up are the main way I interact with my notes. They are not hidden
  inside an app.
- **2026-11-04:** I like having two separate native apps, one for Mac and one for iPad.
- **2026-11-04:** I don't want a `ymap://` URL scheme. Normal Finder and `file://` opening is
  enough.

---

## Brainstorm

### 1. The file: a `.ymap` document package

A `.ymap` file is a **package**: a folder that Finder and the Files app show as a single file
(the same way `.pages` or `.rtfd` files work). Each part is kept as its own file inside it, so
the original image never shares a file with the drawing data (R8).

```
Past Exam 2023 Q4.ymap/            <- shows up as one file in Files / Finder
├── Info.json                      <- small manifest: format version, IDs, dates, hashes, image position
├── Source/                        <- written once on the Mac, never changed afterwards
│   ├── question.tex               <- the LaTeX exactly as written
│   ├── question.pdf               <- the rendered question (vector, sharp at any zoom)
│   └── question.png               <- optional raster copy at a fixed DPI, also never changed
├── Drawing.pkdrawing              <- PKDrawing.dataRepresentation(), written as-is (Apple's format)
└── Preview/                       <- generated from the files above and safe to delete
    └── thumbnail.png              <- for Files / Finder / Quick Look
```

Notes on each part:

- **`Source/` cannot change.** The Mac writes it once. The iPad only reads it. `Info.json` records
  a SHA-256 hash of every source file, so the app can detect if one is ever altered or damaged.
- **`Drawing.pkdrawing`** holds the exact bytes from Apple's `PKDrawing.dataRepresentation()`, and
  the app reads it back with `PKDrawing(data:)`. Nothing is translated, so no custom conversion
  happens anywhere (R9). PencilKit saves every stroke point in full: location, time offset, force,
  azimuth, altitude, size and opacity. Nothing is simplified away.
- **The image is not baked into the drawing.** It is shown as a separate layer under a transparent
  `PKCanvasView`. `Info.json` records where it sits in canvas coordinates (origin and size in
  points), so the drawing and the image always line up. The drawing data never contains image pixels.
- **Why PDF is the main image:** a LaTeX render is vector by nature. A PDF keeps it sharp at any
  zoom, which is lossless in the real sense. The PNG is an optional fallback and is never the
  only copy.
- **Why also keep `question.tex`:** it is the true original. If the image is ever lost, or a
  better renderer comes along, it can be rendered again. The AI prompt that produced it could be
  kept as `Source/prompt.md` too (see open questions).
- **Everything in `Preview/` can be rebuilt** from the other files and is never the source of truth.

### 2. Future-proofing PencilKit (R10)

- Apple keeps `PKDrawing` data readable in newer OS versions: drawings saved years ago still open.
  The format itself is private (not publicly documented), so only Apple platforms can read it.
  That matches what I asked for ("future proof with Apple's systems").
- New ink types (for example the iOS 17 additions) are controlled by **`PKContentVersion`**. Record
  `drawing.requiredContentVersion` in `Info.json` on every save, so it is clear which OS version
  a file needs. Older devices can show a placeholder instead of drawing it wrong.
- Write the PencilKit bytes as-is. Never re-encode or merge them with anything else.
- Optional safety net: a flattened `Preview/full.pdf` export, regenerated on save. It can be read
  without PencilKit, but it is only a convenience and never the source.
- `Info.json` carries `formatVersion: 1`, so the layout can change later without guesswork.

### 3. iPad app: built around the iCloud Drive files (R6, R7, R1)

- Use a **SwiftUI `DocumentGroup`** (which is `UIDocumentBrowserViewController` underneath). The
  app opens directly into the standard Files browser, already showing my iCloud Drive folders.
  There is no separate library, import step or app-only database.
- The app declares the `.ymap` type (`UTExportedTypeDeclarations`, conforming to
  `com.apple.package`), so `.ymap` files show as single files with an icon, open with a tap and
  get thumbnails.
- Opening a file shows a zoomable canvas:
  - the question PDF drawn at its recorded position, behind everything else,
  - a transparent `PKCanvasView` on top, much larger than the image, so there is room all around
    it to work ("draw around it"),
  - `PKToolPicker`, which gives Pencil double-tap to eraser automatically through the system's
    Pencil preference (R1). On newer Pencils, squeeze and the tool palette work too.
- Saving goes through `UIDocument` / `FileWrapper`, which uses `NSFileCoordinator` under the
  hood. That makes it safe with iCloud, autosaves, and supports undo.
- The app never writes to `Source/`. That folder is opened read-only.

### 4. Mac side: preparing the file (R3, R4) and viewing it (R11)

**Preparing (a script can do the work, but see section 6: a small Mac app must exist from day
one, or Finder shows `.ymap` files as plain folders):**
1. Write or generate the LaTeX, using the `standalone` document class so the output is cropped
   tightly to the content, with fonts and sizing set in the preamble.
2. Render to PDF with `tectonic` or `pdflatex`/`lualatex`. Optionally also make a PNG with a
   fixed DPI.
3. Build the `.ymap` package in a **temporary folder**: `Source/`, `Info.json` with hashes and the
   image position, and an empty drawing (an empty file, or no file until the iPad first saves).
4. **Move it into the iCloud Drive folder in one step** (`mv` within the same volume). The iPad
   then never sees a half-written file.

**Viewing and managing (later, a small Mac app):**
- PencilKit works on macOS, so `PKDrawing(data:)` plus `drawing.image(from:scale:)` can render
  the drawing on top of the PDF. This is **viewing only** (R11).
- Use an `NSFilePresenter` / `NSDocument`, so a change saved on the iPad redraws on the Mac by
  itself. That covers the "live" part of R5.
- A **Quick Look extension** gives Finder previews and spacebar previews of `.ymap` files.
- To verify: whether native macOS has an editing `PKCanvasView` or only Mac Catalyst does. This
  doesn't matter while the Mac only views.

### 5. Sync and "perfect live updating" (R5): what is realistic

- The sync layer is **iCloud Drive itself**: the same folders where my notes already are. There
  is no custom server and no CloudKit database. The files are the truth.
- **Honest limit:** iCloud Drive is not real-time collaboration. Changes usually arrive within
  seconds to a minute, sometimes longer. "Perfect" is reachable for *correctness*: no data loss,
  no corruption, every change eventually shows up everywhere. *Speed* depends on iCloud.
- **No conflicts, by design:** the Mac writes only `Source/` (once, at creation). The iPad writes
  only `Drawing.pkdrawing`. The two devices never write the same file, so iCloud should never
  need to resolve a conflict between them. If one ever happens anyway (for example two iPads),
  `UIDocument` exposes `NSFileVersion` conflict versions. Keep every version and never silently
  pick one, to stay lossless.
- If the Mac app is open on a file while the iPad edits it, `presentedItemDidChange` makes it
  reload.
- To verify: iCloud Drive handles a package as one item. Check how quickly a change to one file
  inside a package reaches the other device, and whether files are downloaded on demand ("Keep
  Downloaded" may be needed on the iPad).

### 6. Opening `.ymap` files: Finder double-click and links (R14, R15, R16)

**Short answer:** yes on both platforms, **but only on a device that has a YMap app installed
that declares the `.ymap` type.** A package isn't special to the disk; it's an ordinary folder.
The OS shows it as one file, and knows what opens it, only because an installed app says so.
Without that app, Finder and Files show a plain folder, and double-clicking just opens the folder.

**macOS (the priority):**
- The Mac app's `Info.plist` declares:
  - `UTExportedTypeDeclarations`: the type `com.yates.ymap` (name to be decided), extension
    `ymap`, conforming to `com.apple.package` and `public.composite-content`. This is what makes
    Finder show the folder as a single file with an icon.
  - `CFBundleDocumentTypes`: YMap is the `Owner` (`LSHandlerRank`) of that type, with
    `LSTypeIsPackage = true`. This is what makes double-click open the file in YMap.
- Launch Services registers this once the app is in `/Applications` (or has been run once).
  From then on, these all go through the same path and open the file in YMap:
  - double-clicking in Finder,
  - `open "Q4.ymap"` in Terminal,
  - a `file:///…/Q4.ymap` link clicked anywhere: other notes, Markdown, a browser, Spotlight
    results, recent documents, Dock icon drops.
- ~~**Optional extra: a `ymap://` URL scheme.**~~ **REJECTED 2026-11-04.** Plain Finder and
  `file://` opening only. The original proposal is kept below for the record: `file://` links break when a file moves, and some
  apps refuse to open them. The Mac app could also register a scheme, for example
  `ymap://open?path=…` or `ymap://id/<UUID>`, where the UUID comes from `Info.json` and is
  found with Spotlight. Then a link keeps working after the file is moved or renamed. This is a
  Mac-only feature and doesn't need anything from iPad (R16).
- **Consequence for the plan:** the "script only, no app yet" first step isn't enough for R14.
  The first deliverable becomes a **minimal Mac app**: a type declaration, a document window
  that shows the PDF plus the drawing, and a Quick Look preview. Preparing new files can still be
  a script, or a menu command in that app.
- Fallback without an app: Finder can show a folder as a package if the folder's "bundle bit" is
  set (`SetFile -a B`), but nothing would know how to open it. It's also unverified whether
  iCloud Drive keeps that flag. Not relied on.

**iPadOS:**
- There's no double-click on iPad: a single tap in the Files app opens a file. The iPad app makes
  that work by declaring the same type and the document types, plus `LSSupportsOpeningDocumentsInPlace`
  and `UISupportsDocumentBrowser`. Files then shows `.ymap` as a single file and opens it in
  YMap, editing it in place inside iCloud Drive with no copy made.
- Without the iPad app installed, Files shows a folder (fine, but it doesn't open).

**The Mac and iPad apps are independent (R16). APPROVED 2026-11-04:**
- The only thing they share is the type identifier and the package layout from section 1. Each
  is a separate native app: AppKit/SwiftUI on the Mac, not Mac Catalyst. So no iPad limitation
  shapes the Mac app.
- (Original wording, now replaced by the approval above: "Mac Catalyst stays an option only if
  it ever turns out easier; it isn't the default.")
- Both apps can declare the type. Having the Mac app export it and the iPad app import the same
  identifier (or both export it, since it's the same developer) is standard.

**To verify with a real test build:** create a minimal Mac app with these declarations, put a
`.ymap` folder in iCloud Drive, and check: (a) Finder shows it as one file, (b) double-click and
a `file://` link both open the app, (c) the iPad Files app shows it as one file after the iPad
app is installed.

### 7. Alternatives considered and rejected

| Option | Why not |
|--------|---------|
| Apple Notes / Freeform | No file I own and no API. Data lives in Apple's database, not my iCloud Drive folders. |
| A PDF annotated with PencilKit (PDFKit overlay) | Saving turns strokes into PDF ink annotations, which loses data. The drawing also ends up merged into the question's file, breaking R8. |
| Drawing data in a custom format (SVG, JSON points) | Breaks R9. A custom conversion is exactly what I don't want. |
| CloudKit / a custom sync server | Duplicates iCloud Drive and moves notes out of the folders I already use (R6). |
| Flattening the image into the canvas | Loses the separate, unchanging original (R8). |

---

## Open questions

1. **Several images or pages per file?** For example a question plus a diagram, or a multi-page
   lecture. Should `Source/` allow several items with their own positions?
2. **Keep the AI prompt / conversation** that produced the LaTeX in `Source/` as well?
3. **One endless canvas, or a fixed page size** (A4-like, for printing or exporting)?
4. **Should the Mac ever add to an existing file**, for example attaching the worked solution
   later? Any such addition must never touch existing `Source/` files.
5. **"Mind map":** the name says mind map. Is it just free drawing around the question, or should
   files link to each other (one question linking to the lecture it belongs to)? Links between
   files could be plain relative paths in `Info.json`.
6. **Name of the file type / bundle ID**, for example `com.yates.ymap`.
7. **Lowest OS versions** to support. This decides which `PKContentVersion` ink types can be used.

## Decision log

Decisions are added here as they are made, newest at the bottom.

| Date | Decision | Why |
|------|----------|-----|
| 2026-11-04 | Proposed: `.ymap` is a document package with a never-changing `Source/` and an as-is `Drawing.pkdrawing`. | R8, R9, R10 |
| 2026-11-04 | Proposed: sync through iCloud Drive files only, with each device writing only its own files. | R5, R6; no conflicts by design |
| 2026-11-04 | Proposed: iPad app is a `DocumentGroup` document-based app. | R6: the UI is the Files browser |
| 2026-11-04 | Proposed: first Mac preparation step is a script (LaTeX → PDF → package → one-step move). The Mac app comes later. | Smallest thing that works |
| 2026-11-04 | **Changed:** a minimal native Mac app (type declaration, viewer, Quick Look) is the *first* deliverable. Preparing files can still be a script. | R14: without an installed app, Finder shows a plain folder and double-click doesn't open YMap |
| 2026-11-04 | Mac and iPad are separate native apps that share only the type identifier and file layout. No Mac Catalyst by default. | R16: iPad must not limit the Mac side |
| 2026-11-04 | Proposed (optional): a `ymap://` URL scheme on the Mac, keyed on the UUID in `Info.json`. | R14: links that survive moves and renames |
| 2026-11-04 | **Rejected by me:** the `ymap://` URL scheme. Opening is through Finder, Launch Services and `file://` only. | My decision |
| 2026-11-04 | **Approved by me:** two separate native apps (macOS and iPadOS), sharing only the type identifier and package layout. | My decision; R16 |
