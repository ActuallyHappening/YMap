# YMap
YMap - Yates Mind Map.
A MacOS app and iPad OS app which allows the editting, creation and viewing of
.ymap packages.
A YMap package holds a pdf file which is a question or section of notes which
I want to take notes on.
On my iPad, I can then scribble with my Apple Pen in this package, and changes
are synced between MacOS and iPad OS through iCloud. This way,
all my notes which are already well organised in my iCloud Drive I can convert
into ymap packages and take notes without needing OneNote which is a seperate
app not related to my file system.
I want everything to be file-focussed, Finder focussed. Finder niceties are
important to me.
I want to take notes on my iPad using my Apple Pencil so I get the double tap to erase functionality.
I want the notes to be in a custom file format, which losslessly stores everything relevant for a specific
question or lecture.
My intended use case is that I have pdfs I want to annotate, but I want the annotations to be stored next to the pdfs in my iCloud Drive.

This file should have an 80 character limit.

A .ymap package follows the following structure:
Past Exam 2023 Q4.ymap/            <- shows up as one file in Files / Finder
├── Info.json                      <- small manifest: format version, IDs, dates, hashes
├── Source/                        <- written once on the Mac, never changed afterwards
│   ├── question.pdf               <- the rendered question (vector, sharp at any zoom)
│   └── question.png               <- optional raster copy; not made by Import .tex (section 7)
├── Drawing.pkdrawing              <- PKDrawing.dataRepresentation(), written as-is (Apple's format)
└── Preview/                       <- generated from the files above and safe to delete
    └── thumbnail.png              <- for Files / Finder / Quick Look

| R1 | Notes are taken on iPad with Apple Pencil; Pencil double-tap switches to the eraser. | Vision |
| R2 | One file per question or lecture holds everything relevant to it, losslessly. | Vision |
| R5 | That space syncs between Mac and iPad and updates live. | 2026-11-04 |
| R6 | The iPad UI is built around the iCloud Drive folders where all my notes already are. No separate library. | 2026-11-04 |
| R7 | Opening the file on iPad gives a canvas with the pdf on it, and I can draw around the image. | 2026-11-04 |
| R8 | The original image is stored permanently, in its own file, apart from the drawing data. | 2026-11-04 |
| R9 | The drawing data is stored in Apple's own format (PencilKit), with no custom conversion. | 2026-11-04 |
| R10 | Everything is lossless and future-proof as Apple's systems change. | 2026-11-04 |
| R11 | The Mac can view and manage drawings. It does not need to edit them. | Original header |
| R13 | Requests and decisions are written down in this README as the project goes. | 2026-11-04 |
| R14 | Opening a `.ymap` on macOS by double-clicking it in Finder, or by following a link to it, opens it straight in YMap. This is a top priority. | Feedback 2026-11-04 |
| R15 | Opening it from the Files app on iPad opens it in YMap too, if possible. | Feedback 2026-11-04 |
| R16 | iPad needs never limit the Mac side. The Mac app is designed on its own terms. | Feedback 2026-11-04 |

> A `.ymap` file is a **package**: a folder that Finder and the Files app show as a single file
(the same way `.pages` or `.rtfd` files work). Each part is kept as its own file inside it, so
the original image never shares a file with the drawing data (R8).
