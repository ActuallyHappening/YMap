import PencilKit
import UIKit

// Each pdf in Source/ has its own Drawings/<pdf name>.pkdrawing, in coordinates
// relative to the top-left of the pdf's first page.
final class YMapDocument: UIDocument {
    private(set) var names: [String] = [] // pdfs, in the order they are offered
    var onReload: (() -> Void)?
    private var info = YMapPackage.Info()
    private var drawings: [String: PKDrawing] = [:]
    private var package = FileWrapper(directoryWithFileWrappers: [:])

    override init(fileURL url: URL) {
        super.init(fileURL: url)
        NotificationCenter.default.addObserver(
            self, selector: #selector(stateChanged), name: UIDocument.stateChangedNotification, object: self)
    }

    // The pdf to show first.
    var opened: String? {
        info.lastOpened.flatMap { names.contains($0) ? $0 : nil } ?? names.first
    }

    func open(_ name: String) {
        guard info.lastOpened != name else { return }
        info.lastOpened = name
        updateChangeCount(.done)
    }

    func pdf(_ name: String) -> CGPDFDocument? {
        package.fileWrappers?["Source"]?.fileWrappers?[name]?.regularFileContents
            .flatMap { CGDataProvider(data: $0 as CFData) }.flatMap(CGPDFDocument.init)
    }

    func drawing(_ name: String) -> PKDrawing { drawings[Self.stem(name)] ?? PKDrawing() }

    func setDrawing(_ drawing: PKDrawing, for name: String) {
        drawings[Self.stem(name)] = drawing
        updateChangeCount(.done)
    }

    override func load(fromContents contents: Any, ofType typeName: String?) throws {
        guard let package = contents as? FileWrapper, let files = package.fileWrappers,
              let info = files["Info.json"]?.regularFileContents
        else { throw CocoaError(.fileReadCorruptFile) }
        self.info = try YMapPackage.decoder.decode(YMapPackage.Info.self, from: info)
        self.package = package
        names = YMapPackage.ordered((files["Source"]?.fileWrappers ?? [:]).keys.filter {
            $0.lowercased().hasSuffix(".pdf")
        })
        drawings = try (files["Drawings"]?.fileWrappers ?? [:]).reduce(into: [:]) { drawings, file in
            drawings[Self.stem(file.key)] = try PKDrawing(data: file.value.regularFileContents ?? Data())
        }
        if let old = files["Drawing.pkdrawing"]?.regularFileContents, let only = names.first,
           drawings[Self.stem(only)] == nil {
            drawings[Self.stem(only)] = try PKDrawing(data: old)
        }
        onReload?()
    }

    override func contents(forType typeName: String) throws -> Any {
        info.formatVersion = 2
        Self.replace("Info.json", in: package, with: try YMapPackage.encoder.encode(info))
        let folder = subfolder("Drawings")
        for (stem, drawing) in drawings {
            Self.replace(stem + ".pkdrawing", in: folder, with: drawing.dataRepresentation())
        }
        if let old = package.fileWrappers?["Drawing.pkdrawing"] { package.removeFileWrapper(old) }
        if let thumbnail = thumbnail() {
            Self.replace("thumbnail.png", in: subfolder("Preview"), with: thumbnail)
        }
        return package
    }

    private static func stem(_ name: String) -> String { (name as NSString).deletingPathExtension }

    private func subfolder(_ name: String) -> FileWrapper {
        if let folder = package.fileWrappers?[name] { return folder }
        let folder = FileWrapper(directoryWithFileWrappers: [:])
        folder.preferredFilename = name
        package.addFileWrapper(folder)
        return folder
    }

    private static func replace(_ name: String, in folder: FileWrapper, with data: Data) {
        if let old = folder.fileWrappers?[name] { folder.removeFileWrapper(old) }
        let file = FileWrapper(regularFileWithContents: data)
        file.preferredFilename = name
        folder.addFileWrapper(file)
    }

    private func thumbnail() -> Data? {
        guard let name = names.first, let page = pdf(name)?.page(at: 1) else { return nil }
        let rect = CGRect(origin: .zero, size: PDFPagesView.size(of: page))
        let scale = 600 / max(rect.width, rect.height)
        let format = UIGraphicsImageRendererFormat()
        format.scale = scale
        var ink = UIImage()
        UITraitCollection(userInterfaceStyle: .light).performAsCurrent {
            ink = drawing(name).image(from: rect, scale: scale)
        }
        return UIGraphicsImageRenderer(size: rect.size, format: format).pngData { context in
            PDFPagesView.draw(page, in: rect, context.cgContext)
            ink.draw(in: rect)
        }
    }

    // Edits from two devices at once: keep this device's version.
    @objc private func stateChanged() {
        guard documentState.contains(.inConflict) else { return }
        try? NSFileVersion.removeOtherVersionsOfItem(at: fileURL)
        NSFileVersion.unresolvedConflictVersionsOfItem(at: fileURL)?.forEach { $0.isResolved = true }
    }
}
