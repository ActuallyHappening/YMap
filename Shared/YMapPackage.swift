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
            sourceSHA256: SHA256.hash(data: data).map { String(format: "%02x", $0) }.joined())
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        encoder.dateEncodingStrategy = .iso8601

        let wrapper = FileWrapper(directoryWithFileWrappers: [
            "Info.json": FileWrapper(regularFileWithContents: try encoder.encode(info)),
            "Source": FileWrapper(directoryWithFileWrappers: [
                pdf.lastPathComponent: FileWrapper(regularFileWithContents: data),
            ]),
        ])

        try coordinatedWrite(wrapper, to: package)
        return package
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

    private static func coordinatedWrite(_ wrapper: FileWrapper, to url: URL) throws {
        var error: NSError?
        var result: Result<Void, Error>?
        NSFileCoordinator().coordinate(writingItemAt: url, error: &error) { url in
            result = Result { try wrapper.write(to: url, options: .atomic, originalContentsURL: nil) }
        }
        if let error { throw error }
        try result!.get()
    }
}
