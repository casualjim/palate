#ifndef PALATE_H
#define PALATE_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/** Status codes returned by Palate C API functions. */
typedef enum palate_status {
  /** Operation completed successfully. */
  PALATE_STATUS_OK = 0,
  /** No file type matched. Returned by palate_try_detect only. */
  PALATE_STATUS_NO_MATCH = 1,
  /** A required pointer was null or a length/pointer pair was invalid. */
  PALATE_STATUS_INVALID_ARGUMENT = 2,
  /** The path argument was not valid UTF-8. */
  PALATE_STATUS_INVALID_UTF8 = 3,
  /** A Rust panic was caught and contained at the FFI boundary. */
  PALATE_STATUS_PANIC = 4,
} palate_status_t;

/**
 * Return the Palate adapter version as a null-terminated static string.
 *
 * The returned pointer is owned by the library and MUST NOT be freed by the
 * caller. The pointer remains valid for the lifetime of the loaded library.
 */
const char *palate_version(void);

/**
 * Detect a file type with fallback to "text".
 *
 * `path` must be a non-null, null-terminated UTF-8 string. `content` points to
 * `content_len` bytes and may contain embedded NUL bytes. `content` may be null
 * only when `content_len` is zero. `out_file_type` must be non-null.
 *
 * On PALATE_STATUS_OK, `*out_file_type` receives a null-terminated canonical
 * file type name owned by the library. The caller MUST NOT free this pointer.
 */
palate_status_t palate_detect(
    const char *path,
    const uint8_t *content,
    size_t content_len,
    const char **out_file_type);

/**
 * Try to detect a file type without fallback.
 *
 * Arguments and ownership follow palate_detect. When no file type matches, this
 * function returns PALATE_STATUS_NO_MATCH and writes a null pointer to
 * `*out_file_type`.
 */
palate_status_t palate_try_detect(
    const char *path,
    const uint8_t *content,
    size_t content_len,
    const char **out_file_type);

#ifdef __cplusplus
}
#endif

#endif /* PALATE_H */
