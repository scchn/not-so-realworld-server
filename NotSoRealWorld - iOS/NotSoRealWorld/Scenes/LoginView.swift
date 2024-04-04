//
//  LoginView.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import SwiftUI
import Observation

struct LoginView: View {
    @Bindable
    private var viewModel = LoginViewModel()
    @State
    private var path = NavigationPath()
    @State
    private var error: AlertError?
    
    var body: some View {
        NavigationStack(path: $path) {
            Form {
                Grid {
                    switch viewModel.mode {
                    case .signIn:
                        TextFieldGridRow("Username", text: $viewModel.loginUser.username)
                        TextFieldGridRow("Password", text: $viewModel.loginUser.password)
                    case .signUp:
                        TextFieldGridRow("Username", text: $viewModel.newUser.username)
                        TextFieldGridRow("Email", text: $viewModel.newUser.email)
                        TextFieldGridRow("Password", text: $viewModel.newUser.password)
                    }
                }
                .listRowSeparator(.hidden)
                
                submitButton()
                    .listRowSeparator(.hidden)
                    .errorAlert(error: $error)
                
                modeToggleButton()
            }
            .navigationTitle("Not so realworld app")
            .navigationBarTitleDisplayMode(.inline)
            .navigationDestination(
                for: User.self, 
                destination: ProfileView.init(user:)
            )
        }
    }
    
    private func submitButton() -> some View {
        Button(action: {
            Task {
                do {
                    let user = try await viewModel.submit().get()
                    path.append(user)
                } catch let error as AlertError {
                    self.error = error
                } catch {}
            }
        }, label: {
            Text(viewModel.mode.title)
                .frame(maxWidth: .infinity)
        })
        .buttonStyle(BorderedProminentButtonStyle())
        .disabled(viewModel.loading.isLoading)
    }
    
    private func modeToggleButton() -> some View {
        Button(viewModel.mode == .signIn ? "Sign Up" : "Sign In") {
            viewModel.mode.toggle()
        }
        .frame(maxWidth: .infinity)
    }
}

#Preview {
    LoginView()
}
