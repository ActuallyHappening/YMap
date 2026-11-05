import SwiftUI

@main
struct YMapApp: App {
    var body: some Scene {
        WindowGroup { RootView() }
    }
}

@Observable final class Navigator {
    var path: [URL] = []
}

struct RootView: View {
    @AppStorage("rootBookmark") private var bookmark: Data?
    @State private var root: URL?
    @State private var picking = false
    @State private var navigator = Navigator()

    var body: some View {
        NavigationStack(path: $navigator.path) {
            Group {
                if let root {
                    FolderView(url: root)
                } else {
                    ContentUnavailableView {
                        Label("No Folder", systemImage: "folder")
                    } description: {
                        Text("Choose the iCloud Drive folder that holds your notes.")
                    } actions: {
                        Button("Choose Folder") { picking = true }
                    }
                }
            }
            .navigationDestination(for: URL.self) { url in
                if url.pathExtension == "ymap" { CanvasScreen(url: url) } else { FolderView(url: url) }
            }
            .toolbar {
                Button("Change Folder", systemImage: "folder.badge.gearshape") { picking = true }
            }
        }
        .environment(navigator)
        .fileImporter(isPresented: $picking, allowedContentTypes: [.folder]) { result in
            if case .success(let url) = result { open(url) }
        }
        .onOpenURL { navigator.path.append($0) }
        .onAppear(perform: restore)
    }

    private func open(_ url: URL) {
        guard url.startAccessingSecurityScopedResource() else { return }
        root?.stopAccessingSecurityScopedResource()
        root = url
        navigator.path = []
        bookmark = try? url.bookmarkData()
    }

    private func restore() {
        guard root == nil, let bookmark else { return }
        var stale = false
        if let url = try? URL(resolvingBookmarkData: bookmark, bookmarkDataIsStale: &stale) {
            open(url)
            if stale { self.bookmark = try? url.bookmarkData() }
        }
    }
}
