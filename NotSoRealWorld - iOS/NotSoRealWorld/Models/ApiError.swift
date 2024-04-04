//
//  ApiError.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation

struct ApiError: LocalizedError, Decodable {
    var title: String
    var message: String
    var details: [ApiErrorDetail]?
    var errorDescription: String? {
        title
    }
    var errorMessage: String? {
        guard let details, !details.isEmpty else {
            return message
        }
        let detailsMeesage = details
            .map { detail in
                "\(detail.name): \(detail.messages.joined(separator: "；"))"
            }
            .joined(separator: "\n")
        
        return [message, detailsMeesage].joined(separator: "\n")
    }
}

struct ApiErrorDetail: Decodable {
    var name: String
    var messages: [String]
}
