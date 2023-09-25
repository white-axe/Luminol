@echo off

set "symbol=wbg.__wbg_luminolInterrupt_65f7a2b81ea5fd3c"

if not defined LUMINOL_WASM_OPT (
	echo.
	echo    ERROR: LUMINOL_WASM_OPT is unset
	echo    Please set the LUMINOL_WASM_OPT environment variable to the path to wasm-opt.exe from Binaryen
	echo    You can download Binaryen at: https://github.com/WebAssembly/binaryen/releases
	echo.
	exit 1
)

echo running Asyncify on compiled binary
%LUMINOL_WASM_OPT% --asyncify --pass-arg=asyncify-imports@%symbol% %TRUNK_STAGING_DIR%/luminol_bg.wasm -o %TRUNK_STAGING_DIR%/luminol_bg.wasm || exit /b
