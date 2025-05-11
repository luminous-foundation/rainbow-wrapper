// this file lags neovim
// :)

use crate::{chunks::{Data, FuncRef, StructRef, Type, Number}, WrapperCore, vex};

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
// it's just gotta be nicely formatted, yknow?
// i WILL spend hours on this for no reason

// C - const
// N - name
// T - type
// D - struct
// V - var
// S - stack
// F - function
pub enum Instruction {
    // just nop
    NOP,

    // memory
    MOV_C_V(Data,   String),
    MOV_V_V(String, String),

    PUSH_C(Data),
    PUSH_V(String),

    POP(String),

    DUP,
    SWAP,
    DROP,

    BUFF_T_C_V(Type,   Data,   String),
    BUFF_V_C_V(String, Data,   String),
    BUFF_S_C_V(/*pop*/ Data,   String),
    BUFF_T_V_V(Type,   String, String),
    BUFF_V_V_V(String, String, String),
    BUFF_S_V_V(/*pop*/ String, String),
    BUFF_T_S_V(Type,   /*pop*/ String),
    BUFF_V_S_V(String, /*pop*/ String),
    BUFF_S_S_V(/*pop*/ /*pop*/ String),
    BUFF_T_C_S(Type,   Data    /*push*/),
    BUFF_V_C_S(String, Data    /*push*/),
    BUFF_S_C_S(/*pop*/ Data    /*push*/),
    BUFF_T_V_S(Type,   String  /*push*/),
    BUFF_V_V_S(String, String  /*push*/),
    BUFF_S_V_S(/*pop*/ String  /*push*/),
    BUFF_T_S_S(Type    /*pop*/ /*push*/),
    BUFF_V_S_S(String  /*pop*/ /*push*/),
    BUFF_S_S_S /*pop*/ /*pop*/ /*push*/ ,

    MEMCOPY_V_V_C(String, String, Data),
    MEMCOPY_S_V_C(/*pop*/ String, Data),
    MEMCOPY_V_S_C(String, /*pop*/ Data),
    MEMCOPY_S_S_C(/*pop*/ /*pop*/ Data),
    MEMCOPY_V_V_S(String, String, /*push*/),
    MEMCOPY_S_V_S(/*pop*/ String, /*push*/),
    MEMCOPY_V_S_S(String, /*pop*/ /*push*/),
    MEMCOPY_S_S_S /*pop*/ /*pop*/ /*push*/ ,

    DEREF_V_V(String, String),
    DEREF_S_V(/*pop*/ String),
    DEREF_V_S(String, /*push*/),
    DEREF_S_S /*pop*/ /*push*/ ,

    REF_V_V(String,   String),
    REF_S_V(/*stack*/ String), // in this case /*stack*/ represents the stack pointer

    PMOV_C_V_C(Data,   String, Data),
    PMOV_V_V_C(String, String, Data),
    PMOV_S_V_C(/*pop*/ String, Data),
    PMOV_C_S_C(Data,   /*pop*/ Data),
    PMOV_V_S_C(String, /*pop*/ Data),
    PMOV_S_S_C(/*pop*/ /*pop*/ Data),
    PMOV_C_V_V(Data,   String, String),
    PMOV_V_V_V(String, String, String),
    PMOV_S_V_V(/*pop*/ String, String),
    PMOV_C_S_V(Data,   /*pop*/ String),
    PMOV_V_S_V(String, /*pop*/ String),
    PMOV_S_S_V(/*pop*/ /*pop*/ String),
    PMOV_C_V_S(Data,   String, /*pop*/),
    PMOV_V_V_S(String, String, /*pop*/),
    PMOV_S_V_S(/*pop*/ String, /*pop*/),
    PMOV_C_S_S(Data,   /*pop*/ /*pop*/),
    PMOV_V_S_S(String, /*pop*/ /*pop*/),
    PMOV_S_S_S /*pop*/ /*pop*/ /*pop*/ ,

    DATA_C_C_V(Data,   Data,   String),
    DATA_V_C_V(String, Data,   String),
    DATA_S_C_V(/*pop*/ Data,   String),
    DATA_C_V_V(Data,   String, String),
    DATA_V_V_V(String, String, String),
    DATA_S_V_V(/*pop*/ String, String),
    DATA_C_S_V(Data,   /*pop*/ String),
    DATA_V_S_V(String, /*pop*/ String),
    DATA_S_S_V(/*pop*/ /*pop*/ String),
    DATA_C_C_S(Data,   Data,   /*push*/),
    DATA_V_C_S(String, Data,   /*push*/),
    DATA_S_C_S(/*pop*/ Data,   /*push*/),
    DATA_C_V_S(Data,   String, /*push*/),
    DATA_V_V_S(String, String, /*push*/),
    DATA_S_V_S(/*pop*/ String, /*push*/),
    DATA_C_S_S(Data,   /*pop*/ /*push*/),
    DATA_V_S_S(String, /*pop*/ /*push*/),
    DATA_S_S_S /*pop*/ /*pop*/ /*push*/ ,
	
    // arithmetic
    ADD_C_C_V(Data,   Data,   String),
    ADD_V_C_V(String, Data,   String),
    ADD_S_C_V(/*pop*/ Data,   String),
    ADD_C_V_V(Data,   String, String),
    ADD_V_V_V(String, String, String),
    ADD_S_V_V(/*pop*/ String, String),
    ADD_C_S_V(Data,   /*pop*/ String),
    ADD_V_S_V(String, /*pop*/ String),
    ADD_S_S_V(/*pop*/ /*pop*/ String),
    ADD_C_C_S(Data,   Data,   /*push*/),
    ADD_V_C_S(String, Data,   /*push*/),
    ADD_S_C_S(/*pop*/ Data,   /*push*/),
    ADD_C_V_S(Data,   String, /*push*/),
    ADD_V_V_S(String, String, /*push*/),
    ADD_S_V_S(/*pop*/ String, /*push*/),
    ADD_C_S_S(Data,   /*pop*/ /*push*/),
    ADD_V_S_S(String, /*pop*/ /*push*/),
    ADD_S_S_S /*pop*/ /*pop*/ /*push*/ ,
	
    SUB_C_C_V(Data,   Data,   String),
    SUB_V_C_V(String, Data,   String),
    SUB_S_C_V(/*pop*/ Data,   String),
    SUB_C_V_V(Data,   String, String),
    SUB_V_V_V(String, String, String),
    SUB_S_V_V(/*pop*/ String, String),
    SUB_C_S_V(Data,   /*pop*/ String),
    SUB_V_S_V(String, /*pop*/ String),
    SUB_S_S_V(/*pop*/ /*pop*/ String),
    SUB_C_C_S(Data,   Data,   /*push*/),
    SUB_V_C_S(String, Data,   /*push*/),
    SUB_S_C_S(/*pop*/ Data,   /*push*/),
    SUB_C_V_S(Data,   String, /*push*/),
    SUB_V_V_S(String, String, /*push*/),
    SUB_S_V_S(/*pop*/ String, /*push*/),
    SUB_C_S_S(Data,   /*pop*/ /*push*/),
    SUB_V_S_S(String, /*pop*/ /*push*/),
    SUB_S_S_S /*pop*/ /*pop*/ /*push*/ ,
	
    MUL_C_C_V(Data,   Data,   String),
    MUL_V_C_V(String, Data,   String),
    MUL_S_C_V(/*pop*/ Data,   String),
    MUL_C_V_V(Data,   String, String),
    MUL_V_V_V(String, String, String),
    MUL_S_V_V(/*pop*/ String, String),
    MUL_C_S_V(Data,   /*pop*/ String),
    MUL_V_S_V(String, /*pop*/ String),
    MUL_S_S_V(/*pop*/ /*pop*/ String),
    MUL_C_C_S(Data,   Data,   /*push*/),
    MUL_V_C_S(String, Data,   /*push*/),
    MUL_S_C_S(/*pop*/ Data,   /*push*/),
    MUL_C_V_S(Data,   String, /*push*/),
    MUL_V_V_S(String, String, /*push*/),
    MUL_S_V_S(/*pop*/ String, /*push*/),
    MUL_C_S_S(Data,   /*pop*/ /*push*/),
    MUL_V_S_S(String, /*pop*/ /*push*/),
    MUL_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    DIV_C_C_V(Data,   Data,   String),
    DIV_V_C_V(String, Data,   String),
    DIV_S_C_V(/*pop*/ Data,   String),
    DIV_C_V_V(Data,   String, String),
    DIV_V_V_V(String, String, String),
    DIV_S_V_V(/*pop*/ String, String),
    DIV_C_S_V(Data,   /*pop*/ String),
    DIV_V_S_V(String, /*pop*/ String),
    DIV_S_S_V(/*pop*/ /*pop*/ String),
    DIV_C_C_S(Data,   Data,   /*push*/),
    DIV_V_C_S(String, Data,   /*push*/),
    DIV_S_C_S(/*pop*/ Data,   /*push*/),
    DIV_C_V_S(Data,   String, /*push*/),
    DIV_V_V_S(String, String, /*push*/),
    DIV_S_V_S(/*pop*/ String, /*push*/),
    DIV_C_S_S(Data,   /*pop*/ /*push*/),
    DIV_V_S_S(String, /*pop*/ /*push*/),
    DIV_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    MOD_C_C_V(Data,   Data,   String),
    MOD_V_C_V(String, Data,   String),
    MOD_S_C_V(/*pop*/ Data,   String),
    MOD_C_V_V(Data,   String, String),
    MOD_V_V_V(String, String, String),
    MOD_S_V_V(/*pop*/ String, String),
    MOD_C_S_V(Data,   /*pop*/ String),
    MOD_V_S_V(String, /*pop*/ String),
    MOD_S_S_V(/*pop*/ /*pop*/ String),
    MOD_C_C_S(Data,   Data,   /*push*/),
    MOD_V_C_S(String, Data,   /*push*/),
    MOD_S_C_S(/*pop*/ Data,   /*push*/),
    MOD_C_V_S(Data,   String, /*push*/),
    MOD_V_V_S(String, String, /*push*/),
    MOD_S_V_S(/*pop*/ String, /*push*/),
    MOD_C_S_S(Data,   /*pop*/ /*push*/),
    MOD_V_S_S(String, /*pop*/ /*push*/),
    MOD_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    AND_C_C_V(Data,   Data,   String),
    AND_V_C_V(String, Data,   String),
    AND_S_C_V(/*pop*/ Data,   String),
    AND_C_V_V(Data,   String, String),
    AND_V_V_V(String, String, String),
    AND_S_V_V(/*pop*/ String, String),
    AND_C_S_V(Data,   /*pop*/ String),
    AND_V_S_V(String, /*pop*/ String),
    AND_S_S_V(/*pop*/ /*pop*/ String),
    AND_C_C_S(Data,   Data,   /*push*/),
    AND_V_C_S(String, Data,   /*push*/),
    AND_S_C_S(/*pop*/ Data,   /*push*/),
    AND_C_V_S(Data,   String, /*push*/),
    AND_V_V_S(String, String, /*push*/),
    AND_S_V_S(/*pop*/ String, /*push*/),
    AND_C_S_S(Data,   /*pop*/ /*push*/),
    AND_V_S_S(String, /*pop*/ /*push*/),
    AND_S_S_S /*pop*/ /*pop*/ /*push*/ ,
	
    OR_C_C_V(Data,   Data,   String),
    OR_V_C_V(String, Data,   String),
    OR_S_C_V(/*pop*/ Data,   String),
    OR_C_V_V(Data,   String, String),
    OR_V_V_V(String, String, String),
    OR_S_V_V(/*pop*/ String, String),
    OR_C_S_V(Data,   /*pop*/ String),
    OR_V_S_V(String, /*pop*/ String),
    OR_S_S_V(/*pop*/ /*pop*/ String),
    OR_C_C_S(Data,   Data,   /*push*/),
    OR_V_C_S(String, Data,   /*push*/),
    OR_S_C_S(/*pop*/ Data,   /*push*/),
    OR_C_V_S(Data,   String, /*push*/),
    OR_V_V_S(String, String, /*push*/),
    OR_S_V_S(/*pop*/ String, /*push*/),
    OR_C_S_S(Data,   /*pop*/ /*push*/),
    OR_V_S_S(String, /*pop*/ /*push*/),
    OR_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    NOT_C_V(Data,   String),
    NOT_V_V(String, String),
    NOT_S_V(/*pop*/ String),
    NOT_C_S(Data,   /*push*/),
    NOT_V_S(String, /*push*/),
    NOT_S_S /*pop*/ /*push*/ , 
	
