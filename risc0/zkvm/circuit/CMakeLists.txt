add_library(circuit
	accum_regs.cpp     
	context.cpp        
	decode_cycle.cpp   
	final_cycle.cpp    
	mem_check.cpp      
	multiply_cycle.cpp 
	sha_cycle.cpp
	compute_cycle.cpp  
	data_regs.cpp      
	divide_cycle.cpp   
	gen_context.cpp    
	mem_io_regs.cpp    
	poly_context.cpp   
	step_state.cpp
)

add_subdirectory(make-circuit)
