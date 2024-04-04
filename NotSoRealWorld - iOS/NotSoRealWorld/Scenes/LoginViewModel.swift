//
//  LoginViewModel.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation
import HTTPTypesFoundation
import HTTPTypes

extension LoginViewModel {
    enum FormMode: CustomStringConvertible {
        case signIn
        case signUp
        
        var description: String {
            self == .signIn ? "Sign In" : "Sign Up"
        }
        
        var title: String {
            self == .signIn ? "Sign In" : "Register"
        }
        
        mutating func toggle() {
            self = self == .signIn ? .signUp : .signIn
        }
    }
}

@Observable
class LoginViewModel {
    private(set) var loading = Loading()
    var mode: FormMode = .signIn
    var loginUser = LoginUser(
        username: "",
        password: ""
    )
    var newUser = NewUser(
        email: "", 
        username: "", 
        password: ""
    )
    
    private func makeRequest() -> (HTTPRequest, Data) {
        switch mode {
        case .signIn:
            return (.post(path: "/api/user/login"), try! JSONEncoder().encode(loginUser))
        case .signUp:
            return (.post(path: "/api/user"), try! JSONEncoder().encode(newUser))
        }
    }
    
    func submit() async -> Result<User, AlertError> {
        defer { loading.decrement() }
        
        loading.increment()
        
        do {
            let (request, requestBody) = makeRequest()
            let (responseBody, response) = try await URLSession.shared.upload(for: request, from: requestBody)
            
            guard response.status == .ok else {
                return .failure(.apiError(try JSONDecoder().decode(ApiError.self, from: responseBody)))
            }
            
            let apiResponse = try JSONDecoder().decode(ApiResponse<User>.self, from: responseBody)
            
            return .success(apiResponse.data)
        } catch _ as DecodingError {
            return .failure(.localizedError(CustomError(errorDescription: "Error decoding data")))
        } catch {
            return .failure(.localizedError(CustomError(errorDescription: "Unknown error")))
        }
    }
}