    XOR_C_C_V(Data,   Data,   String),
    XOR_V_C_V(String, Data,   String),
    XOR_S_C_V(/*pop*/ Data,   String),
    XOR_C_V_V(Data,   String, String),
    XOR_V_V_V(String, String, String),
    XOR_S_V_V(/*pop*/ String, String),
    XOR_C_S_V(Data,   /*pop*/ String),
    XOR_V_S_V(String, /*pop*/ String),
    XOR_S_S_V(/*pop*/ /*pop*/ String),
    XOR_C_C_S(Data,   Data,   /*push*/),
    XOR_V_C_S(String, Data,   /*push*/),
    XOR_S_C_S(/*pop*/ Data,   /*push*/),
    XOR_C_V_S(Data,   String, /*push*/),
    XOR_V_V_S(String, String, /*push*/),
    XOR_S_V_S(/*pop*/ String, /*push*/),
    XOR_C_S_S(Data,   /*pop*/ /*push*/),
    XOR_V_S_S(String, /*pop*/ /*push*/),
    XOR_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    LSH_C_C_V(Data,   Data,   String),
    LSH_V_C_V(String, Data,   String),
    LSH_S_C_V(/*pop*/ Data,   String),
    LSH_C_V_V(Data,   String, String),
    LSH_V_V_V(String, String, String),
    LSH_S_V_V(/*pop*/ String, String),
    LSH_C_S_V(Data,   /*pop*/ String),
    LSH_V_S_V(String, /*pop*/ String),
    LSH_S_S_V(/*pop*/ /*pop*/ String),
    LSH_C_C_S(Data,   Data,   /*push*/),
    LSH_V_C_S(String, Data,   /*push*/),
    LSH_S_C_S(/*pop*/ Data,   /*push*/),
    LSH_C_V_S(Data,   String, /*push*/),
    LSH_V_V_S(String, String, /*push*/),
    LSH_S_V_S(/*pop*/ String, /*push*/),
    LSH_C_S_S(Data,   /*pop*/ /*push*/),
    LSH_V_S_S(String, /*pop*/ /*push*/),
    LSH_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    RSH_C_C_V(Data,   Data,   String),
    RSH_V_C_V(String, Data,   String),
    RSH_S_C_V(/*pop*/ Data,   String),
    RSH_C_V_V(Data,   String, String),
    RSH_V_V_V(String, String, String),
    RSH_S_V_V(/*pop*/ String, String),
    RSH_C_S_V(Data,   /*pop*/ String),
    RSH_V_S_V(String, /*pop*/ String),
    RSH_S_S_V(/*pop*/ /*pop*/ String),
    RSH_C_C_S(Data,   Data,   /*push*/),
    RSH_V_C_S(String, Data,   /*push*/),
    RSH_S_C_S(/*pop*/ Data,   /*push*/),
    RSH_C_V_S(Data,   String, /*push*/),
    RSH_V_V_S(String, String, /*push*/),
    RSH_S_V_S(/*pop*/ String, /*push*/),
    RSH_C_S_S(Data,   /*pop*/ /*push*/),
    RSH_V_S_S(String, /*pop*/ /*push*/),
    RSH_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    PADD_C_C_V(Data,   Data,   String),
    PADD_V_C_V(String, Data,   String),
    PADD_S_C_V(/*pop*/ Data,   String),
    PADD_C_V_V(Data,   String, String),
    PADD_V_V_V(String, String, String),
    PADD_S_V_V(/*pop*/ String, String),
    PADD_C_S_V(Data,   /*pop*/ String),
    PADD_V_S_V(String, /*pop*/ String),
    PADD_S_S_V(/*pop*/ /*pop*/ String),
    PADD_C_C_S(Data,   Data,   /*push*/),
    PADD_V_C_S(String, Data,   /*push*/),
    PADD_S_C_S(/*pop*/ Data,   /*push*/),
    PADD_C_V_S(Data,   String, /*push*/),
    PADD_V_V_S(String, String, /*push*/),
    PADD_S_V_S(/*pop*/ String, /*push*/),
    PADD_C_S_S(Data,   /*pop*/ /*push*/),
    PADD_V_S_S(String, /*pop*/ /*push*/),
    PADD_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    PSUB_C_C_V(Data,   Data,   String),
    PSUB_V_C_V(String, Data,   String),
    PSUB_S_C_V(/*pop*/ Data,   String),
    PSUB_C_V_V(Data,   String, String),
    PSUB_V_V_V(String, String, String),
    PSUB_S_V_V(/*pop*/ String, String),
    PSUB_C_S_V(Data,   /*pop*/ String),
    PSUB_V_S_V(String, /*pop*/ String),
    PSUB_S_S_V(/*pop*/ /*pop*/ String),
    PSUB_C_C_S(Data,   Data,   /*push*/),
    PSUB_V_C_S(String, Data,   /*push*/),
    PSUB_S_C_S(/*pop*/ Data,   /*push*/),
    PSUB_C_V_S(Data,   String, /*push*/),
    PSUB_V_V_S(String, String, /*push*/),
    PSUB_S_V_S(/*pop*/ String, /*push*/),
    PSUB_C_S_S(Data,   /*pop*/ /*push*/),
    PSUB_V_S_S(String, /*pop*/ /*push*/),
    PSUB_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    PDIFF_C_C_V(Data,   Data,   String),
    PDIFF_V_C_V(String, Data,   String),
    PDIFF_S_C_V(/*pop*/ Data,   String),
    PDIFF_C_V_V(Data,   String, String),
    PDIFF_V_V_V(String, String, String),
    PDIFF_S_V_V(/*pop*/ String, String),
    PDIFF_C_S_V(Data,   /*pop*/ String),
    PDIFF_V_S_V(String, /*pop*/ String),
    PDIFF_S_S_V(/*pop*/ /*pop*/ String),
    PDIFF_C_C_S(Data,   Data,   /*push*/),
    PDIFF_V_C_S(String, Data,   /*push*/),
    PDIFF_S_C_S(/*pop*/ Data,   /*push*/),
    PDIFF_C_V_S(Data,   String, /*push*/),
    PDIFF_V_V_S(String, String, /*push*/),
    PDIFF_S_V_S(/*pop*/ String, /*push*/),
    PDIFF_C_S_S(Data,   /*pop*/ /*push*/),
    PDIFF_V_S_S(String, /*pop*/ /*push*/),
    PDIFF_S_S_S /*pop*/ /*pop*/ /*push*/ ,

    // variables 
    VAR_T_N(Type,   String),
    VAR_V_N(String, String),
    VAR_S_N(/*pop*/ String),
    VAR_T_V(Type,   String),
    VAR_V_V(String, String),
    VAR_S_V(/*pop*/ String),
    VAR_T_S(Type,   /*pop*/),
    VAR_V_S(String, /*pop*/),
    VAR_S_S /*pop*/ /*pop*/ , 
	
    VAREXISTS_N(String),
    VAREXISTS_V(String),
    VAREXISTS_S /*pop*/, 
	
    GETFIELD_INDEX_N_D_V (String, StructRef, String),
    GETFIELD_INDEX_V_D_V (String, StructRef, String),
    GETFIELD_INDEX_S_D_V (/*pop*/ StructRef, String),
    GETFIELD_INDEX_N_V_V (String, String,    String),
    GETFIELD_INDEX_V_V_V (String, String,    String),
    GETFIELD_INDEX_S_V_V (/*pop*/ String,    String),
    GETFIELD_INDEX_N_S_V (String, /*pop*/    String),
    GETFIELD_INDEX_V_S_V (String, /*pop*/    String),
    GETFIELD_INDEX_S_S_V (/*pop*/ /*pop*/    String),
    GETFIELD_INDEX_N_D_S (String, StructRef, /*push*/),
    GETFIELD_INDEX_V_D_S (String, StructRef, /*push*/),
    GETFIELD_INDEX_S_D_S (/*pop*/ StructRef, /*push*/),
    GETFIELD_INDEX_N_V_S (String, String,    /*push*/),
    GETFIELD_INDEX_V_V_S (String, String,    /*push*/),
    GETFIELD_INDEX_S_V_S (/*pop*/ String,    /*push*/),
    GETFIELD_INDEX_N_S_S (String, /*pop*/    /*push*/),
    GETFIELD_INDEX_V_S_S (String, /*pop*/    /*push*/),
    GETFIELD_INDEX_S_S_S  /*pop*/ /*pop*/    /*push*/ ,
    GETFIELD_VALUE_C_V_V (Data,   String,    String),
    GETFIELD_VALUE_V_V_V (String, String,    String),
    GETFIELD_VALUE_S_V_V (/*pop*/ String,    String),
    GETFIELD_VALUE_C_S_V (Data,   /*pop*/    String),
    GETFIELD_VALUE_V_S_V (String, /*pop*/    String),
    GETFIELD_VALUE_S_S_V (/*pop*/ /*pop*/    String),
    GETFIELD_VALUE_C_V_S (Data,   String,    /*push*/),
    GETFIELD_VALUE_V_V_S (String, String,    /*push*/),
    GETFIELD_VALUE_S_V_S (/*pop*/ String,    /*push*/),
    GETFIELD_VALUE_C_S_S (Data,   /*pop*/    /*push*/),
    GETFIELD_VALUE_V_S_S (String, /*pop*/    /*push*/),
    GETFIELD_VALUE_S_S_S  /*pop*/ /*pop*/    /*push*/ ,
    GETFIELD_OFFSET_N_D_V(String, StructRef, String),
    GETFIELD_OFFSET_V_D_V(String, StructRef, String),
    GETFIELD_OFFSET_S_D_V(/*pop*/ StructRef, String),
    GETFIELD_OFFSET_N_V_V(String, String,    String),
    GETFIELD_OFFSET_V_V_V(String, String,    String),
    GETFIELD_OFFSET_S_V_V(/*pop*/ String,    String),
    GETFIELD_OFFSET_N_S_V(String, /*pop*/    String),
    GETFIELD_OFFSET_V_S_V(String, /*pop*/    String),
    GETFIELD_OFFSET_S_S_V(/*pop*/ /*pop*/    String),
    GETFIELD_OFFSET_N_D_S(String, StructRef, /*push*/),
    GETFIELD_OFFSET_V_D_S(String, StructRef, /*push*/),
    GETFIELD_OFFSET_S_D_S(/*pop*/ StructRef, /*push*/),
    GETFIELD_OFFSET_N_V_S(String, String,    /*push*/),
    GETFIELD_OFFSET_V_V_S(String, String,    /*push*/),
    GETFIELD_OFFSET_S_V_S(/*pop*/ String,    /*push*/),
    GETFIELD_OFFSET_N_S_S(String, /*pop*/    /*push*/),
    GETFIELD_OFFSET_V_S_S(String, /*pop*/    /*push*/),
    GETFIELD_OFFSET_S_S_S /*pop*/ /*pop*/    /*push*/ , 
	
    SETFEILD_C_V_C(Data,   String, Data)  ,
    SETFEILD_V_V_C(String, String, Data)  ,
    SETFEILD_S_V_C(/*pop*/ String, Data)  ,
    SETFEILD_C_S_C(Data,   /*pop*/ Data)  ,
    SETFEILD_V_S_C(String, /*pop*/ Data)  ,
    SETFEILD_S_S_C(/*pop*/ /*pop*/ Data)  ,
    SETFEILD_C_V_V(Data,   String, String),
    SETFEILD_V_V_V(String, String, String),
    SETFEILD_S_V_V(/*pop*/ String, String),
    SETFEILD_C_S_V(Data,   /*pop*/ String),
    SETFEILD_V_S_V(String, /*pop*/ String),
    SETFEILD_S_S_V(/*pop*/ /*pop*/ String),
    SETFEILD_C_V_S(Data,   String, /*pop*/),
    SETFEILD_V_V_S(String, String, /*pop*/),
    SETFEILD_S_V_S(/*pop*/ String, /*pop*/),
    SETFEILD_C_S_S(Data,   /*pop*/ /*pop*/),
    SETFEILD_V_S_S(String, /*pop*/ /*pop*/),
    SETFEILD_S_S_S /*pop*/ /*pop*/ /*pop*/ , 
	
    // function related
    CALL_F(FuncRef),
    CALL_V(String),
    CALL_S /*pop*/, 
	
    SYSCALL_C(Data)  ,
    SYSCALL_V(String),
    SYSCALL_S /*pop*/, 
	
    RET,
    RET_C(Data)  ,
    RET_V(String),
    RET_S /*pop*/, 
	
    JMP_C_C(Data,   Data),
    JMP_V_C(String, Data),
    JMP_S_C(/*pop*/ Data),
    JMP_C_V(Data,   String),
    JMP_V_V(String, String),
    JMP_S_V(/*pop*/ String),
    JMP_S /*pop*/,
	
    CMP_C_C(Data,   Data),
    CMP_V_C(String, Data),
    CMP_S_C(/*pop*/ Data),
    CMP_C_V(Data,   String),
    CMP_V_V(String, String),
    CMP_S_V(/*pop*/ String),
    CMP_C_S(Data,   /*pop*/),
    CMP_V_S(String, /*pop*/),
    CMP_S_S /*pop*/ /*pop*/ ,

    CLR, 
	
    JE_C_C(Data,   Data),
    JE_V_C(String, Data),
    JE_S_C(/*pop*/ Data),
    JE_C_V(Data,   String),
    JE_V_V(String, String),
    JE_S_V(/*pop*/ String),
    JE_C_S(Data,   /*pop*/),
    JE_V_S(String, /*pop*/),
    JE_S_S /*pop*/ /*pop*/ ,
	
    JNE_C_C(Data,   Data),
    JNE_V_C(String, Data),
    JNE_S_C(/*pop*/ Data),
    JNE_C_V(Data,   String),
    JNE_V_V(String, String),
    JNE_S_V(/*pop*/ String),
    JNE_C_S(Data,   /*pop*/),
    JNE_V_S(String, /*pop*/),
    JNE_S_S /*pop*/ /*pop*/ ,
	
