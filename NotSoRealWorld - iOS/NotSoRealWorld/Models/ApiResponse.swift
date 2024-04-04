//
//  ApiResponse.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation

struct ApiResponse<T: Decodable>: Decodable {
    var message: String
    var data: T
}
