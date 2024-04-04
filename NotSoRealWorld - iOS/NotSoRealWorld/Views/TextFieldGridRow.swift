//
//  TextFieldGridRow.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import SwiftUI

struct TextFieldGridRow: View {
    let title: String
    let text: Binding<String>
    
    init(_ title: String, text: Binding<String>) {
        self.title = title
        self.text = text
    }
    
    var body: some View {
        GridRow {
            Text(title)
            TextField(title, text: text)
                .textFieldStyle(.roundedBorder)
        }
        .gridCellAnchor(.leading)
    }
}

#Preview {
    TextFieldGridRow("Title", text: .constant("Content"))
}
