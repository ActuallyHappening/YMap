import SwiftUI

struct FolderView: View {
    let url: URL
    @Environment(Navigator.self) private var navigator
    @State private var items: [URL] = []
    @State private var convertible: Set<URL> = [] // folders of pdfs without a .ymap yet
    @State private var reloadable: Set<URL> = []
    @State private var failure: (title: String, message: String)?

    var body: some View {
        List(items, id: \.self) { item in
            let name = item.lastPathComponent
            switch item.pathExtension.lowercased() {
            case "ymap":
                NavigationLink(value: item) {
                    Label { Text(item.deletingPathExtension().lastPathComponent) } icon: { Thumbnail(package: item) }
                }
                .contextMenu { reloadButton(item) }
                .swipeActions { reloadButton(item) }
            case "pdf":
                Button { convert(item, from: [item]) } label: { Label(name, systemImage: "doc.richtext") }
            default:
                if item.hasDirectoryPath {
                    NavigationLink(value: item) { Label(name, systemImage: "folder") }
                        .contextMenu {
                            if convertible.contains(item) {
                                Button("Convert Folder to YMap", systemImage: "scribble") {
                                    convert(item, from: YMapPackage.pdfs(in: item))
                                }
                            }
                        }
                }
            }
        }
        .navigationTitle(url.lastPathComponent)
        .task { load() }
        .refreshable { load() }
        .alert(failure?.title ?? "", isPresented: .constant(failure != nil)) {
            Button("OK") { failure = nil }
        } message: {
            Text(failure?.message ?? "")
        }
    }

    @ViewBuilder private func reloadButton(_ package: URL) -> some View {
        if reloadable.contains(package) {
            Button("Reload PDFs", systemImage: "arrow.clockwise") {
                do {
                    try YMapPackage.reload(package)
                    load()
                } catch {
                    failure = ("Couldn't Reload PDFs", error.localizedDescription)
                }
            }
            .tint(.blue)
        }
    }

    private func load() {
        items = YMapPackage.listing(url)
        let names = Set(items.map(\.lastPathComponent))
        convertible = Set(items.filter { item in
            item.hasDirectoryPath && item.pathExtension != "ymap"
                && !names.contains(YMapPackage.package(for: item).lastPathComponent)
                && !YMapPackage.pdfs(in: item).isEmpty
        })
        reloadable = Set(items.filter { $0.pathExtension == "ymap" && !YMapPackage.reloadSources(of: $0).isEmpty })
    }

    // Opens the .ymap for a pdf or folder, creating it first if needed.
    private func convert(_ source: URL, from pdfs: [URL]) {
        let package = YMapPackage.package(for: source)
        do {
            if !items.contains(where: { $0.lastPathComponent == package.lastPathComponent }) {
                try YMapPackage.create(package, from: pdfs)
            }
            load()
            navigator.path.append(package)
        } catch {
            failure = ("Couldn't Convert", error.localizedDescription)
        }
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
