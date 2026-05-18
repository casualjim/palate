// Package palate wraps the Palate C API through cgo.
package palate

/*
#cgo CFLAGS: -I${SRCDIR}/../../crates/palate-capi/include
#cgo darwin LDFLAGS: -L${SRCDIR}/../../target/debug -lpalate_capi -Wl,-rpath,${SRCDIR}/../../target/debug
#cgo linux LDFLAGS: -L${SRCDIR}/../../target/debug -lpalate_capi -Wl,-rpath,${SRCDIR}/../../target/debug
#include <stdlib.h>
#include "palate.h"
*/
import "C"

import (
	"errors"
	"fmt"
	"unsafe"
)

const (
	statusOK              = int(C.PALATE_STATUS_OK)
	statusNoMatch         = int(C.PALATE_STATUS_NO_MATCH)
	statusInvalidArgument = int(C.PALATE_STATUS_INVALID_ARGUMENT)
)

// StatusError reports a non-success palate_status_t returned by the C API.
type StatusError struct {
	Status int
}

func (e StatusError) Error() string {
	return fmt.Sprintf("palate C API returned status %d", e.Status)
}

// Version returns the packaged Palate adapter version.
func Version() string {
	return C.GoString(C.palate_version())
}

// Detect detects a file type from a path/name and caller-provided content.
func Detect(path string, content []byte) (string, error) {
	fileType, matched, err := callDetect(path, content, true)
	if err != nil {
		return "", err
	}
	if !matched {
		return "", StatusError{Status: statusNoMatch}
	}
	return fileType, nil
}

// TryDetect tries to detect a file type without falling back to text.
func TryDetect(path string, content []byte) (string, bool, error) {
	return callDetect(path, content, false)
}

func callDetect(path string, content []byte, fallback bool) (string, bool, error) {
	cPath, err := cString(path)
	if err != nil {
		return "", false, err
	}
	defer C.free(unsafe.Pointer(cPath))

	contentPtr, freeContent := cBytes(content)
	defer freeContent()

	var out *C.char
	var status C.palate_status_t
	if fallback {
		status = C.palate_detect(cPath, contentPtr, C.size_t(len(content)), &out)
	} else {
		status = C.palate_try_detect(cPath, contentPtr, C.size_t(len(content)), &out)
	}

	return resultFromStatus(status, out)
}

func cString(value string) (*C.char, error) {
	cValue := C.CString(value)
	if cValue == nil {
		return nil, errors.New("failed to allocate C string")
	}
	if C.GoString(cValue) != value {
		C.free(unsafe.Pointer(cValue))
		return nil, errors.New("path contains embedded NUL byte")
	}
	return cValue, nil
}

func cBytes(content []byte) (*C.uint8_t, func()) {
	if len(content) == 0 {
		return nil, func() {}
	}
	ptr := C.CBytes(content)
	return (*C.uint8_t)(ptr), func() { C.free(ptr) }
}

func resultFromStatus(status C.palate_status_t, out *C.char) (string, bool, error) {
	return resultFromStatusCode(int(status), out)
}

func resultFromStatusCode(status int, out *C.char) (string, bool, error) {
	switch status {
	case statusOK:
		if out == nil {
			return "", false, StatusError{Status: status}
		}
		return C.GoString(out), true, nil
	case statusNoMatch:
		return "", false, nil
	default:
		return "", false, StatusError{Status: status}
	}
}
