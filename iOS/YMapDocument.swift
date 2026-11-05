import PencilKit
import UIKit

// Drawing coordinates are relative to the top-left of the pdf's first page.
final class YMapDocument: UIDocument {
    private(set) var pdf: CGPDFDocument?
    var drawing = PKDrawing()
    var onReload: (() -> Void)?
    private var package = FileWrapper(directoryWithFileWrappers: [:])

    override init(fileURL url: URL) {
        super.init(fileURL: url)
        NotificationCenter.default.addObserver(
            self, selector: #selector(stateChanged), name: UIDocument.stateChangedNotification, object: self)
    }

    override func load(fromContents contents: Any, ofType typeName: String?) throws {
        guard let package = contents as? FileWrapper, let files = package.fileWrappers,
              let info = files["Info.json"]?.regularFileContents
        else { throw CocoaError(.fileReadCorruptFile) }
        let source = try YMapPackage.decoder.decode(YMapPackage.Info.self, from: info).source
            .split(separator: "/").reduce(package as FileWrapper?) { $0?.fileWrappers?[String($1)] }
        self.package = package
        pdf = source?.regularFileContents
            .flatMap { CGDataProvider(data: $0 as CFData) }.flatMap(CGPDFDocument.init)
        drawing = try files["Drawing.pkdrawing"]?.regularFileContents.map(PKDrawing.init(data:)) ?? PKDrawing()
        onReload?()
    }

    override func contents(forType typeName: String) throws -> Any {
        Self.replace("Drawing.pkdrawing", in: package, with: drawing.dataRepresentation())
        if let thumbnail = thumbnail() {
            let preview = package.fileWrappers?["Preview"] ?? {
                let folder = FileWrapper(directoryWithFileWrappers: [:])
                folder.preferredFilename = "Preview"
                package.addFileWrapper(folder)
                return folder
            }()
            Self.replace("thumbnail.png", in: preview, with: thumbnail)
        }
        return package
    }

    private static func replace(_ name: String, in folder: FileWrapper, with data: Data) {
        if let old = folder.fileWrappers?[name] { folder.removeFileWrapper(old) }
        let file = FileWrapper(regularFileWithContents: data)
        file.preferredFilename = name
        folder.addFileWrapper(file)
    }

    private func thumbnail() -> Data? {
        guard let page = pdf?.page(at: 1) else { return nil }
        let rect = CGRect(origin: .zero, size: PDFPagesView.size(of: page))
        let scale = 600 / max(rect.width, rect.height)
        let format = UIGraphicsImageRendererFormat()
        format.scale = scale
        var ink = UIImage()
        UITraitCollection(userInterfaceStyle: .light).performAsCurrent {
            ink = drawing.image(from: rect, scale: scale)
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
