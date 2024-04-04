//
//  ProfileViewModel.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation
import HTTPTypes

@Observable
class ProfileViewModel {
    private(set) var loading = Loading()
    var user: User
    
    init(user: User) {
        self.user = user
    }
    
    func submit() async -> Result<String, AlertError> {
        defer { loading.decrement() }
        
        loading.increment()
        
        let updateUser = UpdateUser(
            username: user.username.isEmpty ? nil : user.username,
            email: user.email.isEmpty ? nil : user.email
        )
        
        do {
            let request = HTTPRequest.patch(path: "/api/user", token: user.token)
            let requestBody = try! JSONEncoder().encode(updateUser)
            let (responseBody, response) = try await URLSession.shared.upload(for: request, from: requestBody)
            
            guard response.status == .ok else {
                return .failure(.apiError(try JSONDecoder().decode(ApiError.self, from: responseBody)))
            }
            
            let apiResponse = try JSONDecoder().decode(ApiResponse<User>.self, from: responseBody)
            
            self.user = apiResponse.data
            
            return .success(apiResponse.message)
        } catch _ as DecodingError {
            return .failure(.localizedError(CustomError(errorDescription: "Error decoding data")))
        } catch {
            return .failure(.localizedError(CustomError(errorDescription: "Unknown error")))
        }
    }
}