    JL_C_C(Data,   Data),
    JL_V_C(String, Data),
    JL_S_C(/*pop*/ Data),
    JL_C_V(Data,   String),
    JL_V_V(String, String),
    JL_S_V(/*pop*/ String),
    JL_C_S(Data,   /*pop*/),
    JL_V_S(String, /*pop*/),
    JL_S_S /*pop*/ /*pop*/ ,
	
    JLE_C_C(Data,   Data),
    JLE_V_C(String, Data),
    JLE_S_C(/*pop*/ Data),
    JLE_C_V(Data,   String),
    JLE_V_V(String, String),
    JLE_S_V(/*pop*/ String),
    JLE_C_S(Data,   /*pop*/),
    JLE_V_S(String, /*pop*/),
    JLE_S_S /*pop*/ /*pop*/ ,
	
    JG_C_C(Data,   Data),
    JG_V_C(String, Data),
    JG_S_C(/*pop*/ Data) ,
    JG_C_V(Data,   String),
    JG_V_V(String, String),
    JG_S_V(/*pop*/ String),
    JG_C_S(Data,   /*pop*/),
    JG_V_S(String, /*pop*/),
    JG_S_S /*pop*/ /*pop*/ ,
	
    JGE_C_C(Data,   Data),
    JGE_V_C(String, Data),
    JGE_S_C(/*pop*/ Data) ,
    JGE_C_V(Data,   String),
    JGE_V_V(String, String),
    JGE_S_V(/*pop*/ String),
    JGE_C_S(Data,   /*pop*/),
    JGE_V_S(String, /*pop*/),
    JGE_S_S /*pop*/ /*pop*/ ,

    ASSERT_C_C(Data,   String),
    ASSERT_V_C(String, String),
    ASSERT_P_C(/*pop*/ String),

    // type related 
    CAST_T_V_V(Type,   String, String),
    CAST_V_V_V(String, String, String),
    CAST_S_V_V(/*pop*/ String, String),
    CAST_T_S_V(Type,   /*pop*/ String),
    CAST_V_S_V(String, /*pop*/ String),
    CAST_S_S_V(/*pop*/ /*pop*/ String),
    CAST_T_V_S(Type,   String, /*push*/),
    CAST_V_V_S(String, String, /*push*/),
    CAST_S_V_S(/*pop*/ String, /*push*/),
    CAST_T_S_S(Type,   /*pop*/ /*push*/),
    CAST_V_S_S(String, /*pop*/ /*push*/),
    CAST_S_S_S /*pop*/ /*pop*/ /*push*/ , 
	
    TYPEOF_V_V(String, String),
    TYPEOF_S_V(/*pop*/ String),
    TYPEOF_V_S(String, /*push*/),
    TYPEOF_S_S /*pop*/ /*push*/ , 
	
    TYPECMP_STRICT_C_C(Data,   Data),
    TYPECMP_STRICT_V_C(String, Data),
    TYPECMP_STRICT_S_C(/*pop*/ Data),
    TYPECMP_STRICT_C_V(Data,   String),
    TYPECMP_STRICT_V_V(String, String),
    TYPECMP_STRICT_S_V(/*pop*/ String),
    TYPECMP_STRICT_C_S(Data,   /*pop*/),
    TYPECMP_STRICT_V_S(String, /*pop*/),
    TYPECMP_STRICT_S_S /*pop*/ /*pop*/,
    TYPECMP_STRUCT_C_C(Data,   Data),
    TYPECMP_STRUCT_V_C(String, Data),
    TYPECMP_STRUCT_S_C(/*pop*/ Data),
    TYPECMP_STRUCT_C_V(Data,   String),
    TYPECMP_STRUCT_V_V(String, String),
    TYPECMP_STRUCT_S_V(/*pop*/ String),
    TYPECMP_STRUCT_C_S(Data,   /*pop*/),
    TYPECMP_STRUCT_V_S(String, /*pop*/),
    TYPECMP_STRUCT_S_S /*pop*/ /*pop*/,
    TYPECMP_LOOSE_C_C (Data,   Data),
    TYPECMP_LOOSE_V_C (String, Data),
    TYPECMP_LOOSE_S_C (/*pop*/ Data),
    TYPECMP_LOOSE_C_V (Data,   String),
    TYPECMP_LOOSE_V_V (String, String),
    TYPECMP_LOOSE_S_V (/*pop*/ String),
    TYPECMP_LOOSE_C_S (Data,   /*pop*/),
    TYPECMP_LOOSE_V_S (String, /*pop*/),
    TYPECMP_LOOSE_S_S  /*pop*/ /*pop*/ , 

    SIZEOF_TYPE_T_V(Type,   String),
    SIZEOF_TYPE_V_V(String, String),
    SIZEOF_TYPE_S_V(/*pop*/ String),
    SIZEOF_TYPE_T_S(Type,   /*push*/),
    SIZEOF_TYPE_V_S(String, /*push*/),
    SIZEOF_TYPE_S_S /*pop*/ /*push*/ ,
    SIZEOF_VAR_V_V (String, String),
    SIZEOF_VAR_S_V (/*pop*/ String),
    SIZEOF_VAR_V_S (String, /*push*/),
    SIZEOF_VAR_S_S  /*pop*/ /*push*/,

    GENTYPE_CREATE_C_V  (u8,     String),
    GENTYPE_CREATE_V_V  (String, String),
    GENTYPE_CREATE_S_V  (/*pop*/ String),
    GENTYPE_CREATE_C_S  (u8      /*push*/),
    GENTYPE_CREATE_V_S  (String  /*push*/),
    GENTYPE_CREATE_S_S   /*pop*/ /*push*/ ,
    GENTYPE_MODIFY_C_C_V(u8,     Data,   String),
    GENTYPE_MODIFY_V_C_V(String, Data,   String),
    GENTYPE_MODIFY_S_C_V(/*pop*/ Data,   String),
    GENTYPE_MODIFY_C_V_V(u8,     String, String),
    GENTYPE_MODIFY_V_V_V(String, String, String),
    GENTYPE_MODIFY_S_V_V(/*pop*/ String, String),
    GENTYPE_MODIFY_C_S_V(u8,     /*pop*/ String),
    GENTYPE_MODIFY_V_S_V(String, /*pop*/ String),
    GENTYPE_MODIFY_S_S_V(/*pop*/ /*pop*/ String),
    GENTYPE_MODIFY_C_C_S(u8,     Data    /*push*/),
    GENTYPE_MODIFY_V_C_S(String, Data    /*push*/),
    GENTYPE_MODIFY_S_C_S(/*pop*/ Data    /*push*/),
    GENTYPE_MODIFY_C_V_S(u8,     String  /*push*/),
    GENTYPE_MODIFY_V_V_S(String, String  /*push*/),
    GENTYPE_MODIFY_S_V_S(/*pop*/ String  /*push*/),
    GENTYPE_MODIFY_C_S_S(u8,     /*pop*/ /*push*/),
    GENTYPE_MODIFY_V_S_S(String  /*pop*/ /*push*/),
    GENTYPE_MODIFY_S_S_S /*pop*/ /*pop*/ /*push*/ ,
}

