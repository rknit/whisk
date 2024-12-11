use crate::{
    ast::{
        location::{Located, LocationRange},
        parsing::token::Operator,
    },
    symbol_table::{SymbolAttribute, SymbolKind},
    ty::Type,
};

#[derive(Debug, Clone)]
pub enum ResolveError {
    LiteralResolveError(ValueResolveError),
    IdentResolveError(IdentResolveError),
    TypeResolveError(TypeResolveError),
    ControlFlowError(ControlFlowError),
}

#[derive(Debug, Clone)]
pub enum ControlFlowError {
    NotAllFuncPathReturned(Located<String>),
}
impl Into<ResolveError> for ControlFlowError {
    fn into(self) -> ResolveError {
        ResolveError::ControlFlowError(self)
    }
}

#[derive(Debug, Clone)]
pub enum ValueResolveError {
    IntegerOutOfRange {
        is_signed: bool,
        int_size: u16,
        val: Located<i64>,
    },
    ArgumentCountMismatch {
        func_ty: Located<Type>,
        expect_count: usize,
        actual_count: usize,
    },
    ArrayLengthMismatch {
        loc: LocationRange,
        expect_count: usize,
        actual_count: usize,
    },
}
impl Into<ResolveError> for ValueResolveError {
    fn into(self) -> ResolveError {
        ResolveError::LiteralResolveError(self)
    }
}

#[derive(Debug, Clone)]
pub enum IdentResolveError {
    GlobalIdentAlreadyUsed {
        ident: String,
        first_origin: (SymbolKind, LocationRange),
        dup_origin: (SymbolKind, LocationRange),
    },
    VarNameAlreadyUsed {
        ident: String,
        first_origin: (Type, LocationRange),
        dup_origin: (Type, LocationRange),
    },
    UnknownIdentifier(Located<String>),
    UnexpectedAttrib {
        attribute: Located<SymbolAttribute>,
        allowed_attributes: Vec<SymbolAttribute>,
    },
}
impl Into<ResolveError> for IdentResolveError {
    fn into(self) -> ResolveError {
        ResolveError::IdentResolveError(self)
    }
}

#[derive(Debug, Clone)]
pub enum TypeResolveError {
    ReturnTypeMismatch {
        function_name: String,
        expected_type: Type,
        actual_type: Located<Type>,
    },
    NonBoolInIfCond(Located<Type>),
    NonAssignableType(Located<Type>),
    AssignmentTypeMismatch {
        target_ty: Located<Type>,
        value_ty: Located<Type>,
    },
    UnknownTypeForIdent(Located<String>),
    NonBoolUsedInNotOp(Located<Type>),
    NonNumericInUnaryOp(Operator, Located<Type>),
    UnsignedIntegerInUnaryOp(Located<Operator>),
    NonNumericTypeInBinaryOp {
        op: Located<Operator>,
        ty: Located<Type>,
    },
    UnorderedTypeInBinaryOp {
        op: Located<Operator>,
        ty: Located<Type>,
    },
    UnexpectedTypeInBinaryOp {
        op: Located<Operator>,
        expect_type: Type,
        actual_type: Located<Type>,
    },
    TypeMismatchInBinaryOp {
        op: Located<Operator>,
        left_ty: Type,
        right_ty: Type,
    },
    CallOnNonFunctionType(Located<Type>),
    ArgumentTypeMismatch {
        func_ty: Located<Type>,
        argument_index: usize,
        expect_type: Type,
        actual_type: Located<Type>,
    },
    ArrayElementTypeMismatch {
        element_index: usize,
        expect_type: Located<Type>,
        actual_type: Located<Type>,
    },
    IndexingOnNonArrayType(Located<Type>),
    ExpectUnsignedIntOnArrayIndex {
        arr_ty: Located<Type>,
        index_ty: Located<Type>,
    },
    InvalidTypeCast {
        loc: LocationRange,
        from_ty: Type,
        to_ty: Type,
    },
}
impl Into<ResolveError> for TypeResolveError {
    fn into(self) -> ResolveError {
        ResolveError::TypeResolveError(self)
    }
}
