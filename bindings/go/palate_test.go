package palate

import (
	"errors"
	"testing"
)

func TestVersionIsNonEmpty(t *testing.T) {
	if Version() == "" {
		t.Fatal("expected non-empty version")
	}
}

func TestDetectsRust(t *testing.T) {
	got, err := Detect("main.rs", []byte("fn main() {}\n"))
	if err != nil {
		t.Fatalf("Detect returned error: %v", err)
	}
	if got != "rust" {
		t.Fatalf("Detect() = %q, want rust", got)
	}
}

func TestTryDetectNoMatch(t *testing.T) {
	got, ok, err := TryDetect("unknown.file", []byte{})
	if err != nil {
		t.Fatalf("TryDetect returned error: %v", err)
	}
	if ok || got != "" {
		t.Fatalf("TryDetect() = (%q, %v), want no match", got, ok)
	}
}

func TestCAPIErrorPropagation(t *testing.T) {
	_, _, err := resultFromStatusCode(statusInvalidArgument, nil)
	var statusErr StatusError
	if !errors.As(err, &statusErr) {
		t.Fatalf("expected StatusError, got %T", err)
	}
}

func TestEmbeddedNULContent(t *testing.T) {
	got, err := Detect("main.rs", []byte("fn main() {\x00}\n"))
	if err != nil {
		t.Fatalf("Detect returned error: %v", err)
	}
	if got != "rust" {
		t.Fatalf("Detect() = %q, want rust", got)
	}
}