// dont worry, this is all generated
impl Instruction {
    pub fn to_bytes(&self, wrapper: &mut WrapperCore) -> Vec<u8> {
        match self {
            // just nop
            Instruction::NOP => vec![0x00],

            // memory
            Instruction::MOV_C_V(a, b) => vex![0x01, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MOV_V_V(a, b) => vex![0x01, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],

            Instruction::PUSH_C(a) => vex![0x02, 0x00 ; a.to_bytes(wrapper)],
            Instruction::PUSH_V(a) => vex![0x02, 0x01 ; wrapper.add_data(Data::Name(a.clone()))],

            Instruction::POP(a) => vex![0x03, 0x00 ; wrapper.add_data(Data::Name(a.clone()))],

            Instruction::DUP => vec![0x04],

            Instruction::SWAP => vec![0x05],

            Instruction::DROP => vec![0x06],

            Instruction::BUFF_T_C_V(a, b, c) => vex![0x07, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::BUFF_V_C_V(a, b, c) => vex![0x07, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::BUFF_S_C_V(a, b)    => vex![0x07, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::BUFF_T_V_V(a, b, c) => vex![0x07, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::BUFF_V_V_V(a, b, c) => vex![0x07, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::BUFF_S_V_V(a, b)    => vex![0x07, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::BUFF_T_S_V(a, b)    => vex![0x07, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::BUFF_V_S_V(a, b)    => vex![0x07, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::BUFF_S_S_V(a)       => vex![0x07, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::BUFF_T_C_S(a, b)    => vex![0x07, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::BUFF_V_C_S(a, b)    => vex![0x07, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::BUFF_S_C_S(a)       => vex![0x07, 0x0B ; a.to_bytes(wrapper)],
            Instruction::BUFF_T_V_S(a, b)    => vex![0x07, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::BUFF_V_V_S(a, b)    => vex![0x07, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::BUFF_S_V_S(a)       => vex![0x07, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::BUFF_T_S_S(a)       => vex![0x07, 0x0F ; a.to_bytes(wrapper)],
            Instruction::BUFF_V_S_S(a)       => vex![0x07, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::BUFF_S_S_S          => vec![0x07, 0x11],

            Instruction::MEMCOPY_V_V_C(a, b, c) => vex![0x08, 0x00 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), c.to_bytes(wrapper)],
            Instruction::MEMCOPY_S_V_C(a, b)    => vex![0x08, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::MEMCOPY_V_S_C(a, b)    => vex![0x08, 0x02 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::MEMCOPY_S_S_C(a)       => vex![0x08, 0x03 ; a.to_bytes(wrapper)],
            Instruction::MEMCOPY_V_V_S(a, b)    => vex![0x08, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MEMCOPY_S_V_S(a)       => vex![0x08, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MEMCOPY_V_S_S(a)       => vex![0x08, 0x06 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MEMCOPY_S_S_S          => vec![0x08, 0x07],

            Instruction::DEREF_V_V(a, b) => vex![0x09, 0x00 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DEREF_S_V(a)    => vex![0x09, 0x01 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DEREF_V_S(a)    => vex![0x09, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DEREF_S_S       => vec![0x09, 0x03],

            Instruction::REF_V_V(a, b) => vex![0x0A, 0x00 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::REF_S_V(a) => vex![0x0A, 0x01 ; wrapper.add_data(Data::Name(a.clone()))],

            Instruction::PMOV_C_V_C(a, b, c) => vex![0x0B, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), c.to_bytes(wrapper)],
            Instruction::PMOV_V_V_C(a, b, c) => vex![0x0B, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), c.to_bytes(wrapper)],
            Instruction::PMOV_S_V_C(a, b)    => vex![0x0B, 0x02 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::PMOV_C_S_C(a, b)    => vex![0x0B, 0x03 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::PMOV_V_S_C(a, b)    => vex![0x0B, 0x04 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::PMOV_S_S_C(a)       => vex![0x0B, 0x05 ; a.to_bytes(wrapper)],
            Instruction::PMOV_C_V_V(a, b, c) => vex![0x0B, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PMOV_V_V_V(a, b, c) => vex![0x0B, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PMOV_S_V_V(a, b)    => vex![0x0B, 0x08 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PMOV_C_S_V(a, b)    => vex![0x0B, 0x09 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PMOV_V_S_V(a, b)    => vex![0x0B, 0x0A ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PMOV_S_S_V(a)       => vex![0x0B, 0x0B ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PMOV_C_V_S(a, b)    => vex![0x0B, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PMOV_V_V_S(a, b)    => vex![0x0B, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PMOV_S_V_S(a)       => vex![0x0B, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PMOV_C_S_S(a)       => vex![0x0B, 0x0F ; a.to_bytes(wrapper)],
            Instruction::PMOV_V_S_S(a)       => vex![0x0B, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PMOV_S_S_S          => vec![0x0B, 0x11],

            Instruction::DATA_C_C_V(a, b, c) => vex![0x0C, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DATA_V_C_V(a, b, c) => vex![0x0C, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DATA_S_C_V(a, b)    => vex![0x0C, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DATA_C_V_V(a, b, c) => vex![0x0C, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DATA_V_V_V(a, b, c) => vex![0x0C, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DATA_S_V_V(a, b)    => vex![0x0C, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DATA_C_S_V(a, b)    => vex![0x0C, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DATA_V_S_V(a, b)    => vex![0x0C, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DATA_S_S_V(a)       => vex![0x0C, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DATA_C_C_S(a, b)    => vex![0x0C, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::DATA_V_C_S(a, b)    => vex![0x0C, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::DATA_S_C_S(a)       => vex![0x0C, 0x0B ; a.to_bytes(wrapper)],
            Instruction::DATA_C_V_S(a, b)    => vex![0x0C, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DATA_V_V_S(a, b)    => vex![0x0C, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DATA_S_V_S(a)       => vex![0x0C, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DATA_C_S_S(a)       => vex![0x0C, 0x0F ; a.to_bytes(wrapper)],
            Instruction::DATA_V_S_S(a)       => vex![0x0C, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DATA_S_S_S          => vec![0x0C, 0x11],

            // arithmetic
            Instruction::ADD_C_C_V(a, b, c) => vex![0x0D, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::ADD_V_C_V(a, b, c) => vex![0x0D, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::ADD_S_C_V(a, b)    => vex![0x0D, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ADD_C_V_V(a, b, c) => vex![0x0D, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::ADD_V_V_V(a, b, c) => vex![0x0D, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::ADD_S_V_V(a, b)    => vex![0x0D, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ADD_C_S_V(a, b)    => vex![0x0D, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ADD_V_S_V(a, b)    => vex![0x0D, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ADD_S_S_V(a)       => vex![0x0D, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::ADD_C_C_S(a, b)    => vex![0x0D, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::ADD_V_C_S(a, b)    => vex![0x0D, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::ADD_S_C_S(a)       => vex![0x0D, 0x0B ; a.to_bytes(wrapper)],
            Instruction::ADD_C_V_S(a, b)    => vex![0x0D, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ADD_V_V_S(a, b)    => vex![0x0D, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ADD_S_V_S(a)       => vex![0x0D, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::ADD_C_S_S(a)       => vex![0x0D, 0x0F ; a.to_bytes(wrapper)],
            Instruction::ADD_V_S_S(a)       => vex![0x0D, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::ADD_S_S_S          => vec![0x0D, 0x11],

            Instruction::SUB_C_C_V(a, b, c) => vex![0x0E, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::SUB_V_C_V(a, b, c) => vex![0x0E, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::SUB_S_C_V(a, b)    => vex![0x0E, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SUB_C_V_V(a, b, c) => vex![0x0E, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::SUB_V_V_V(a, b, c) => vex![0x0E, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::SUB_S_V_V(a, b)    => vex![0x0E, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SUB_C_S_V(a, b)    => vex![0x0E, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SUB_V_S_V(a, b)    => vex![0x0E, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SUB_S_S_V(a)       => vex![0x0E, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SUB_C_C_S(a, b)    => vex![0x0E, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::SUB_V_C_S(a, b)    => vex![0x0E, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::SUB_S_C_S(a)       => vex![0x0E, 0x0B ; a.to_bytes(wrapper)],
            Instruction::SUB_C_V_S(a, b)    => vex![0x0E, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SUB_V_V_S(a, b)    => vex![0x0E, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SUB_S_V_S(a)       => vex![0x0E, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SUB_C_S_S(a)       => vex![0x0E, 0x0F ; a.to_bytes(wrapper)],
            Instruction::SUB_V_S_S(a)       => vex![0x0E, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SUB_S_S_S          => vec![0x0E, 0x11],

            Instruction::MUL_C_C_V(a, b, c) => vex![0x0F, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MUL_V_C_V(a, b, c) => vex![0x0F, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MUL_S_C_V(a, b)    => vex![0x0F, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MUL_C_V_V(a, b, c) => vex![0x0F, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MUL_V_V_V(a, b, c) => vex![0x0F, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MUL_S_V_V(a, b)    => vex![0x0F, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MUL_C_S_V(a, b)    => vex![0x0F, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MUL_V_S_V(a, b)    => vex![0x0F, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MUL_S_S_V(a)       => vex![0x0F, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MUL_C_C_S(a, b)    => vex![0x0F, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::MUL_V_C_S(a, b)    => vex![0x0F, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::MUL_S_C_S(a)       => vex![0x0F, 0x0B ; a.to_bytes(wrapper)],
            Instruction::MUL_C_V_S(a, b)    => vex![0x0F, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MUL_V_V_S(a, b)    => vex![0x0F, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MUL_S_V_S(a)       => vex![0x0F, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MUL_C_S_S(a)       => vex![0x0F, 0x0F ; a.to_bytes(wrapper)],
            Instruction::MUL_V_S_S(a)       => vex![0x0F, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MUL_S_S_S          => vec![0x0F, 0x11],

            Instruction::DIV_C_C_V(a, b, c) => vex![0x10, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DIV_V_C_V(a, b, c) => vex![0x10, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DIV_S_C_V(a, b)    => vex![0x10, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DIV_C_V_V(a, b, c) => vex![0x10, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DIV_V_V_V(a, b, c) => vex![0x10, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::DIV_S_V_V(a, b)    => vex![0x10, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DIV_C_S_V(a, b)    => vex![0x10, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DIV_V_S_V(a, b)    => vex![0x10, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DIV_S_S_V(a)       => vex![0x10, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DIV_C_C_S(a, b)    => vex![0x10, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::DIV_V_C_S(a, b)    => vex![0x10, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::DIV_S_C_S(a)       => vex![0x10, 0x0B ; a.to_bytes(wrapper)],
            Instruction::DIV_C_V_S(a, b)    => vex![0x10, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DIV_V_V_S(a, b)    => vex![0x10, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::DIV_S_V_S(a)       => vex![0x10, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DIV_C_S_S(a)       => vex![0x10, 0x0F ; a.to_bytes(wrapper)],
            Instruction::DIV_V_S_S(a)       => vex![0x10, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::DIV_S_S_S          => vec![0x10, 0x11],

            Instruction::MOD_C_C_V(a, b, c) => vex![0x11, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MOD_V_C_V(a, b, c) => vex![0x11, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MOD_S_C_V(a, b)    => vex![0x11, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MOD_C_V_V(a, b, c) => vex![0x11, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MOD_V_V_V(a, b, c) => vex![0x11, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::MOD_S_V_V(a, b)    => vex![0x11, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MOD_C_S_V(a, b)    => vex![0x11, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MOD_V_S_V(a, b)    => vex![0x11, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MOD_S_S_V(a)       => vex![0x11, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MOD_C_C_S(a, b)    => vex![0x11, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::MOD_V_C_S(a, b)    => vex![0x11, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::MOD_S_C_S(a)       => vex![0x11, 0x0B ; a.to_bytes(wrapper)],
            Instruction::MOD_C_V_S(a, b)    => vex![0x11, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MOD_V_V_S(a, b)    => vex![0x11, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::MOD_S_V_S(a)       => vex![0x11, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MOD_C_S_S(a)       => vex![0x11, 0x0F ; a.to_bytes(wrapper)],
            Instruction::MOD_V_S_S(a)       => vex![0x11, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::MOD_S_S_S          => vec![0x11, 0x11],

            Instruction::AND_C_C_V(a, b, c) => vex![0x12, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::AND_V_C_V(a, b, c) => vex![0x12, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::AND_S_C_V(a, b)    => vex![0x12, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::AND_C_V_V(a, b, c) => vex![0x12, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::AND_V_V_V(a, b, c) => vex![0x12, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::AND_S_V_V(a, b)    => vex![0x12, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::AND_C_S_V(a, b)    => vex![0x12, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::AND_V_S_V(a, b)    => vex![0x12, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::AND_S_S_V(a)       => vex![0x12, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::AND_C_C_S(a, b)    => vex![0x12, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::AND_V_C_S(a, b)    => vex![0x12, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::AND_S_C_S(a)       => vex![0x12, 0x0B ; a.to_bytes(wrapper)],
            Instruction::AND_C_V_S(a, b)    => vex![0x12, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::AND_V_V_S(a, b)    => vex![0x12, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::AND_S_V_S(a)       => vex![0x12, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::AND_C_S_S(a)       => vex![0x12, 0x0F ; a.to_bytes(wrapper)],
            Instruction::AND_V_S_S(a)       => vex![0x12, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::AND_S_S_S          => vec![0x12, 0x11],

            Instruction::OR_C_C_V(a, b, c) => vex![0x13, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::OR_V_C_V(a, b, c) => vex![0x13, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::OR_S_C_V(a, b)    => vex![0x13, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::OR_C_V_V(a, b, c) => vex![0x13, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::OR_V_V_V(a, b, c) => vex![0x13, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::OR_S_V_V(a, b)    => vex![0x13, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::OR_C_S_V(a, b)    => vex![0x13, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::OR_V_S_V(a, b)    => vex![0x13, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::OR_S_S_V(a)       => vex![0x13, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::OR_C_C_S(a, b)    => vex![0x13, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::OR_V_C_S(a, b)    => vex![0x13, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::OR_S_C_S(a)       => vex![0x13, 0x0B ; a.to_bytes(wrapper)],
            Instruction::OR_C_V_S(a, b)    => vex![0x13, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::OR_V_V_S(a, b)    => vex![0x13, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::OR_S_V_S(a)       => vex![0x13, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::OR_C_S_S(a)       => vex![0x13, 0x0F ; a.to_bytes(wrapper)],
            Instruction::OR_V_S_S(a)       => vex![0x13, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::OR_S_S_S          => vec![0x13, 0x11],

            Instruction::NOT_C_V(a, b) => vex![0x14, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::NOT_V_V(a, b) => vex![0x14, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::NOT_S_V(a)    => vex![0x14, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::NOT_C_S(a)    => vex![0x14, 0x03 ; a.to_bytes(wrapper)],
            Instruction::NOT_V_S(a)    => vex![0x14, 0x04 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::NOT_S_S       => vec![0x14, 0x05],

            Instruction::XOR_C_C_V(a, b, c) => vex![0x15, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::XOR_V_C_V(a, b, c) => vex![0x15, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::XOR_S_C_V(a, b)    => vex![0x15, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::XOR_C_V_V(a, b, c) => vex![0x15, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::XOR_V_V_V(a, b, c) => vex![0x15, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::XOR_S_V_V(a, b)    => vex![0x15, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::XOR_C_S_V(a, b)    => vex![0x15, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::XOR_V_S_V(a, b)    => vex![0x15, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::XOR_S_S_V(a)       => vex![0x15, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::XOR_C_C_S(a, b)    => vex![0x15, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::XOR_V_C_S(a, b)    => vex![0x15, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::XOR_S_C_S(a)       => vex![0x15, 0x0B ; a.to_bytes(wrapper)],
            Instruction::XOR_C_V_S(a, b)    => vex![0x15, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::XOR_V_V_S(a, b)    => vex![0x15, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::XOR_S_V_S(a)       => vex![0x15, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::XOR_C_S_S(a)       => vex![0x15, 0x0F ; a.to_bytes(wrapper)],
            Instruction::XOR_V_S_S(a)       => vex![0x15, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::XOR_S_S_S          => vec![0x15, 0x11],

            Instruction::LSH_C_C_V(a, b, c) => vex![0x16, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::LSH_V_C_V(a, b, c) => vex![0x16, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::LSH_S_C_V(a, b)    => vex![0x16, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::LSH_C_V_V(a, b, c) => vex![0x16, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::LSH_V_V_V(a, b, c) => vex![0x16, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::LSH_S_V_V(a, b)    => vex![0x16, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::LSH_C_S_V(a, b)    => vex![0x16, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::LSH_V_S_V(a, b)    => vex![0x16, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::LSH_S_S_V(a)       => vex![0x16, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::LSH_C_C_S(a, b)    => vex![0x16, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::LSH_V_C_S(a, b)    => vex![0x16, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::LSH_S_C_S(a)       => vex![0x16, 0x0B ; a.to_bytes(wrapper)],
            Instruction::LSH_C_V_S(a, b)    => vex![0x16, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::LSH_V_V_S(a, b)    => vex![0x16, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::LSH_S_V_S(a)       => vex![0x16, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::LSH_C_S_S(a)       => vex![0x16, 0x0F ; a.to_bytes(wrapper)],
            Instruction::LSH_V_S_S(a)       => vex![0x16, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::LSH_S_S_S          => vec![0x16, 0x11],

            Instruction::RSH_C_C_V(a, b, c) => vex![0x17, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::RSH_V_C_V(a, b, c) => vex![0x17, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::RSH_S_C_V(a, b)    => vex![0x17, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::RSH_C_V_V(a, b, c) => vex![0x17, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::RSH_V_V_V(a, b, c) => vex![0x17, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::RSH_S_V_V(a, b)    => vex![0x17, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::RSH_C_S_V(a, b)    => vex![0x17, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::RSH_V_S_V(a, b)    => vex![0x17, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::RSH_S_S_V(a)       => vex![0x17, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::RSH_C_C_S(a, b)    => vex![0x17, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::RSH_V_C_S(a, b)    => vex![0x17, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::RSH_S_C_S(a)       => vex![0x17, 0x0B ; a.to_bytes(wrapper)],
            Instruction::RSH_C_V_S(a, b)    => vex![0x17, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::RSH_V_V_S(a, b)    => vex![0x17, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::RSH_S_V_S(a)       => vex![0x17, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::RSH_C_S_S(a)       => vex![0x17, 0x0F ; a.to_bytes(wrapper)],
            Instruction::RSH_V_S_S(a)       => vex![0x17, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::RSH_S_S_S          => vec![0x17, 0x11],

            Instruction::PADD_C_C_V(a, b, c) => vex![0x18, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PADD_V_C_V(a, b, c) => vex![0x18, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PADD_S_C_V(a, b)    => vex![0x18, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PADD_C_V_V(a, b, c) => vex![0x18, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PADD_V_V_V(a, b, c) => vex![0x18, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PADD_S_V_V(a, b)    => vex![0x18, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PADD_C_S_V(a, b)    => vex![0x18, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PADD_V_S_V(a, b)    => vex![0x18, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PADD_S_S_V(a)       => vex![0x18, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PADD_C_C_S(a, b)    => vex![0x18, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::PADD_V_C_S(a, b)    => vex![0x18, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::PADD_S_C_S(a)       => vex![0x18, 0x0B ; a.to_bytes(wrapper)],
            Instruction::PADD_C_V_S(a, b)    => vex![0x18, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PADD_V_V_S(a, b)    => vex![0x18, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PADD_S_V_S(a)       => vex![0x18, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PADD_C_S_S(a)       => vex![0x18, 0x0F ; a.to_bytes(wrapper)],
            Instruction::PADD_V_S_S(a)       => vex![0x18, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PADD_S_S_S          => vec![0x18, 0x11],

            Instruction::PSUB_C_C_V(a, b, c) => vex![0x19, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PSUB_V_C_V(a, b, c) => vex![0x19, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PSUB_S_C_V(a, b)    => vex![0x19, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PSUB_C_V_V(a, b, c) => vex![0x19, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PSUB_V_V_V(a, b, c) => vex![0x19, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PSUB_S_V_V(a, b)    => vex![0x19, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PSUB_C_S_V(a, b)    => vex![0x19, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PSUB_V_S_V(a, b)    => vex![0x19, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PSUB_S_S_V(a)       => vex![0x19, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PSUB_C_C_S(a, b)    => vex![0x19, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::PSUB_V_C_S(a, b)    => vex![0x19, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::PSUB_S_C_S(a)       => vex![0x19, 0x0B ; a.to_bytes(wrapper)],
            Instruction::PSUB_C_V_S(a, b)    => vex![0x19, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PSUB_V_V_S(a, b)    => vex![0x19, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PSUB_S_V_S(a)       => vex![0x19, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PSUB_C_S_S(a)       => vex![0x19, 0x0F ; a.to_bytes(wrapper)],
            Instruction::PSUB_V_S_S(a)       => vex![0x19, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PSUB_S_S_S          => vec![0x19, 0x11],

            Instruction::PDIFF_C_C_V(a, b, c) => vex![0x1A, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PDIFF_V_C_V(a, b, c) => vex![0x1A, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PDIFF_S_C_V(a, b)    => vex![0x1A, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PDIFF_C_V_V(a, b, c) => vex![0x1A, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PDIFF_V_V_V(a, b, c) => vex![0x1A, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::PDIFF_S_V_V(a, b)    => vex![0x1A, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PDIFF_C_S_V(a, b)    => vex![0x1A, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PDIFF_V_S_V(a, b)    => vex![0x1A, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PDIFF_S_S_V(a)       => vex![0x1A, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PDIFF_C_C_S(a, b)    => vex![0x1A, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::PDIFF_V_C_S(a, b)    => vex![0x1A, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::PDIFF_S_C_S(a)       => vex![0x1A, 0x0B ; a.to_bytes(wrapper)],
            Instruction::PDIFF_C_V_S(a, b)    => vex![0x1A, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PDIFF_V_V_S(a, b)    => vex![0x1A, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::PDIFF_S_V_S(a)       => vex![0x1A, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PDIFF_C_S_S(a)       => vex![0x1A, 0x0F ; a.to_bytes(wrapper)],
            Instruction::PDIFF_V_S_S(a)       => vex![0x1A, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::PDIFF_S_S_S          => vec![0x1A, 0x11],

            // variables 
            Instruction::VAR_T_N(a, b) => vex![0x1B, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::VAR_V_N(a, b) => vex![0x1B, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::VAR_S_N(a)    => vex![0x1B, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::VAR_T_V(a, b) => vex![0x1B, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::VAR_V_V(a, b) => vex![0x1B, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::VAR_S_V(a)    => vex![0x1B, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::VAR_T_S(a)    => vex![0x1B, 0x06 ; a.to_bytes(wrapper)],
            Instruction::VAR_V_S(a)    => vex![0x1B, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::VAR_S_S       => vec![0x1B, 0x08],

            Instruction::VAREXISTS_N(a) => vex![0x1C, 0x00 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::VAREXISTS_V(a) => vex![0x1C, 0x01 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::VAREXISTS_S    => vec![0x1C, 0x02],

            Instruction::GETFIELD_INDEX_N_D_V(a, b, c) => vex![0x1D, 0x00 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_INDEX_V_D_V(a, b, c) => vex![0x1D, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_INDEX_S_D_V(a, b)    => vex![0x1D, 0x02 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_INDEX_N_V_V(a, b, c) => vex![0x1D, 0x03 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_INDEX_V_V_V(a, b, c) => vex![0x1D, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_INDEX_S_V_V(a, b)    => vex![0x1D, 0x05 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_INDEX_N_S_V(a, b)    => vex![0x1D, 0x06 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_INDEX_V_S_V(a, b)    => vex![0x1D, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_INDEX_S_S_V(a)       => vex![0x1D, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_INDEX_N_D_S(a, b)    => vex![0x1D, 0x09 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::GETFIELD_INDEX_V_D_S(a, b)    => vex![0x1D, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::GETFIELD_INDEX_S_D_S(a)       => vex![0x1D, 0x0B ; a.to_bytes(wrapper)],
            Instruction::GETFIELD_INDEX_N_V_S(a, b)    => vex![0x1D, 0x0C ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_INDEX_V_V_S(a, b)    => vex![0x1D, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_INDEX_S_V_S(a)       => vex![0x1D, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_INDEX_N_S_S(a)       => vex![0x1D, 0x0F ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_INDEX_V_S_S(a)       => vex![0x1D, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_INDEX_S_S_S          => vec![0x1D, 0x11],
            Instruction::GETFIELD_VALUE_C_V_V(a, b, c) => vex![0x1D, 0x12 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_VALUE_V_V_V(a, b, c) => vex![0x1D, 0x13 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_VALUE_S_V_V(a, b)    => vex![0x1D, 0x14 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_VALUE_C_S_V(a, b)    => vex![0x1D, 0x15 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_VALUE_V_S_V(a, b)    => vex![0x1D, 0x16 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_VALUE_S_S_V(a)       => vex![0x1D, 0x17 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_VALUE_C_V_S(a, b)    => vex![0x1D, 0x18 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_VALUE_V_V_S(a, b)    => vex![0x1D, 0x19 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_VALUE_S_V_S(a)       => vex![0x1D, 0x1A ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_VALUE_C_S_S(a)       => vex![0x1D, 0x1B ; a.to_bytes(wrapper)],
            Instruction::GETFIELD_VALUE_V_S_S(a)       => vex![0x1D, 0x1C ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_VALUE_S_S_S          => vec![0x1D, 0x1D],
            Instruction::GETFIELD_OFFSET_N_D_V(a, b, c) => vex![0x1D, 0x1E ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_OFFSET_V_D_V(a, b, c) => vex![0x1D, 0x1F ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_OFFSET_S_D_V(a, b)    => vex![0x1D, 0x20 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_OFFSET_N_V_V(a, b, c) => vex![0x1D, 0x21 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_OFFSET_V_V_V(a, b, c) => vex![0x1D, 0x22 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GETFIELD_OFFSET_S_V_V(a, b)    => vex![0x1D, 0x23 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_OFFSET_N_S_V(a, b)    => vex![0x1D, 0x24 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_OFFSET_V_S_V(a, b)    => vex![0x1D, 0x25 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_OFFSET_S_S_V(a)       => vex![0x1D, 0x26 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_OFFSET_N_D_S(a, b)    => vex![0x1D, 0x27 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::GETFIELD_OFFSET_V_D_S(a, b)    => vex![0x1D, 0x28 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::GETFIELD_OFFSET_S_D_S(a)       => vex![0x1D, 0x29 ; a.to_bytes(wrapper)],
            Instruction::GETFIELD_OFFSET_N_V_S(a, b)    => vex![0x1D, 0x2A ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_OFFSET_V_V_S(a, b)    => vex![0x1D, 0x2B ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GETFIELD_OFFSET_S_V_S(a)       => vex![0x1D, 0x2C ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_OFFSET_N_S_S(a)       => vex![0x1D, 0x2D ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_OFFSET_V_S_S(a)       => vex![0x1D, 0x2E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GETFIELD_OFFSET_S_S_S          => vec![0x1D, 0x2F],

            Instruction::SETFEILD_C_V_C(a, b, c) => vex![0x1E, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), c.to_bytes(wrapper)],
            Instruction::SETFEILD_V_V_C(a, b, c) => vex![0x1E, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), c.to_bytes(wrapper)],
            Instruction::SETFEILD_S_V_C(a, b)    => vex![0x1E, 0x02 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::SETFEILD_C_S_C(a, b)    => vex![0x1E, 0x03 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::SETFEILD_V_S_C(a, b)    => vex![0x1E, 0x04 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::SETFEILD_S_S_C(a)       => vex![0x1E, 0x05 ; a.to_bytes(wrapper)],
            Instruction::SETFEILD_C_V_V(a, b, c) => vex![0x1E, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::SETFEILD_V_V_V(a, b, c) => vex![0x1E, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::SETFEILD_S_V_V(a, b)    => vex![0x1E, 0x08 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SETFEILD_C_S_V(a, b)    => vex![0x1E, 0x09 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SETFEILD_V_S_V(a, b)    => vex![0x1E, 0x0A ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SETFEILD_S_S_V(a)       => vex![0x1E, 0x0B ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SETFEILD_C_V_S(a, b)    => vex![0x1E, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SETFEILD_V_V_S(a, b)    => vex![0x1E, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SETFEILD_S_V_S(a)       => vex![0x1E, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SETFEILD_C_S_S(a)       => vex![0x1E, 0x0F ; a.to_bytes(wrapper)],
            Instruction::SETFEILD_V_S_S(a)       => vex![0x1E, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SETFEILD_S_S_S          => vec![0x1E, 0x11],

            // function related
            Instruction::CALL_F(a) => vex![0x1F, 0x00 ; a.to_bytes(wrapper)],
            Instruction::CALL_V(a) => vex![0x1F, 0x01 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::CALL_S    => vec![0x1F, 0x02],

            Instruction::SYSCALL_C(a) => vex![0x20, 0x00 ; a.to_bytes(wrapper)],
            Instruction::SYSCALL_V(a) => vex![0x20, 0x01 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SYSCALL_S    => vec![0x20, 0x02],

            Instruction::RET => vec![0x21],
            Instruction::RET_C(a) => vex![0x21, 0x01 ; a.to_bytes(wrapper)],
            Instruction::RET_V(a) => vex![0x21, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::RET_S    => vec![0x21, 0x03],

            Instruction::JMP_C_C(a, b) => vex![0x22, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::JMP_V_C(a, b) => vex![0x22, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::JMP_S_C(a)    => vex![0x22, 0x02 ; a.to_bytes(wrapper)],
            Instruction::JMP_C_V(a, b) => vex![0x22, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JMP_V_V(a, b) => vex![0x22, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JMP_S_V(a)    => vex![0x22, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JMP_S    => vec![0x22, 0x06],

            Instruction::CMP_C_C(a, b) => vex![0x23, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::CMP_V_C(a, b) => vex![0x23, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::CMP_S_C(a)    => vex![0x23, 0x02 ; a.to_bytes(wrapper)],
            Instruction::CMP_C_V(a, b) => vex![0x23, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::CMP_V_V(a, b) => vex![0x23, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::CMP_S_V(a)    => vex![0x23, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::CMP_C_S(a)    => vex![0x23, 0x06 ; a.to_bytes(wrapper)],
            Instruction::CMP_V_S(a)    => vex![0x23, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::CMP_S_S       => vec![0x23, 0x08],

            Instruction::CLR => vec![0x24],

            Instruction::JE_C_C(a, b) => vex![0x25, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::JE_V_C(a, b) => vex![0x25, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::JE_S_C(a)    => vex![0x25, 0x02 ; a.to_bytes(wrapper)],
            Instruction::JE_C_V(a, b) => vex![0x25, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JE_V_V(a, b) => vex![0x25, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JE_S_V(a)    => vex![0x25, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JE_C_S(a)    => vex![0x25, 0x06 ; a.to_bytes(wrapper)],
            Instruction::JE_V_S(a)    => vex![0x25, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JE_S_S       => vec![0x25, 0x08],

            Instruction::JNE_C_C(a, b) => vex![0x26, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::JNE_V_C(a, b) => vex![0x26, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::JNE_S_C(a)    => vex![0x26, 0x02 ; a.to_bytes(wrapper)],
            Instruction::JNE_C_V(a, b) => vex![0x26, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JNE_V_V(a, b) => vex![0x26, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JNE_S_V(a)    => vex![0x26, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JNE_C_S(a)    => vex![0x26, 0x06 ; a.to_bytes(wrapper)],
            Instruction::JNE_V_S(a)    => vex![0x26, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JNE_S_S       => vec![0x26, 0x08],

            Instruction::JL_C_C(a, b) => vex![0x27, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::JL_V_C(a, b) => vex![0x27, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::JL_S_C(a)    => vex![0x27, 0x02 ; a.to_bytes(wrapper)],
            Instruction::JL_C_V(a, b) => vex![0x27, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JL_V_V(a, b) => vex![0x27, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JL_S_V(a)    => vex![0x27, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JL_C_S(a)    => vex![0x27, 0x06 ; a.to_bytes(wrapper)],
            Instruction::JL_V_S(a)    => vex![0x27, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JL_S_S       => vec![0x27, 0x08],

            Instruction::JLE_C_C(a, b) => vex![0x28, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::JLE_V_C(a, b) => vex![0x28, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::JLE_S_C(a)    => vex![0x28, 0x02 ; a.to_bytes(wrapper)],
            Instruction::JLE_C_V(a, b) => vex![0x28, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JLE_V_V(a, b) => vex![0x28, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JLE_S_V(a)    => vex![0x28, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JLE_C_S(a)    => vex![0x28, 0x06 ; a.to_bytes(wrapper)],
            Instruction::JLE_V_S(a)    => vex![0x28, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JLE_S_S       => vec![0x28, 0x08],

            Instruction::JG_C_C(a, b) => vex![0x29, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::JG_V_C(a, b) => vex![0x29, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::JG_S_C(a)    => vex![0x29, 0x02 ; a.to_bytes(wrapper)],
            Instruction::JG_C_V(a, b) => vex![0x29, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JG_V_V(a, b) => vex![0x29, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JG_S_V(a)    => vex![0x29, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JG_C_S(a)    => vex![0x29, 0x06 ; a.to_bytes(wrapper)],
            Instruction::JG_V_S(a)    => vex![0x29, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JG_S_S       => vec![0x29, 0x08],

            Instruction::JGE_C_C(a, b) => vex![0x2A, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::JGE_V_C(a, b) => vex![0x2A, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::JGE_S_C(a)    => vex![0x2A, 0x02 ; a.to_bytes(wrapper)],
            Instruction::JGE_C_V(a, b) => vex![0x2A, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JGE_V_V(a, b) => vex![0x2A, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::JGE_S_V(a)    => vex![0x2A, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JGE_C_S(a)    => vex![0x2A, 0x06 ; a.to_bytes(wrapper)],
            Instruction::JGE_V_S(a)    => vex![0x2A, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::JGE_S_S       => vec![0x2A, 0x08],

            Instruction::ASSERT_C_C(a, b) => vex![0x2B, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ASSERT_V_C(a, b) => vex![0x2B, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::ASSERT_P_C(a)    => vex![0x2B, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],

            // type related 
            Instruction::CAST_T_V_V(a, b, c) => vex![0x2C, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::CAST_V_V_V(a, b, c) => vex![0x2C, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::CAST_S_V_V(a, b)    => vex![0x2C, 0x02 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::CAST_T_S_V(a, b)    => vex![0x2C, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::CAST_V_S_V(a, b)    => vex![0x2C, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::CAST_S_S_V(a)       => vex![0x2C, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::CAST_T_V_S(a, b)    => vex![0x2C, 0x06 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::CAST_V_V_S(a, b)    => vex![0x2C, 0x07 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::CAST_S_V_S(a)       => vex![0x2C, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::CAST_T_S_S(a)       => vex![0x2C, 0x09 ; a.to_bytes(wrapper)],
            Instruction::CAST_V_S_S(a)       => vex![0x2C, 0x0A ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::CAST_S_S_S          => vec![0x2C, 0x0B],

            Instruction::TYPEOF_V_V(a, b) => vex![0x2D, 0x00 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::TYPEOF_S_V(a)    => vex![0x2D, 0x01 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPEOF_V_S(a)    => vex![0x2D, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPEOF_S_S       => vec![0x2D, 0x03],

            Instruction::TYPECMP_STRICT_C_C(a, b) => vex![0x2E, 0x00 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::TYPECMP_STRICT_V_C(a, b) => vex![0x2E, 0x01 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::TYPECMP_STRICT_S_C(a)    => vex![0x2E, 0x02 ; a.to_bytes(wrapper)],
            Instruction::TYPECMP_STRICT_C_V(a, b) => vex![0x2E, 0x03 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::TYPECMP_STRICT_V_V(a, b) => vex![0x2E, 0x04 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::TYPECMP_STRICT_S_V(a)    => vex![0x2E, 0x05 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPECMP_STRICT_C_S(a)    => vex![0x2E, 0x06 ; a.to_bytes(wrapper)],
            Instruction::TYPECMP_STRICT_V_S(a)    => vex![0x2E, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPECMP_STRICT_S_S       => vec![0x2E, 0x08],
            Instruction::TYPECMP_STRUCT_C_C(a, b) => vex![0x2E, 0x09 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::TYPECMP_STRUCT_V_C(a, b) => vex![0x2E, 0x0A ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::TYPECMP_STRUCT_S_C(a)    => vex![0x2E, 0x0B ; a.to_bytes(wrapper)],
            Instruction::TYPECMP_STRUCT_C_V(a, b) => vex![0x2E, 0x0C ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::TYPECMP_STRUCT_V_V(a, b) => vex![0x2E, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::TYPECMP_STRUCT_S_V(a)    => vex![0x2E, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPECMP_STRUCT_C_S(a)    => vex![0x2E, 0x0F ; a.to_bytes(wrapper)],
            Instruction::TYPECMP_STRUCT_V_S(a)    => vex![0x2E, 0x10 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPECMP_STRUCT_S_S       => vec![0x2E, 0x11],
            Instruction::TYPECMP_LOOSE_C_C(a, b)  => vex![0x2E, 0x12 ; a.to_bytes(wrapper), b.to_bytes(wrapper)],
            Instruction::TYPECMP_LOOSE_V_C(a, b)  => vex![0x2E, 0x13 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::TYPECMP_LOOSE_S_C(a)     => vex![0x2E, 0x14 ; a.to_bytes(wrapper)],
            Instruction::TYPECMP_LOOSE_C_V(a, b)  => vex![0x2E, 0x15 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::TYPECMP_LOOSE_V_V(a, b)  => vex![0x2E, 0x16 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::TYPECMP_LOOSE_S_V(a)     => vex![0x2E, 0x17 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPECMP_LOOSE_C_S(a)     => vex![0x2E, 0x18 ; a.to_bytes(wrapper)],
            Instruction::TYPECMP_LOOSE_V_S(a)     => vex![0x2E, 0x19 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::TYPECMP_LOOSE_S_S        => vec![0x2E, 0x1A],

            Instruction::SIZEOF_TYPE_T_V(a, b) => vex![0x2F, 0x00 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SIZEOF_TYPE_V_V(a, b) => vex![0x2F, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SIZEOF_TYPE_S_V(a)    => vex![0x2F, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SIZEOF_TYPE_T_S(a)    => vex![0x2F, 0x03 ; a.to_bytes(wrapper)],
            Instruction::SIZEOF_TYPE_V_S(a)    => vex![0x2F, 0x04 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SIZEOF_TYPE_S_S       => vec![0x2F, 0x05],
            Instruction::SIZEOF_VAR_V_V(a, b)  => vex![0x2F, 0x06 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::SIZEOF_VAR_S_V(a)     => vex![0x2F, 0x07 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SIZEOF_VAR_V_S(a)     => vex![0x2F, 0x08 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::SIZEOF_VAR_S_S        => vec![0x2F, 0x09],

            Instruction::GENTYPE_CREATE_C_V(a, b)      => vex![0x30, 0x00 ; wrapper.add_data(Data::Number(Number::U8(*a))), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_CREATE_V_V(a, b)      => vex![0x30, 0x01 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_CREATE_S_V(a)         => vex![0x30, 0x02 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GENTYPE_CREATE_C_S(a)         => vex![0x30, 0x03 ; wrapper.add_data(Data::Number(Number::U8(*a)))],
            Instruction::GENTYPE_CREATE_V_S(a)         => vex![0x30, 0x04 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GENTYPE_CREATE_S_S            => vec![0x30, 0x05],
            Instruction::GENTYPE_MODIFY_C_C_V(a, b, c) => vex![0x30, 0x06 ; wrapper.add_data(Data::Number(Number::U8(*a))), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GENTYPE_MODIFY_V_C_V(a, b, c) => vex![0x30, 0x07 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GENTYPE_MODIFY_S_C_V(a, b)    => vex![0x30, 0x08 ; a.to_bytes(wrapper), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_MODIFY_C_V_V(a, b, c) => vex![0x30, 0x09 ; wrapper.add_data(Data::Number(Number::U8(*a))), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GENTYPE_MODIFY_V_V_V(a, b, c) => vex![0x30, 0x0A ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone())), wrapper.add_data(Data::Name(c.clone()))],
            Instruction::GENTYPE_MODIFY_S_V_V(a, b)    => vex![0x30, 0x0B ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_MODIFY_C_S_V(a, b)    => vex![0x30, 0x0C ; wrapper.add_data(Data::Number(Number::U8(*a))), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_MODIFY_V_S_V(a, b)    => vex![0x30, 0x0D ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_MODIFY_S_S_V(a)       => vex![0x30, 0x0E ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GENTYPE_MODIFY_C_C_S(a, b)    => vex![0x30, 0x0F ; wrapper.add_data(Data::Number(Number::U8(*a))), b.to_bytes(wrapper)],
            Instruction::GENTYPE_MODIFY_V_C_S(a, b)    => vex![0x30, 0x10 ; wrapper.add_data(Data::Name(a.clone())), b.to_bytes(wrapper)],
            Instruction::GENTYPE_MODIFY_S_C_S(a)       => vex![0x30, 0x11 ; a.to_bytes(wrapper)],
            Instruction::GENTYPE_MODIFY_C_V_S(a, b)    => vex![0x30, 0x12 ; wrapper.add_data(Data::Number(Number::U8(*a))), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_MODIFY_V_V_S(a, b)    => vex![0x30, 0x13 ; wrapper.add_data(Data::Name(a.clone())), wrapper.add_data(Data::Name(b.clone()))],
            Instruction::GENTYPE_MODIFY_S_V_S(a)       => vex![0x30, 0x14 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GENTYPE_MODIFY_C_S_S(a)       => vex![0x30, 0x15 ; wrapper.add_data(Data::Number(Number::U8(*a)))],
            Instruction::GENTYPE_MODIFY_V_S_S(a)       => vex![0x30, 0x16 ; wrapper.add_data(Data::Name(a.clone()))],
            Instruction::GENTYPE_MODIFY_S_S_S          => vec![0x30, 0x17],
        }
    }
}

#[macro_export]
macro_rules! number {
    (u8 $v:expr) => {
        {
            let v: u8 = $v;
            rainbow_wrapper::chunks::Number::U8(v)
        }
    };
    (u16 $v:expr) => {
        {
            let v: u16 = $v;
            rainbow_wrapper::chunks::Number::U16(v)
        }
    };
    (u32 $v:expr) => {
        {
            let v: u32 = $v;
            rainbow_wrapper::chunks::Number::U32(v)
        }
    };
    (u64 $v:expr) => {
        {
            let v: u64 = $v;
            rainbow_wrapper::chunks::Number::U64(v)
        }
    };
    (i8 $v:expr) => {
        {
            let v: i8 = $v;
            rainbow_wrapper::chunks::Number::I8(v)
        }
    };
    (i16 $v:expr) => {
        {
            let v: i16 = $v;
            rainbow_wrapper::chunks::Number::I16(v)
        }
    };
    (i32 $v:expr) => {
        {
            let v: i32 = $v;
            rainbow_wrapper::chunks::Number::I32(v)
        }
    };
    (i64 $v:expr) => {
        {
            let v: i64 = $v;
            rainbow_wrapper::chunks::Number::I64(v)
        }
    };
    (f8 $v:expr) => {
        {
            let v: u8 = $v;
            rainbow_wrapper::chunks::Number::F8(v)
        }
    };
    (f16 $v:expr) => {
        {
            let v: f16 = $v;
            rainbow_wrapper::chunks::Number::F16(v)
        }
    };
    (f32 $v:expr) => {
        {
            let v: f32 = $v;
            rainbow_wrapper::chunks::Number::F32(v)
        }
    };
    (f64 $v:expr) => {
        {
            let v: f64 = $v;
            rainbow_wrapper::chunks::Number::F64(v)
        }
    };
    ($type:expr, $v:expr) => {
        match $type {
            Type::U8  => rainbow_wrapper::chunks::Number::U8 ($v as u8 ),
            Type::U16 => rainbow_wrapper::chunks::Number::U16($v as u16),
            Type::U32 => rainbow_wrapper::chunks::Number::U32($v as u32),
            Type::U64 => rainbow_wrapper::chunks::Number::U64($v as u64),
            Type::I8  => rainbow_wrapper::chunks::Number::I8 ($v as i8 ),
            Type::I16 => rainbow_wrapper::chunks::Number::I16($v as i16),
            Type::I32 => rainbow_wrapper::chunks::Number::I32($v as i32),
            Type::I64 => rainbow_wrapper::chunks::Number::I64($v as i64),
            Type::F8  => rainbow_wrapper::chunks::Number::F8 ($v as u8 ),
            Type::F16 => rainbow_wrapper::chunks::Number::F16(half::f16::from_f32($v as f32)),
            Type::F32 => rainbow_wrapper::chunks::Number::F32($v as f32),
            Type::F64 => rainbow_wrapper::chunks::Number::F64($v as f64),
            _ => {
                fox::scritical!("expected number type for number constant, got {}", $type);
                std::process::exit(1);
            }
        }
    }
}

// TODO
#[macro_export]
macro_rules! rbtype {
    () => {
        
    };
}

// generated codeeeeeeee

/// Does nothing.
#[macro_export]
macro_rules! nop {
    () => {
        Instruction::NOP
    };
}

/// Moves A into B.
#[macro_export]
macro_rules! mov {
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::MOV_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::MOV_C_V(a, b)
    }};
}

/// Pushes A onto the stack.
/// Can cause stack overflow if the stack runs out of space, halting execution with an error.
#[macro_export]
macro_rules! push {
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::PUSH_V(a)
    }};
    ($t0:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::PUSH_C(a)
    }};
}

/// Pops a value off of the stack and puts it in A.
/// Expects types to match.
/// If the stack is empty execution will halt with an error.
#[macro_export]
macro_rules! pop {
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::POP(a)
    }};
}

/// Duplicates the element on the top of the stack.
/// If the stack is empty execution will halt with an error.
#[macro_export]
macro_rules! dup {
    () => {
        Instruction::DUP
    };
}

/// Swaps the top two elements on the stack.
/// If the stack is empty execution will halt with an error.
#[macro_export]
macro_rules! swap {
    () => {
        Instruction::SWAP
    };
}

/// Removes the element on the top of the stack.
/// If the stack is empty execution will halt with an error.
#[macro_export]
macro_rules! drop {
    () => {
        Instruction::DROP
    };
}

/// Allocates memory on the stack of type A with B elements and outputs to C.
/// Expects C to be a pointer to elements of type A.
/// Can cause stack overflow if the stack runs out of space and the allocation is too big, halting execution with an error.
#[macro_export]
macro_rules! buff {
    (pop, pop, push) => {{
        Instruction::BUFF_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::BUFF_V_S_S(a)
    }};
    (($a:expr), pop, push) => {{
        let a: Type = $a;
        Instruction::BUFF_T_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::BUFF_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::BUFF_V_V_S(a, b)
    }};
    (($a:expr), $b:expr, push) => {{
        let a: Type = $a;
        let b: String = $b.to_string(); // type checking
        Instruction::BUFF_T_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::BUFF_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::BUFF_V_C_S(a, b)
    }};
    (($a:expr), $t1:expr; push) => {{
        let a: Type = $a;
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::BUFF_T_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_V_S_V(a, c)
    }};
    (($a:expr), pop, $c:expr) => {{
        let a: Type = $a;
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_T_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_V_V_V(a, b, c)
    }};
    (($a:expr), $b:expr, $c:expr) => {{
        let a: Type = $a;
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_T_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_V_C_V(a, b, c)
    }};
    (($a:expr), $t1:expr; $c:expr) => {{
        let a: Type = $a;
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::BUFF_T_C_V(a, b, c)
    }};
}

/// Copies A to B with size C.
/// Size is in elements.
/// Expects A and B to both point to the same type.
#[macro_export]
macro_rules! memcopy {
    (pop, pop, push) => {{
        Instruction::MEMCOPY_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::MEMCOPY_V_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::MEMCOPY_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::MEMCOPY_V_V_S(a, b)
    }};
    (pop, pop, $t2:expr) => {{
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::MEMCOPY_S_S_C(c)
    }};
    ($a:expr, pop, $t2:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::MEMCOPY_V_S_C(a, c)
    }};
    (pop, $b:expr, $t2:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::MEMCOPY_S_V_C(b, c)
    }};
    ($a:expr, $b:expr, $t2:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::MEMCOPY_V_V_C(a, b, c)
    }};
}

/// Copies the value at A and outputs to B.
/// Expects B to have the type that A points to.
/// Expects A to be a pointer.
#[macro_export]
macro_rules! deref {
    (pop, push) => {{
        Instruction::DEREF_S_S
    }};
    ($a:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::DEREF_V_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::DEREF_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::DEREF_V_V(a, b)
    }};
}

/// Gets the address of A and outputs to B.
/// Expects B to be a pointer to elements of type A.
/// When [stack] is used it gets the current stack pointer.
#[macro_export]
macro_rules! r#ref {
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::REF_S_V(a)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::REF_V_V(a, b)
    }};
}

/// Moves A into the memory at B with offset C. (B[C] = A)
/// C is in offset of elements of the type that B points to, as opposed to bytes.
/// If you want to access something byte-wise cast B to a `void*` or `u8*`.
/// Expects A to have the type that B points to.
#[macro_export]
macro_rules! pmov {
    (pop, pop, pop) => {{
        Instruction::PMOV_S_S_S
    }};
    ($a:expr, pop, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::PMOV_V_S_S(a)
    }};
    ($t0:expr; pop, pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::PMOV_C_S_S(a)
    }};
    (pop, $b:expr, pop) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::PMOV_S_V_S(b)
    }};
    ($a:expr, $b:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::PMOV_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::PMOV_C_V_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::PMOV_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PMOV_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PMOV_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PMOV_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PMOV_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PMOV_C_V_V(a, b, c)
    }};
    (pop, pop, $t2:expr) => {{
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::PMOV_S_S_C(c)
    }};
    ($a:expr, pop, $t2:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::PMOV_V_S_C(a, c)
    }};
    ($t0:expr; pop, $t2:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::PMOV_C_S_C(a, c)
    }};
    (pop, $b:expr, $t2:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::PMOV_S_V_C(b, c)
    }};
    ($a:expr, $b:expr, $t2:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::PMOV_V_V_C(a, b, c)
    }};
    ($t0:expr; $b:expr, $t2:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::PMOV_C_V_C(a, b, c)
    }};
}

/// Gets a constant from the data section from chunk A at index B and outputs to C.
/// Expects C to have the type of the constant.
/// Different from 'MOV' as it can access any arbitrary value from the data section with the indices not known at parse time.
#[macro_export]
macro_rules! data {
    (pop, pop, push) => {{
        Instruction::DATA_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::DATA_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::DATA_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::DATA_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::DATA_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::DATA_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::DATA_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::DATA_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::DATA_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DATA_C_C_V(a, b, c)
    }};
}

/// Performs B + A and stores the output in C.
/// Expects all types to match.
#[macro_export]
macro_rules! add {
    (pop, pop, push) => {{
        Instruction::ADD_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::ADD_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::ADD_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::ADD_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::ADD_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::ADD_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::ADD_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::ADD_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::ADD_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::ADD_C_C_V(a, b, c)
    }};
}

/// Performs B - A and stores the output in C.
/// Expects all types to match.
#[macro_export]
macro_rules! sub {
    (pop, pop, push) => {{
        Instruction::SUB_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::SUB_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::SUB_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::SUB_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::SUB_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::SUB_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::SUB_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::SUB_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::SUB_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::SUB_C_C_V(a, b, c)
    }};
}

/// Performs B * A and stores the output in C.
/// Expects all types to match.
#[macro_export]
macro_rules! mul {
    (pop, pop, push) => {{
        Instruction::MUL_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::MUL_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::MUL_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::MUL_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::MUL_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::MUL_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::MUL_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::MUL_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::MUL_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MUL_C_C_V(a, b, c)
    }};
}

/// Performs B / A and stores the output in C.
/// Expects all types to match.
#[macro_export]
macro_rules! div {
    (pop, pop, push) => {{
        Instruction::DIV_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::DIV_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::DIV_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::DIV_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::DIV_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::DIV_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::DIV_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::DIV_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::DIV_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::DIV_C_C_V(a, b, c)
    }};
}

/// Performs B % A and stores the output in C.
/// Expects all types to match.
/// Modulo on floating point numbers performed to IEEE 754 standards.
#[macro_export]
macro_rules! r#mod {
    (pop, pop, push) => {{
        Instruction::MOD_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::MOD_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::MOD_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::MOD_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::MOD_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::MOD_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::MOD_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::MOD_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::MOD_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::MOD_C_C_V(a, b, c)
    }};
}

/// Performs B & A and stores the output in C.
/// Expects all types to be the same.
#[macro_export]
macro_rules! and {
    (pop, pop, push) => {{
        Instruction::AND_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::AND_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::AND_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::AND_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::AND_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::AND_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::AND_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::AND_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::AND_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::AND_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::AND_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::AND_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::AND_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::AND_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::AND_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::AND_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::AND_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::AND_C_C_V(a, b, c)
    }};
}

/// Performs B & A and stores the output in C.
/// Expects all types to be the same.
#[macro_export]
macro_rules! or {
    (pop, pop, push) => {{
        Instruction::OR_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::OR_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::OR_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::OR_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::OR_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::OR_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::OR_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::OR_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::OR_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::OR_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::OR_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::OR_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::OR_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::OR_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::OR_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::OR_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::OR_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::OR_C_C_V(a, b, c)
    }};
}

/// Performs ~A and stores the output in B.
/// Expects types to be the same.
#[macro_export]
macro_rules! not {
    (pop, push) => {{
        Instruction::NOT_S_S
    }};
    ($a:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::NOT_V_S(a)
    }};
    ($t0:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::NOT_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::NOT_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::NOT_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::NOT_C_V(a, b)
    }};
}

/// Performs B & A and stores the output in C.
/// Expects all types to be the same.
#[macro_export]
macro_rules! xor {
    (pop, pop, push) => {{
        Instruction::XOR_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::XOR_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::XOR_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::XOR_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::XOR_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::XOR_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::XOR_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::XOR_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::XOR_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::XOR_C_C_V(a, b, c)
    }};
}

/// Performs B & A and stores the output in C.
/// Expects all types to be the same.
#[macro_export]
macro_rules! lsh {
    (pop, pop, push) => {{
        Instruction::LSH_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::LSH_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::LSH_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::LSH_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::LSH_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::LSH_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::LSH_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::LSH_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::LSH_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::LSH_C_C_V(a, b, c)
    }};
}

/// Performs B & A and stores the output in C.
/// Expects all types to be the same.
#[macro_export]
macro_rules! rsh {
    (pop, pop, push) => {{
        Instruction::RSH_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::RSH_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::RSH_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::RSH_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::RSH_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::RSH_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::RSH_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::RSH_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::RSH_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::RSH_C_C_V(a, b, c)
    }};
}

/// Adds an element offset B to pointer A and outputs to C.
/// Expects B to be an integer, and for A and C to point to the same elements.
/// Expects pointer alignment to match.
#[macro_export]
macro_rules! padd {
    (pop, pop, push) => {{
        Instruction::PADD_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::PADD_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::PADD_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::PADD_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::PADD_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::PADD_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PADD_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PADD_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PADD_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PADD_C_C_V(a, b, c)
    }};
}

/// Subtracts an element offset B to pointer A and outputs to C.
/// Expects B to be an integer, and for A and C to point to the same elements.
/// Expects pointer alignment to match.
#[macro_export]
macro_rules! psub {
    (pop, pop, push) => {{
        Instruction::PSUB_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::PSUB_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::PSUB_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::PSUB_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::PSUB_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::PSUB_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PSUB_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PSUB_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PSUB_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PSUB_C_C_V(a, b, c)
    }};
}

/// Gets the element difference of pointer A from pointer B and outputs to C. (B - A)
/// Expects C to be an integer, and for A and C to point to the same elements.
/// Expects pointer alignment to match.
#[macro_export]
macro_rules! pdiff {
    (pop, pop, push) => {{
        Instruction::PDIFF_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::PDIFF_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::PDIFF_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::PDIFF_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::PDIFF_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::PDIFF_C_V_S(a, b)
    }};
    (pop, $t1:expr; push) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PDIFF_S_C_S(b)
    }};
    ($a:expr, $t1:expr; push) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PDIFF_V_C_S(a, b)
    }};
    ($t0:expr; $t1:expr; push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::PDIFF_C_C_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_C_V_V(a, b, c)
    }};
    (pop, $t1:expr; $c:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_S_C_V(b, c)
    }};
    ($a:expr, $t1:expr; $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_V_C_V(a, b, c)
    }};
    ($t0:expr; $t1:expr; $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        let c: String = $c.to_string(); // type checking
        Instruction::PDIFF_C_C_V(a, b, c)
    }};
}

/// Creates a variable with `type` A and `name` B.
/// Allocates onto the stack.
/// Variables are automatically dropped once the scope they are created in ends.
#[macro_export]
macro_rules! var {
    (pop, pop) => {{
        Instruction::VAR_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::VAR_V_S(a)
    }};
    (($a:expr), pop) => {{
        let a: Type = $a;
        Instruction::VAR_T_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::VAR_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::VAR_V_V(a, b)
    }};
    (($a:expr), $b:expr) => {{
        let a: Type = $a;
        let b: String = $b.to_string(); // type checking
        Instruction::VAR_T_V(a, b)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::VAR_S_N(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::VAR_V_N(a, b)
    }};
    (($a:expr), $b:expr) => {{
        let a: Type = $a;
        let b: String = $b.to_string(); // type checking
        Instruction::VAR_T_N(a, b)
    }};
}

/// Checks if variable with `name` A exists. Sets the EQ flag to 1 if it does exist and to 0 if it doesn't.
/// Useful for dynamically named variables.
#[macro_export]
macro_rules! varexists {
    (pop) => {{
        Instruction::VAREXISTS_S
    }};
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::VAREXISTS_V(a)
    }};
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::VAREXISTS_N(a)
    }};
}

/// TODO-- MANUAL
#[macro_export]
macro_rules! getfield {
    (pop, pop, push) => {{
        Instruction::GETFIELD_OFFSET_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_V_S_S(a)
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_N_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_V_V_S(a, b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_N_V_S(a, b)
    }};
    (pop, $b:expr, push) => {{
        let b: StructRef = $b; // type checking
        Instruction::GETFIELD_OFFSET_S_D_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        Instruction::GETFIELD_OFFSET_V_D_S(a, b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        Instruction::GETFIELD_OFFSET_N_D_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_V_S_V(a, c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_N_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_V_V_V(a, b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_N_V_V(a, b, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: StructRef = $b; // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_S_D_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_V_D_V(a, b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_OFFSET_N_D_V(a, b, c)
    }};
    (pop, pop, push) => {{
        Instruction::GETFIELD_VALUE_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::GETFIELD_VALUE_V_S_S(a)
    }};
    ($t0:expr; pop, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::GETFIELD_VALUE_C_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_VALUE_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_VALUE_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, push) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_VALUE_C_V_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_VALUE_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_VALUE_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_VALUE_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_VALUE_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_VALUE_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_VALUE_C_V_V(a, b, c)
    }};
    (pop, pop, push) => {{
        Instruction::GETFIELD_INDEX_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::GETFIELD_INDEX_V_S_S(a)
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::GETFIELD_INDEX_N_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_INDEX_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_INDEX_V_V_S(a, b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::GETFIELD_INDEX_N_V_S(a, b)
    }};
    (pop, $b:expr, push) => {{
        let b: StructRef = $b; // type checking
        Instruction::GETFIELD_INDEX_S_D_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        Instruction::GETFIELD_INDEX_V_D_S(a, b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        Instruction::GETFIELD_INDEX_N_D_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_V_S_V(a, c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_N_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_V_V_V(a, b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_N_V_V(a, b, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: StructRef = $b; // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_S_D_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_V_D_V(a, b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: StructRef = $b; // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::GETFIELD_INDEX_N_D_V(a, b, c)
    }};
}

/// TODO-- MANUAL
#[macro_export]
macro_rules! setfield {
    (pop, pop, pop) => {{
        Instruction::SETFIELD_S_S_S
    }};
    ($a:expr, pop, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::SETFIELD_V_S_S(a)
    }};
    ($t0:expr; pop, pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::SETFIELD_C_S_S(a)
    }};
    (pop, $b:expr, pop) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::SETFIELD_S_V_S(b)
    }};
    ($a:expr, $b:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::SETFIELD_V_V_S(a, b)
    }};
    ($t0:expr; $b:expr, pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::SETFIELD_C_V_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::SETFIELD_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SETFIELD_V_S_V(a, c)
    }};
    ($t0:expr; pop, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c: String = $c.to_string(); // type checking
        Instruction::SETFIELD_C_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SETFIELD_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SETFIELD_V_V_V(a, b, c)
    }};
    ($t0:expr; $b:expr, $c:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::SETFIELD_C_V_V(a, b, c)
    }};
    (pop, pop, $t2:expr) => {{
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::SETFIELD_S_S_C(c)
    }};
    ($a:expr, pop, $t2:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::SETFIELD_V_S_C(a, c)
    }};
    ($t0:expr; pop, $t2:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::SETFIELD_C_S_C(a, c)
    }};
    (pop, $b:expr, $t2:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::SETFIELD_S_V_C(b, c)
    }};
    ($a:expr, $b:expr, $t2:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::SETFIELD_V_V_C(a, b, c)
    }};
    ($t0:expr; $b:expr, $t2:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        let c = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t2.0, $t2.1));
        Instruction::SETFIELD_C_V_C(a, b, c)
    }};
}

/// Calls function A.
/// If a variable is passed, it expects it to be of type `funcref`.
/// Externs are treated like normal functions, however the `funcref` must have the `extern` flag set.
/// All arguments are passed through the stack.
#[macro_export]
macro_rules! call {
    (pop) => {{
        Instruction::CALL_S
    }};
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::CALL_V(a)
    }};
    ($a:expr) => {{
        let a: FuncRef = $a; // type checking
        Instruction::CALL_F(a)
    }};
}

/// Calls syscall with id A.
/// Arguments are fetched from the stack.
#[macro_export]
macro_rules! syscall {
    (pop) => {{
        Instruction::SYSCALL_S
    }};
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::SYSCALL_V(a)
    }};
    ($t0:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::SYSCALL_C(a)
    }};
}

/// Returns value A, pushing it on to the stack of the function caller.
/// If not inside of a function, return the value to the host.
/// Expects A to match the return type of the parent function.
#[macro_export]
macro_rules! ret {
    (pop) => {{
        Instruction::RET_S
    }};
    ($a:expr) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::RET_V(a)
    }};
    ($t0:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::RET_C(a)
    }};
}

/// Jumps to instruction B in code block A.
/// Expects integer types.
#[macro_export]
macro_rules! jmp {
    (pop) => {{
        Instruction::JMP_S
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::JMP_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::JMP_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::JMP_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JMP_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JMP_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JMP_C_C(a, b)
    }};
}

/// Compares A with B and sets flags accordingly.
/// Expects types to match and be numeral.
/// Comparisons for floating point numbers are done according to IEEE 754.
#[macro_export]
macro_rules! cmp {
    (pop, pop) => {{
        Instruction::CMP_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::CMP_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::CMP_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::CMP_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::CMP_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::CMP_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::CMP_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::CMP_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::CMP_C_C(a, b)
    }};
}

/// Clears all flags set by CMP.
#[macro_export]
macro_rules! clr {
    () => {
        Instruction::CLR
    };
}

/// Jumps to instruction B in code block A if the last CMP instruction resulted in EQ.
/// Expects integer types.
#[macro_export]
macro_rules! je {
    (pop, pop) => {{
        Instruction::JE_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::JE_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::JE_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::JE_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::JE_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::JE_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JE_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JE_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JE_C_C(a, b)
    }};
}

/// Jumps to instruction B in code block A if the last CMP instruction resulted in NE.
/// Expects integer types.
#[macro_export]
macro_rules! jne {
    (pop, pop) => {{
        Instruction::JNE_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::JNE_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::JNE_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::JNE_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::JNE_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::JNE_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JNE_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JNE_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JNE_C_C(a, b)
    }};
}

/// Jumps to instruction B in code block A if the last CMP instruction resulted in LT.
/// Expects integer types.
#[macro_export]
macro_rules! jl {
    (pop, pop) => {{
        Instruction::JL_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::JL_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::JL_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::JL_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::JL_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::JL_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JL_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JL_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JL_C_C(a, b)
    }};
}

/// Jumps to instruction B in code block A if the last CMP instruction resulted in LE.
/// Expects integer types.
#[macro_export]
macro_rules! jle {
    (pop, pop) => {{
        Instruction::JLE_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::JLE_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::JLE_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::JLE_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::JLE_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::JLE_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JLE_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JLE_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JLE_C_C(a, b)
    }};
}

/// Jumps to instruction B in code block A if the last CMP instruction resulted in GT.
/// Expects integer types.
#[macro_export]
macro_rules! jg {
    (pop, pop) => {{
        Instruction::JG_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::JG_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::JG_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::JG_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::JG_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::JG_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JG_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JG_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JG_C_C(a, b)
    }};
}

/// Jumps to instruction B in code block A if the last CMP instruction resulted in GE.
/// Expects integer types.
#[macro_export]
macro_rules! jge {
    (pop, pop) => {{
        Instruction::JGE_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::JGE_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::JGE_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::JGE_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::JGE_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::JGE_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JGE_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JGE_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::JGE_C_C(a, b)
    }};
}

/// Checks if A is 0 or 1. If A is 0, execution is halted and message B is displayed. If A is 1, execution continues normally.
/// B is a constant containing an index to the data section containing UTF-8 encoded text.
#[macro_export]
macro_rules! assert {
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::ASSERT_P_C(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::ASSERT_V_C(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::ASSERT_C_C(a, b)
    }};
}

/// Casts B into type A and outputs into C.
/// Expects C to have type A.
/// Casts based on the Type Cast Table defined above.
/// [var] and [pop] for type A expect a variable containing a `type` value.
/// If the attempted cast is not present in the type casting table execution will halt with an error.
#[macro_export]
macro_rules! cast {
    (pop, pop, push) => {{
        Instruction::CAST_S_S_S
    }};
    ($a:expr, pop, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::CAST_V_S_S(a)
    }};
    (($a:expr), pop, push) => {{
        let a: Type = $a;
        Instruction::CAST_T_S_S(a)
    }};
    (pop, $b:expr, push) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::CAST_S_V_S(b)
    }};
    ($a:expr, $b:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::CAST_V_V_S(a, b)
    }};
    (($a:expr), $b:expr, push) => {{
        let a: Type = $a;
        let b: String = $b.to_string(); // type checking
        Instruction::CAST_T_V_S(a, b)
    }};
    (pop, pop, $c:expr) => {{
        let c: String = $c.to_string(); // type checking
        Instruction::CAST_S_S_V(c)
    }};
    ($a:expr, pop, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::CAST_V_S_V(a, c)
    }};
    (($a:expr), pop, $c:expr) => {{
        let a: Type = $a;
        let c: String = $c.to_string(); // type checking
        Instruction::CAST_T_S_V(a, c)
    }};
    (pop, $b:expr, $c:expr) => {{
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::CAST_S_V_V(b, c)
    }};
    ($a:expr, $b:expr, $c:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::CAST_V_V_V(a, b, c)
    }};
    (($a:expr), $b:expr, $c:expr) => {{
        let a: Type = $a;
        let b: String = $b.to_string(); // type checking
        let c: String = $c.to_string(); // type checking
        Instruction::CAST_T_V_V(a, b, c)
    }};
}

/// Gets the type of A and outputs to B.
/// Expects B to have the `type` type.
#[macro_export]
macro_rules! r#typeof {
    (pop, push) => {{
        Instruction::TYPEOF_S_S
    }};
    ($a:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::TYPEOF_V_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::TYPEOF_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::TYPEOF_V_V(a, b)
    }};
}

/// TODO-- MANUAL
#[macro_export]
macro_rules! typecmp {
    (pop, pop) => {{
        Instruction::TYPECMP_LOOSE_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::TYPECMP_LOOSE_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::TYPECMP_LOOSE_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_LOOSE_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_LOOSE_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_LOOSE_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_LOOSE_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_LOOSE_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_LOOSE_C_C(a, b)
    }};
    (pop, pop) => {{
        Instruction::TYPECMP_STRUCT_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::TYPECMP_STRUCT_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::TYPECMP_STRUCT_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_STRUCT_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_STRUCT_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_STRUCT_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_STRUCT_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_STRUCT_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_STRUCT_C_C(a, b)
    }};
    (pop, pop) => {{
        Instruction::TYPECMP_STRICT_S_S
    }};
    ($a:expr, pop) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::TYPECMP_STRICT_V_S(a)
    }};
    ($t0:expr; pop) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        Instruction::TYPECMP_STRICT_C_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_STRICT_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_STRICT_V_V(a, b)
    }};
    ($t0:expr; $b:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b: String = $b.to_string(); // type checking
        Instruction::TYPECMP_STRICT_C_V(a, b)
    }};
    (pop, $t1:expr) => {{
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_STRICT_S_C(b)
    }};
    ($a:expr, $t1:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_STRICT_V_C(a, b)
    }};
    ($t0:expr; $t1:expr) => {{
        let a = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t0.0, $t0.1));
        let b = rainbow_wrapper::chunks::Data::Number(rainbow_wrapper::number!($t1.0, $t1.1));
        Instruction::TYPECMP_STRICT_C_C(a, b)
    }};
}

/// TODO-- MANUAL
#[macro_export]
macro_rules! sizeof {
    (pop, push) => {{
        Instruction::SIZEOF_VAR_S_S
    }};
    ($a:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::SIZEOF_VAR_V_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::SIZEOF_VAR_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::SIZEOF_VAR_V_V(a, b)
    }};
    (pop, push) => {{
        Instruction::SIZEOF_TYPE_S_S
    }};
    ($a:expr, push) => {{
        let a: String = $a.to_string(); // type checking
        Instruction::SIZEOF_TYPE_V_S(a)
    }};
    (($a:expr), push) => {{
        let a: Type = $a;
        Instruction::SIZEOF_TYPE_T_S(a)
    }};
    (pop, $b:expr) => {{
        let b: String = $b.to_string(); // type checking
        Instruction::SIZEOF_TYPE_S_V(b)
    }};
    ($a:expr, $b:expr) => {{
        let a: String = $a.to_string(); // type checking
        let b: String = $b.to_string(); // type checking
        Instruction::SIZEOF_TYPE_V_V(a, b)
    }};
    (($a:expr), $b:expr) => {{
        let a: Type = $a;
        let b: String = $b.to_string(); // type checking
        Instruction::SIZEOF_TYPE_T_V(a, b)
    }};
}
