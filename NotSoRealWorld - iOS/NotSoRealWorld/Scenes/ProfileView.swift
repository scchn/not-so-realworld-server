//
//  ProfileView.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import SwiftUI

struct ProfileView: View {
    @Bindable private var viewModel: ProfileViewModel
    @State private var message: String?
    @State private var showsMessage = false
    @State private var error: AlertError?
    
    init(user: User) {
        self.viewModel = .init(user: user)
    }
    
    var body: some View {
        Form {
            Grid {
                TextFieldGridRow("Email", text: $viewModel.user.email)
                TextFieldGridRow("Username", text: $viewModel.user.username)
            }
            .listRowSeparator(.hidden)
            
            submitButton()
                .alert(
                    "Message",
                    isPresented: $showsMessage,
                    presenting: message
                ) { _ in
                    EmptyView()
                } message: { message in
                    Text(message)
                }
                .errorAlert(error: $error)
        }
        .navigationTitle(viewModel.user.username)
        .navigationBarTitleDisplayMode(.inline)
    }
    
    private func submitButton() -> some View {
        Button(action: {
            Task {
                switch await viewModel.submit() {
                case let .success(message):
                    self.message = message
                    self.showsMessage = true
                case let .failure(error):
                    self.error = error
                }
            }
        }, label: {
            Text("Save")
                .frame(maxWidth: .infinity)
        })
        .buttonStyle(BorderedProminentButtonStyle())
        .disabled(viewModel.loading.isLoading)
    }
}

#Preview {
    ProfileView(user: .init(username: "", email: "", token: ""))
}
