import SwiftUI

struct FolderView: View {
    let url: URL
    @Environment(Navigator.self) private var navigator
    @State private var items: [URL] = []
    @State private var error: String?

    var body: some View {
        List(items, id: \.self) { item in
            let name = item.lastPathComponent
            switch item.pathExtension.lowercased() {
            case "ymap":
                NavigationLink(value: item) {
                    Label { Text(item.deletingPathExtension().lastPathComponent) } icon: { Thumbnail(package: item) }
                }
            case "pdf":
                Button { open(item) } label: { Label(name, systemImage: "doc.richtext") }
            default:
                if item.hasDirectoryPath {
                    NavigationLink(value: item) { Label(name, systemImage: "folder") }
                }
            }
        }
        .navigationTitle(url.lastPathComponent)
        .task { load() }
        .refreshable { load() }
        .alert("Couldn't Convert", isPresented: .constant(error != nil)) {
            Button("OK") { error = nil }
        } message: {
            Text(error ?? "")
        }
    }

    private func load() {
        let found = (try? FileManager.default.contentsOfDirectory(
            at: url, includingPropertiesForKeys: [.isDirectoryKey])) ?? []
        items = found.compactMap(Self.visible)
            .sorted { $0.lastPathComponent.localizedStandardCompare($1.lastPathComponent) == .orderedAscending }
    }

    private func open(_ pdf: URL) {
        let package = pdf.deletingPathExtension().appendingPathExtension("ymap")
        do {
            if !items.contains(where: { $0.lastPathComponent == package.lastPathComponent }) { try YMapPackage.create(from: pdf) }
            load()
            navigator.path.append(package)
        } catch {
            self.error = error.localizedDescription
        }
    }

    // iCloud items not yet downloaded appear as ".Name.pdf.icloud".
    private static func visible(_ item: URL) -> URL? {
        let name = item.lastPathComponent
        guard name.hasPrefix(".") else { return item }
        guard name.hasSuffix(".icloud") else { return nil }
        let real = String(name.dropFirst().dropLast(".icloud".count))
        return item.deletingLastPathComponent().appendingPathComponent(real, isDirectory: !real.contains("."))
    }
}

private struct Thumbnail: View {
    let package: URL

    var body: some View {
        if let image = UIImage(contentsOfFile: package.appending(path: "Preview/thumbnail.png").path) {
            Image(uiImage: image).resizable().scaledToFit().frame(width: 32, height: 32)
        } else {
            Image(systemName: "scribble")
        }
    }
}
