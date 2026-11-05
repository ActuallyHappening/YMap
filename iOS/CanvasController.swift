import PencilKit
import SwiftUI

struct CanvasScreen: View {
    let url: URL
    @State private var document: YMapDocument?
    @State private var names: [String] = []
    @State private var selected: String?
    @State private var reloads = 0
    @State private var sidebar = false
    @State private var failed = false
    @State private var scoped = false
    @State private var handle = CanvasHandle()

    var body: some View {
        HStack(spacing: 0) {
            if sidebar {
                List(names, id: \.self, selection: $selected) { name in
                    Label((name as NSString).deletingPathExtension, systemImage: "doc.richtext")
                }
                .listStyle(.sidebar)
                .frame(width: 260)
                .transition(.move(edge: .leading))
                Divider()
            }
            if let document, let selected {
                Canvas(document: document, name: selected, reloads: reloads, handle: handle)
                    .id(selected)
                    .ignoresSafeArea(edges: .bottom)
            } else if document != nil {
                ContentUnavailableView("No PDFs", systemImage: "doc.richtext")
            } else if failed {
                ContentUnavailableView("Couldn't Open", systemImage: "exclamationmark.triangle")
            } else {
                ProgressView().frame(maxWidth: .infinity)
            }
        }
        .navigationTitle(url.deletingPathExtension().lastPathComponent)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .topBarLeading) {
                Button("PDFs", systemImage: "sidebar.left") { withAnimation { sidebar.toggle() } }
            }
            ToolbarItemGroup {
                Button("Undo", systemImage: "arrow.uturn.backward") { handle.controller?.canvas.undoManager?.undo() }
                Button("Redo", systemImage: "arrow.uturn.forward") { handle.controller?.canvas.undoManager?.redo() }
            }
        }
        .onChange(of: selected) { if let selected { document?.open(selected) } }
        .task {
            scoped = url.startAccessingSecurityScopedResource()
            try? FileManager.default.startDownloadingUbiquitousItem(at: url)
            let document = YMapDocument(fileURL: url)
            guard await document.open() else { failed = true; return }
            document.onReload = {
                names = document.names
                if !names.contains(selected ?? "") { selected = document.opened }
                reloads += 1
            }
            names = document.names
            selected = document.opened
            self.document = document
        }
        .onDisappear {
            document?.close { [url, scoped] _ in if scoped { url.stopAccessingSecurityScopedResource() } }
        }
    }
}

final class CanvasHandle {
    weak var controller: CanvasController?
}

private struct Canvas: UIViewControllerRepresentable {
    let document: YMapDocument
    let name: String
    let reloads: Int
    let handle: CanvasHandle

    func makeUIViewController(context: Context) -> CanvasController {
        let controller = CanvasController(document: document, name: name)
        controller.reloads = reloads
        handle.controller = controller
        return controller
    }

    func updateUIViewController(_ controller: CanvasController, context: Context) {
        guard controller.reloads != reloads else { return }
        controller.reloads = reloads
        controller.reload()
    }
}

// Shows one pdf of the document with its drawing.
final class CanvasController: UIViewController, PKCanvasViewDelegate {
    let canvas = PKCanvasView()
    var reloads = 0
    private let document: YMapDocument
    private let name: String
    private let toolPicker = PKToolPicker()
    private var pages: PDFPagesView?
    private var origin = CGPoint.zero // where the pdf's top-left sits on the canvas
    private var applying = false
    private var positioned = false

    init(document: YMapDocument, name: String) {
        self.document = document
        self.name = name
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) { fatalError() }

    override func viewDidLoad() {
        super.viewDidLoad()
        overrideUserInterfaceStyle = .light
        canvas.frame = view.bounds
        canvas.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        view.backgroundColor = .secondarySystemBackground
        canvas.backgroundColor = .clear
        canvas.isOpaque = false
        canvas.minimumZoomScale = 0.2
        canvas.maximumZoomScale = 10
        canvas.delegate = self
        if let pdf = document.pdf(name) {
            let pages = PDFPagesView(pdf: pdf)
            view.addSubview(pages)
            self.pages = pages
        }
        view.addSubview(canvas)
        let margin: CGFloat = 1000
        let bounds = document.drawing(name).bounds
        origin = bounds.isNull ? CGPoint(x: margin, y: margin)
            : CGPoint(x: max(margin, margin - bounds.minX), y: max(margin, margin - bounds.minY))
        show(document.drawing(name))

        toolPicker.stateAutosaveName = "YMap"
        toolPicker.addObserver(canvas)
        toolPicker.setVisible(true, forFirstResponder: canvas)
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        canvas.undoManager?.removeAllActions() // undo steps from the previously shown pdf
        canvas.becomeFirstResponder()
    }

