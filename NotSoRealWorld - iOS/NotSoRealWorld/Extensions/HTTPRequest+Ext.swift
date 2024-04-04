//
//  HTTPRequest+Ext.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation
import HTTPTypes

extension HTTPRequest {
    private static func baseRequest(method: Method, path: String, token: String?) -> HTTPRequest {
        var request = HTTPRequest(method: method, scheme: "http", authority: "192.168.0.151:4000", path: path)
        if let token {
            request.headerFields[.authorization] = "Bearer \(token)"
        }
        return request
    }
    
    static func get(path: String, token: String? = nil) -> Self {
        baseRequest(method: .get, path: path, token: token)
    }
    
    static func post(path: String, token: String? = nil) -> Self {
        var request = baseRequest(method: .post, path: path, token: token)
        request.headerFields[.contentType] = "application/json"
        return request
    }
    
    static func patch(path: String, token: String? = nil) -> Self {
        var request = baseRequest(method: .patch, path: path, token: token)
        request.headerFields[.contentType] = "application/json"
        return request
    }
}
