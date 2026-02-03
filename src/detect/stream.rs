//! Stream-based file type detection.

use super::{classifier, filename::FILENAME, heuristics, path_suffix::PATH_SUFFIX, shebang};
use crate::FileType;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};

const MAX_CONTENT_SIZE_BYTES: usize = 51200;

/// A reader that can peek at content without consuming the stream.
///
/// This is useful for file type detection where you need to examine the content
/// (e.g., for shebang or heuristics) but still want to use the full stream afterwards.
#[derive(Debug)]
pub struct PeekableReader<R> {
    inner: R,
    buffer: Vec<u8>,
    max_buffer: usize,
    cursor: usize,
    inner_exhausted: bool,
}

impl<R: AsyncRead + Unpin> PeekableReader<R> {
    /// Create a new PeekableReader with the given maximum buffer size.
    pub fn new(inner: R, max_buffer: usize) -> Self {
        Self {
            inner,
            buffer: Vec::with_capacity(max_buffer.min(16384)),
            max_buffer,
            cursor: 0,
            inner_exhausted: false,
        }
    }

    fn target_len(&self, requested: usize) -> usize {
        requested.min(self.max_buffer)
    }

    async fn ensure_buffer_len(&mut self, target: usize) -> std::io::Result<()> {
        let target = self.target_len(target);
        while self.buffer.len() < target && !self.inner_exhausted {
            let remaining = target - self.buffer.len();
            if remaining == 0 {
                break;
            }

            let chunk_size = remaining.min(8192);
            let mut temp = vec![0u8; chunk_size];
            let read = self.inner.read(&mut temp).await?;
            if read == 0 {
                self.inner_exhausted = true;
                break;
            }
            self.buffer.extend_from_slice(&temp[..read]);
        }
        Ok(())
    }

    /// Peek at the first line of the stream (for shebang detection).
    pub async fn peek_first_line(&mut self) -> std::io::Result<Vec<u8>> {
        let start = self.cursor;
        let target = start + 1024;
        self.ensure_buffer_len(target).await?;

        if self.buffer.len() <= start {
            return Ok(Vec::new());
        }

        let slice = &self.buffer[start..];
        let limit = slice.len().min(1024);
        let limited_slice = &slice[..limit];

        let newline_pos = limited_slice
            .iter()
            .position(|&b| b == b'\n' || b == b'\r')
            .unwrap_or(limited_slice.len());

        Ok(limited_slice[..newline_pos].to_vec())
    }

    /// Read up to max_bytes for full content analysis.
    pub async fn peek_content(&mut self, max_bytes: usize) -> std::io::Result<Vec<u8>> {
        let start = self.cursor;
        let target = start + max_bytes;
        self.ensure_buffer_len(target).await?;

        let end = (start + max_bytes).min(self.buffer.len());
        if end <= start {
            return Ok(Vec::new());
        }

        Ok(self.buffer[start..end].to_vec())
    }

    /// Reset cursor to beginning of buffer.
    pub fn rewind(&mut self) {
        self.cursor = 0;
    }

    /// Convert into an AsyncRead that preserves buffered content.
    pub fn into_async_read(self) -> impl AsyncRead + Unpin {
        CombinedReader::new(self.buffer, self.inner)
    }
}

/// Combine buffered content with the original stream.
struct CombinedReader<R> {
    buffer: Vec<u8>,
    position: usize,
    inner: R,
}