    override func viewDidLayoutSubviews() {
        super.viewDidLayoutSubviews()
        guard !positioned, view.bounds.width > 0 else { return }
        positioned = true
        let gap: CGFloat = 40
        canvas.zoomScale = view.bounds.width / ((pages?.bounds.width ?? 800) + 2 * gap)
        updateLayout()
        let zoom = canvas.zoomScale
        canvas.contentOffset = CGPoint(
            x: (origin.x - gap) * zoom, y: (origin.y - gap) * zoom - canvas.adjustedContentInset.top)
    }

    func reload() { show(document.drawing(name)) }

    private func show(_ drawing: PKDrawing) {
        applying = true
        canvas.drawing = drawing.transformed(using: .init(translationX: origin.x, y: origin.y))
        applying = false
        updateLayout()
    }

    // The canvas extends well past the pdf and the ink, so there is always room to draw.
    private func updateLayout() {
        let zoom = canvas.zoomScale
        var area = CGRect(x: 0, y: 0, width: 2 * origin.x + (pages?.bounds.width ?? 0),
                          height: 2 * origin.y + (pages?.bounds.height ?? 0))
        let ink = canvas.drawing.bounds
        if !ink.isNull { area = area.union(ink.insetBy(dx: -1000, dy: -1000)) }
        canvas.contentSize = CGSize(width: area.maxX * zoom, height: area.maxY * zoom)
        placePages()
    }

    // The pages sit behind the transparent canvas and follow its scrolling and zooming.
    private func placePages() {
        let zoom = canvas.zoomScale
        let offset = canvas.contentOffset
        pages?.transform = CGAffineTransform(scaleX: zoom, y: zoom)
        pages?.frame.origin = CGPoint(x: origin.x * zoom - offset.x, y: origin.y * zoom - offset.y)
    }

    func canvasViewDrawingDidChange(_ canvasView: PKCanvasView) {
        guard !applying else { return }
        document.setDrawing(canvas.drawing.transformed(using: .init(translationX: -origin.x, y: -origin.y)), for: name)
        updateLayout()
    }

    func scrollViewDidZoom(_ scrollView: UIScrollView) { placePages() }
    func scrollViewDidScroll(_ scrollView: UIScrollView) { placePages() }
}

// Pages stacked vertically, drawn as vectors so they stay sharp at any zoom.
final class PDFPagesView: UIView {
    override class var layerClass: AnyClass { CATiledLayer.self }
    private let pages: [(page: CGPDFPage, frame: CGRect)]

    init(pdf: CGPDFDocument) {
        var y: CGFloat = 0
        pages = (1...max(pdf.numberOfPages, 1)).compactMap { pdf.page(at: $0) }.map { page in
            let frame = CGRect(origin: CGPoint(x: 0, y: y), size: Self.size(of: page))
            y = frame.maxY + 24
            return (page, frame)
        }
        super.init(frame: CGRect(x: 0, y: 0, width: pages.map(\.frame.maxX).max() ?? 0, height: max(y - 24, 0)))
        backgroundColor = .clear
        isUserInteractionEnabled = false
        let tiled = layer as! CATiledLayer
        tiled.levelsOfDetail = 4
        tiled.levelsOfDetailBias = 5
        tiled.tileSize = CGSize(width: 512, height: 512)
    }

    required init?(coder: NSCoder) { fatalError() }

    override func draw(_ rect: CGRect) {
        guard let context = UIGraphicsGetCurrentContext() else { return }
        for (page, frame) in pages where frame.intersects(rect) { Self.draw(page, in: frame, context) }
    }

    static func size(of page: CGPDFPage) -> CGSize {
        let box = page.getBoxRect(.cropBox)
        return page.rotationAngle % 180 == 0 ? box.size : CGSize(width: box.height, height: box.width)
    }

    static func draw(_ page: CGPDFPage, in rect: CGRect, _ context: CGContext) {
        context.saveGState()
        context.setFillColor(UIColor.white.cgColor)
        context.fill(rect)
        context.translateBy(x: rect.minX, y: rect.maxY)
        context.scaleBy(x: 1, y: -1)
        context.concatenate(page.getDrawingTransform(
            .cropBox, rect: CGRect(origin: .zero, size: rect.size), rotate: 0, preserveAspectRatio: true))
        context.drawPDFPage(page)
        context.restoreGState()
    }
}
