import CryptoKit
import Foundation

enum YMapPackage {
    struct Info: Codable {
        var formatVersion = 1
        var id = UUID()
        var created = Date()
        var source: String
        var sourceSHA256: String
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

    @discardableResult
    static func create(from pdf: URL) throws -> URL {
        let package = pdf.deletingPathExtension().appendingPathExtension("ymap")
        guard !FileManager.default.fileExists(atPath: package.path) else {
            throw AlreadyExists(name: package.lastPathComponent)
        }

        // A coordinated read downloads the pdf from iCloud first if needed.
        let data = try coordinatedRead(pdf)

        let info = Info(
            source: "Source/\(pdf.lastPathComponent)",
            sourceSHA256: sha256(data))
        let wrapper = FileWrapper(directoryWithFileWrappers: [
            "Info.json": FileWrapper(regularFileWithContents: try encoder.encode(info)),
            "Source": FileWrapper(directoryWithFileWrappers: [
                pdf.lastPathComponent: FileWrapper(regularFileWithContents: data),
            ]),
        ])

        try coordinatedWrite(package) { try wrapper.write(to: $0, options: .atomic, originalContentsURL: nil) }
        return package
    }

    // The pdf next to the package with the same name as the one stored inside it, if there is one.
    static func sibling(of package: URL) -> URL? {
        guard let data = try? Data(contentsOf: package.appending(path: "Info.json")),
              let info = try? decoder.decode(Info.self, from: data) else { return nil }
        let pdf = package.deletingLastPathComponent().appending(path: (info.source as NSString).lastPathComponent)
        let placeholder = pdf.deletingLastPathComponent().appending(path: ".\(pdf.lastPathComponent).icloud")
        let files = FileManager.default
        return files.fileExists(atPath: pdf.path) || files.fileExists(atPath: placeholder.path) ? pdf : nil
    }

    // Replaces the stored pdf with the one next to the package. The drawing is kept.
    static func reloadSource(of package: URL) throws {
        guard let pdf = sibling(of: package) else { throw CocoaError(.fileNoSuchFile) }
        let data = try coordinatedRead(pdf)
        try coordinatedWrite(package) { package in
            let infoURL = package.appending(path: "Info.json")
            var info = try decoder.decode(Info.self, from: Data(contentsOf: infoURL))
            info.sourceSHA256 = sha256(data)
            try data.write(to: package.appending(path: info.source), options: .atomic)
            try encoder.encode(info).write(to: infoURL, options: .atomic)
            try? FileManager.default.removeItem(at: package.appending(path: "Preview"))
        }
    }

    private static func sha256(_ data: Data) -> String {
        SHA256.hash(data: data).map { String(format: "%02x", $0) }.joined()
    }

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
