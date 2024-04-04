//
//  ApiErrorAlert.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import SwiftUI

private struct ErrorAlert: ViewModifier {
    private var showsAlert: Binding<Bool> {
        .init {
            error != nil
        } set: { _ in
            error = nil
        }
    }
    
    @Binding var error: AlertError?
    
    func body(content: Content) -> some View {
        content.alert(isPresented: showsAlert, error: error) { _ in
            
        } message: { error in
            switch error {
            case .localizedError: 
                EmptyView()
            case let .apiError(error):
                if let message = error.errorMessage {
                    Text(message)
                } else {
                    EmptyView()
                }
            }
        }
    }
}

extension View {
    func errorAlert(error: Binding<AlertError?>) -> some View {
        modifier(ErrorAlert(error: error))
    }
}
