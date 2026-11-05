# YMap

YMap - Yates Mind Map. A macOS and iPadOS app for creating, editing and
viewing `.ymap` packages.

I have pdfs (questions or sections of notes) I want to annotate, and they are
already well organised in my iCloud Drive. I want the annotations stored next
to the pdfs, without needing OneNote, which is a separate app not related to my
file system. Everything is file-focussed and Finder-focussed; Finder niceties
are important to me.

## Project Policies
Use PROGRESS.md to record progress.
No need to manually date anything, git commit history is enough.
Try to write as little code as possible to get the job done,
eventually I would be interested in reading it as a human,
use minimal code comments and minimal documentation unless the documentation
adds something not immediately obvious if you already understand the whole
project.

## Agent coordination
AI agents working at the same time use `.ai-agent-coordination/coord`. Its
state lives in `.git/ai-agent-coordination/`, shared by all worktrees and never
committed. Pick a short unique name and pass it first on every call:

```
C=.ai-agent-coordination/coord
$C NAME join "what I'm working on"   # first; run again to update the task
$C NAME who                          # agents, their tasks, held locks
$C NAME lock PATH [WAIT_SECONDS]     # fails (exit 1) if someone else holds it
$C NAME unlock PATH
$C NAME tell OTHER "text"            # message one agent
$C NAME say "text"                   # message every other agent
$C NAME inbox                        # print and clear your messages
$C NAME leave                        # last; releases all your locks
```

- Run `who` and `inbox` when you start and between steps.
- Before editing README.md or PROGRESS.md: `lock` it with a wait, edit, then
  `unlock` straight away.
- For a longer piece of work, `lock` its folder (e.g. `iOS/`) for as long as
  you work there, and stay out of folders others have locked. Locks are plain
  names: `iOS/` does not block `iOS/FolderView.swift`, so lock at the level
  others will check.
- An agent idle for over 30 minutes is probably gone. Message it first; if
  there's no reply, `coord THEIRNAME leave` clears it and its locks.

## Requirements

### Note taking

- **R1** Notes are taken on iPad with Apple Pencil; Pencil double-tap switches
  to the eraser.
- **R2** Opening the file on iPad gives a canvas with the pdf on it, and I can
  draw around it.
- **R3** The iPad UI is built around the iCloud Drive folders where all my
  notes already are. No separate library.
- **R4** Changes sync between Mac and iPad through iCloud and update live.

### Storage

- **R5** One file per question or lecture holds everything relevant to it,
  losslessly, in a custom format.
- **R6** The original pdf is stored permanently, in its own file, apart from
  the drawing data.
- **R7** The drawing data is stored in Apple's own format (PencilKit), with no
  custom conversion.
- **R8** Everything is future-proof as Apple's systems change.

### Opening and the Mac

- **R9** Opening a `.ymap` on macOS by double-clicking it in Finder, or by
  following a link to it, opens it straight in YMap. This is a top priority.
- **R10** Opening it from the Files app on iPad opens it in YMap too, if
  possible.
- **R11** The Mac can view and manage drawings. It does not need to edit them.
- **R12** iPad needs never limit the Mac side. The Mac app is designed on its
  own terms.

### This README

- **R13** Requests and decisions are written down here as the project goes.
- Lines are kept to 80 characters.

## The `.ymap` package

A `.ymap` file is a **package**: a folder that Finder and the Files app show
as a single file (the same way `.pages` or `.rtfd` files work).

```
Past Exam 2023 Q4.ymap/
├── Info.json              <- format version, ID, date, pdf hashes, last opened
├── Source/                <- the pdfs; changed only by Reload PDFs
│   ├── Q4.pdf
│   └── Q4 Solutions.pdf
├── Drawings/              <- one per pdf, PKDrawing.dataRepresentation()
│   ├── Q4.pkdrawing
│   └── Q4 Solutions.pkdrawing
└── Preview/               <- generated from the files above; safe to delete
    └── thumbnail.png      <- for Files / Finder / Quick Look
```

Drawings are in points from the top-left of their pdf's first page. Format 1
had one pdf and a single `Drawing.pkdrawing`; YMap moves it into `Drawings/`
on the next save.

On iPad, tapping `X.pdf` in the folder list creates `X.ymap` next to it, and
long-pressing folder `X` offers **Convert Folder to YMap**, which copies every
pdf directly inside it. Originals are left in place; doing either again opens
the existing `X.ymap`.

**Reload PDFs** (long-press or swipe a `.ymap`) copies in every pdf from the
folder named like the package, and any pdf beside it with exactly the name of
one stored inside. It adds or replaces pdfs, never removes one, and keeps the
drawings. It only appears when there is something to reload from.

In a `.ymap`, the sidebar button lists its pdfs. It opens the last opened pdf,
or else the first by name, with names containing the word sol, solution or
solutions last.

## Building

The Xcode project is generated from `project.yml` and not checked in. Needs
full Xcode and XcodeGen (`brew install xcodegen`).

```
xcodegen generate && open YMap.xcodeproj
```
