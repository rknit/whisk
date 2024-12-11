use crate::{
    ast::parsing::token::Operator,
    ast_resolved::nodes::expr::{BinaryExpr, CallExpr, Expr, ExprKind, UnaryExpr},
    cfg::{
        builder::{BuildContext, BuildVisitor, Builder},
        nodes::{
            inst::{
                AddInst, AndInst, CallInst, CompareCond, CompareInst, Inst, InstKind, NegateInst,
                NotInst, OrInst, SubInst,
            },
            value::{ConstantValue, InstValue, Value, ValueKind},
        },
    },
    symbol_table::Symbol,
    ty::Type,
};

impl BuildVisitor<Value> for Expr {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder) -> Value {
        match &self.kind {
            ExprKind::Integer(expr) => expr.visit(ctx, builder, self.ty),
            ExprKind::Bool(expr) => expr.visit(ctx, builder, self.ty),
            ExprKind::Identifier(expr) => expr.visit(ctx, builder, self.ty),
            ExprKind::Unary(expr) => expr.visit(ctx, builder, self.ty),
            ExprKind::Binary(expr) => expr.visit(ctx, builder, self.ty),
            ExprKind::Call(expr) => expr.visit(ctx, builder, self.ty),
        }
    }
}

pub(super) trait BuildExprVisitor {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder, ty: Type) -> Value;
}

impl BuildExprVisitor for i64 {
    fn visit(&self, _ctx: &mut BuildContext, _builder: &mut Builder, ty: Type) -> Value {
        Value {
            kind: ValueKind::Constant(ConstantValue::Integer(*self)),
            ty,
        }
    }
}

impl BuildExprVisitor for bool {
    fn visit(&self, _ctx: &mut BuildContext, _builder: &mut Builder, ty: Type) -> Value {
        Value {
            kind: ValueKind::Constant(ConstantValue::Bool(*self)),
            ty,
        }
    }
}

impl BuildExprVisitor for String {
    fn visit(&self, ctx: &mut BuildContext, _builder: &mut Builder, _ty: Type) -> Value {
        match ctx.get_symbol_by_name(&self).unwrap() {
            Symbol::Variable(symbol) => symbol.get_value().expect("evaluated value"),
            Symbol::Function(symbol) => Value {
                kind: ValueKind::Function(symbol.get_id()),
                ty: symbol.get_type(),
            },
        }
    }
}

impl BuildExprVisitor for UnaryExpr {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder, ty: Type) -> Value {
        let value = self.expr.visit(ctx, builder);

        let inst = match self.op {
            Operator::Sub => Inst {
                kind: InstKind::Negate(NegateInst { value }),
                ty,
            },
            Operator::Not => Inst {
                kind: InstKind::Not(NotInst { value }),
                ty,
            },
            _ => unimplemented!("{:?}", self.op),
        };

        let inst = builder.push_inst(inst);
        Value {
            kind: ValueKind::Inst(InstValue {
                bb: *builder.get_current_block_id(),
                inst,
            }),
            ty,
        }
    }
}

impl BuildExprVisitor for BinaryExpr {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder, ty: Type) -> Value {
        let lhs = self.left.visit(ctx, builder);
        let rhs = self.right.visit(ctx, builder);

        let inst = Inst {
            kind: match self.op {
                Operator::Add => InstKind::Add(AddInst { lhs, rhs }),
                Operator::Sub => InstKind::Sub(SubInst { lhs, rhs }),
                Operator::And => InstKind::And(AndInst { lhs, rhs }),
                Operator::Or => InstKind::Or(OrInst { lhs, rhs }),
                Operator::Equal => InstKind::Compare(CompareInst {
                    cond: CompareCond::Equal,
                    lhs,
                    rhs,
                }),
                Operator::NotEqual => InstKind::Compare(CompareInst {
                    cond: CompareCond::NotEqual,
                    lhs,
                    rhs,
                }),
                Operator::Less => InstKind::Compare(CompareInst {
                    cond: CompareCond::Less,
                    lhs,
                    rhs,
                }),
                Operator::LessEqual => InstKind::Compare(CompareInst {
                    cond: CompareCond::LessEqual,
                    lhs,
                    rhs,
                }),
                Operator::Greater => InstKind::Compare(CompareInst {
                    cond: CompareCond::Greater,
                    lhs,
                    rhs,
                }),
                Operator::GreaterEqual => InstKind::Compare(CompareInst {
                    cond: CompareCond::GreaterEqual,
                    lhs,
                    rhs,
                }),
                _ => unimplemented!("{:?}", self.op),
            },
            ty,
        };

        let inst = builder.push_inst(inst);
        Value {
            kind: ValueKind::Inst(InstValue {
                bb: *builder.get_current_block_id(),
                inst,
            }),
            ty,
        }
    }
}

impl BuildExprVisitor for CallExpr {
    fn visit(&self, ctx: &mut BuildContext, builder: &mut Builder, ty: Type) -> Value {
        let callee = self.callee.visit(ctx, builder);

        let args = self
            .args
            .iter()
            .map(|v| v.visit(ctx, builder))
            .collect::<Vec<_>>();

        let inst = builder.push_inst(Inst {
            kind: InstKind::Call(CallInst { callee, args }),
            ty,
        });

        Value {
            kind: ValueKind::Inst(InstValue {
                bb: *builder.get_current_block_id(),
                inst,
            }),
            ty,
        }
    }
}