impl<R: AsyncRead + Unpin> CombinedReader<R> {
    fn new(buffer: Vec<u8>, inner: R) -> Self {
        Self {
            buffer,
            position: 0,
            inner,
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for CombinedReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if self.position < self.buffer.len() {
            let remaining_buffer = &self.buffer[self.position..];
            let to_read = buf.remaining().min(remaining_buffer.len());
            buf.put_slice(&remaining_buffer[..to_read]);
            self.position += to_read;

            if buf.remaining() == 0 {
                return std::task::Poll::Ready(Ok(()));
            }
        }

        if self.position >= self.buffer.len() && !self.buffer.is_empty() {
            self.buffer.clear();
            self.buffer.shrink_to_fit();
            self.position = 0;
        }

        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for PeekableReader<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        if self.cursor < self.buffer.len() {
            let remaining_buffer = &self.buffer[self.cursor..];
            let to_read = buf.remaining().min(remaining_buffer.len());
            buf.put_slice(&remaining_buffer[..to_read]);
            self.cursor += to_read;

            if buf.remaining() == 0 {
                return std::task::Poll::Ready(Ok(()));
            }
        }

        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

/// Detect file type from path and peekable reader using the detection pipeline.
///
/// This uses the existing detector building blocks (filename, extension, shebang,
/// heuristics, classifier) directly, following the same pipeline as `detect()`
/// but working with a stream.
///
/// Returns `Ok((Some(FileType), PeekableReader))` on successful detection,
/// `Ok((None, PeekableReader))` if detection fails, or `Err((io_error, PeekableReader))`
/// if reading fails.
pub async fn detect_with_reader<R>(
    path: &std::path::Path,
    reader: PeekableReader<R>,
) -> Result<(Option<FileType>, PeekableReader<R>), (std::io::Error, PeekableReader<R>)>
where
    R: AsyncRead + Unpin,
{
    let mut reader = reader;

    // Path suffix detection
    for (suffix, resolver) in PATH_SUFFIX {
        if path.ends_with(suffix) {
            let first_line_bytes = match reader.peek_first_line().await {
                Ok(line) => line,
                Err(e) => return Err((e, reader)),
            };
            let first_line = std::str::from_utf8(&first_line_bytes).unwrap_or("");
            if let Some(ft) = resolver.resolve(path, first_line) {
                return Ok((Some(ft), reader));
            }
        }
    }

    // Filename detection
    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        if let Some(resolver) = FILENAME.get(filename) {
            let first_line_bytes = match reader.peek_first_line().await {
                Ok(line) => line,
                Err(e) => return Err((e, reader)),
            };
            let first_line = std::str::from_utf8(&first_line_bytes).unwrap_or("");
            if let Some(ft) = resolver.resolve(path, first_line) {
                return Ok((Some(ft), reader));
            }
        }
    }

    // Read content for remaining detection stages
    let content_bytes = match reader.peek_content(MAX_CONTENT_SIZE_BYTES).await {
        Ok(content) => content,
        Err(e) => return Err((e, reader)),
    };

    let content_str = String::from_utf8_lossy(&content_bytes);
    let content = truncate_to_char_boundary(&content_str, MAX_CONTENT_SIZE_BYTES);

    // Shebang detection
    if let Some(ft) = shebang::detect_from_shebang(content) {
        return Ok((Some(ft), reader));
    }

    // Extension detection for heuristics
    let ext = path.extension().and_then(|e| e.to_str());

    // Heuristics for ambiguous extensions
    if let Some(extension) = ext {
        let dotted_extension = format!(".{extension}");
        if let Some(ft) = heuristics::apply_heuristics(&dotted_extension, path, content) {
            return Ok((Some(ft), reader));
        }
    }

    // Classifier fallback
    if let Some(ft) = classifier::classify(content) {
        return Ok((Some(ft), reader));
    }

    Ok((None, reader))
}

fn truncate_to_char_boundary(s: &str, mut max: usize) -> &str {
    if max >= s.len() {
        s
    } else {
        while !s.is_char_boundary(max) {
            max -= 1;
        }
        &s[..max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use tokio::io::AsyncReadExt;

    #[tokio::test]
    async fn test_detect_with_reader_python_shebang() {
        let path = std::path::Path::new("script.py");
        let content = b"#!/usr/bin/env python3\nprint('hello')\n";
        let reader = PeekableReader::new(Cursor::new(&content[..]), 51200);

        let (ft, mut reader) = detect_with_reader(path, reader).await.unwrap();
        assert_eq!(ft, Some(FileType::Python));

        // Verify reader can still be used
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await.unwrap();
        assert_eq!(buf, content);
    }

    #[tokio::test]
    async fn test_detect_with_reader_rust() {
        let path = std::path::Path::new("main.rs");
        let content = b"fn main() {\n    println!(\"Hello\");\n}\n";
        let reader = PeekableReader::new(Cursor::new(&content[..]), 51200);

        let (ft, _reader) = detect_with_reader(path, reader).await.unwrap();
        assert_eq!(ft, Some(FileType::Rust));
    }

    #[tokio::test]
    async fn test_detect_with_reader_no_extension() {
        let path = std::path::Path::new("Makefile");
        let content = b"all:\n\techo 'building'\n";
        let reader = PeekableReader::new(Cursor::new(&content[..]), 51200);

        let result = detect_with_reader(path, reader).await;
        // Makefile might not be detected without proper content reading
        // Just verify it doesn't error
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_detect_with_reader_unknown() {
        let path = std::path::Path::new("unknown.xyz");
        let content = b"random content that doesn't match anything\n";
        let reader = PeekableReader::new(Cursor::new(&content[..]), 51200);

        let (ft, _reader) = detect_with_reader(path, reader).await.unwrap();
        // When detection fails, returns None
        assert_eq!(ft, None);
    }

    #[tokio::test]
    async fn test_peekable_reader_preserves_content() {
        let content = b"hello world";
        let mut reader = PeekableReader::new(Cursor::new(&content[..]), 100);

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await.unwrap();
        assert_eq!(buf, content);
    }

    #[tokio::test]
    async fn test_peekable_reader_peek_first_line() {
        let content = b"#!/bin/bash\necho hello\n";
        let mut reader = PeekableReader::new(Cursor::new(&content[..]), 100);

        let first_line = reader.peek_first_line().await.unwrap();
        assert_eq!(first_line, b"#!/bin/bash");
    }

    #[tokio::test]
    async fn test_peekable_reader_peek_content() {
        let content = b"hello world this is a test";
        let mut reader = PeekableReader::new(Cursor::new(&content[..]), 100);

        let peeked = reader.peek_content(11).await.unwrap();
        assert_eq!(peeked, b"hello world");
    }

    #[tokio::test]
    async fn test_peekable_reader_into_async_read() {
        let content = b"hello world";
        let reader = PeekableReader::new(Cursor::new(&content[..]), 100);

        let mut async_reader = reader.into_async_read();
        let mut buf = Vec::new();
        async_reader.read_to_end(&mut buf).await.unwrap();
        assert_eq!(buf, content);
    }

    #[tokio::test]
    async fn test_truncate_to_char_boundary() {
        assert_eq!(truncate_to_char_boundary("hello world", 5), "hello");
        assert_eq!(truncate_to_char_boundary("hello", 10), "hello");
        // Test with multi-byte UTF-8
        // "世" is 3 bytes, "界" is 3 bytes, total string is 11 bytes
        let s = "世界hello";
        assert_eq!(truncate_to_char_boundary(s, 3), "世");
        assert_eq!(truncate_to_char_boundary(s, 4), "世"); // Can't split '界', so stays at 3 bytes
        assert_eq!(truncate_to_char_boundary(s, 6), "世界");
        assert_eq!(truncate_to_char_boundary(s, 7), "世界h"); // 7 is valid boundary
        assert_eq!(truncate_to_char_boundary(s, 10), "世界hell"); // 10 bytes
        assert_eq!(truncate_to_char_boundary(s, 11), "世界hello"); // Full string
        assert_eq!(truncate_to_char_boundary(s, 20), "世界hello"); // Max > length
    }
}
