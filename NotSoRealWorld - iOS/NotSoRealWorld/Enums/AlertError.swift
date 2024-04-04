//
//  AlertError.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation

enum AlertError: LocalizedError {
    case localizedError(LocalizedError)
    case apiError(ApiError)
    
    var errorDescription: String? {
        switch self {
        case let .localizedError(error): error.errorDescription
        case let .apiError(error): error.errorDescription
        }
    }
}
