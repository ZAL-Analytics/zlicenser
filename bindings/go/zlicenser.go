package zlicenser

// #cgo LDFLAGS: -L${SRCDIR}/../../target/debug -lzlicenser_go -Wl,-rpath,${SRCDIR}/../../target/debug
// #include "zlicenser.h"
import "C"

func HelloWorld() string {
	return C.GoString(C.zlicenser_hello_world())
}
