//
//  User.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation

struct User: Hashable, Decodable {
    var username: String
    var email: String
    var token: String
}
