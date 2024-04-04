//
//  Loading.swift
//  NotSoRealWorld
//
//  Created by chen on 2024/4/4.
//

import Foundation

@Observable
class Loading {
    private var count = 0
    
    private(set) var isLoading = false
    
    private func update() {
        isLoading = count > 1
    }
    
    func increment() {
        count += 1
        update()
    }
    
    func decrement() {
        guard count > 1 else {
            return
        }
        count -= 1
        update()
    }
}
