import PencilKit
import SwiftUI

struct CanvasScreen: View {
    let url: URL
    @State private var canvas: CanvasController?
    @State private var failed = false

    var body: some View {
        Group {
            if let canvas {
                Canvas(controller: canvas).ignoresSafeArea(edges: .bottom)
            } else if failed {
                ContentUnavailableView("Couldn't Open", systemImage: "exclamationmark.triangle")
            } else {
                ProgressView()
            }
        }
        .navigationTitle(url.deletingPathExtension().lastPathComponent)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            if let canvas {
                Button("Undo", systemImage: "arrow.uturn.backward") { canvas.canvas.undoManager?.undo() }
                Button("Redo", systemImage: "arrow.uturn.forward") { canvas.canvas.undoManager?.redo() }
            }
        }
        .task {
            let scoped = url.startAccessingSecurityScopedResource()
            try? FileManager.default.startDownloadingUbiquitousItem(at: url)
            let document = YMapDocument(fileURL: url)
            if await document.open() {
                canvas = CanvasController(document: document, scoped: scoped)
            } else {
                failed = true
            }
        }
        .onDisappear { canvas?.close() }
    }
}

private struct Canvas: UIViewControllerRepresentable {
    let controller: CanvasController
    func makeUIViewController(context: Context) -> CanvasController { controller }
    func updateUIViewController(_ controller: CanvasController, context: Context) {}
}

final class CanvasController: UIViewController, PKCanvasViewDelegate {
    let canvas = PKCanvasView()
    private let document: YMapDocument
    private let scoped: Bool
    private let toolPicker = PKToolPicker()
    private var pages: PDFPagesView?
    private var origin = CGPoint.zero // where the pdf's top-left sits on the canvas
    private var applying = false
    private var positioned = false

    init(document: YMapDocument, scoped: Bool) {
        self.document = document
        self.scoped = scoped
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) { fatalError() }

    func close() {
        document.close { [document, scoped] _ in
            if scoped { document.fileURL.stopAccessingSecurityScopedResource() }
        }
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        overrideUserInterfaceStyle = .light
        canvas.frame = view.bounds
        canvas.autoresizingMask = [.flexibleWidth, .flexibleHeight]
        canvas.backgroundColor = .secondarySystemBackground
        canvas.minimumZoomScale = 0.2
        canvas.maximumZoomScale = 10
        canvas.delegate = self
        view.addSubview(canvas)

        if let pdf = document.pdf {
            let pages = PDFPagesView(pdf: pdf)
            canvas.insertSubview(pages, at: 0)
            self.pages = pages
        }
        let margin: CGFloat = 1000
        let bounds = document.drawing.bounds
        origin = bounds.isNull ? CGPoint(x: margin, y: margin)
            : CGPoint(x: max(margin, margin - bounds.minX), y: max(margin, margin - bounds.minY))
        show(document.drawing)
        document.onReload = { [weak self] in self.map { $0.show($0.document.drawing) } }

        toolPicker.addObserver(canvas)
        toolPicker.setVisible(true, forFirstResponder: canvas)
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
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

    private func placePages() {
        let zoom = canvas.zoomScale
        pages?.transform = CGAffineTransform(scaleX: zoom, y: zoom)
        pages?.frame.origin = CGPoint(x: origin.x * zoom, y: origin.y * zoom)
    }

    func canvasViewDrawingDidChange(_ canvasView: PKCanvasView) {
        guard !applying else { return }
        document.drawing = canvas.drawing.transformed(using: .init(translationX: -origin.x, y: -origin.y))
        document.updateChangeCount(.done)
        updateLayout()
    }

    func scrollViewDidZoom(_ scrollView: UIScrollView) { placePages() }
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
