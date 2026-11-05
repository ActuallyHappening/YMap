# YMap

YMap - Yates Mind Map. A macOS and iPadOS app for creating, editing and
viewing `.ymap` packages.

I have pdfs (questions or sections of notes) I want to annotate, and they are
already well organised in my iCloud Drive. I want the annotations stored next
to the pdfs, without needing OneNote, which is a separate app not related to my
file system. Everything is file-focussed and Finder-focussed; Finder niceties
are important to me.

## Requirements

### Note taking

- **R1** Notes are taken on iPad with Apple Pencil; Pencil double-tap switches
  to the eraser.
- **R7** Opening the file on iPad gives a canvas with the pdf on it, and I can
  draw around it.
- **R6** The iPad UI is built around the iCloud Drive folders where all my
  notes already are. No separate library.
- **R5** Changes sync between Mac and iPad through iCloud and update live.

### Storage

- **R2** One file per question or lecture holds everything relevant to it,
  losslessly, in a custom format.
- **R8** The original pdf is stored permanently, in its own file, apart from
  the drawing data.
- **R9** The drawing data is stored in Apple's own format (PencilKit), with no
  custom conversion.
- **R10** Everything is future-proof as Apple's systems change.

### Opening and the Mac

- **R14** Opening a `.ymap` on macOS by double-clicking it in Finder, or by
  following a link to it, opens it straight in YMap. This is a top priority.
- **R15** Opening it from the Files app on iPad opens it in YMap too, if
  possible.
- **R11** The Mac can view and manage drawings. It does not need to edit them.
- **R16** iPad needs never limit the Mac side. The Mac app is designed on its
  own terms.

### This README

- **R13** Requests and decisions are written down here as the project goes.
- Lines are kept to 80 characters.

## The `.ymap` package

A `.ymap` file is a **package**: a folder that Finder and the Files app show
as a single file (the same way `.pages` or `.rtfd` files work).

```
Past Exam 2023 Q4.ymap/
├── Info.json           <- manifest: format version, IDs, dates, hashes
├── Source/             <- written once on the Mac, never changed after
│   ├── question.pdf    <- the question (vector, sharp at any zoom)
│   └── question.png    <- optional raster copy
├── Drawing.pkdrawing   <- PKDrawing.dataRepresentation(), written as-is
└── Preview/            <- generated from the files above; safe to delete
    └── thumbnail.png   <- for Files / Finder / Quick Look
```
