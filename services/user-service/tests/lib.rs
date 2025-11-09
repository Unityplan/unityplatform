// Integration tests for user-service
//
// This file serves as the main entry point for all integration tests.
// Following Rust best practices for scalable test organization:
// - All test modules are organized under tests/integration/
// - Common test utilities are in tests/common/
// - Each module can be run independently or as part of the full suite
//
// Run all tests: cargo test
// Run specific module: cargo test --test integration profile
// Run specific test: cargo test --test integration test_create_profile

mod common;
mod integration;
