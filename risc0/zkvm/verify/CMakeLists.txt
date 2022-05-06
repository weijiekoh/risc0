add_library(zkvm-verify
	method_id.cpp 
	riscv.cpp
)


add_custom_command(OUTPUT step.cpp.inc COMMAND make-circuit ${CMAKE_BINARY_DIR}/step.cpp.inc  DEPENDS make-circuit VERBATIM) 
add_custom_target(step.cpp.inc ALL )
add_dependencies(zkvm-verify step.cpp.inc)


