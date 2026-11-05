import CryptoKit
import Foundation

enum YMapPackage {
    // Format 1 had `source` and `sourceSHA256` for a single pdf and one Drawing.pkdrawing.
    struct Info: Codable {
        var formatVersion = 2
        var id = UUID()
        var created = Date()
        var sha256: [String: String]? = [:] // pdf name in Source/ -> hash
        var lastOpened: String?
    }

    static let decoder: JSONDecoder = {
        let decoder = JSONDecoder()
        decoder.dateDecodingStrategy = .iso8601
        return decoder
    }()

    static let encoder: JSONEncoder = {
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        encoder.dateEncodingStrategy = .iso8601
        return encoder
    }()

    struct AlreadyExists: LocalizedError {
        let name: String
        var errorDescription: String? { "\(name) already exists." }
    }

    // X.pdf and the folder X both become X.ymap next to them.
    static func package(for source: URL) -> URL {
        let name = source.hasDirectoryPath ? source.lastPathComponent : source.deletingPathExtension().lastPathComponent
        return source.deletingLastPathComponent().appending(path: name + ".ymap")
    }

    // Folder contents, with iCloud items not yet downloaded (".Name.pdf.icloud") under their real names.
    static func listing(_ folder: URL) -> [URL] {
        let found = (try? FileManager.default.contentsOfDirectory(
            at: folder, includingPropertiesForKeys: [.isDirectoryKey])) ?? []
        return found.compactMap { item in
            let name = item.lastPathComponent
            guard name.hasPrefix(".") else { return item }
            guard name.hasSuffix(".icloud") else { return nil }
            let real = String(name.dropFirst().dropLast(".icloud".count))
            return folder.appendingPathComponent(real, isDirectory: !real.contains("."))
        }
        .sorted { $0.lastPathComponent.localizedStandardCompare($1.lastPathComponent) == .orderedAscending }
    }

    static func pdfs(in folder: URL) -> [URL] {
        listing(folder).filter { $0.pathExtension.lowercased() == "pdf" }
    }

    // Questions first, solutions last.
    static func ordered(_ names: [String]) -> [String] {
        names.sorted { a, b in
            isSolution(a) == isSolution(b)
                ? a.localizedStandardCompare(b) == .orderedAscending : isSolution(b)
        }
    }

    private static func isSolution(_ name: String) -> Bool {
        (name as NSString).deletingPathExtension.contains(#/(?i)(^|[^a-z])sol(ution)?s?([^a-z]|$)/#)
    }

    @discardableResult
    static func create(_ package: URL, from pdfs: [URL]) throws -> URL {
        guard !FileManager.default.fileExists(atPath: package.path) else {
            throw AlreadyExists(name: package.lastPathComponent)
        }
        var info = Info()
        var source: [String: FileWrapper] = [:]
        for pdf in pdfs {
            let data = try coordinatedRead(pdf)
            info.sha256?[pdf.lastPathComponent] = sha256(data)
            source[pdf.lastPathComponent] = FileWrapper(regularFileWithContents: data)
        }
        let wrapper = FileWrapper(directoryWithFileWrappers: [
            "Info.json": FileWrapper(regularFileWithContents: try encoder.encode(info)),
            "Source": FileWrapper(directoryWithFileWrappers: source),
        ])
        try coordinatedWrite(package) { try wrapper.write(to: $0, options: .atomic, originalContentsURL: nil) }
        return package
    }

    // PDFs to reload from: all of those in the folder named like the package, and any next to the
    // package with exactly the name of one stored inside it.
    static func reloadSources(of package: URL) -> [URL] {
        let stored = (try? FileManager.default.contentsOfDirectory(
            atPath: package.appending(path: "Source").path)) ?? []
        let folder = pdfs(in: package.deletingPathExtension())
        let beside = listing(package.deletingLastPathComponent()).filter { pdf in
            stored.contains(pdf.lastPathComponent) && !folder.contains { $0.lastPathComponent == pdf.lastPathComponent }
        }
        return folder + beside
    }

    // Adds or replaces pdfs in Source/; drawings are kept and no pdf is removed.
    static func reload(_ package: URL) throws {
        let pdfs = try reloadSources(of: package).map { ($0.lastPathComponent, try coordinatedRead($0)) }
        try coordinatedWrite(package) { package in
            let infoURL = package.appending(path: "Info.json")
            var info = try decoder.decode(Info.self, from: Data(contentsOf: infoURL))
            var hashes = info.sha256 ?? [:]
            for (name, data) in pdfs {
                try data.write(to: package.appending(path: "Source/\(name)"), options: .atomic)
                hashes[name] = sha256(data)
            }
            info.sha256 = hashes
            try encoder.encode(info).write(to: infoURL, options: .atomic)
            try? FileManager.default.removeItem(at: package.appending(path: "Preview"))
        }
    }

    private static func sha256(_ data: Data) -> String {
        SHA256.hash(data: data).map { String(format: "%02x", $0) }.joined()
    }

    // A coordinated read downloads the file from iCloud first if needed.
    private static func coordinatedRead(_ url: URL) throws -> Data {
        var error: NSError?
        var result: Result<Data, Error>?
        NSFileCoordinator().coordinate(readingItemAt: url, error: &error) { url in
            result = Result { try Data(contentsOf: url) }
        }
        if let error { throw error }
        return try result!.get()
    }

    private static func coordinatedWrite(_ url: URL, _ write: (URL) throws -> Void) throws {
        var error: NSError?
        var result: Result<Void, Error>?
        NSFileCoordinator().coordinate(writingItemAt: url, error: &error) { url in
            result = Result { try write(url) }
        }
        if let error { throw error }
        try result!.get()
    }
}
