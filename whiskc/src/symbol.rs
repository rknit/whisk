#![allow(dead_code)]

#[derive(Debug, Default)]
pub struct SymbolTable<'table> {
    globals: Vec<GlobalSymbol>,
    locals: Vec<LocalSymbol<'table>>,
}
impl<'table> SymbolTable<'table> {
    /// Add the function to the global symbol table, returning its reference if there is no collision.
    /// None is returned if there is a function with the same name presented in the global symbol
    /// table.
    pub fn new_function(&'table mut self, name: String, arity: usize) -> Option<&mut Function> {
        if self
            .globals
            .iter()
            .any(|v| matches!(v, GlobalSymbol::Function(f) if f.get_name() == name))
        {
            return None;
        }

        self.globals.push(Function::new(name, arity).into());

        let GlobalSymbol::Function(func) = self.globals.last_mut().unwrap();
        Some(func)
    }

    pub fn new_block(&'table mut self, parent_func: &'table Function) -> &mut Block {
        self.locals.push(LocalSymbol::Block(Block::new(
            self.locals.len(),
            parent_func,
        )));
        let LocalSymbol::Block(block) = self.locals.last_mut().unwrap() else {
            unreachable!();
        };
        block
    }

    /// Add the variable to the local symbol table, returning its reference if there is no collision.
    /// None is returned if there is a variable with the same name presented in the same block.
    pub fn new_variable(
        &'table mut self,
        name: String,
        parent_block: &'table Block,
    ) -> Option<&mut Variable> {
        if self.locals.iter().any(|v| {
            matches!(v, LocalSymbol::Variable(v)
                if v.get_block().get_index() == parent_block.get_index() && v.get_name() == name
            )
        }) {
            return None;
        }
        self.locals
            .push(LocalSymbol::Variable(Variable::new(name, parent_block)));
        let LocalSymbol::Variable(var) = self.locals.last_mut().unwrap() else {
            unreachable!();
        };
        Some(var)
    }
}

#[derive(Debug)]
enum GlobalSymbol {
    Function(Function),
}
impl From<Function> for GlobalSymbol {
    fn from(value: Function) -> Self {
        Self::Function(value)
    }
}

#[derive(Debug)]
enum LocalSymbol<'table> {
    Block(Block<'table>),
    Variable(Variable<'table>),
}

#[derive(Debug)]
pub struct Function {
    name: String,
    params: Vec<Param>,
}
impl Function {
    fn new(name: String, arity: usize) -> Self {
        Self {
            name,
            params: vec![Param::default(); arity],
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_param_name(&mut self, index: usize, name: String) -> Option<&mut Self> {
        self.params.get_mut(index)?.name = name;
        Some(self)
    }

    pub fn get_param(&self, index: usize) -> Option<&Param> {
        self.params.get(index)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Param {
    pub name: String,
}

#[derive(Debug)]
pub struct Block<'table> {
    index: usize, // added an index field to identify blocks
    func: &'table Function,
    parent_block: Option<&'table Block<'table>>,
}
impl<'table> Block<'table> {
    fn new(index: usize, parent_func: &'table Function) -> Self {
        Self {
            index,
            func: parent_func,
            parent_block: None,
        }
    }

    pub fn set_parent_block(&mut self, block: &'table Block) -> &mut Self {
        assert!(
            self.get_index() != block.get_index(),
            "cannot assign the block itself as its parent block"
        );
        self.parent_block = Some(block);
        self
    }

    pub fn get_function(&self) -> &Function {
        self.func
    }

    fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Debug)]
pub struct Variable<'table> {
    block: &'table Block<'table>,
    name: String,
}
impl<'table> Variable<'table> {
    fn new(name: String, parent_block: &'table Block) -> Self {
        Self {
            block: parent_block,
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_block(&self) -> &Block {
        self.block
    }
}
