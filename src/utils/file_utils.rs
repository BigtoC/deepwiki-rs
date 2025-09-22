use std::path::Path;

/// 检查文件是否为测试文件
pub fn is_test_file(path: &Path) -> bool {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_lowercase();

    let path_str = path.to_string_lossy().to_lowercase();

    // 基于路径的检查 (支持不同的路径分隔符)
    if path_str.contains("/test/")
        || path_str.contains("\\test\\")
        || path_str.contains("/tests/")
        || path_str.contains("\\tests\\")
        || path_str.contains("/__tests__/")
        || path_str.contains("\\__tests__\\")
        || path_str.contains("/spec/")
        || path_str.contains("\\spec\\")
        || path_str.contains("/specs/")
        || path_str.contains("\\specs\\")
        || path_str.starts_with("test/")
        || path_str.starts_with("test\\")
        || path_str.starts_with("tests/")
        || path_str.starts_with("tests\\")
        || path_str.starts_with("__tests__/")
        || path_str.starts_with("__tests__\\")
        || path_str.starts_with("spec/")
        || path_str.starts_with("spec\\")
        || path_str.starts_with("specs/")
        || path_str.starts_with("specs\\")
    {
        return true;
    }

    // 基于文件名的检查
    // Python测试文件
    if file_name.starts_with("test_") || file_name.ends_with("_test.py") {
        return true;
    }

    // JavaScript/TypeScript测试文件
    if file_name.ends_with(".test.js")
        || file_name.ends_with(".spec.js")
        || file_name.ends_with(".test.ts")
        || file_name.ends_with(".spec.ts")
        || file_name.ends_with(".test.jsx")
        || file_name.ends_with(".spec.jsx")
        || file_name.ends_with(".test.tsx")
        || file_name.ends_with(".spec.tsx")
    {
        return true;
    }

    // Java测试文件
    if file_name.ends_with("test.java") || file_name.ends_with("tests.java") {
        return true;
    }

    // Rust测试文件
    if file_name.ends_with("_test.rs") || file_name.ends_with("_tests.rs") {
        return true;
    }

    // Go测试文件
    if file_name.ends_with("_test.go") {
        return true;
    }

    // C/C++测试文件
    if file_name.ends_with("_test.c")
        || file_name.ends_with("_test.cpp")
        || file_name.ends_with("_test.cc")
        || file_name.ends_with("test.c")
        || file_name.ends_with("test.cpp")
        || file_name.ends_with("test.cc")
    {
        return true;
    }

    // 通用测试文件名模式
    if file_name.contains("test")
        && (file_name.starts_with("test")
            || file_name.ends_with("test")
            || file_name.contains("_test_")
            || file_name.contains(".test.")
            || file_name.contains("-test-")
            || file_name.contains("-test.")
            || file_name.contains(".spec.")
            || file_name.contains("_spec_")
            || file_name.contains("-spec-")
            || file_name.contains("-spec."))
    {
        return true;
    }

    false
}

/// 检查目录是否为测试目录
pub fn is_test_directory(dir_name: &str) -> bool {
    let name_lower = dir_name.to_lowercase();

    // 常见的测试目录名
    matches!(
        name_lower.as_str(),
        "test"
            | "tests"
            | "__tests__"
            | "spec"
            | "specs"
            | "testing"
            | "test_data"
            | "testdata"
            | "fixtures"
            | "e2e"
            | "integration"
            | "unit"
            | "acceptance"
    ) || name_lower.ends_with("_test")
        || name_lower.ends_with("_tests")
        || name_lower.ends_with("-test")
        || name_lower.ends_with("-tests")
}

/// 检查是否为二进制文件路径
pub fn is_binary_file_path(path: &Path) -> bool {
    if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = extension.to_lowercase();
        matches!(
            ext_lower.as_str(),
            // 图片文件
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "ico" | "svg" | "webp" |
            // 音频文件
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" |
            // 视频文件
            "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" |
            // 压缩文件
            "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" |
            // 可执行文件
            "exe" | "dll" | "so" | "dylib" | "bin" |
            // 文档文件
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" |
            // 字体文件
            "ttf" | "otf" | "woff" | "woff2" |
            // 其他二进制文件
            "db" | "sqlite" | "sqlite3" | "dat" | "cache" |
            "archive"
        )
    } else {
        false
    }
}
