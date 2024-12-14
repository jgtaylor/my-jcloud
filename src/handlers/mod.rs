pub mod default;
pub mod directory;
pub mod file;

// Re-export handlers to make them accessible from the parent module
pub use default::unknown_file;
pub use directory::list_directory;
pub use file::serve_file;
